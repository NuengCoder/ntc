use std::collections::HashMap;

/// Interpolate `$var`, `${var}`, `${var:-default}`, positional args (`$1`..`$9`,
/// `$@`, `$*`) in a script line.
///
/// Resolution order per reference:
/// - `${var:-default}` / `${var}` / `$var` → user variables, else default, else left literal.
/// - `$1`..`$9` / `${1}` → script positional args (missing → empty string).
/// - `$@` / `$*` / `${@}` / `${*}` → all positional args joined by spaces.
///
/// Unset references without a default are preserved literally (`$name` stays as-is).
pub(crate) fn interpolate(line: &str, vars: &HashMap<String, String>, args: &[String]) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut result = String::with_capacity(line.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            if let Some((value, end)) = resolve_ref(&chars, i, vars, args) {
                result.push_str(&value);
                i = end;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Try to resolve a `$` reference starting at `start`. Returns the value and the
/// index just past the reference, or `None` if it is not a valid reference.
fn resolve_ref(
    chars: &[char],
    start: usize,
    vars: &HashMap<String, String>,
    args: &[String],
) -> Option<(String, usize)> {
    let i = start + 1; // skip '$'
    if i >= chars.len() {
        return None;
    }

    if chars[i] == '{' {
        return resolve_braced(chars, i, vars, args);
    }

    if chars[i] == '@' || chars[i] == '*' {
        return Some((args.join(" "), i + 1));
    }

    if let Some(d) = chars[i].to_digit(10) {
        return Some((positional(d, args), i + 1));
    }

    if chars[i].is_alphabetic() || chars[i] == '_' {
        let mut j = i;
        while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
            j += 1;
        }
        let name: String = chars[i..j].iter().collect();
        return vars.get(&name).cloned().map(|v| (v, j));
    }

    None
}

/// Resolve `${...}` references (`${name}`, `${name:-default}`, `${1}`, `${@}`).
fn resolve_braced(
    chars: &[char],
    open: usize,
    vars: &HashMap<String, String>,
    args: &[String],
) -> Option<(String, usize)> {
    let mut j = open + 1;
    let mut colon_pos: Option<usize> = None;
    while j < chars.len() && chars[j] != '}' {
        if chars[j] == ':' && j + 1 < chars.len() && chars[j + 1] == '-' && colon_pos.is_none() {
            colon_pos = Some(j);
        }
        j += 1;
    }
    if j >= chars.len() {
        return None; // unclosed brace — leave literal
    }

    let name: String = if let Some(cp) = colon_pos {
        chars[open + 1..cp].iter().collect()
    } else {
        chars[open + 1..j].iter().collect()
    };
    let default: Option<String> = colon_pos.map(|cp| chars[cp + 2..j].iter().collect());

    let value = lookup(&name, vars, args);
    match value {
        Some(v) if !v.is_empty() => Some((v, j + 1)),
        _ => match default {
            Some(d) => Some((d, j + 1)),
            None => value.map(|v| (v, j + 1)),
        },
    }
}

/// Resolve a name against positional args and user variables.
fn lookup(name: &str, vars: &HashMap<String, String>, args: &[String]) -> Option<String> {
    match name {
        "@" | "*" => Some(args.join(" ")),
        _ => {
            if let Ok(d) = name.parse::<u32>() {
                return Some(positional(d, args));
            }
            vars.get(name).cloned()
        }
    }
}

fn positional(d: u32, args: &[String]) -> String {
    if d == 0 {
        return String::new();
    }
    args.get(d as usize - 1).cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("name".to_string(), "Alice".to_string());
        m.insert("dir".to_string(), "src".to_string());
        m
    }

    fn args() -> Vec<String> {
        vec!["dev".to_string(), "prod".to_string()]
    }

    #[test]
    fn test_simple_var() {
        assert_eq!(interpolate("cd $dir", &vars(), &[]), "cd src");
    }

    #[test]
    fn test_braced_var() {
        assert_eq!(interpolate("${name}", &vars(), &[]), "Alice");
    }

    #[test]
    fn test_default_used_when_unset() {
        assert_eq!(
            interpolate("${missing:-fallback}", &vars(), &[]),
            "fallback"
        );
    }

    #[test]
    fn test_default_ignored_when_set() {
        assert_eq!(interpolate("${name:-fallback}", &vars(), &[]), "Alice");
    }

    #[test]
    fn test_unset_without_default_left_literal() {
        assert_eq!(interpolate("echo $nope", &vars(), &[]), "echo $nope");
    }

    #[test]
    fn test_positional_args() {
        assert_eq!(
            interpolate("say $1 and $2", &vars(), &args()),
            "say dev and prod"
        );
    }

    #[test]
    fn test_positional_missing_empty() {
        assert_eq!(interpolate("say $3", &vars(), &args()), "say ");
    }

    #[test]
    fn test_positional_zero_empty() {
        assert_eq!(interpolate("say $0", &vars(), &[]), "say ");
    }

    #[test]
    fn test_all_args() {
        assert_eq!(interpolate("go $@", &vars(), &args()), "go dev prod");
        assert_eq!(interpolate("go $*", &vars(), &args()), "go dev prod");
        assert_eq!(interpolate("go ${@}", &vars(), &args()), "go dev prod");
    }

    #[test]
    fn test_braced_positional() {
        assert_eq!(interpolate("echo ${1}", &vars(), &args()), "echo dev");
    }

    #[test]
    fn test_embedded_in_text() {
        assert_eq!(
            interpolate("mv $dir/main.rs out/$1", &vars(), &args()),
            "mv src/main.rs out/dev"
        );
    }

    #[test]
    fn test_dollar_in_quotes_is_not_special() {
        // Quotes are not special to interpolation — a bare $ followed by a
        // non-name char stays literal; a digit is always a positional ref.
        assert_eq!(interpolate("echo $5.00", &vars(), &[]), "echo .00");
        assert_eq!(interpolate("echo $", &vars(), &[]), "echo $");
    }
}
