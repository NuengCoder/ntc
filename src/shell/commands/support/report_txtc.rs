//! Shared `txtc` parsing and line-number selection.

use crate::config::Config;

use super::report_view::tokenize_spans;

/// Parsed `txtc` args: an optional file plus the `--line` / `--no-line`
/// override. Flags may come before or after the file (`txtc --line a.txt`).
#[derive(Debug, Default)]
pub(crate) struct TxtcArgs {
    pub file: Option<String>,
    pub lines: Option<bool>,
}

/// Parse `txtc` args. `Err((key, arg))` is a lang key plus the offending
/// value for `print_error(&t!(key, arg))`, mirroring [`parse_txtf_args`].
pub(crate) fn parse_txtc_args(args: &str) -> Result<TxtcArgs, (String, String)> {
    let mut out = TxtcArgs::default();
    let toks: Vec<String> = tokenize_spans(args)
        .into_iter()
        .map(|(s, _, _)| s)
        .collect();
    for tok in toks {
        match tok.as_str() {
            "--line" | "--lines" | "-l" => out.lines = Some(true),
            "--no-line" | "--no-lines" | "-nl" => out.lines = Some(false),
            other => {
                if out.file.is_some() {
                    return Err(("cmd.report.unknown_option".to_string(), other.to_string()));
                }
                out.file = Some(other.to_string());
            }
        }
    }
    Ok(out)
}

/// Effective line-number setting for one `txtc` call: an explicit
/// `--line`/`--no-line` always wins over the config default, exactly like
/// `txtf`. Shared with the interactive picker so both paths agree.
pub(crate) fn txtc_show_lines(lines: Option<bool>) -> bool {
    lines.unwrap_or_else(Config::global_get_show_line_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_txtc_args_file_and_line_flags() {
        let a = parse_txtc_args("a.txt").unwrap();
        assert_eq!(a.file.as_deref(), Some("a.txt"));
        assert_eq!(a.lines, None);

        let a = parse_txtc_args("--line a.txt").unwrap();
        assert_eq!((a.file.as_deref(), a.lines), (Some("a.txt"), Some(true)));

        let a = parse_txtc_args("a.txt --no-line").unwrap();
        assert_eq!((a.file.as_deref(), a.lines), (Some("a.txt"), Some(false)));

        // Short forms.
        assert_eq!(parse_txtc_args("-l a.txt").unwrap().lines, Some(true));
        assert_eq!(parse_txtc_args("a.txt -nl").unwrap().lines, Some(false));

        // Flags only, no file: the picker runs with the override.
        let a = parse_txtc_args("--no-line").unwrap();
        assert_eq!(a.file, None);
        assert_eq!(a.lines, Some(false));

        // Quoted names with spaces survive.
        let a = parse_txtc_args("--line \"my file.txt\"").unwrap();
        assert_eq!(a.file.as_deref(), Some("my file.txt"));

        // A second bare token (or a range flag) is an error, never silent.
        assert!(parse_txtc_args("a.txt b.txt").is_err());
        assert!(parse_txtc_args("--head 5 a.txt").is_err());

        // An explicit flag always wins over the configured default, so
        // `txtc --no-line` can never keep line numbers — same rule as txtf.
        assert!(!txtc_show_lines(Some(false)));
        assert!(txtc_show_lines(Some(true)));
    }
}
