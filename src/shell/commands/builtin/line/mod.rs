//! `line` — prefix every input line with a running number.
//!
//!   line [-A <prefix>] [-B <separator>] [file]
//!
//! Each input line becomes `<prefix><n><separator> <line>`. One space is
//! inserted after the separator unless it already ends in whitespace, so
//! `cmd | line -A 'line' -B ':'` prints `line1: first line`, `line2: ...`
//! and the bare `line` prints `1: first line`. Line endings (LF, CRLF and a
//! missing final newline) are preserved so the output composes in pipelines.

pub(crate) mod help;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::navigator::Navigator;
use crate::output::{print_error, read_file_as_string};
use crate::shell::stdin;
use crate::t;
use anyhow::Result;
use std::path::Path;

/// Quote-aware whitespace split (double quotes only, like `rev`), so
/// `line -A "step " -B ':'` keeps the prefix intact.
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

#[derive(Debug, PartialEq, Eq)]
struct LineArgs {
    prefix: String,
    separator: String,
    file: Option<String>,
}

/// Parse `line [-A <prefix>] [-B <separator>] [file]`. The error is the
/// offending token (a bad flag, a flag missing its value, or an extra path).
fn parse_args(args: &str) -> std::result::Result<LineArgs, String> {
    let mut parsed = LineArgs {
        prefix: String::new(),
        separator: ":".to_string(),
        file: None,
    };
    let mut tokens = tokenize(args).into_iter();
    while let Some(tok) = tokens.next() {
        match tok.as_str() {
            "-A" | "--prefix" => {
                parsed.prefix = tokens.next().ok_or_else(|| tok.clone())?;
            }
            "-B" | "--separator" => {
                parsed.separator = tokens.next().ok_or_else(|| tok.clone())?;
            }
            // A lone `-` is the stdin marker, not a filename.
            "-" => {}
            t if t.starts_with('-') && t.len() > 1 => return Err(tok),
            _ => {
                if parsed.file.is_some() {
                    return Err(tok);
                }
                parsed.file = Some(tok);
            }
        }
    }
    Ok(parsed)
}

/// Split one `split_inclusive('\n')` chunk into its body and terminator.
fn split_ending(chunk: &str) -> (&str, &str) {
    if let Some(body) = chunk.strip_suffix("\r\n") {
        (body, "\r\n")
    } else if let Some(body) = chunk.strip_suffix('\n') {
        (body, "\n")
    } else {
        (chunk, "")
    }
}

/// Number every line, preserving each line's own terminator.
fn number_lines(text: &str, prefix: &str, separator: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for (idx, chunk) in text.split_inclusive('\n').enumerate() {
        let (body, ending) = split_ending(chunk);
        out.push_str(prefix);
        out.push_str(&(idx + 1).to_string());
        out.push_str(separator);
        if !separator.ends_with(char::is_whitespace) {
            out.push(' ');
        }
        out.push_str(body);
        out.push_str(ending);
    }
    out
}

/// Read text from an explicit file (which wins) or piped/redirected input.
fn read_source(file: &Option<String>) -> Option<String> {
    if let Some(path) = file {
        let expanded = crate::utils::paths::expand_path_str(path);
        match read_file_as_string(Path::new(&expanded)) {
            Ok(text) => return Some(text),
            Err(_) => {
                print_error(&t!("cmd.report.file_not_found", path));
                return None;
            }
        }
    }
    stdin::read_to_string()
}

pub fn cmd_line(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_args(args) {
        Ok(p) => p,
        Err(bad) => {
            print_error(&t!("misc.line.bad_arg", bad));
            println!("{}", t!("misc.line.usage"));
            return Ok(false);
        }
    };

    let text = match read_source(&parsed.file) {
        Some(text) => text,
        None => {
            if parsed.file.is_none() {
                print_error(&t!("misc.window.no_input", "line"));
                println!("{}", t!("misc.line.usage"));
            }
            return Ok(false);
        }
    };

    print!("{}", number_lines(&text, &parsed.prefix, &parsed.separator));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "line",
    aliases: &[],
    help_topic: Some("line"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_line),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_every_line_with_the_default_separator() {
        assert_eq!(number_lines("a\nb\n", "", ":"), "1: a\n2: b\n");
    }

    #[test]
    fn prefix_and_separator_build_the_requested_header() {
        assert_eq!(
            number_lines("first\nsecond\n", "line", ":"),
            "line1: first\nline2: second\n"
        );
    }

    #[test]
    fn separator_ending_in_space_is_not_followed_by_another() {
        assert_eq!(number_lines("a\n", "", "- "), "1- a\n");
    }

    #[test]
    fn keeps_a_missing_final_newline_missing() {
        assert_eq!(number_lines("a\nb", "", ":"), "1: a\n2: b");
    }

    #[test]
    fn preserves_crlf_endings() {
        assert_eq!(number_lines("a\r\nb\r\n", "", ":"), "1: a\r\n2: b\r\n");
    }

    #[test]
    fn empty_input_produces_no_output() {
        assert_eq!(number_lines("", "", ":"), "");
    }

    #[test]
    fn counts_empty_lines() {
        assert_eq!(number_lines("a\n\nb\n", "", ":"), "1: a\n2: \n3: b\n");
    }

    #[test]
    fn parse_reads_flags_and_one_optional_file() {
        assert_eq!(
            parse_args(""),
            Ok(LineArgs {
                prefix: String::new(),
                separator: ":".to_string(),
                file: None,
            })
        );
        assert_eq!(
            parse_args("-A line -B : data.txt"),
            Ok(LineArgs {
                prefix: "line".to_string(),
                separator: ":".to_string(),
                file: Some("data.txt".to_string()),
            })
        );
        assert_eq!(
            parse_args("--prefix \"step \" --separator \") \""),
            Ok(LineArgs {
                prefix: "step ".to_string(),
                separator: ") ".to_string(),
                file: None,
            })
        );
        // A lone `-` means stdin, so it is never a filename.
        assert_eq!(parse_args("-").map(|p| p.file), Ok(None));
        // Bad flags, missing values and extra paths are rejected.
        assert!(parse_args("-x").is_err());
        assert!(parse_args("-A").is_err());
        assert!(parse_args("a.txt b.txt").is_err());
    }
}
