/// A single parsed statement in a `.ntc` script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stmt {
    /// `set NAME = value` — define a variable.
    Set { name: String, value: String },
    /// A command line (ntc builtin, alias, or system fallback).
    Cmd(String),
    /// A `@ <cmd>` line — force system execution, bypassing ntc builtins.
    ForceCmd(String),
}

/// Parse a `.ntc` script body into statements.
///
/// Rules:
/// - Blank lines and `#` / `//` comments are skipped.
/// - `#` inline comments are stripped (quote-aware, so `#` inside quotes survives).
/// - `@ <cmd>` forces a system command.
/// - `set NAME = value` defines a variable (surrounding quotes stripped from value).
/// - Everything else is a command line.
pub(crate) fn parse(content: &str) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        let line = strip_inline_comment(line).trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix('@') {
            let rest = rest.trim();
            if !rest.is_empty() {
                stmts.push(Stmt::ForceCmd(rest.to_string()));
            }
        } else if let Some(rest) = line.strip_prefix("set ") {
            if let Some(eq) = rest.find('=') {
                let name = rest[..eq].trim().to_string();
                let mut value = rest[eq + 1..].trim().to_string();
                value = strip_quotes(&value);
                if !name.is_empty() {
                    stmts.push(Stmt::Set { name, value });
                    continue;
                }
            }
            // Malformed `set` — fall through and treat as a normal command.
            stmts.push(Stmt::Cmd(line.to_string()));
        } else {
            stmts.push(Stmt::Cmd(line.to_string()));
        }
    }
    stmts
}

/// Strip a trailing `#` inline comment, respecting double/single quotes.
fn strip_inline_comment(line: &str) -> &str {
    let mut in_dq = false;
    let mut in_sq = false;
    for (idx, ch) in line.char_indices() {
        match ch {
            '"' if !in_sq => in_dq = !in_dq,
            '\'' if !in_dq => in_sq = !in_sq,
            '#' if !in_dq && !in_sq => return &line[..idx],
            _ => {}
        }
    }
    line
}

/// Remove one layer of surrounding matching quotes.
fn strip_quotes(value: &str) -> String {
    let v = value.trim();
    if v.len() >= 2 {
        let bytes = v.as_bytes();
        let first = bytes[0];
        let last = bytes[v.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return v[1..v.len() - 1].to_string();
        }
    }
    v.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(stmts: &[Stmt]) -> Vec<&str> {
        stmts
            .iter()
            .map(|s| match s {
                Stmt::Set { name, .. } => name.as_str(),
                Stmt::Cmd(_) => "CMD",
                Stmt::ForceCmd(_) => "FORCE",
            })
            .collect()
    }

    #[test]
    fn test_blank_and_comments_skipped() {
        let stmts = parse("# comment\n\n// also comment\n   \nsay hi\n");
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], Stmt::Cmd("say hi".to_string()));
    }

    #[test]
    fn test_set_parses() {
        let stmts = parse("set name = world");
        assert_eq!(
            stmts[0],
            Stmt::Set {
                name: "name".into(),
                value: "world".into()
            }
        );
    }

    #[test]
    fn test_set_quoted_value() {
        let stmts = parse(r#"set msg = "hello world""#);
        assert_eq!(
            stmts[0],
            Stmt::Set {
                name: "msg".into(),
                value: "hello world".into()
            }
        );
    }

    #[test]
    fn test_set_single_quotes() {
        let stmts = parse("set msg = 'hello world'");
        assert_eq!(
            stmts[0],
            Stmt::Set {
                name: "msg".into(),
                value: "hello world".into()
            }
        );
    }

    #[test]
    fn test_force_prefix() {
        let stmts = parse("@ echo forced");
        assert_eq!(stmts[0], Stmt::ForceCmd("echo forced".to_string()));
    }

    #[test]
    fn test_inline_comment_stripped() {
        let stmts = parse("say hello # a comment");
        assert_eq!(stmts[0], Stmt::Cmd("say hello".to_string()));
    }

    #[test]
    fn test_inline_comment_respects_quotes() {
        let stmts = parse(r#"say "a # b" # comment"#);
        assert_eq!(stmts[0], Stmt::Cmd(r#"say "a # b""#.to_string()));
    }

    #[test]
    fn test_malformed_set_falls_through() {
        let stmts = parse("set is a command");
        assert_eq!(stmts[0], Stmt::Cmd("set is a command".to_string()));
    }

    #[test]
    fn test_parse_result_is_ok() {
        let _ = parse("cd src\nsay hi");
    }

    #[test]
    fn test_order_preserved() {
        let stmts = parse("set a = 1\n@ ls\nsay ok");
        assert_eq!(names(&stmts), vec!["a", "FORCE", "CMD"]);
    }
}
