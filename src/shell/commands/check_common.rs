//! Shared reporter for the `<cmd> check [file]` subcommands.
//!
//! Every format check reads a file, runs that format's own validator and
//! prints one `path:line:col: message` block per problem together with the
//! offending source line and an optional hint. This module owns that
//! presentation so `ral`, `math`, `igcare`, `theme`, `fake`, `scr` and `ran`
//! all report the same way.
//!
//! With no argument a check covers every matching format file in the given
//! directory, so `math check` validates the whole directory at once.

use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};

/// One problem found while interpreting a format file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Diagnostic {
    /// 1-based line number; 0 when the problem is not tied to a line.
    pub(crate) line: usize,
    /// 1-based column number; 0 when unknown.
    pub(crate) col: usize,
    pub(crate) message: String,
    pub(crate) hint: Option<String>,
}

impl Diagnostic {
    pub(crate) fn at(
        line: usize,
        col: usize,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self {
            line,
            col,
            message: message.into(),
            hint,
        }
    }

    /// A problem that covers a whole line (no column to point at).
    pub(crate) fn line(line: usize, message: impl Into<String>, hint: Option<String>) -> Self {
        Self::at(line, 0, message, hint)
    }
}

/// 1-based `(line, column)` of a byte offset.
pub(crate) fn line_col(content: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(content.len());
    let before = &content[..offset];
    let line = before.matches('\n').count() + 1;
    let col = before
        .rsplit('\n')
        .next()
        .map(|s| s.chars().count())
        .unwrap_or(0)
        + 1;
    (line, col)
}

/// 1-based line number of the first line containing `needle`.
pub(crate) fn find_line(content: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    content
        .lines()
        .position(|line| line.contains(needle))
        .map(|idx| idx + 1)
}

/// `<cmd> check [file]` — validate one file, or every `*<ext>` file in `cwd`.
///
/// Returns `true` when every file checked was clean. The subtitle uses `ext`
/// for the "nothing to check" message, so callers pass the format suffix
/// (`.ntc.math`, `.ntc_theme`, …).
pub(crate) fn run(
    cmd: &str,
    args: &str,
    cwd: &Path,
    ext: &str,
    validator: fn(&str) -> Vec<Diagnostic>,
) -> bool {
    println!("{}", t!("check.title", cmd).cyan().bold());

    let targets = match target_files(args, cwd, ext) {
        Ok(targets) => targets,
        Err(e) => {
            print_error(&format!("{}", e));
            return false;
        }
    };
    if targets.is_empty() {
        print_info(&t!("check.no_files", ext, cwd.display()));
        return false;
    }

    let mut ok = true;
    for (idx, path) in targets.iter().enumerate() {
        if idx > 0 {
            println!();
        }
        let display = display_name(path, cwd);
        ok &= run_one(path, &display, validator);
    }
    ok
}

/// Validate exactly one file and print its report.
pub(crate) fn run_one(path: &Path, display: &str, validator: fn(&str) -> Vec<Diagnostic>) -> bool {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(e) => {
            print_error(&t!("check.read_fail", display, e));
            return false;
        }
    };
    let diags = validator(&source);
    report(display, &source, &diags)
}

/// Print one report. Returns `true` when `diags` is empty.
pub(crate) fn report(display: &str, source: &str, diags: &[Diagnostic]) -> bool {
    if diags.is_empty() {
        print_success(&t!("check.ok", display));
        return true;
    }

    let lines: Vec<&str> = source.lines().collect();
    for diag in diags {
        let location = match (diag.line, diag.col) {
            (0, _) => display.to_string(),
            (line, 0) => format!("{}:{}", display, line),
            (line, col) => format!("{}:{}:{}", display, line, col),
        };
        println!("{} {}", location.cyan(), diag.message);
        if diag.line > 0 {
            if let Some(text) = lines.get(diag.line - 1) {
                if !text.trim().is_empty() {
                    println!("   {}", text.trim().dimmed());
                }
            }
        }
        if let Some(hint) = &diag.hint {
            println!("   {}: {}", t!("check.hint").yellow(), hint);
        }
    }

    println!();
    print_error(&t!("check.summary", diags.len(), display));
    false
}

/// Files to check: the one named in `args`, or every `*<ext>` file in `cwd`.
fn target_files(args: &str, cwd: &Path, ext: &str) -> Result<Vec<PathBuf>> {
    let args = args.trim();
    if args.is_empty() {
        let mut files: Vec<PathBuf> = Vec::new();
        let entries = std::fs::read_dir(cwd).with_context(|| format!("{}", cwd.display()))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let matches = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.to_ascii_lowercase().ends_with(ext));
            if matches {
                files.push(path);
            }
        }
        files.sort();
        return Ok(files);
    }

    let expanded = crate::utils::paths::expand_path_str(args);
    let path = Path::new(&expanded);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    if !path.exists() {
        anyhow::bail!(t!("check.not_found", args));
    }
    Ok(vec![path])
}

/// Name shown in the report: the bare file name when it lives in `cwd`,
/// otherwise the path as given.
fn display_name(path: &Path, cwd: &Path) -> String {
    if path.parent() == Some(cwd) {
        if let Some(name) = path.file_name() {
            return name.to_string_lossy().to_string();
        }
    }
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_counts_chars_not_bytes() {
        let content = "ab\né=1\n";
        let offset = content.find("=1").unwrap();
        assert_eq!(line_col(content, offset), (2, 2));
    }

    #[test]
    fn find_line_is_one_based_and_none_when_absent() {
        assert_eq!(find_line("a\nb\n", "b"), Some(2));
        assert_eq!(find_line("a\n", "zz"), None);
        assert_eq!(find_line("a\n", ""), None);
    }

    #[test]
    fn clean_report_returns_true() {
        assert!(report("x.ntc.math", "1+1", &[]));
    }

    #[test]
    fn problems_return_false() {
        let diags = vec![Diagnostic::at(1, 2, "boom", Some("fix it".into()))];
        assert!(!report("x.ntc.math", "1+1", &diags));
    }

    #[test]
    fn display_name_is_bare_inside_cwd() {
        let cwd = std::path::Path::new("/tmp/dir");
        assert_eq!(display_name(&cwd.join("a.ntc"), cwd), "a.ntc");
        assert_eq!(
            display_name(std::path::Path::new("/other/b.ntc"), cwd),
            "/other/b.ntc"
        );
    }
}
