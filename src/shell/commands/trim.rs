// src/shell/commands/trim.rs
//! `head`, `tail`, `betw` — line-window filters for the ntc shell.
//!
//! Each reads its lines from an explicit file argument (which wins) or from
//! piped / redirected input, then prints a slice:
//!
//!   head [N] [file]    first N lines         (N defaults to 10)
//!   tail [N] [file]    last N lines          (N defaults to 10)
//!   betw A:B [file]    lines A..B inclusive  (1-based)
//!
//! Output is plain — no line numbers, no header — so the commands compose in
//! pipelines; the pipeline capture path strips any ANSI when redirected.

use crate::navigator::Navigator;
use crate::output::{print_error, read_file_as_string, slice_numbered_lines, FileLineRange};
use crate::shell::stdin;
use crate::t;
use anyhow::Result;
use std::path::Path;

/// Default line count for a bare `head` / `tail` (matches the Unix tools).
const DEFAULT_COUNT: usize = 10;

/// Which line window a command prints.
#[derive(Clone, Copy)]
enum Kind {
    Head,
    Tail,
    Betw,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Head => "head",
            Kind::Tail => "tail",
            Kind::Betw => "betw",
        }
    }

    fn usage_key(self) -> &'static str {
        match self {
            Kind::Head => "misc.head.usage",
            Kind::Tail => "misc.tail.usage",
            Kind::Betw => "misc.betw.usage",
        }
    }
}

/// Why parsing failed; the caller turns each into a localized message.
enum ParseError {
    /// Missing required argument (`betw` with no range).
    Usage,
    /// Malformed `betw A:B` range.
    Range(String),
    /// Unexpected extra or flag-like token.
    BadArg(String),
}

/// Quote-aware whitespace split (double quotes only, mirroring the shell's
/// other arg parsers) so `head 5 "my file.txt"` keeps the path intact.
fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            c if c.is_whitespace() && !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Parse `[N] [file]` (head/tail) or `A:B [file]` (betw) into a range plus an
/// optional file path. The count/range must come before the file; any token
/// starting with `-` is rejected (there are no flags).
fn parse(
    kind: Kind,
    args: &str,
) -> std::result::Result<(FileLineRange, Option<String>), ParseError> {
    let toks = tokenize(args);
    let mut file: Option<String> = None;

    match kind {
        Kind::Head | Kind::Tail => {
            let mut count: Option<usize> = None;
            for tok in toks {
                if count.is_none() && file.is_none() {
                    if let Ok(n) = tok.parse::<usize>() {
                        count = Some(n);
                        continue;
                    }
                }
                if file.is_none() && !tok.starts_with('-') {
                    file = Some(tok);
                } else {
                    return Err(ParseError::BadArg(tok));
                }
            }
            let n = count.unwrap_or(DEFAULT_COUNT);
            let range = if matches!(kind, Kind::Head) {
                FileLineRange::Head(n)
            } else {
                FileLineRange::Tail(n)
            };
            Ok((range, file))
        }
        Kind::Betw => {
            let mut range: Option<FileLineRange> = None;
            for tok in toks {
                if range.is_none() {
                    if let Some((a, b)) = crate::output::parse_betw(&tok) {
                        range = Some(FileLineRange::Between(a, b));
                        continue;
                    }
                    // A token that looks like a range but is malformed is an
                    // error, not a filename.
                    if tok.contains(':') {
                        return Err(ParseError::Range(tok));
                    }
                }
                if file.is_none() && !tok.starts_with('-') {
                    file = Some(tok);
                } else {
                    return Err(ParseError::BadArg(tok));
                }
            }
            match range {
                Some(r) => Ok((r, file)),
                None => Err(ParseError::Usage),
            }
        }
    }
}

/// Shared entry point: parse, read, slice and print.
fn run(kind: Kind, args: &str) -> Result<bool> {
    let (range, file) = match parse(kind, args) {
        Ok(v) => v,
        Err(ParseError::Usage) => {
            print_error(&t!(kind.usage_key()));
            return Ok(false);
        }
        Err(ParseError::Range(v)) => {
            print_error(&t!("cmd.report.invalid_range", v));
            println!("{}", t!(kind.usage_key()));
            return Ok(false);
        }
        Err(ParseError::BadArg(v)) => {
            print_error(&t!("misc.window.bad_arg", kind.name(), v));
            println!("{}", t!(kind.usage_key()));
            return Ok(false);
        }
    };

    let text = match read_source(&file) {
        Some(text) => text,
        None => {
            // `read_source` reports a missing file itself; only the
            // no-input-at-all case still needs a message.
            if file.is_none() {
                print_error(&t!("misc.window.no_input", kind.name()));
                println!("{}", t!(kind.usage_key()));
            }
            return Ok(false);
        }
    };

    let lines: Vec<&str> = text.lines().collect();
    let mut out = String::new();
    for (_, line) in slice_numbered_lines(&lines, Some(range)) {
        out.push_str(line);
        out.push('\n');
    }
    print!("{out}");
    Ok(false)
}

/// Read lines from an explicit file (which wins) or piped/redirected input.
/// Reports a missing file itself; returns `None` on any failure.
fn read_source(file: &Option<String>) -> Option<String> {
    if let Some(path) = file {
        match read_file_as_string(Path::new(path)) {
            Ok(text) => return Some(text),
            Err(_) => {
                print_error(&t!("cmd.report.file_not_found", path));
                return None;
            }
        }
    }
    stdin::read_to_string()
}

pub fn cmd_head(args: &str, _nav: &mut Navigator) -> Result<bool> {
    run(Kind::Head, args)
}

pub fn cmd_tail(args: &str, _nav: &mut Navigator) -> Result<bool> {
    run(Kind::Tail, args)
}

pub fn cmd_betw(args: &str, _nav: &mut Navigator) -> Result<bool> {
    run(Kind::Betw, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range_of(kind: Kind, args: &str) -> (FileLineRange, Option<String>) {
        parse(kind, args).ok().expect("expected parse to succeed")
    }

    #[test]
    fn head_defaults_to_ten_lines() {
        let (r, f) = range_of(Kind::Head, "");
        assert_eq!(r, FileLineRange::Head(DEFAULT_COUNT));
        assert_eq!(r, FileLineRange::Head(10));
        assert!(f.is_none());
    }

    #[test]
    fn head_parses_count_before_file() {
        assert_eq!(range_of(Kind::Head, "5"), (FileLineRange::Head(5), None));
        assert_eq!(
            range_of(Kind::Head, "5 app.log"),
            (FileLineRange::Head(5), Some("app.log".to_string()))
        );
        assert_eq!(range_of(Kind::Head, "0"), (FileLineRange::Head(0), None));
        // A number after the file is not a count — it is an extra argument.
        assert!(matches!(
            parse(Kind::Head, "app.log 3"),
            Err(ParseError::BadArg(_))
        ));
    }

    #[test]
    fn tail_parses_like_head() {
        assert_eq!(range_of(Kind::Tail, "").0, FileLineRange::Tail(10));
        assert_eq!(
            range_of(Kind::Tail, "20 app.log"),
            (FileLineRange::Tail(20), Some("app.log".to_string()))
        );
    }

    #[test]
    fn quoted_file_with_spaces_survives() {
        let (r, f) = range_of(Kind::Head, "5 \"my file.txt\"");
        assert_eq!(r, FileLineRange::Head(5));
        assert_eq!(f, Some("my file.txt".to_string()));
    }

    #[test]
    fn betw_requires_a_valid_range() {
        assert!(matches!(parse(Kind::Betw, ""), Err(ParseError::Usage)));
        assert_eq!(
            range_of(Kind::Betw, "3:7"),
            (FileLineRange::Between(3, 7), None)
        );
        assert_eq!(
            range_of(Kind::Betw, "3:7 app.log").1,
            Some("app.log".to_string())
        );
        // Malformed ranges are errors, not filenames.
        assert!(matches!(
            parse(Kind::Betw, "10:"),
            Err(ParseError::Range(_))
        ));
        assert!(matches!(
            parse(Kind::Betw, "20:10"),
            Err(ParseError::Range(_))
        ));
        assert!(matches!(
            parse(Kind::Betw, "a:b"),
            Err(ParseError::Range(_))
        ));
        // `betw <file>` has no range at all.
        assert!(matches!(
            parse(Kind::Betw, "app.log"),
            Err(ParseError::Usage)
        ));
    }

    #[test]
    fn flags_and_extras_are_rejected() {
        assert!(matches!(
            parse(Kind::Head, "-5"),
            Err(ParseError::BadArg(_))
        ));
        assert!(matches!(
            parse(Kind::Head, "5 a.txt b.txt"),
            Err(ParseError::BadArg(_))
        ));
        assert!(matches!(
            parse(Kind::Betw, "3:7 a.txt b.txt"),
            Err(ParseError::BadArg(_))
        ));
    }
}
