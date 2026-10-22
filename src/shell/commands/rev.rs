// src/shell/commands/rev.rs
//! `rev` — reverse-each-line filter for the ntc shell.
//!
//! Reads an explicit file argument (which wins) or piped / redirected input,
//! then prints every line with its characters reversed (Unix `rev`):
//!
//!   rev [file]
//!
//! Line endings are preserved (a missing final newline stays missing, CRLF
//! stays CRLF) and no flags are accepted, so the output composes in
//! pipelines exactly like `head` / `sort`.

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

/// Parse `rev [file]` into an optional file path. There are no flags, so any
/// `-x` / `--long` token and a second bare token are errors.
fn parse_args(args: &str) -> std::result::Result<Option<String>, String> {
    let mut file: Option<String> = None;
    for tok in tokenize(args) {
        // A lone `-` is the stdin marker (like `sort`/`head`), not a filename.
        if tok == "-" {
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
    Ok(file)
}

/// Reverse the characters of one line, keeping its terminator (`\n` or
/// `\r\n`) in place.
fn reverse_line(line: &str) -> String {
    let (body, ending) = if let Some(b) = line.strip_suffix("\r\n") {
        (b, "\r\n")
    } else if let Some(b) = line.strip_suffix('\n') {
        (b, "\n")
    } else {
        (line, "")
    };
    let mut out: String = body.chars().rev().collect();
    out.push_str(ending);
    out
}

/// Reverse every line of `text`, preserving each line's terminator. Pure, so
/// it is unit-testable.
fn reverse_text(text: &str) -> String {
    text.split_inclusive('\n').map(reverse_line).collect()
}

/// Read text from an explicit file (which wins) or piped/redirected input.
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

pub fn cmd_rev(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let file = match parse_args(args) {
        Ok(f) => f,
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

    print!("{}", reverse_text(&text));
    Ok(false)
}

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
        assert_eq!(parse_args(""), Ok(None));
        assert_eq!(parse_args("data.txt"), Ok(Some("data.txt".to_string())));
        assert_eq!(
            parse_args("\"my file.txt\""),
            Ok(Some("my file.txt".to_string()))
        );
        // A lone `-` means stdin, so it is never a filename.
        assert_eq!(parse_args("-"), Ok(None));
        // Flags and extra bare tokens are rejected.
        assert!(parse_args("-r").is_err());
        assert!(parse_args("a.txt b.txt").is_err());
    }
}
