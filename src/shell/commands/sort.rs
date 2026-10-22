// src/shell/commands/sort.rs
//! `sort` — line-sorting filter for the ntc shell.
//!
//! Reads an explicit file argument (which wins) or piped / redirected input,
//! then prints the lines sorted:
//!
//!   sort [-r] [-n] [-u] [-f] [file]
//!
//!   -r/--reverse       descending order
//!   -n/--numeric       compare the trimmed line as a number
//!   -u/--unique        drop duplicate lines (by the active comparison key)
//!   -f/--ignore-case   case-insensitive comparison
//!
//! Output is plain (no header, no line numbers) so it composes in pipelines.

use crate::navigator::Navigator;
use crate::output::{print_error, read_file_as_string};
use crate::shell::stdin;
use crate::t;
use anyhow::Result;
use std::path::Path;

/// Sorting behaviour selected by the command-line flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct SortOptions {
    reverse: bool,
    numeric: bool,
    unique: bool,
    ignore_case: bool,
}

/// Parsed command line: options plus an optional file path.
struct Parsed {
    options: SortOptions,
    file: Option<String>,
}

/// Why parsing failed; the caller localizes the message.
#[derive(Debug)]
enum ParseError {
    /// Unknown `-x` / `--long` flag.
    BadFlag(String),
    /// More than one non-flag argument.
    ExtraArg(String),
}

/// Quote-aware whitespace split (double quotes only, mirroring the shell's
/// other arg parsers) so `sort "my file.txt"` keeps the path intact.
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

/// Parse flags (long or combined short) plus at most one file argument.
fn parse_args(args: &str) -> std::result::Result<Parsed, ParseError> {
    let mut options = SortOptions::default();
    let mut file: Option<String> = None;
    let mut end_of_flags = false;

    for tok in tokenize(args) {
        if !end_of_flags && tok == "--" {
            end_of_flags = true;
            continue;
        }
        if !end_of_flags && tok.starts_with("--") {
            match tok.as_str() {
                "--reverse" => options.reverse = true,
                "--numeric" => options.numeric = true,
                "--unique" => options.unique = true,
                "--ignore-case" => options.ignore_case = true,
                _ => return Err(ParseError::BadFlag(tok)),
            }
            continue;
        }
        // `-` (or nothing) means stdin, so it is never a filename here.
        if !end_of_flags && tok.starts_with('-') && tok.len() > 1 {
            for ch in tok.chars().skip(1) {
                match ch {
                    'r' => options.reverse = true,
                    'n' => options.numeric = true,
                    'u' => options.unique = true,
                    'f' => options.ignore_case = true,
                    _ => return Err(ParseError::BadFlag(format!("-{ch}"))),
                }
            }
            continue;
        }
        if tok == "-" {
            continue;
        }
        if file.is_none() {
            file = Some(tok);
        } else {
            return Err(ParseError::ExtraArg(tok));
        }
    }

    Ok(Parsed { options, file })
}

/// Sort `text`'s lines according to `opts`. Pure, so it is unit-testable.
fn sort_lines(text: &str, opts: &SortOptions) -> Vec<String> {
    // (numeric key, comparison key, original line)
    let mut rows: Vec<(f64, String, String)> = text
        .lines()
        .map(|line| {
            let cmp = if opts.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            let num = if opts.numeric {
                line.trim().parse::<f64>().unwrap_or(0.0)
            } else {
                0.0
            };
            (num, cmp, line.to_string())
        })
        .collect();

    if opts.numeric {
        rows.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.1.cmp(&b.1))
        });
    } else {
        rows.sort_by(|a, b| a.1.cmp(&b.1));
    }

    if opts.unique {
        // Rows are sorted, so duplicates are adjacent.
        rows.dedup_by(|a, b| {
            if opts.numeric {
                a.0 == b.0 && a.1 == b.1
            } else {
                a.1 == b.1
            }
        });
    }

    if opts.reverse {
        rows.reverse();
    }

    rows.into_iter().map(|(_, _, line)| line).collect()
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

pub fn cmd_sort(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_args(args) {
        Ok(p) => p,
        Err(ParseError::BadFlag(v)) => {
            print_error(&t!("misc.sort.bad_arg", v));
            println!("{}", t!("misc.sort.usage"));
            return Ok(false);
        }
        Err(ParseError::ExtraArg(v)) => {
            print_error(&t!("misc.window.bad_arg", "sort", v));
            println!("{}", t!("misc.sort.usage"));
            return Ok(false);
        }
    };

    let text = match read_source(&parsed.file) {
        Some(text) => text,
        None => {
            if parsed.file.is_none() {
                print_error(&t!("misc.window.no_input", "sort"));
                println!("{}", t!("misc.sort.usage"));
            }
            return Ok(false);
        }
    };

    let sorted = sort_lines(&text, &parsed.options);
    if !sorted.is_empty() {
        let mut out = sorted.join("\n");
        out.push('\n');
        print!("{out}");
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(args: &str) -> SortOptions {
        parse_args(args)
            .ok()
            .expect("expected parse to succeed")
            .options
    }

    fn sorted(text: &str, args: &str) -> Vec<String> {
        sort_lines(text, &opts(args))
    }

    fn v(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn default_sorts_lexicographically() {
        assert_eq!(
            sorted("pear\napple\nbanana\n", ""),
            v(&["apple", "banana", "pear"])
        );
    }

    #[test]
    fn reverse_reverses_the_order() {
        assert_eq!(sorted("a\nc\nb\n", "-r"), v(&["c", "b", "a"]));
        assert_eq!(sorted("a\nc\nb\n", "--reverse"), v(&["c", "b", "a"]));
    }

    #[test]
    fn numeric_orders_by_value_with_lexical_tiebreak() {
        // "2" and "2.0" have the same value, so the tie breaks on text.
        assert_eq!(sorted("10\n2\n1\n2.0\n", "-n"), v(&["1", "2", "2.0", "10"]));
        // Non-numeric lines behave like 0.
        assert_eq!(sorted("x\n5\n", "-n"), v(&["x", "5"]));
    }

    #[test]
    fn unique_drops_adjacent_duplicates_after_sorting() {
        assert_eq!(sorted("b\na\nb\na\n", "-u"), v(&["a", "b"]));
        // Reverse+unique: descending, still deduped.
        assert_eq!(sorted("b\na\nb\na\n", "-ru"), v(&["b", "a"]));
    }

    #[test]
    fn ignore_case_compares_lowercased() {
        // "Apple" and "apple" compare equal, so unique keeps the first.
        assert_eq!(
            sorted("Banana\napple\nApple\n", "-fu"),
            v(&["apple", "Banana"])
        );
        // Without -f, case matters (uppercase sorts first).
        assert_eq!(
            sorted("Banana\napple\nApple\n", "-u"),
            v(&["Apple", "Banana", "apple"])
        );
    }

    #[test]
    fn combined_short_flags_and_long_forms_parse() {
        assert_eq!(
            opts("-rnuf"),
            SortOptions {
                reverse: true,
                numeric: true,
                unique: true,
                ignore_case: true,
            }
        );
        assert_eq!(
            opts("--numeric --unique --ignore-case"),
            SortOptions {
                reverse: false,
                numeric: true,
                unique: true,
                ignore_case: true,
            }
        );
    }

    #[test]
    fn file_argument_and_end_of_flags() {
        // Two bare tokens: the second is an extra argument.
        assert!(matches!(
            parse_args("5 file.txt"),
            Err(ParseError::ExtraArg(_))
        ));
        let p = parse_args("-n data.txt").expect("parse");
        assert_eq!(p.file.as_deref(), Some("data.txt"));
        let p = parse_args("-- -weird.txt").expect("parse");
        assert_eq!(p.file.as_deref(), Some("-weird.txt"));
        assert!(matches!(
            parse_args("a.txt b.txt"),
            Err(ParseError::ExtraArg(_))
        ));
        assert!(matches!(parse_args("--bogus"), Err(ParseError::BadFlag(_))));
        assert!(matches!(parse_args("-z"), Err(ParseError::BadFlag(_))));
    }

    #[test]
    fn empty_input_yields_nothing() {
        assert!(sorted("", "").is_empty());
    }

    #[test]
    fn stdin_marker_is_not_a_file() {
        let p = parse_args("-n -").expect("parse");
        assert!(p.file.is_none());
    }
}
