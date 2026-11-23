//! Shared `txtf` parsing and file display.

use crate::config::Config;
use crate::filetype::is_supported_format;
use crate::output::{print_error, print_warning};
use crate::t;
use anyhow::Result;
use std::path::Path;

use super::report_view::{next_count, tokenize_spans};

/// Parsed `txtf` args: an optional file plus display range flags.
/// Flags may come before or after the file (`txtf --head 5 a.txt`).
#[derive(Debug, Default)]
pub(crate) struct TxtfArgs {
    pub file: Option<String>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub betw: Option<String>,
    /// `--line` / `--no-line`: override the configured line numbers.
    pub lines: Option<bool>,
}

/// Parse `txtf` args. `Err((key, arg))` is a lang key plus the
/// offending value for `print_error(&t!(key, arg))`.
pub(crate) fn parse_txtf_args(args: &str) -> Result<TxtfArgs, (String, String)> {
    let mut out = TxtfArgs::default();
    let toks: Vec<String> = tokenize_spans(args)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    let mut i = 0;
    while i < toks.len() {
        match toks[i].as_str() {
            "--head" => match next_count(&toks, i) {
                Some(n) => {
                    out.head = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        toks.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--tail" => match next_count(&toks, i) {
                Some(n) => {
                    out.tail = Some(n);
                    i += 1;
                }
                None => {
                    return Err((
                        "cmd.report.invalid_count".to_string(),
                        toks.get(i + 1).cloned().unwrap_or_default(),
                    ));
                }
            },
            "--betw" => match toks.get(i + 1) {
                Some(v) => {
                    out.betw = Some(v.clone());
                    i += 1;
                }
                None => {
                    return Err(("cmd.report.invalid_range".to_string(), String::new()));
                }
            },
            "--line" | "--lines" | "-l" => out.lines = Some(true),
            "--no-line" | "--no-lines" | "-nl" => out.lines = Some(false),
            other => {
                if out.file.is_some() {
                    return Err(("cmd.report.unknown_option".to_string(), other.to_string()));
                }
                out.file = Some(other.to_string());
            }
        }
        i += 1;
    }
    Ok(out)
}

/// Display one file for `txtf` / `view <file>`, honoring `range`
/// (original line numbers kept). `label` is echoed in messages.
pub(crate) fn display_txtf_file(
    target: &Path,
    range: Option<crate::output::FileLineRange>,
    label: &str,
    lines: Option<bool>,
) -> Result<()> {
    if target.is_file() {
        if is_supported_format(target) {
            let show_lines = lines.unwrap_or_else(Config::global_get_show_line_numbers);
            crate::output::cat_file_with_syntax_range(target, show_lines, range)?;
        } else {
            print_warning(&t!("cmd.report.skipped_unsupported", label));
        }
    } else {
        print_error(&t!("cmd.report.file_not_found", label));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_txtf_args_file_and_flags_any_order() {
        let p = parse_txtf_args("a.txt").unwrap();
        assert_eq!(p.file.as_deref(), Some("a.txt"));
        assert!(p.head.is_none() && p.tail.is_none() && p.betw.is_none());
        let p = parse_txtf_args("a.txt --head 5").unwrap();
        assert_eq!((p.file.as_deref(), p.head), (Some("a.txt"), Some(5)));
        let p = parse_txtf_args("--tail 3 --betw 2:4 a.txt").unwrap();
        assert_eq!(p.tail, Some(3));
        assert_eq!(p.betw.as_deref(), Some("2:4"));
        let p = parse_txtf_args("--head 5").unwrap();
        assert!(p.file.is_none());
        assert_eq!(p.head, Some(5));
        // Quoted names with spaces survive.
        let p = parse_txtf_args("\"my file.txt\" --tail 2").unwrap();
        assert_eq!(p.file.as_deref(), Some("my file.txt"));
        // Line-number overrides for one call only.
        assert_eq!(parse_txtf_args("a.txt --line").unwrap().lines, Some(true));
        assert_eq!(
            parse_txtf_args("--no-line a.txt").unwrap().lines,
            Some(false)
        );
        assert_eq!(parse_txtf_args("a.txt").unwrap().lines, None);
        // Errors carry keys.
        assert!(parse_txtf_args("--head 0").is_err());
        assert!(parse_txtf_args("--head").is_err());
        assert!(parse_txtf_args("--tail x").is_err());
        assert!(parse_txtf_args("--betw").is_err());
        assert!(parse_txtf_args("a.txt b.txt").is_err());
    }
}
