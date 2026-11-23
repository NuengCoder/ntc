//! Expression evaluation for `.ntc` scripts.
//!
//! One tiny language is shared by `set`, `if`, `while`, `for` and `loop`:
//!
//! ```text
//! 1 + 2                 arithmetic (`+ - * / %`, parentheses)
//! $count > 3            comparisons (`== != < <= > >=`)
//! $a && !$b             logic (`&& || !`, or `and` / `or` / `not`)
//! "a" + $b              `+` concatenates when a side is not a number
//! $name contains "x"    substring test
//! nprintf("%02d", $i)   format a value with the nprintf builtin
//! ```
//!
//! Values are either an integer or a string. Arithmetic coerces to integers
//! and reports an error when a side is not numeric; `+` degrades to string
//! concatenation instead, so `set path = $dir + "/" + $file` works.
//!
//! Parsing and evaluation happen in one pass: there is no AST, and the only
//! side effects are the pure builtin functions below.

use std::fmt;

/// A value produced by an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Value {
    Num(i64),
    Text(String),
}

impl Value {
    /// Render the value the way `set` stores it.
    pub(crate) fn as_text(&self) -> String {
        match self {
            Value::Num(n) => n.to_string(),
            Value::Text(s) => s.clone(),
        }
    }

    /// Interpret the value as an integer, if it is one.
    pub(crate) fn as_int(&self) -> Option<i64> {
        match self {
            Value::Num(n) => Some(*n),
            Value::Text(s) => s.trim().parse::<i64>().ok(),
        }
    }

    /// Truthiness: numbers are false at zero; strings are false when empty or
    /// one of `0`, `false`, `no`, `off` (case-insensitive).
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Value::Num(n) => *n != 0,
            Value::Text(s) => {
                let t = s.trim();
                !t.is_empty()
                    && !matches!(
                        t.to_ascii_lowercase().as_str(),
                        "0" | "false" | "no" | "off"
                    )
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_text())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Contains,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    Num(i64),
    Text(String),
    Ident(String),
    Op(Op),
    LParen,
    RParen,
    Comma,
}

/// Evaluate `src` against a variable lookup.
///
/// The lookup receives a bare name (`name` from `$name`) and returns its
/// value. An unknown name is substituted literally, matching the way
/// `interp::interpolate` leaves `$name` alone.
pub(crate) fn eval(src: &str, lookup: &dyn Fn(&str) -> Option<String>) -> Result<Value, String> {
    let toks = tokenize(src, lookup)?;
    if toks.is_empty() {
        return Ok(Value::Text(String::new()));
    }
    let mut parser = Parser { toks, pos: 0 };
    let value = parser.expr()?;
    if parser.pos < parser.toks.len() {
        return Err(format!("unexpected '{}'", parser.describe(parser.pos)));
    }
    Ok(value)
}

/// True when `src` is a plain value with no operators — `set` stores those
/// verbatim so a path like `set p = a b` keeps its spacing.
pub(crate) fn is_plain(src: &str) -> bool {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return true;
    }
    let bare = trimmed.trim_matches('"').trim_matches('\'');
    !bare.chars().any(|c| {
        matches!(
            c,
            '+' | '-' | '*' | '/' | '%' | '(' | ')' | ',' | '<' | '>' | '!' | '&' | '|' | '='
        )
    }) && !bare.split_whitespace().any(|word| {
        matches!(
            word.to_ascii_lowercase().as_str(),
            "and" | "or" | "not" | "contains"
        )
    })
}

// ── tokenizer ───────────────────────────────────────────────────────────

fn tokenize(src: &str, lookup: &dyn Fn(&str) -> Option<String>) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        match c {
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            ',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            '"' | '\'' => {
                let (text, end) = read_string(&chars, i, lookup)?;
                out.push(Tok::Text(text));
                i = end;
            }
            '$' => {
                let (text, end) = read_reference(&chars, i, lookup);
                out.push(Tok::Text(text));
                i = end;
            }
            '+' => {
                out.push(Tok::Op(Op::Add));
                i += 1;
            }
            '-' => {
                out.push(Tok::Op(Op::Sub));
                i += 1;
            }
            '*' => {
                out.push(Tok::Op(Op::Mul));
                i += 1;
            }
            '/' => {
                out.push(Tok::Op(Op::Div));
                i += 1;
            }
            '%' => {
                out.push(Tok::Op(Op::Mod));
                i += 1;
            }
            '=' => {
                // `=` and `==` both mean equality; scripts are not C.
                if chars.get(i + 1) == Some(&'=') {
                    i += 2;
                } else {
                    i += 1;
                }
                out.push(Tok::Op(Op::Eq));
            }
            '!' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Op(Op::Ne));
                    i += 2;
                } else {
                    out.push(Tok::Op(Op::Not));
                    i += 1;
                }
            }
            '<' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Op(Op::Le));
                    i += 2;
                } else {
                    out.push(Tok::Op(Op::Lt));
                    i += 1;
                }
            }
            '>' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Op(Op::Ge));
                    i += 2;
                } else {
                    out.push(Tok::Op(Op::Gt));
                    i += 1;
                }
            }
            '&' => {
                if chars.get(i + 1) == Some(&'&') {
                    out.push(Tok::Op(Op::And));
                    i += 2;
                } else {
                    return Err("expected '&&'".to_string());
                }
            }
            '|' => {
                if chars.get(i + 1) == Some(&'|') {
                    out.push(Tok::Op(Op::Or));
                    i += 2;
                } else {
                    return Err("expected '||'".to_string());
                }
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let digits: String = chars[start..i].iter().collect();
                let n = digits
                    .parse::<i64>()
                    .map_err(|_| format!("number '{digits}' is out of range"))?;
                out.push(Tok::Num(n));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                match word.to_ascii_lowercase().as_str() {
                    "and" => out.push(Tok::Op(Op::And)),
                    "or" => out.push(Tok::Op(Op::Or)),
                    "not" => out.push(Tok::Op(Op::Not)),
                    "contains" => out.push(Tok::Op(Op::Contains)),
                    _ => out.push(Tok::Ident(word)),
                }
            }
            other => return Err(format!("unexpected character '{other}'")),
        }
    }

    Ok(out)
}

/// Read a quoted string. Double quotes honour backslash escapes and `$`
/// references; single quotes are literal, like the shell.
fn read_string(
    chars: &[char],
    open: usize,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> Result<(String, usize), String> {
    let quote = chars[open];
    let mut out = String::new();
    let mut i = open + 1;
    while i < chars.len() {
        let c = chars[i];
        if c == quote {
            return Ok((out, i + 1));
        }
        if quote == '"' {
            if c == '\\' && i + 1 < chars.len() {
                let next = chars[i + 1];
                match next {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    '\'' => out.push('\''),
                    other => {
                        out.push('\\');
                        out.push(other);
                    }
                }
                i += 2;
                continue;
            }
            if c == '$' {
                let (text, end) = read_reference(chars, i, lookup);
                out.push_str(&text);
                i = end;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    Err(format!("unterminated '{quote}' string"))
}

/// Read `$name`, `${name}` or `${name:-default}` and return its value.
fn read_reference(
    chars: &[char],
    start: usize,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> (String, usize) {
    let mut i = start + 1;
    if i >= chars.len() {
        return ("$".to_string(), i);
    }

    if chars[i] == '{' {
        let open = i;
        let mut close = None;
        let mut j = i + 1;
        while j < chars.len() {
            if chars[j] == '}' {
                close = Some(j);
                break;
            }
            j += 1;
        }
        let Some(close) = close else {
            return (chars[start..].iter().collect(), chars.len());
        };
        let inner: String = chars[open + 1..close].iter().collect();
        let (name, default) = match inner.split_once(":-") {
            Some((name, default)) => (name.trim().to_string(), Some(default.to_string())),
            None => (inner.trim().to_string(), None),
        };
        let value = lookup(&name).filter(|v| !v.is_empty());
        let text = match (value, default) {
            (Some(v), _) => v,
            (None, Some(d)) => d,
            (None, None) => String::new(),
        };
        return (text, close + 1);
    }

    if chars[i] == '@' || chars[i] == '*' {
        return (lookup("@").unwrap_or_default(), i + 1);
    }

    if chars[i].is_ascii_digit() {
        let d = chars[i];
        return (lookup(&d.to_string()).unwrap_or_default(), i + 1);
    }

    if chars[i].is_alphabetic() || chars[i] == '_' {
        let name_start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        let name: String = chars[name_start..i].iter().collect();
        return match lookup(&name) {
            Some(v) => (v, i),
            // Leave an unknown reference alone, like `interp::interpolate`.
            None => (format!("${name}"), i),
        };
    }

    ("$".to_string(), i)
}

// ── parser + evaluator ──────────────────────────────────────────────────

struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn peek_op(&self) -> Option<Op> {
        match self.peek() {
            Some(Tok::Op(op)) => Some(*op),
            _ => None,
        }
    }

    fn describe(&self, at: usize) -> String {
        match self.toks.get(at) {
            Some(Tok::Num(n)) => n.to_string(),
            Some(Tok::Text(t)) => t.clone(),
            Some(Tok::Ident(i)) => i.clone(),
            Some(Tok::Op(op)) => format!("{op:?}").to_lowercase(),
            Some(Tok::LParen) => "(".to_string(),
            Some(Tok::RParen) => ")".to_string(),
            Some(Tok::Comma) => ",".to_string(),
            None => "end of expression".to_string(),
        }
    }

    fn expr(&mut self) -> Result<Value, String> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Value, String> {
        let mut left = self.and_expr()?;
        while self.peek_op() == Some(Op::Or) {
            self.pos += 1;
            let right = self.and_expr()?;
            left = Value::Num(i64::from(left.truthy() || right.truthy()));
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Value, String> {
        let mut left = self.not_expr()?;
        while self.peek_op() == Some(Op::And) {
            self.pos += 1;
            let right = self.not_expr()?;
            left = Value::Num(i64::from(left.truthy() && right.truthy()));
        }
        Ok(left)
    }

    fn not_expr(&mut self) -> Result<Value, String> {
        if self.peek_op() == Some(Op::Not) {
            self.pos += 1;
            let value = self.not_expr()?;
            return Ok(Value::Num(i64::from(!value.truthy())));
        }
        self.cmp_expr()
    }

    fn cmp_expr(&mut self) -> Result<Value, String> {
        let left = self.add_expr()?;
        let Some(op) = self.peek_op() else {
            return Ok(left);
        };
        if !matches!(
            op,
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge | Op::Contains
        ) {
            return Ok(left);
        }
        self.pos += 1;
        let right = self.add_expr()?;
        let result = match op {
            Op::Contains => left.as_text().contains(&right.as_text()),
            _ => {
                let ordering = match (left.as_int(), right.as_int()) {
                    (Some(a), Some(b)) => a.cmp(&b),
                    _ => left.as_text().cmp(&right.as_text()),
                };
                match op {
                    Op::Eq => ordering.is_eq(),
                    Op::Ne => !ordering.is_eq(),
                    Op::Lt => ordering.is_lt(),
                    Op::Le => ordering.is_le(),
                    Op::Gt => ordering.is_gt(),
                    Op::Ge => ordering.is_ge(),
                    _ => false,
                }
            }
        };
        Ok(Value::Num(i64::from(result)))
    }

    fn add_expr(&mut self) -> Result<Value, String> {
        let mut left = self.mul_expr()?;
        loop {
            match self.peek_op() {
                Some(Op::Add) => {
                    self.pos += 1;
                    let right = self.mul_expr()?;
                    left = match (left.as_int(), right.as_int()) {
                        (Some(a), Some(b)) => Value::Num(a + b),
                        _ => Value::Text(format!("{}{}", left.as_text(), right.as_text())),
                    };
                }
                Some(Op::Sub) => {
                    self.pos += 1;
                    let right = self.mul_expr()?;
                    let a = require_int(&left, "-")?;
                    let b = require_int(&right, "-")?;
                    left = Value::Num(a - b);
                }
                _ => return Ok(left),
            }
        }
    }

    fn mul_expr(&mut self) -> Result<Value, String> {
        let mut left = self.unary_expr()?;
        loop {
            let op = match self.peek_op() {
                Some(op @ (Op::Mul | Op::Div | Op::Mod)) => op,
                _ => return Ok(left),
            };
            self.pos += 1;
            let right = self.unary_expr()?;
            let a = require_int(&left, "a multiplication")?;
            let b = require_int(&right, "a multiplication")?;
            left = match op {
                Op::Mul => Value::Num(a * b),
                Op::Div => {
                    if b == 0 {
                        return Err("division by zero".to_string());
                    }
                    Value::Num(a / b)
                }
                _ => {
                    if b == 0 {
                        return Err("division by zero".to_string());
                    }
                    Value::Num(a % b)
                }
            };
        }
    }

    fn unary_expr(&mut self) -> Result<Value, String> {
        match self.peek_op() {
            Some(Op::Sub) => {
                self.pos += 1;
                let value = self.unary_expr()?;
                Ok(Value::Num(-require_int(&value, "unary '-'")?))
            }
            Some(Op::Add) => {
                self.pos += 1;
                self.unary_expr()
            }
            Some(Op::Not) => {
                self.pos += 1;
                let value = self.unary_expr()?;
                Ok(Value::Num(i64::from(!value.truthy())))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Value, String> {
        let tok = self
            .peek()
            .cloned()
            .ok_or_else(|| "expression ended early".to_string())?;
        self.pos += 1;

        match tok {
            Tok::Num(n) => Ok(Value::Num(n)),
            Tok::Text(t) => Ok(Value::Text(t)),
            Tok::LParen => {
                let value = self.expr()?;
                match self.peek() {
                    Some(Tok::RParen) => {
                        self.pos += 1;
                        Ok(value)
                    }
                    _ => Err("missing ')'".to_string()),
                }
            }
            Tok::Ident(name) => {
                if self.peek() == Some(&Tok::LParen) {
                    self.pos += 1;
                    let args = self.call_args()?;
                    call_builtin(&name, args)
                } else {
                    // A bare word is a literal, not a variable: variables are
                    // always spelled `$name` so conditions stay predictable.
                    Ok(Value::Text(name))
                }
            }
            _ => Err(format!("unexpected '{}'", self.describe(self.pos - 1))),
        }
    }

    fn call_args(&mut self) -> Result<Vec<Value>, String> {
        let mut args = Vec::new();
        if self.peek() == Some(&Tok::RParen) {
            self.pos += 1;
            return Ok(args);
        }
        loop {
            args.push(self.expr()?);
            match self.peek() {
                Some(Tok::Comma) => {
                    self.pos += 1;
                }
                Some(Tok::RParen) => {
                    self.pos += 1;
                    return Ok(args);
                }
                _ => return Err("missing ')' in function call".to_string()),
            }
        }
    }
}

fn require_int(value: &Value, what: &str) -> Result<i64, String> {
    value
        .as_int()
        .ok_or_else(|| format!("{what} needs a number, got '{}'", value.as_text()))
}

fn call_builtin(name: &str, args: Vec<Value>) -> Result<Value, String> {
    let arg = |idx: usize| args.get(idx).map(Value::as_text).unwrap_or_default();

    match name.to_ascii_lowercase().as_str() {
        "nprintf" | "printf" => {
            if args.is_empty() {
                return Err("nprintf() needs a format string".to_string());
            }
            let format = arg(0);
            let rest: Vec<String> = args.iter().skip(1).map(Value::as_text).collect();
            let (out, _) = crate::shell::commands::builtin::nprintf::format_output(&format, &rest);
            Ok(Value::Text(out))
        }
        "len" => Ok(Value::Num(arg(0).chars().count() as i64)),
        "upper" => Ok(Value::Text(arg(0).to_uppercase())),
        "lower" => Ok(Value::Text(arg(0).to_lowercase())),
        "trim" => Ok(Value::Text(arg(0).trim().to_string())),
        "str" => Ok(Value::Text(arg(0))),
        "int" => Ok(Value::Num(
            args.first()
                .and_then(Value::as_int)
                .ok_or_else(|| format!("int('{}') is not a number", arg(0)))?,
        )),
        "abs" => Ok(Value::Num(
            args.first()
                .and_then(Value::as_int)
                .ok_or_else(|| format!("abs('{}') is not a number", arg(0)))?
                .abs(),
        )),
        "env" => Ok(Value::Text(
            std::env::var(arg(0)).unwrap_or_default(),
        )),
        other => Err(format!("unknown function '{other}'")),
    }
}

/// Helper so `primary` can build an error without moving the token twice.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("name".to_string(), "Alice".to_string());
        m.insert("n".to_string(), "7".to_string());
        m.insert("empty".to_string(), String::new());
        m
    }

    fn ev(src: &str) -> Value {
        let vars = vars();
        eval(src, &|name| vars.get(name).cloned()).unwrap()
    }

    fn ev_err(src: &str) -> String {
        let vars = vars();
        eval(src, &|name| vars.get(name).cloned()).unwrap_err()
    }

    #[test]
    fn plain_values_stay_text() {
        assert_eq!(ev("hello"), Value::Text("hello".into()));
        assert_eq!(ev("\"hello world\""), Value::Text("hello world".into()));
        assert_eq!(ev("$name"), Value::Text("Alice".into()));
    }

    #[test]
    fn arithmetic_is_integer() {
        assert_eq!(ev("1 + 2 * 3"), Value::Num(7));
        assert_eq!(ev("(1 + 2) * 3"), Value::Num(9));
        assert_eq!(ev("10 / 3"), Value::Num(3));
        assert_eq!(ev("10 % 3"), Value::Num(1));
        assert_eq!(ev("-4 + 1"), Value::Num(-3));
        assert_eq!(ev("$n + 1"), Value::Num(8));
    }

    #[test]
    fn plus_concatenates_when_a_side_is_text() {
        assert_eq!(ev("\"a\" + \"b\""), Value::Text("ab".into()));
        assert_eq!(ev("\"n=\" + $n"), Value::Text("n=7".into()));
        assert_eq!(ev("$name + \"!\""), Value::Text("Alice!".into()));
    }

    #[test]
    fn comparisons_use_numbers_then_text() {
        assert_eq!(ev("2 > 1"), Value::Num(1));
        assert_eq!(ev("2 = 2"), Value::Num(1));
        assert_eq!(ev("2 != 2"), Value::Num(0));
        assert_eq!(ev("\"abc\" < \"abd\""), Value::Num(1));
        assert_eq!(ev("$name == \"Alice\""), Value::Num(1));
        // 10 > 9 numerically, even though "10" < "9" as text.
        assert_eq!(ev("\"10\" > \"9\""), Value::Num(1));
    }

    #[test]
    fn logic_and_truthiness() {
        assert_eq!(ev("1 && 0"), Value::Num(0));
        assert_eq!(ev("1 || 0"), Value::Num(1));
        assert_eq!(ev("!0"), Value::Num(1));
        assert_eq!(ev("not $empty"), Value::Num(1));
        assert_eq!(ev("$name and 1"), Value::Num(1));
        assert_eq!(ev("\"0\""), Value::Text("0".into()));
        assert!(!Value::Text("0".into()).truthy());
        assert!(!Value::Text("false".into()).truthy());
        assert!(Value::Text("yes".into()).truthy());
    }

    #[test]
    fn contains_tests_substrings() {
        assert_eq!(ev("$name contains \"lic\""), Value::Num(1));
        assert_eq!(ev("$name contains \"zz\""), Value::Num(0));
    }

    #[test]
    fn default_and_literal_references() {
        assert_eq!(ev("${missing:-fallback}"), Value::Text("fallback".into()));
        assert_eq!(ev("${name:-x}"), Value::Text("Alice".into()));
        assert_eq!(ev("$missing"), Value::Text("$missing".into()));
    }

    #[test]
    fn nprintf_formats_a_value() {
        assert_eq!(
            ev("nprintf(\"%02d\", $n)"),
            Value::Text("07".into())
        );
        assert_eq!(
            ev("nprintf(\"%s-%s\", \"a\", \"b\")"),
            Value::Text("a-b".into())
        );
        assert_eq!(ev("nprintf(\"%s\", $name)"), Value::Text("Alice".into()));
    }

    #[test]
    fn string_helpers() {
        assert_eq!(ev("upper($name)"), Value::Text("ALICE".into()));
        assert_eq!(ev("lower(\"AB\")"), Value::Text("ab".into()));
        assert_eq!(ev("len(\"abc\")"), Value::Num(3));
        assert_eq!(ev("trim(\"  x  \")"), Value::Text("x".into()));
        assert_eq!(ev("int(\"42\")"), Value::Num(42));
        assert_eq!(ev("abs(-3)"), Value::Num(3));
    }

    #[test]
    fn errors_are_reported() {
        assert!(ev_err("1 +").contains("ended early"));
        assert!(ev_err("(1 + 2").contains("missing ')'"));
        assert!(ev_err("1 / 0").contains("division by zero"));
        assert!(ev_err("nope(\"x\")").contains("unknown function"));
        assert!(ev_err("\"unterminated").contains("unterminated"));
        assert!(ev_err("1 2").contains("unexpected"));
    }

    #[test]
    fn plain_detection_keeps_paths_verbatim() {
        assert!(is_plain("hello world"));
        assert!(is_plain("C:\\Users\\x"));
        assert!(is_plain("\"quoted text\""));
        assert!(!is_plain("1 + 1"));
        assert!(!is_plain("nprintf(\"%d\", 1)"));
        assert!(!is_plain("a contains b"));
    }
}
