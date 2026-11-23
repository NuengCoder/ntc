//! Shared argument tokenization and help-flag detection.

/// Split on whitespace while preserving double-quoted groups. `\"` and `\\`
/// inside double quotes stand for a literal quote/backslash, mirroring the
/// pipeline tokenizer, so re-quoted one-shot arguments round-trip.
pub(crate) fn tokenize_quoted(input: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_quotes {
            match ch {
                '\\' => match chars.peek().copied() {
                    Some(next) if next == '"' || next == '\\' => {
                        current.push(next);
                        chars.next();
                    }
                    _ => current.push('\\'),
                },
                '"' => in_quotes = false,
                _ => current.push(ch),
            }
        } else {
            match ch {
                '"' => in_quotes = true,
                ' ' | '\t' => {
                    if !current.is_empty() {
                        parts.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(ch),
            }
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// True only for an explicit help request (`help`/`--help`/`-h` as the
/// first token). Empty args mean "run with defaults", never help —
/// `serve` intercepts a bare invocation before its services see any args.
pub(crate) fn is_help_request(tokens: &[String]) -> bool {
    matches!(
        tokens.first().map(|s| s.as_str()),
        Some("help" | "--help" | "-h")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescapes_quotes_and_backslashes_inside_double_quotes() {
        assert_eq!(
            tokenize_quoted(r#""a \"b\"" "c\\d" e\n"#),
            vec![r#"a "b""#, r"c\d", r"e\n"]
        );
    }
}
