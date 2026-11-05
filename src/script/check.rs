//! `scr check` — syntax-level validation for `.ntc` automation scripts.
//!
//! A script has no formal grammar: every line is a command line, an `@`-forced
//! system command or a `set NAME = value`. What can still break a run is
//! reported here — an unterminated quote swallows the rest of the script, a
//! `set` without `=` silently becomes a command, an unclosed `${` expands to
//! nothing, and an empty `@` does nothing at all.

use crate::shell::commands::check_common::Diagnostic;

/// Validate a script body and return every problem, ordered by position.
pub(crate) fn check_text(content: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let body = strip_comment(trimmed);

        // A quote that never closes eats the following lines when the script
        // runs, so it is always an error.
        if let Some(offset) = unterminated_quote(body) {
            let quote = body[offset..].chars().next().unwrap_or('"');
            out.push(Diagnostic::at(
                line_no,
                offset + 1,
                format!("unterminated '{}' quote", quote),
                Some("close the string before the end of the line".to_string()),
            ));
        }

        if let Some(offset) = unclosed_placeholder(body) {
            out.push(Diagnostic::at(
                line_no,
                offset + 1,
                "unclosed '${' placeholder",
                Some("close it with '}' or escape the '$'".to_string()),
            ));
        }

        if body == "@" {
            out.push(Diagnostic::at(
                line_no,
                1,
                "empty '@' line",
                Some("write `@ <command>` to force a system command".to_string()),
            ));
        }

        if let Some(rest) = body.strip_prefix("set ") {
            match rest.split_once('=') {
                None => out.push(Diagnostic::at(
                    line_no,
                    1,
                    "`set` needs a value",
                    Some("write `set NAME = value`".to_string()),
                )),
                Some((name, _)) if name.trim().is_empty() => out.push(Diagnostic::at(
                    line_no,
                    1,
                    "`set` needs a name",
                    Some("write `set NAME = value`".to_string()),
                )),
                _ => {}
            }
        }

        // A `$` at the very end names no variable and expands to nothing.
        if let Some(offset) = body.rfind('$') {
            if offset + 1 == body.len() {
                out.push(Diagnostic::at(
                    line_no,
                    offset + 1,
                    "'$' at the end of the line expands to nothing",
                    Some("name the variable (`$NAME`) or drop the '$'".to_string()),
                ));
            }
        }
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

/// Strip a trailing `#` inline comment, respecting both quote styles.
fn strip_comment(line: &str) -> &str {
    let mut in_dq = false;
    let mut in_sq = false;
    for (idx, ch) in line.char_indices() {
        match ch {
            '"' if !in_sq => in_dq = !in_dq,
            '\'' if !in_dq => in_sq = !in_sq,
            '#' if !in_dq && !in_sq => return line[..idx].trim_end(),
            _ => {}
        }
    }
    line
}

/// Byte offset of the quote that is still open at the end of the line.
fn unterminated_quote(line: &str) -> Option<usize> {
    let mut in_dq = false;
    let mut in_sq = false;
    let mut open_at = 0usize;
    for (idx, ch) in line.char_indices() {
        match ch {
            '"' if !in_sq => {
                if in_dq {
                    in_dq = false;
                } else {
                    in_dq = true;
                    open_at = idx;
                }
            }
            '\'' if !in_dq => {
                if in_sq {
                    in_sq = false;
                } else {
                    in_sq = true;
                    open_at = idx;
                }
            }
            _ => {}
        }
    }
    if in_dq || in_sq {
        Some(open_at)
    } else {
        None
    }
}

/// Byte offset of a `${` that never closes on this line.
fn unclosed_placeholder(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'{' && !line[i + 2..].contains('}') {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn messages(content: &str) -> String {
        check_text(content)
            .iter()
            .map(|d| format!("{}:{} {}", d.line, d.col, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn clean_script_has_no_problems() {
        let content = "# comment\n\nset name = \"world\"\nsay \"Hello $name\"\n@ echo forced\n";
        assert!(check_text(content).is_empty(), "{}", messages(content));
    }

    #[test]
    fn unterminated_quote_is_reported_at_its_column() {
        let diags = check_text("say \"hello\n");
        assert_eq!(diags.len(), 1, "{:?}", diags);
        assert_eq!(diags[0].line, 1);
        assert_eq!(diags[0].col, 5);
    }

    #[test]
    fn balanced_quotes_are_accepted() {
        assert!(check_text("say \"a # b\" # comment\n").is_empty());
        assert!(check_text("say 'x'\n").is_empty());
    }

    #[test]
    fn set_without_value_is_reported() {
        let all = messages("set name\n");
        assert!(all.contains("`set` needs a value"), "{all}");
    }

    #[test]
    fn set_without_name_is_reported() {
        let all = messages("set = 5\n");
        assert!(all.contains("`set` needs a name"), "{all}");
    }

    #[test]
    fn empty_force_line_is_reported() {
        let all = messages("@\n");
        assert!(all.contains("empty '@'"), "{all}");
    }

    #[test]
    fn unclosed_placeholder_is_reported() {
        let all = messages("say \"${dir\"\n");
        assert!(all.contains("unclosed '${'"), "{all}");
    }

    #[test]
    fn trailing_dollar_is_reported() {
        let all = messages("say cost $\n");
        assert!(all.contains("at the end of the line"), "{all}");
    }

    #[test]
    fn comments_are_ignored() {
        assert!(check_text("# set\n// @\n").is_empty());
    }
}
