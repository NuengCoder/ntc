// src/shell/commands/count.rs
//! `count <needle> [text]` — count how many times `needle` occurs.
//!
//!   count 'the target' 'the target is so nice and the target is so good'  -> 2
//!   gs error | count error                                               -> piped input
//!   gs error | count -l                                                  -> piped line count
//!
//! `-l` / `--line` counts the lines of piped input instead. It needs a pipe
//! (or a `<` redirect), because a text argument would be a single line at
//! best and the point is to measure another command's output.
//!
//! Output is just the number so the command composes in pipelines.

pub(crate) mod completion;
pub(crate) mod help;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::stdin;
use crate::t;
use anyhow::Result;

/// Quote-aware whitespace split (double quotes only, like the other
/// shell arg parsers) so a multi-word needle or text stays one token.
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

/// Non-overlapping occurrences of `needle` in `haystack`.
pub(crate) fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut rest = haystack;
    while let Some(idx) = rest.find(needle) {
        count += 1;
        rest = &rest[idx + needle.len()..];
    }
    count
}

/// Lines in `text`. A trailing newline does not add an empty line, so the
/// output of a normal command counts as its visible rows.
pub(crate) fn count_lines(text: &str) -> usize {
    text.lines().count()
}

pub(crate) fn cmd_count(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);

    if tokens.iter().any(|t| is_line_flag(t)) {
        count_piped_lines(&tokens);
        return Ok(false);
    }

    let Some(needle) = tokens.first() else {
        print_error(&t!("misc.count.usage"));
        return Ok(false);
    };
    let has_text = tokens.len() > 1;
    let haystack = if has_text {
        tokens[1..].join(" ")
    } else {
        match stdin::read_to_string() {
            Some(text) => text,
            None => {
                print_error(&t!("misc.count.usage"));
                return Ok(false);
            }
        }
    };
    println!("{}", count_occurrences(&haystack, needle));
    Ok(false)
}

/// Whether a token asks for line counting. The value form `--line=…` and
/// short bundles such as `-la` / `-al` match too, so they are rejected as
/// misuse instead of being counted as a needle.
fn is_line_flag(tok: &str) -> bool {
    if tok == "-l" || tok == "--line" {
        return true;
    }
    if tok.starts_with("--") {
        return tok.starts_with("--line=");
    }
    // `-l` takes no value and cannot be combined, so any short token that
    // carries an `l` is a bad use of the flag rather than a needle.
    tok.starts_with('-') && tok.len() > 1 && tok[1..].contains('l')
}

/// The flag must stand alone: `count -l <arg>`, `count <arg> -l` and
/// `count --line=5` all fail rather than falling back to needle counting.
fn bare_line_flag(tokens: &[String]) -> bool {
    tokens.len() == 1 && (tokens[0] == "-l" || tokens[0] == "--line")
}

/// `count -l|--line`: print the number of lines in piped input. Any extra
/// argument (a needle or text) is a usage error, and no piped input at all
/// is reported instead of silently printing 0.
fn count_piped_lines(tokens: &[String]) {
    if !bare_line_flag(tokens) {
        if tokens.len() == 1 {
            // A lone `-la` or `--line=5`: name the offending token so the
            // misuse is obvious instead of a generic usage line.
            print_error(&t!("misc.count.line_bad_flag", &tokens[0]));
        } else {
            print_error(&t!("misc.count.line_usage"));
        }
        return;
    }
    match stdin::read_to_string() {
        Some(text) => println!("{}", count_lines(&text)),
        None => print_error(&t!("misc.count.line_no_input")),
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "count",
    aliases: &[],
    help_topic: Some("count"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_count),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_non_overlapping_occurrences() {
        assert_eq!(
            count_occurrences(
                "the target is so nice and the target is so good",
                "the target"
            ),
            2
        );
        assert_eq!(count_occurrences("aaaa", "aa"), 2);
        assert_eq!(count_occurrences("aaaa", "a"), 4);
        assert_eq!(count_occurrences("abc", "z"), 0);
        assert_eq!(count_occurrences("abc", ""), 0);
    }

    #[test]
    fn counts_lines_without_a_phantom_trailing_line() {
        assert_eq!(count_lines(""), 0);
        assert_eq!(count_lines("a\n"), 1);
        assert_eq!(count_lines("a\nb\n"), 2);
        // No trailing newline: the last line still counts.
        assert_eq!(count_lines("a\nb"), 2);
        assert_eq!(count_lines("\n"), 1);
        assert_eq!(count_lines("a\r\nb\r\n"), 2);
    }

    #[test]
    fn line_flag_takes_no_arguments() {
        let toks = |args: &str| tokenize(args);
        assert!(bare_line_flag(&toks("-l")));
        assert!(bare_line_flag(&toks("--line")));
        // Any argument, before or after, makes it a usage error.
        assert!(!bare_line_flag(&toks("-l needle")));
        assert!(!bare_line_flag(&toks("needle -l")));
        assert!(!bare_line_flag(&toks("-l a b")));
        // The value form is flagged so it cannot be counted as a needle.
        assert!(is_line_flag("--line=5"));
        assert!(!bare_line_flag(&toks("--line=5")));
        // Bundled short flags are line-flag misuse, never a needle.
        assert!(is_line_flag("-la"));
        assert!(is_line_flag("-al"));
        assert!(!bare_line_flag(&toks("-la")));
        assert!(!is_line_flag("line"));
        assert!(!is_line_flag("-x"));
    }

    #[test]
    fn tokenizes_quoted_text() {
        let toks = tokenize("\"the target\" \"the target is good\"");
        assert_eq!(
            toks,
            vec!["the target".to_string(), "the target is good".to_string()]
        );
    }
}
