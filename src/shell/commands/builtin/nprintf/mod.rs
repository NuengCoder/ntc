//! `nprintf` — bash-style formatted output for the ntc shell.
//!
//!   nprintf FORMAT [ARGUMENT...]
//!
//! The format is interpreted like bash's `printf`: `%s`, `%d`, `%i`, `%u`,
//! `%o`, `%x`, `%X`, `%c`, `%b`, `%q`, the floating-point conversions and
//! `%%`, each with the usual flags (`-+ #0`), width and precision. Backslash
//! escapes are interpreted in the format (and in a `%b` argument), `\c` stops
//! output immediately, and the format is reused while arguments remain.
//! Nothing is appended automatically, so `nprintf` composes in pipelines.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

/// Conversion flags parsed between `%` and the conversion character.
#[derive(Clone, Copy, Default)]
struct Flags {
    left: bool,
    plus: bool,
    space: bool,
    alt: bool,
    zero: bool,
}

#[derive(Clone, Copy)]
struct Spec {
    flags: Flags,
    width: Option<usize>,
    precision: Option<usize>,
    conv: char,
}

/// Quote-aware argument split that keeps empty quoted arguments (`""`) and
/// leaves backslash escapes alone, so `nprintf "%s\n" ""` keeps both tokens.
fn tokenize(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut started = false;
    let mut quote: Option<char> = None;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                } else if ch == '\\' && q == '"' {
                    match chars.peek().copied() {
                        Some(next) if next == '"' || next == '\\' => {
                            current.push(next);
                            chars.next();
                        }
                        _ => current.push('\\'),
                    }
                } else {
                    current.push(ch);
                }
            }
            None => match ch {
                '"' | '\'' => {
                    quote = Some(ch);
                    started = true;
                }
                c if c.is_whitespace() => {
                    if started {
                        out.push(std::mem::take(&mut current));
                        started = false;
                    }
                }
                c => {
                    current.push(c);
                    started = true;
                }
            },
        }
    }
    if started {
        out.push(current);
    }
    out
}

/// Parse one argument the way bash printf reads a number: `0x` hex, a leading
/// zero means octal, `'c` / `"c` is the character code, and a bad value is
/// reported instead of silently becoming 0.
fn parse_int(raw: &str) -> Option<i128> {
    let text = raw.trim();
    if text.is_empty() {
        return Some(0);
    }

    let mut chars = text.chars();
    let first = chars.next().unwrap();
    if first == '\'' || first == '"' {
        return chars.next().map(|c| c as i128);
    }

    let (negative, rest) = match first {
        '-' => (true, &text[first.len_utf8()..]),
        '+' => (false, &text[first.len_utf8()..]),
        _ => (false, text),
    };
    if rest.is_empty() {
        return None;
    }

    let (radix, digits) =
        if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
            (16, hex)
        } else if rest.len() > 1 && rest.starts_with('0') {
            (8, &rest[1..])
        } else {
            (10, rest)
        };
    if digits.is_empty() {
        return Some(0);
    }

    let value = i128::from_str_radix(digits, radix).ok()?;
    Some(if negative { -value } else { value })
}

/// Expand the escapes bash printf understands. The bool is true when `\c`
/// stopped processing.
fn expand_escapes(input: &str) -> (String, bool) {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '\\' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        if push_escape(&chars, &mut i, &mut out) {
            return (out, true);
        }
    }
    (out, false)
}

/// Interpret the escape at `i` (which points at the backslash) and advance.
/// Returns true for `\c`, which ends all output.
fn push_escape(chars: &[char], i: &mut usize, out: &mut String) -> bool {
    *i += 1;
    if *i >= chars.len() {
        out.push('\\');
        return false;
    }

    let c = chars[*i];
    *i += 1;
    match c {
        'a' => out.push('\u{07}'),
        'b' => out.push('\u{08}'),
        'e' | 'E' => out.push('\u{1b}'),
        'f' => out.push('\u{0c}'),
        'n' => out.push('\n'),
        'r' => out.push('\r'),
        't' => out.push('\t'),
        'v' => out.push('\u{0b}'),
        '\\' => out.push('\\'),
        '"' => out.push('"'),
        '\'' => out.push('\''),
        'c' => return true,
        'x' => {
            let (value, digits) = take_hex(chars, i, 2);
            if digits == 0 {
                out.push('\\');
                out.push('x');
            } else {
                out.push(char::from_u32(value).unwrap_or('\u{fffd}'));
            }
        }
        'u' | 'U' => {
            let max = if c == 'u' { 4 } else { 8 };
            let (value, digits) = take_hex(chars, i, max);
            if digits == 0 {
                out.push('\\');
                out.push(c);
            } else {
                out.push(char::from_u32(value).unwrap_or('\u{fffd}'));
            }
        }
        '0'..='7' => {
            let mut value = c.to_digit(8).unwrap();
            let mut used = 1;
            // `\0NNN` takes three digits after the 0; `\NNN` takes three total.
            let max = if c == '0' { 4 } else { 3 };
            while used < max && *i < chars.len() {
                match chars[*i].to_digit(8) {
                    Some(digit) => {
                        value = value * 8 + digit;
                        *i += 1;
                        used += 1;
                    }
                    None => break,
                }
            }
            if let Some(ch) = char::from_u32(value & 0xff) {
                out.push(ch);
            }
        }
        other => {
            out.push('\\');
            out.push(other);
        }
    }
    false
}

fn take_hex(chars: &[char], i: &mut usize, max: usize) -> (u32, usize) {
    let mut value = 0u32;
    let mut used = 0;
    while used < max && *i < chars.len() {
        match chars[*i].to_digit(16) {
            Some(digit) => {
                value = value * 16 + digit;
                *i += 1;
                used += 1;
            }
            None => break,
        }
    }
    (value, used)
}

/// `%q`: single-quote the value, or return a `$'...'` form when it holds
/// control characters. Safe words are left bare, like bash.
fn quote_shell(text: &str) -> String {
    if text.is_empty() {
        return "''".into();
    }
    if text
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "_@%+=:,./-".contains(c))
    {
        return text.to_string();
    }

    if text.chars().any(|c| c.is_control()) {
        let mut out = String::from("$'");
        for c in text.chars() {
            match c {
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                '\r' => out.push_str("\\r"),
                '\\' => out.push_str("\\\\"),
                '\'' => out.push_str("\\'"),
                c if c.is_control() => out.push_str(&format!("\\x{:02x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('\'');
        return out;
    }

    let mut out = String::from("'");
    for c in text.chars() {
        if c == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(c);
        }
    }
    out.push('\'');
    out
}

fn pad(text: &str, flags: Flags, width: Option<usize>) -> String {
    let width = width.unwrap_or(0);
    let len = text.chars().count();
    if len >= width {
        return text.to_string();
    }
    let fill = " ".repeat(width - len);
    if flags.left {
        format!("{text}{fill}")
    } else {
        format!("{fill}{text}")
    }
}

fn pad_number(
    sign: &str,
    prefix: &str,
    digits: &str,
    flags: Flags,
    width: Option<usize>,
    allow_zero: bool,
) -> String {
    let width = width.unwrap_or(0);
    let len = sign.chars().count() + prefix.chars().count() + digits.chars().count();
    if len >= width {
        return format!("{sign}{prefix}{digits}");
    }
    let fill = width - len;
    if flags.left {
        format!("{sign}{prefix}{digits}{}", " ".repeat(fill))
    } else if flags.zero && allow_zero {
        format!("{sign}{prefix}{}{digits}", "0".repeat(fill))
    } else {
        format!("{}{sign}{prefix}{digits}", " ".repeat(fill))
    }
}

fn take_arg<'a>(args: &'a [String], index: &mut usize) -> Option<&'a str> {
    if *index < args.len() {
        let value = args[*index].as_str();
        *index += 1;
        Some(value)
    } else {
        None
    }
}

fn take_int_arg(args: &[String], index: &mut usize) -> i128 {
    take_arg(args, index)
        .map(|raw| parse_int(raw).unwrap_or(0))
        .unwrap_or(0)
}

/// Parse one conversion. `i` points just past the `%`; on success it is left
/// just past the conversion character.
fn parse_spec(
    chars: &[char],
    i: &mut usize,
    args: &[String],
    arg_index: &mut usize,
) -> Option<Spec> {
    let mut flags = Flags::default();
    while *i < chars.len() {
        match chars[*i] {
            '-' => flags.left = true,
            '+' => flags.plus = true,
            ' ' => flags.space = true,
            '#' => flags.alt = true,
            '0' => flags.zero = true,
            '\'' => {} // grouping flag: accepted, not applied
            _ => break,
        }
        *i += 1;
    }

    let width = if *i < chars.len() && chars[*i] == '*' {
        *i += 1;
        let value = take_int_arg(args, arg_index);
        if value < 0 {
            flags.left = true;
            Some(value.unsigned_abs() as usize)
        } else {
            Some(value as usize)
        }
    } else {
        let start = *i;
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            *i += 1;
        }
        if *i > start {
            chars[start..*i]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .ok()
        } else {
            None
        }
    };

    let precision = if *i < chars.len() && chars[*i] == '.' {
        *i += 1;
        if *i < chars.len() && chars[*i] == '*' {
            *i += 1;
            let value = take_int_arg(args, arg_index);
            if value < 0 {
                None
            } else {
                Some(value as usize)
            }
        } else {
            let start = *i;
            while *i < chars.len() && chars[*i].is_ascii_digit() {
                *i += 1;
            }
            if *i > start {
                chars[start..*i]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .ok()
            } else {
                Some(0)
            }
        }
    } else {
        None
    };

    while *i < chars.len() && matches!(chars[*i], 'h' | 'l' | 'L' | 'j' | 'z' | 't') {
        *i += 1;
    }
    if *i >= chars.len() {
        return None;
    }
    let conv = chars[*i];
    *i += 1;
    if !"diouxXeEfFgGaAcsbq%".contains(conv) {
        return None;
    }

    Some(Spec {
        flags,
        width,
        precision,
        conv,
    })
}

fn render_int(spec: Spec, arg: Option<&str>, errors: &mut Vec<String>) -> String {
    let raw = arg.unwrap_or("");
    let value = if raw.is_empty() {
        0
    } else {
        match parse_int(raw) {
            Some(value) => value,
            None => {
                errors.push(raw.to_string());
                0
            }
        }
    };

    let mut prefix = String::new();
    let (negative, mut digits) = match spec.conv {
        'd' | 'i' => (value < 0, value.unsigned_abs().to_string()),
        'u' => (false, (value as u64).to_string()),
        'o' => {
            let digits = format!("{:o}", value as u64);
            if spec.flags.alt && !digits.starts_with('0') {
                prefix.push('0');
            }
            (false, digits)
        }
        'x' | 'X' => {
            if spec.flags.alt && value != 0 {
                prefix.push_str(if spec.conv == 'x' { "0x" } else { "0X" });
            }
            let digits = if spec.conv == 'x' {
                format!("{:x}", value as u64)
            } else {
                format!("{:X}", value as u64)
            };
            (false, digits)
        }
        _ => (false, String::new()),
    };

    if let Some(precision) = spec.precision {
        if digits.len() < precision {
            digits = format!("{}{digits}", "0".repeat(precision - digits.len()));
        }
        if precision == 0 && value == 0 {
            digits.clear();
        }
    }
    if spec.flags.alt && spec.conv == 'o' && digits.is_empty() {
        digits.push('0');
    }

    let signed = matches!(spec.conv, 'd' | 'i');
    let sign = if negative {
        "-"
    } else if signed && spec.flags.plus {
        "+"
    } else if signed && spec.flags.space {
        " "
    } else {
        ""
    };
    pad_number(
        sign,
        &prefix,
        &digits,
        spec.flags,
        spec.width,
        spec.precision.is_none(),
    )
}

fn fmt_e(value: f64, precision: usize, upper: bool) -> String {
    let rendered = format!("{:.*e}", precision, value);
    let (mantissa, exponent) = rendered.split_once('e').unwrap_or((rendered.as_str(), "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let text = format!(
        "{mantissa}e{}{:02}",
        if exponent < 0 { '-' } else { '+' },
        exponent.abs()
    );
    if upper {
        text.to_uppercase()
    } else {
        text
    }
}

fn strip_trailing_zeros(text: &str) -> String {
    let (mantissa, exponent) = match text.find(['e', 'E']) {
        Some(pos) => (&text[..pos], Some(&text[pos..])),
        None => (text, None),
    };
    let mut mantissa = mantissa.to_string();
    if mantissa.contains('.') {
        while mantissa.ends_with('0') {
            mantissa.pop();
        }
        if mantissa.ends_with('.') {
            mantissa.pop();
        }
    }
    match exponent {
        Some(exponent) => format!("{mantissa}{exponent}"),
        None => mantissa,
    }
}

fn fmt_g(value: f64, precision: usize, upper: bool, alt: bool) -> String {
    let precision = precision.max(1);
    let exponent = if value == 0.0 {
        0
    } else {
        value.abs().log10().floor() as i32
    };
    let rendered = if exponent < -4 || exponent >= precision as i32 {
        fmt_e(value, precision - 1, upper)
    } else {
        let decimals = (precision as i32 - 1 - exponent).max(0) as usize;
        format!("{:.*}", decimals, value)
    };
    if alt {
        rendered
    } else {
        strip_trailing_zeros(&rendered)
    }
}

fn wrap_hex_float(body: &str, upper: bool) -> String {
    if upper {
        format!("0X{}", body.to_uppercase())
    } else {
        format!("0x{body}")
    }
}

fn fmt_a(value: f64, precision: Option<usize>, alt: bool, upper: bool) -> String {
    if value == 0.0 {
        let body = match precision {
            Some(0) if !alt => "0p+0".to_string(),
            Some(0) => "0.p+0".to_string(),
            Some(p) => format!("0.{:0>width$}p+0", "", width = p),
            None if alt => "0.p+0".to_string(),
            None => "0p+0".to_string(),
        };
        return wrap_hex_float(&body, upper);
    }

    let bits = value.to_bits();
    let exponent_bits = ((bits >> 52) & 0x7ff) as i64;
    let mantissa = bits & ((1u64 << 52) - 1);
    let (mut lead, exponent) = if exponent_bits == 0 {
        (0u64, -1022i64)
    } else {
        (1, exponent_bits - 1023)
    };

    let fraction = match precision {
        None => {
            let mut fraction = format!("{:013x}", mantissa);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            fraction
        }
        Some(p) if p >= 13 => format!("{:013x}{}", mantissa, "0".repeat(p - 13)),
        Some(p) => {
            let shift = (13 - p) * 4;
            let mut kept = mantissa >> shift;
            let remainder = mantissa & ((1u64 << shift) - 1);
            let half = 1u64 << (shift - 1);
            if remainder >= half {
                kept += 1;
            }
            if p == 0 {
                lead += kept;
                String::new()
            } else {
                if kept >= 1u64 << (p * 4) {
                    kept = 0;
                    lead += 1;
                }
                format!("{:0width$x}", kept, width = p)
            }
        }
    };

    let body = if fraction.is_empty() && !alt {
        format!(
            "{lead}p{}{}",
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    } else {
        format!(
            "{lead}.{fraction}p{}{}",
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    };
    wrap_hex_float(&body, upper)
}

fn render_float(spec: Spec, arg: Option<&str>, errors: &mut Vec<String>) -> String {
    let raw = arg.unwrap_or("");
    let value = if raw.is_empty() {
        0.0
    } else {
        match raw.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                errors.push(raw.to_string());
                0.0
            }
        }
    };

    let negative = value.is_sign_negative();
    let abs = value.abs();
    let precision = spec.precision.unwrap_or(6);
    let upper = spec.conv.is_ascii_uppercase();
    let mut body = if !abs.is_finite() {
        if abs.is_nan() {
            if upper { "NAN" } else { "nan" }.to_string()
        } else if upper {
            "INF".to_string()
        } else {
            "inf".to_string()
        }
    } else {
        match spec.conv {
            'f' | 'F' => format!("{:.*}", precision, abs),
            'e' | 'E' => fmt_e(abs, precision, upper),
            'g' | 'G' => fmt_g(abs, precision, upper, spec.flags.alt),
            'a' | 'A' => fmt_a(abs, spec.precision, spec.flags.alt, upper),
            _ => format!("{:.*}", precision, abs),
        }
    };
    if abs.is_finite()
        && spec.flags.alt
        && matches!(spec.conv, 'f' | 'F' | 'e' | 'E' | 'g' | 'G')
        && !body.contains('.')
    {
        match body.find(['e', 'E']) {
            Some(pos) => body.insert(pos, '.'),
            None => body.push('.'),
        }
    }

    let sign = if negative {
        "-"
    } else if spec.flags.plus {
        "+"
    } else if spec.flags.space {
        " "
    } else {
        ""
    };
    pad_number(sign, "", &body, spec.flags, spec.width, abs.is_finite())
}

fn render(spec: Spec, arg: Option<&str>, errors: &mut Vec<String>) -> (String, bool) {
    match spec.conv {
        '%' => ("%".into(), false),
        's' => {
            let mut text = arg.unwrap_or("").to_string();
            if let Some(precision) = spec.precision {
                text = text.chars().take(precision).collect();
            }
            (pad(&text, spec.flags, spec.width), false)
        }
        'c' => {
            let text = arg
                .and_then(|value| value.chars().next())
                .map(|ch| ch.to_string())
                .unwrap_or_default();
            (pad(&text, spec.flags, spec.width), false)
        }
        'b' => {
            let (mut text, stop) = expand_escapes(arg.unwrap_or(""));
            if let Some(precision) = spec.precision {
                text = text.chars().take(precision).collect();
            }
            (pad(&text, spec.flags, spec.width), stop)
        }
        'q' => (
            pad(&quote_shell(arg.unwrap_or("")), spec.flags, spec.width),
            false,
        ),
        'd' | 'i' | 'u' | 'o' | 'x' | 'X' => (render_int(spec, arg, errors), false),
        'e' | 'E' | 'f' | 'F' | 'g' | 'G' | 'a' | 'A' => (render_float(spec, arg, errors), false),
        _ => (String::new(), false),
    }
}

/// One pass over the format. Returns true when `\c` stopped the output.
fn render_pass(
    format: &str,
    args: &[String],
    arg_index: &mut usize,
    errors: &mut Vec<String>,
    out: &mut String,
) -> bool {
    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '\\' => {
                if push_escape(&chars, &mut i, out) {
                    return true;
                }
            }
            '%' => {
                let before = *arg_index;
                let mut probe = i + 1;
                match parse_spec(&chars, &mut probe, args, arg_index) {
                    Some(spec) => {
                        let arg = if spec.conv == '%' {
                            None
                        } else {
                            take_arg(args, arg_index)
                        };
                        let (text, stop) = render(spec, arg, errors);
                        out.push_str(&text);
                        if stop {
                            return true;
                        }
                        i = probe;
                    }
                    None => {
                        *arg_index = before;
                        out.push('%');
                        i += 1;
                    }
                }
            }
            ch => {
                out.push(ch);
                i += 1;
            }
        }
    }
    false
}

/// Render `format` against `args`, repeating the format while arguments
/// remain. Returns the output and the raw arguments that were not numbers.
pub(crate) fn format_output(format: &str, args: &[String]) -> (String, Vec<String>) {
    let mut out = String::new();
    let mut errors = Vec::new();
    let mut arg_index = 0;
    loop {
        let before = arg_index;
        let stopped = render_pass(format, args, &mut arg_index, &mut errors, &mut out);
        if stopped || arg_index >= args.len() || arg_index == before {
            break;
        }
    }
    (out, errors)
}

pub(crate) fn cmd_nprintf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let mut tokens = tokenize(args);
    if matches!(tokens.first().map(String::as_str), Some("--")) {
        tokens.remove(0);
    }
    let Some(format) = tokens.first() else {
        print_error(&t!("misc.nprintf.usage"));
        print_info(&t!("misc.nprintf.example1"));
        print_info(&t!("misc.nprintf.example2"));
        return Ok(false);
    };

    let (out, errors) = format_output(format, &tokens[1..]);
    for raw in errors {
        print_error(&t!("misc.nprintf.invalid_number", raw));
    }
    print!("{out}");
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "nprintf",
    aliases: &[],
    help_topic: Some("nprintf"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_nprintf),
};

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_string()).collect()
    }

    fn run(format: &str, items: &[&str]) -> String {
        format_output(format, &args(items)).0
    }

    #[test]
    fn tokenize_keeps_empty_quoted_arguments() {
        assert_eq!(
            tokenize(r#""%s|%s\n" "a b" """#),
            vec!["%s|%s\\n".to_string(), "a b".to_string(), String::new()]
        );
        assert_eq!(
            tokenize(r#"'%s' 'a b'"#),
            vec!["%s".to_string(), "a b".to_string()]
        );
    }

    #[test]
    fn formats_strings_and_numbers() {
        assert_eq!(
            run("%s has %d items\n", &["cart", "3"]),
            "cart has 3 items\n"
        );
        assert_eq!(run("%05d|%-4d|%+d", &["42", "7", "9"]), "00042|7   |+9");
        assert_eq!(run("%x %X %o %u", &["255", "255", "8", "7"]), "ff FF 10 7");
        assert_eq!(run("%#x %#o", &["255", "8"]), "0xff 010");
        assert_eq!(run("%.3d", &["7"]), "007");
    }

    #[test]
    fn reuses_the_format_while_arguments_remain() {
        assert_eq!(run("%s ", &["a", "b", "c"]), "a b c ");
        assert_eq!(run("hello", &["ignored"]), "hello");
        assert_eq!(run("%s-%s;", &["a", "b", "c", "d"]), "a-b;c-d;");
    }

    #[test]
    fn interprets_escapes_and_percent_b() {
        assert_eq!(run(r"a\tb\n", &[]), "a\tb\n");
        assert_eq!(run(r"%b", &[r"x\ty\n"]), "x\ty\n");
        assert_eq!(run(r"\x41\102\u0043", &[]), "ABC");
        assert_eq!(run(r"%b", &[r"left\cRIGHT"]), "left");
        assert_eq!(run(r"a\cb", &[]), "a");
    }

    #[test]
    fn supports_width_precision_and_flags_for_strings() {
        assert_eq!(
            run("[%5s][%-5s][%.2s]", &["ab", "ab", "abcdef"]),
            "[   ab][ab   ][ab]"
        );
        assert_eq!(run("%*s", &["6", "hi"]), "    hi");
        assert_eq!(run("%-*s|", &["4", "hi"]), "hi  |");
    }

    #[test]
    fn formats_floats_scientific_and_general() {
        assert_eq!(run("%.2f", &["3.14159"]), "3.14");
        assert_eq!(run("%e", &["1500"]), "1.500000e+03");
        assert_eq!(run("%E", &["0.5"]), "5.000000E-01");
        assert_eq!(run("%g", &["0.0001"]), "0.0001");
        assert_eq!(run("%g", &["1500000"]), "1.5e+06");
        assert_eq!(run("%.3g", &["1234"]), "1.23e+03");
        assert_eq!(run("%a", &["3"]), "0x1.8p+1");
        assert_eq!(run("%08.2f", &["3.5"]), "00003.50");
    }

    #[test]
    fn character_quote_and_percent_conversions() {
        assert_eq!(run("%c%c", &["abc", "Z"]), "aZ");
        assert_eq!(run("%q", &["hello"]), "hello");
        assert_eq!(run("%q", &["a b"]), "'a b'");
        assert_eq!(run("%q", &["it's"]), "'it'\\''s'");
        assert_eq!(run("%q", &["a\nb"]), "$'a\\nb'");
        assert_eq!(run("100%%", &[]), "100%");
    }

    #[test]
    fn missing_arguments_become_zero_or_empty() {
        assert_eq!(run("%s|%d|%.2f", &[]), "|0|0.00");
        assert_eq!(run("%5s|", &[]), "     |");
    }

    #[test]
    fn invalid_numbers_are_reported() {
        let (out, errors) = format_output("%d", &args(&["oops"]));
        assert_eq!(out, "0");
        assert_eq!(errors, vec!["oops".to_string()]);
    }

    #[test]
    fn parses_bash_style_numbers() {
        assert_eq!(parse_int("42"), Some(42));
        assert_eq!(parse_int("0x10"), Some(16));
        assert_eq!(parse_int("010"), Some(8));
        assert_eq!(parse_int("'A"), Some(65));
        assert_eq!(parse_int("-7"), Some(-7));
        assert_eq!(parse_int(""), Some(0));
        assert_eq!(parse_int("nope"), None);
    }

    #[test]
    fn negative_numbers_wrap_for_unsigned_conversions() {
        assert_eq!(run("%u", &["-1"]), u64::MAX.to_string());
    }

    #[test]
    fn unsigned_conversions_ignore_signed_flags() {
        assert_eq!(run("%+u % u", &["5", "5"]), "5 5");
    }

    #[test]
    fn negative_star_precision_is_ignored() {
        assert_eq!(run("%*.*f", &["10", "-1", "3.14159"]), "  3.141590");
    }

    #[test]
    fn special_floats_and_hex_precision() {
        assert_eq!(run("%f", &["inf"]), "inf");
        assert_eq!(run("%F", &["inf"]), "INF");
        assert_eq!(run("%e", &["nan"]), "nan");
        assert_eq!(run("%a", &["1"]), "0x1p+0");
        assert_eq!(run("%A", &["3"]), "0X1.8P+1");
        assert_eq!(run("%.3a", &["3"]), "0x1.800p+1");
        assert_eq!(run("%.0a", &["3"]), "0x2p+1");
        assert_eq!(run("%#a", &["1"]), "0x1.p+0");
    }

    #[test]
    fn zero_and_octal_escape_edge_cases() {
        assert_eq!(run("%#.0o", &["0"]), "0");
        assert_eq!(run("%#.0x", &["0"]), "");
        assert_eq!(run("%.0d", &["0"]), "");
        assert_eq!(run("\\777", &[]), char::from_u32(0xff).unwrap().to_string());
    }

    #[test]
    fn dangling_percent_is_literal() {
        assert_eq!(run("50%", &[]), "50%");
        assert_eq!(run("%y", &["x"]), "%y");
    }
}
