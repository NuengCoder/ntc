// src/shell/commands/rev.rs
//! `rev` — reverse-each-line filter for the ntc shell.
//!
//! Reads an explicit file argument (which wins) or piped / redirected input,
//! then prints every line with its characters reversed (Unix `rev`):
//!
//!   rev [-u] [file]
//!
//! With `-u` the text is flipped upside down instead: the first line swaps
//! with the last, the second with the second-to-last, loop until the middle.
//!
//! Line endings are preserved (a missing final newline stays missing, CRLF
//! stays CRLF), so the output composes in pipelines exactly like `head` /
//! `sort`.

pub(crate) mod help;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::navigator::Navigator;
use crate::output::{print_error, read_file_as_string};
use crate::shell::stdin;
use crate::t;
use anyhow::Result;
use std::path::Path;

/// Quote-aware whitespace split (double quotes only, mirroring the shell's
/// other arg parsers) so `rev "my file.txt"` keeps the path intact.
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

/// Parse `rev [-u] [file]` into the reversed-numbering flag and an optional
/// file path. Unknown `-x` / `--long` tokens and a second bare token are errors.
fn parse_args(args: &str) -> std::result::Result<(bool, Option<String>), String> {
    let mut file: Option<String> = None;
    let mut upend = false;
    for tok in tokenize(args) {
        // A lone `-` is the stdin marker (like `sort`/`head`), not a filename.
        if tok == "-" {
            continue;
        }
        if tok == "-u" {
            upend = true;
            continue;
        }
        if tok.starts_with('-') && tok.len() > 1 {
            return Err(tok);
        }
        if file.is_none() {
            file = Some(tok);
        } else {
            return Err(tok);
        }
    }
    Ok((upend, file))
}

/// Split one `split_inclusive('\n')` chunk into its body and terminator
/// (`\n`, `\r\n`, or nothing for a missing final newline).
fn split_ending(line: &str) -> (&str, &str) {
    if let Some(b) = line.strip_suffix("\r\n") {
        (b, "\r\n")
    } else if let Some(b) = line.strip_suffix('\n') {
        (b, "\n")
    } else {
        (line, "")
    }
}

/// Reverse the characters of one line, keeping its terminator (`\n` or
/// `\r\n`) in place.
fn reverse_line(line: &str) -> String {
    let (body, ending) = split_ending(line);
    let mut out: String = body.chars().rev().collect();
    out.push_str(ending);
    out
}

/// Reverse every line of `text`, preserving each line's terminator. Pure, so
/// it is unit-testable.
fn reverse_text(text: &str) -> String {
    text.split_inclusive('\n').map(reverse_line).collect()
}

/// Flip the text upside down: swap the first line with the last, the second
/// with the second-to-last, and so on until the middle. Only the line bodies
/// move; terminators stay in their original positions, so `a\nb\nc` (no final
/// newline) becomes `c\nb\na`.
fn reverse_line_order(text: &str) -> String {
    let lines: Vec<(&str, &str)> = text.split_inclusive('\n').map(split_ending).collect();
    let mut out = String::with_capacity(text.len());
    for (i, (_, ending)) in lines.iter().enumerate() {
        out.push_str(lines[lines.len() - 1 - i].0);
        out.push_str(ending);
    }
    out
}

/// Read text from an explicit file (which wins) or piped/redirected input.
/// Reports a missing file itself; returns `None` on any failure.
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

pub fn cmd_rev(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let (upend, file) = match parse_args(args) {
        Ok(parsed) => parsed,
        Err(v) => {
            print_error(&t!("misc.rev.bad_arg", v));
            println!("{}", t!("misc.rev.usage"));
            return Ok(false);
        }
    };

    let text = match read_source(&file) {
        Some(text) => text,
        None => {
            if file.is_none() {
                print_error(&t!("misc.window.no_input", "rev"));
                println!("{}", t!("misc.rev.usage"));
            }
            return Ok(false);
        }
    };

    if upend {
        print!("{}", reverse_line_order(&text));
    } else {
        print!("{}", reverse_text(&text));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "rev",
    aliases: &[],
    help_topic: Some("rev"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: false,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_rev),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_characters_per_line() {
        assert_eq!(reverse_text("abc\ndef\n"), "cba\nfed\n");
    }

    #[test]
    fn keeps_a_missing_final_newline_missing() {
        assert_eq!(reverse_text("abc\ndef"), "cba\nfed");
    }

    #[test]
    fn preserves_crlf_endings() {
        assert_eq!(reverse_text("abc\r\ndef\r\n"), "cba\r\nfed\r\n");
    }

    #[test]
    fn reverses_unicode_by_character_not_byte() {
        assert_eq!(reverse_text("héllo\n"), "olléh\n");
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(reverse_text(""), "");
        assert_eq!(reverse_text("\n"), "\n");
    }

    #[test]
    fn parse_takes_one_optional_file() {
        assert_eq!(parse_args(""), Ok((false, None)));
        assert_eq!(
            parse_args("data.txt"),
            Ok((false, Some("data.txt".to_string())))
        );
        assert_eq!(
            parse_args("\"my file.txt\""),
            Ok((false, Some("my file.txt".to_string())))
        );
        // A lone `-` means stdin, so it is never a filename.
        assert_eq!(parse_args("-"), Ok((false, None)));
        // Flags and extra bare tokens are rejected.
        assert!(parse_args("-r").is_err());
        assert!(parse_args("a.txt b.txt").is_err());
    }

    #[test]
    fn parse_accepts_upend_anywhere() {
        assert_eq!(parse_args("-u"), Ok((true, None)));
        assert_eq!(
            parse_args("-u data.txt"),
            Ok((true, Some("data.txt".to_string())))
        );
        assert_eq!(
            parse_args("data.txt -u"),
            Ok((true, Some("data.txt".to_string())))
        );
        assert_eq!(parse_args("-u -u"), Ok((true, None)));
    }

    #[test]
    fn upend_flips_line_order() {
        assert_eq!(reverse_line_order("a\nb\nc\n"), "c\nb\na\n");
        assert_eq!(
            reverse_line_order("1: a\n2: b\n3: c\n4: d\n"),
            "4: d\n3: c\n2: b\n1: a\n"
        );
        assert_eq!(reverse_line_order("only\n"), "only\n");
    }

    #[test]
    fn upend_keeps_terminators_in_place() {
        assert_eq!(reverse_line_order("a\r\nb\r\n"), "b\r\na\r\n");
        // Missing final newline stays missing.
        assert_eq!(reverse_line_order("a\nb\nc"), "c\nb\na");
        // Blank lines are ordinary lines.
        assert_eq!(reverse_line_order("a\n\nb\n"), "b\n\na\n");
    }

    #[test]
    fn upend_of_empty_input_is_empty() {
        assert_eq!(reverse_line_order(""), "");
        assert_eq!(reverse_line_order("\n"), "\n");
    }
}
