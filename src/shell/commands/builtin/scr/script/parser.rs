//! Parser for `.ntc` script files.
//!
//! A script is a line-oriented language with block statements:
//!
//! ```text
//! set name = "world"          variable (plain value or expression)
//! say "Hello $name"           command line (builtin, alias, or system)
//! @ echo forced               forced system command
//!
//! if $n > 3                   condition block, closed by `end`
//!   say big
//! else if $n == 3
//!   say three
//! else
//!   say small
//! end
//!
//! while $i < 5                loop with a condition
//!   set i = $i + 1
//! end
//!
//! for f in a b c              loop over words (or an expression)
//!   say $f
//! end
//!
//! loop 3                      repeat a fixed number of times
//!   say again
//! end
//!
//! func greet(who)             user function
//!   say "Hello $who"
//!   return 0
//! end
//! call greet "world"
//!
//! input name "Your name"      read a line into a variable
//! ```
//!
//! `end` also accepts `endif` / `fi` / `done` / `}` so the closing word can
//! match the opener. Every statement keeps its source line so runtime errors
//! point at the right place.

/// A statement together with the line it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stmt {
    pub line: usize,
    pub kind: Kind,
}

/// One `if` / `else if` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Branch {
    pub cond: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Kind {
    /// `set NAME = value` — a plain value or an expression.
    Set { name: String, expr: String },
    /// A command line (ntc builtin, alias, or system fallback).
    Cmd { text: String, force: bool },
    /// `if` / `else if` / `else`.
    If {
        branches: Vec<Branch>,
        otherwise: Option<Vec<Stmt>>,
    },
    /// `while <cond>` … `end`
    While { cond: String, body: Vec<Stmt> },
    /// `for <var> in <list>` … `end`
    For {
        var: String,
        list: String,
        body: Vec<Stmt>,
    },
    /// `loop [count]` … `end`
    Loop {
        count: Option<String>,
        body: Vec<Stmt>,
    },
    /// `func name(params)` … `end`
    Func {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    /// `call name args…`
    Call { name: String, args: Vec<String> },
    /// `return [value]`
    Return(Option<String>),
    /// `input NAME [prompt]`
    Input { name: String, prompt: Option<String> },
    /// `break` / `continue` / `exit`
    Break,
    Continue,
    Exit,
}

/// A structural problem found while parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParseError {
    pub line: usize,
    pub message: String,
}

/// Words that close any block.
const END_WORDS: &[&str] = &["end", "endif", "fi", "done", "}"];
/// Words that close an `if` and may start another arm.
const IF_WORDS: &[&str] = &["end", "endif", "fi", "}", "else", "elseif", "elif"];

/// Parse a `.ntc` script body.
///
/// Returns the statements and every structural error found. A script with
/// errors is not executed — the caller reports them and stops.
pub(crate) fn parse(content: &str) -> (Vec<Stmt>, Vec<ParseError>) {
    let mut lines = Vec::new();
    for (idx, raw) in content.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        let body = strip_inline_comment(trimmed).trim();
        if body.is_empty() {
            continue;
        }
        // `set-pipe` / `set-showexc` configure the run; they are read by
        // `directives::directives` and must never execute as commands.
        if super::directives::split_directive(body).is_some() {
            continue;
        }
        lines.push(Line {
            no: idx + 1,
            text: body.to_string(),
        });
    }

    let mut parser = Parser {
        lines,
        pos: 0,
        errors: Vec::new(),
    };
    let stmts = parser.block(&[]);
    (stmts, parser.errors)
}

struct Line {
    no: usize,
    text: String,
}

struct Parser {
    lines: Vec<Line>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    fn error(&mut self, line: usize, message: impl Into<String>) {
        self.errors.push(ParseError {
            line,
            message: message.into(),
        });
    }

    fn current(&self) -> Option<(usize, String, String, String)> {
        let line = self.lines.get(self.pos)?;
        let (word, rest) = split_word(&line.text);
        Some((
            line.no,
            line.text.clone(),
            word.to_string(),
            rest.to_string(),
        ))
    }

    /// Parse statements until one of `stop` is reached (left unconsumed).
    fn block(&mut self, stop: &[&str]) -> Vec<Stmt> {
        let mut out = Vec::new();
        while let Some((line_no, text, word, _rest)) = self.current() {
            let lower = word.to_ascii_lowercase();
            if stop.contains(&lower.as_str()) {
                break;
            }
            // A terminator that no open block asked for: report it and move
            // on, otherwise the same line would be visited forever.
            if END_WORDS.contains(&lower.as_str())
                || matches!(lower.as_str(), "else" | "elseif" | "elif")
            {
                self.error(line_no, format!("`{word}` does not close any block"));
                self.pos += 1;
                continue;
            }

            match lower.as_str() {
                "if" => out.push(self.parse_if()),
                "while" => out.push(self.parse_while()),
                "for" => out.push(self.parse_for()),
                "loop" => out.push(self.parse_loop()),
                "func" | "function" | "fn" => out.push(self.parse_func()),
                _ => {
                    self.pos += 1;
                    if let Some(stmt) = self.simple(line_no, &text) {
                        out.push(stmt);
                    }
                }
            }
        }
        out
    }

    /// Consume the closing word of a block, reporting an unterminated block.
    fn close_block(&mut self, opener: &str, line: usize) -> bool {
        match self.current() {
            Some((_, _, word, _)) if END_WORDS.contains(&word.to_ascii_lowercase().as_str()) => {
                self.pos += 1;
                true
            }
            _ => {
                self.error(line, format!("`{opener}` has no matching `end`"));
                false
            }
        }
    }

    fn parse_if(&mut self) -> Stmt {
        let line = self.lines[self.pos].no;
        let (_, _, _, cond) = self.current().expect("checked by block()");
        self.pos += 1;

        let mut cond = cond;
        if cond.trim().is_empty() {
            self.error(line, "`if` needs a condition");
        }

        let mut branches = Vec::new();
        let mut otherwise = None;

        loop {
            let body = self.block(IF_WORDS);
            let Some((_, _, word, rest)) = self.current() else {
                self.error(line, "`if` has no matching `end`");
                branches.push(Branch {
                    cond: std::mem::take(&mut cond),
                    body,
                });
                break;
            };
            match word.to_ascii_lowercase().as_str() {
                "else" => {
                    let rest_lower = rest.to_ascii_lowercase();
                    if rest_lower == "if" || rest_lower.starts_with("if ") {
                        // `else if <cond>` starts another arm.
                        branches.push(Branch {
                            cond: std::mem::take(&mut cond),
                            body,
                        });
                        cond = rest[2..].trim().to_string();
                        self.pos += 1;
                        continue;
                    }
                    self.pos += 1;
                    branches.push(Branch {
                        cond: std::mem::take(&mut cond),
                        body,
                    });
                    otherwise = Some(self.block(END_WORDS));
                    self.close_block("if", line);
                    break;
                }
                "elseif" | "elif" => {
                    branches.push(Branch {
                        cond: std::mem::take(&mut cond),
                        body,
                    });
                    cond = rest;
                    self.pos += 1;
                }
                _ => {
                    // One of the END_WORDS.
                    self.pos += 1;
                    branches.push(Branch {
                        cond: std::mem::take(&mut cond),
                        body,
                    });
                    break;
                }
            }
        }

        Stmt {
            line,
            kind: Kind::If {
                branches,
                otherwise,
            },
        }
    }

    fn parse_while(&mut self) -> Stmt {
        let line = self.lines[self.pos].no;
        let (_, _, _, cond) = self.current().expect("checked by block()");
        if cond.trim().is_empty() {
            self.error(line, "`while` needs a condition");
        }
        self.pos += 1;
        let body = self.block(END_WORDS);
        self.close_block("while", line);
        Stmt {
            line,
            kind: Kind::While { cond, body },
        }
    }

    fn parse_for(&mut self) -> Stmt {
        let line = self.lines[self.pos].no;
        let (_, _, _, rest) = self.current().expect("checked by block()");
        self.pos += 1;

        let mut parts = rest.splitn(2, char::is_whitespace);
        let var = parts.next().unwrap_or("").trim().to_string();
        let tail = parts.next().unwrap_or("").trim().to_string();

        let mut list = String::new();
        if var.is_empty() {
            self.error(line, "`for` needs a variable name");
        } else if !tail.to_ascii_lowercase().starts_with("in ") {
            self.error(line, "`for` needs `in`, e.g. `for f in a b c`");
        } else {
            list = tail[3..].trim().to_string();
        }

        let body = self.block(END_WORDS);
        self.close_block("for", line);
        Stmt {
            line,
            kind: Kind::For { var, list, body },
        }
    }

    fn parse_loop(&mut self) -> Stmt {
        let line = self.lines[self.pos].no;
        let (_, _, _, rest) = self.current().expect("checked by block()");
        self.pos += 1;
        let count = if rest.trim().is_empty() {
            None
        } else {
            Some(rest)
        };
        let body = self.block(END_WORDS);
        self.close_block("loop", line);
        Stmt {
            line,
            kind: Kind::Loop { count, body },
        }
    }

    fn parse_func(&mut self) -> Stmt {
        let line = self.lines[self.pos].no;
        let (_, _, word, rest) = self.current().expect("checked by block()");
        self.pos += 1;

        let (name, params) = parse_signature(&rest);
        if name.is_empty() {
            self.error(line, format!("`{word}` needs a function name"));
        }
        let body = self.block(END_WORDS);
        self.close_block(word.as_str(), line);
        Stmt {
            line,
            kind: Kind::Func { name, params, body },
        }
    }

    /// A single-line statement (no block).
    fn simple(&mut self, line: usize, text: &str) -> Option<Stmt> {
        let (word, rest) = split_word(text);
        let kind = match word.to_ascii_lowercase().as_str() {
            "set" => match rest.split_once('=') {
                Some((name, value)) if !name.trim().is_empty() => Kind::Set {
                    name: name.trim().to_string(),
                    expr: value.trim().to_string(),
                },
                // A malformed `set` stays a command, as it always has.
                _ => Kind::Cmd {
                    text: text.to_string(),
                    force: false,
                },
            },
            "call" => match parse_call(rest) {
                Some((name, args)) => Kind::Call { name, args },
                None => {
                    self.error(line, "`call` needs a function name");
                    return None;
                }
            },
            "return" => Kind::Return(if rest.trim().is_empty() {
                None
            } else {
                Some(rest.to_string())
            }),
            "input" => match parse_input(rest) {
                Some((name, prompt)) => Kind::Input { name, prompt },
                None => {
                    self.error(line, "`input` needs a variable name");
                    return None;
                }
            },
            "break" => Kind::Break,
            "continue" => Kind::Continue,
            "exit" => Kind::Exit,
            _ => {
                if let Some(forced) = text.strip_prefix('@') {
                    let forced = forced.trim();
                    if forced.is_empty() {
                        self.error(line, "empty '@' line");
                        return None;
                    }
                    Kind::Cmd {
                        text: forced.to_string(),
                        force: true,
                    }
                } else {
                    Kind::Cmd {
                        text: text.to_string(),
                        force: false,
                    }
                }
            }
        };
        Some(Stmt { line, kind })
    }
}

/// Split `name` into a leading word and the rest of the line.
fn split_word(text: &str) -> (&str, &str) {
    let trimmed = text.trim_start();
    match trimmed.find(char::is_whitespace) {
        Some(idx) => (&trimmed[..idx], trimmed[idx..].trim_start()),
        None => (trimmed, ""),
    }
}

/// Parse `name(a, b)` / `name (a, b)` / `name`.
fn parse_signature(rest: &str) -> (String, Vec<String>) {
    let rest = rest.trim();
    if let Some(open) = rest.find('(') {
        let name = rest[..open].trim().to_string();
        let inner = rest[open + 1..]
            .rsplit_once(')')
            .map(|(before, _)| before)
            .unwrap_or(&rest[open + 1..]);
        let params = split_top_level(inner)
            .into_iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
        return (name, params);
    }
    let mut parts = rest.split_whitespace();
    (
        parts.next().unwrap_or("").to_string(),
        parts.map(|p| p.to_string()).collect(),
    )
}

/// Parse `name(a, b)` or `name a b`.
pub(crate) fn parse_call(rest: &str) -> Option<(String, Vec<String>)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }
    if let Some(open) = rest.find('(') {
        if rest.ends_with(')') {
            let name = rest[..open].trim().to_string();
            if name.is_empty() {
                return None;
            }
            let inner = &rest[open + 1..rest.len() - 1];
            let args = split_top_level(inner)
                .into_iter()
                .map(|a| a.trim().to_string())
                .filter(|a| !a.is_empty())
                .collect();
            return Some((name, args));
        }
    }
    let mut parts = rest.split_whitespace();
    let name = parts.next()?.to_string();
    Some((name, parts.map(|p| p.to_string()).collect()))
}

/// Parse `NAME [prompt]`.
fn parse_input(rest: &str) -> Option<(String, Option<String>)> {
    let (name, tail) = split_word(rest.trim());
    if name.is_empty() {
        return None;
    }
    let prompt = if tail.trim().is_empty() {
        None
    } else {
        Some(tail.trim().to_string())
    };
    Some((name.to_string(), prompt))
}

/// Split on commas that are not inside quotes or parentheses.
fn split_top_level(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    let mut quote: Option<char> = None;

    for ch in input.chars() {
        match quote {
            Some(q) => {
                current.push(ch);
                if ch == q {
                    quote = None;
                }
            }
            None => match ch {
                '"' | '\'' => {
                    quote = Some(ch);
                    current.push(ch);
                }
                '(' => {
                    depth += 1;
                    current.push(ch);
                }
                ')' => {
                    depth = depth.saturating_sub(1);
                    current.push(ch);
                }
                ',' if depth == 0 => {
                    out.push(std::mem::take(&mut current));
                }
                _ => current.push(ch),
            },
        }
    }
    if !current.trim().is_empty() {
        out.push(current);
    }
    out
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(content: &str) -> Vec<Stmt> {
        let (stmts, errors) = parse(content);
        assert!(errors.is_empty(), "unexpected errors: {errors:?}");
        stmts
    }

    fn first_kind(content: &str) -> Kind {
        parse_ok(content).into_iter().next().unwrap().kind
    }

    #[test]
    fn test_blank_and_comments_skipped() {
        let stmts = parse_ok("# comment\n\n// also comment\n   \nsay hi\n");
        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0].kind,
            Kind::Cmd {
                text: "say hi".into(),
                force: false
            }
        );
    }

    #[test]
    fn test_set_parses() {
        assert_eq!(
            first_kind("set name = world"),
            Kind::Set {
                name: "name".into(),
                expr: "world".into()
            }
        );
    }

    #[test]
    fn test_set_keeps_quotes_for_the_evaluator() {
        assert_eq!(
            first_kind(r#"set msg = "hello world""#),
            Kind::Set {
                name: "msg".into(),
                expr: r#""hello world""#.into()
            }
        );
    }

    #[test]
    fn test_force_prefix() {
        assert_eq!(
            first_kind("@ echo forced"),
            Kind::Cmd {
                text: "echo forced".into(),
                force: true
            }
        );
    }

    #[test]
    fn test_inline_comment_stripped() {
        assert_eq!(
            first_kind("say hello # a comment"),
            Kind::Cmd {
                text: "say hello".into(),
                force: false
            }
        );
    }

    #[test]
    fn test_inline_comment_respects_quotes() {
        assert_eq!(
            first_kind(r#"say "a # b" # comment"#),
            Kind::Cmd {
                text: r#"say "a # b""#.into(),
                force: false
            }
        );
    }

    #[test]
    fn test_malformed_set_falls_through() {
        assert_eq!(
            first_kind("set is a command"),
            Kind::Cmd {
                text: "set is a command".into(),
                force: false
            }
        );
    }

    #[test]
    fn line_numbers_survive_skipped_lines() {
        let stmts = parse_ok("# c\n\nsay one\n\nsay two");
        assert_eq!(stmts[0].line, 3);
        assert_eq!(stmts[1].line, 5);
    }

    #[test]
    fn if_else_if_else_parses() {
        let stmts = parse_ok("if $a\n  say a\nelse if $b\n  say b\nelse\n  say c\nend");
        match &stmts[0].kind {
            Kind::If {
                branches,
                otherwise,
            } => {
                assert_eq!(branches.len(), 2);
                assert_eq!(branches[0].cond, "$a");
                assert_eq!(branches[1].cond, "$b");
                assert!(otherwise.is_some());
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn else_if_without_space_is_one_arm() {
        let stmts = parse_ok("if $a\n say a\nelseif $b\n say b\nend");
        match &stmts[0].kind {
            Kind::If { branches, .. } => assert_eq!(branches.len(), 2),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn nested_blocks_close_in_order() {
        let stmts = parse_ok("while $i < 3\n  if $i == 1\n    say one\n  end\n  set i = $i + 1\nend");
        match &stmts[0].kind {
            Kind::While { body, .. } => {
                assert_eq!(body.len(), 2);
                assert!(matches!(body[0].kind, Kind::If { .. }));
                assert!(matches!(body[1].kind, Kind::Set { .. }));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn alternate_end_words_are_accepted() {
        assert!(parse("if 1\n say x\nendif").1.is_empty());
        assert!(parse("while 1\n break\n done").1.is_empty());
        assert!(parse("if 1\n say x\n}").1.is_empty());
    }

    #[test]
    fn for_and_loop_parse() {
        assert_eq!(
            first_kind("for f in a b c\n  say $f\nend"),
            Kind::For {
                var: "f".into(),
                list: "a b c".into(),
                body: vec![Stmt {
                    line: 2,
                    kind: Kind::Cmd {
                        text: "say $f".into(),
                        force: false
                    }
                }]
            }
        );
        match first_kind("loop 3\n say x\nend") {
            Kind::Loop { count, body } => {
                assert_eq!(count.as_deref(), Some("3"));
                assert_eq!(body.len(), 1);
            }
            other => panic!("{other:?}"),
        }
        match first_kind("loop\n break\nend") {
            Kind::Loop { count, .. } => assert!(count.is_none()),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn func_and_call_parse() {
        match first_kind("func greet(who, other)\n  say hi\nend") {
            Kind::Func { name, params, body } => {
                assert_eq!(name, "greet");
                assert_eq!(params, vec!["who".to_string(), "other".to_string()]);
                assert_eq!(body.len(), 1);
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(
            first_kind("call greet(\"a\", \"b\")"),
            Kind::Call {
                name: "greet".into(),
                args: vec!["\"a\"".into(), "\"b\"".into()]
            }
        );
        assert_eq!(
            first_kind("call greet a b"),
            Kind::Call {
                name: "greet".into(),
                args: vec!["a".into(), "b".into()]
            }
        );
    }

    #[test]
    fn control_words_parse() {
        assert_eq!(first_kind("return 1"), Kind::Return(Some("1".into())));
        assert_eq!(first_kind("return"), Kind::Return(None));
        assert_eq!(first_kind("break"), Kind::Break);
        assert_eq!(first_kind("continue"), Kind::Continue);
        assert_eq!(first_kind("exit"), Kind::Exit);
        assert_eq!(
            first_kind(r#"input name "Your name""#),
            Kind::Input {
                name: "name".into(),
                prompt: Some(r#""Your name""#.into())
            }
        );
    }

    #[test]
    fn missing_end_is_reported() {
        let (_, errors) = parse("if 1\n say x");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("no matching `end`"));
        assert_eq!(errors[0].line, 1);
    }

    #[test]
    fn stray_end_is_reported() {
        let (_, errors) = parse("say x\nend");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("does not close any block"));
    }

    #[test]
    fn else_outside_if_is_reported() {
        let (_, errors) = parse("else\n say x");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn for_without_in_is_reported() {
        let (_, errors) = parse("for f\n say x\nend");
        assert!(errors[0].message.contains("needs `in`"));
    }

    #[test]
    fn call_without_name_is_reported() {
        let (_, errors) = parse("call\n");
        assert!(errors[0].message.contains("needs a function name"));
    }

    #[test]
    fn directives_are_not_statements() {
        let stmts = parse_ok("set-pipe = fail\nset-showexc = 0\nsay hi\n");
        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0].kind,
            Kind::Cmd {
                text: "say hi".into(),
                force: false
            }
        );
        // Line numbers still count the skipped directive lines.
        assert_eq!(stmts[0].line, 3);
    }

    #[test]
    fn parse_result_is_ok() {
        let _ = parse("cd src\nsay hi");
    }

    #[test]
    fn test_order_preserved() {
        let stmts = parse_ok("set a = 1\n@ ls\nsay ok");
        assert!(matches!(stmts[0].kind, Kind::Set { .. }));
        assert!(matches!(
            stmts[1].kind,
            Kind::Cmd { force: true, .. }
        ));
        assert!(matches!(
            stmts[2].kind,
            Kind::Cmd { force: false, .. }
        ));
    }
}
