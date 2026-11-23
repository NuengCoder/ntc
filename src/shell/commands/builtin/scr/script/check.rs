//! `scr check` — syntax-level validation for `.ntc` automation scripts.
//!
//! The parser catches structural problems when a script runs; this module
//! reports them ahead of time, plus the mistakes that only show up as silent
//! surprises at runtime: an unterminated quote swallows the rest of the line,
//! a `set` without `=` silently becomes a command, an unclosed `${` expands to
//! nothing, and an empty `@` does nothing at all.

use crate::shell::commands::support::check::Diagnostic;
use std::collections::HashMap;

/// Words that open a block.
const OPENERS: &[&str] = &["if", "while", "for", "loop", "func", "function", "fn"];
/// Words that close a block.
const CLOSERS: &[&str] = &["end", "endif", "fi", "done", "}"];
/// Words that start another arm of an `if`.
const BRANCHES: &[&str] = &["else", "elseif", "elif"];

/// Validate a script body and return every problem, ordered by position.
pub(crate) fn check_text(content: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let mut stack: Vec<(&'static str, usize)> = Vec::new();
    let mut functions: HashMap<String, usize> = HashMap::new();

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

        // `set-pipe` / `set-showexc` take an on/off switch; a typo would
        // silently leave the default in place.
        if let Some((name, value)) = super::directives::split_directive(body) {
            if super::directives::parse_bool(value).is_none() {
                out.push(Diagnostic::at(
                    line_no,
                    1,
                    format!("`{name}` needs on or off"),
                    Some("use 1 / true / on, or 0 / false / off / fail".to_string()),
                ));
            }
        }

        check_block(body, line_no, &mut stack, &mut functions, &mut out);
    }

    for (kind, line) in stack {
        out.push(Diagnostic::at(
            line,
            1,
            format!("`{kind}` has no matching `end`"),
            Some("close the block with `end`".to_string()),
        ));
    }

    out.sort_by_key(|d| (d.line, d.col));
    out
}

/// Track block nesting, `if` arms and function names.
fn check_block(
    body: &str,
    line_no: usize,
    stack: &mut Vec<(&'static str, usize)>,
    functions: &mut HashMap<String, usize>,
    out: &mut Vec<Diagnostic>,
) {
    let word = body
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    if let Some(kind) = OPENERS.iter().copied().find(|k| *k == word.as_str()) {
        if kind == "if" && body[2..].trim().is_empty() {
            out.push(Diagnostic::at(
                line_no,
                1,
                "`if` needs a condition",
                Some("write `if <condition>`".to_string()),
            ));
        }
        if kind == "while" && body[5..].trim().is_empty() {
            out.push(Diagnostic::at(
                line_no,
                1,
                "`while` needs a condition",
                Some("write `while <condition>`".to_string()),
            ));
        }
        if kind == "for" && !body.to_ascii_lowercase().contains(" in ") {
            out.push(Diagnostic::at(
                line_no,
                1,
                "`for` needs `in`",
                Some("write `for NAME in a b c`".to_string()),
            ));
        }
        if matches!(kind, "func" | "function" | "fn") {
            let name = body[word.len()..]
                .trim()
                .split(|c: char| c == '(' || c.is_whitespace())
                .next()
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                out.push(Diagnostic::at(
                    line_no,
                    1,
                    "`func` needs a name",
                    Some("write `func NAME(params)`".to_string()),
                ));
            } else if let Some(first) = functions.insert(name.clone(), line_no) {
                out.push(Diagnostic::at(
                    line_no,
                    1,
                    format!("function '{name}' is already defined on line {first}"),
                    Some("rename one of the two functions".to_string()),
                ));
            }
        }
        stack.push((kind, line_no));
        return;
    }

    if CLOSERS.contains(&word.as_str()) {
        // Any closing word closes the innermost open block; the parser is just
        // as lenient, so `endif` after a `while` is only a style problem.
        if stack.pop().is_none() {
            out.push(Diagnostic::at(
                line_no,
                1,
                format!("`{word}` does not close any block"),
                Some("remove it or open a block first".to_string()),
            ));
        }
        return;
    }

    if BRANCHES.contains(&word.as_str()) {
        let in_if = stack.last().is_some_and(|(kind, _)| *kind == "if");
        if !in_if {
            out.push(Diagnostic::at(
                line_no,
                1,
                format!("`{word}` outside an `if`"),
                Some("`else` belongs between `if` and `end`".to_string()),
            ));
        }
        return;
    }

    if matches!(word.as_str(), "break" | "continue")
        && !stack
            .iter()
            .any(|(kind, _)| matches!(*kind, "while" | "for" | "loop"))
    {
        out.push(Diagnostic::at(
            line_no,
            1,
            format!("`{word}` outside a loop"),
            Some("`break` and `continue` need an enclosing while/for/loop".to_string()),
        ));
    }
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
    fn a_full_block_script_is_clean() {
        let content = "\
set n = 0
while $n < 3
  if $n == 1
    say one
  else if $n == 2
    say two
  else
    say other
  end
  set n = $n + 1
end
for f in a b c
  say $f
end
loop 2
  say again
end
func greet(who)
  return $who
end
call greet x
";
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

    #[test]
    fn missing_end_is_reported_at_the_opener() {
        let all = messages("if 1\n  say x\n");
        assert!(all.contains("`if` has no matching `end`"), "{all}");
        assert!(all.starts_with("1:"), "{all}");
    }

    #[test]
    fn stray_end_is_reported() {
        let all = messages("say x\nend\n");
        assert!(all.contains("`end` does not close any block"), "{all}");
    }

    #[test]
    fn else_outside_if_is_reported() {
        let all = messages("else\n  say x\n");
        assert!(all.contains("`else` outside an `if`"), "{all}");
    }

    #[test]
    fn duplicate_function_is_reported() {
        let all = messages("func f()\nend\nfunc f()\nend\n");
        assert!(all.contains("already defined on line 1"), "{all}");
    }

    #[test]
    fn if_without_condition_is_reported() {
        let all = messages("if\nend\n");
        assert!(all.contains("`if` needs a condition"), "{all}");
    }

    #[test]
    fn for_without_in_is_reported() {
        let all = messages("for f\nend\n");
        assert!(all.contains("`for` needs `in`"), "{all}");
    }

    #[test]
    fn break_outside_a_loop_is_reported() {
        let all = messages("break\n");
        assert!(all.contains("`break` outside a loop"), "{all}");
        // Inside a loop it is fine, including from a nested `if`.
        assert!(check_text("while 1\n  if 1\n    break\n  end\nend\n").is_empty());
    }

    #[test]
    fn directive_values_are_checked() {
        assert!(check_text("set-pipe = fail\nset-showexc = 1\n").is_empty());
        let all = messages("set-pipe = maybe\n");
        assert!(all.contains("`set-pipe` needs on or off"), "{all}");
        let all = messages("set-showexc\n");
        assert!(all.contains("`set-showexc` needs on or off"), "{all}");
    }
}
