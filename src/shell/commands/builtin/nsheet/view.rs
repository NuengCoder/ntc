use crate::navigator::Navigator;
use crate::output::{print_info, print_success, print_warning};
use crate::shell::commands::builtin::ndb::ndb::render::{format_box_table, MAX_CELL_WIDTH};
use crate::shell::commands::builtin::nsheet::nsheet::{
    load_file, parse_filter, resolve_col, row_matches, sort_rows, to_json, to_markdown, to_opt_rows,
};
use crate::t;
use crate::utils::terminal::asciify;
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::args::NsheetArgs;

/// Default rows printed by `view` (mirrors `ndb`'s 50-row default).
const DEFAULT_LIMIT: usize = 50;
/// Per-column width cap inside the TUI grid.
/// Loaded + filtered + sorted view: (headers, rows, total pre-filter rows).
pub(super) type SheetView = (Vec<String>, Vec<Vec<String>>, usize);

/// Load + filter + sort. Returns (headers, view rows, total rows).
fn prepare_view(nav: &Navigator, args: &NsheetArgs) -> Result<SheetView, String> {
    prepare_view_in(nav.current_path(), args)
}

pub(super) fn prepare_view_in(cwd: &Path, args: &NsheetArgs) -> Result<SheetView, String> {
    let path = resolve_sheet_path(cwd, &args.file);
    if !path.is_file() {
        return Err(t!("cmd.nsheet.file_not_found", args.file));
    }
    let sheet =
        load_file(&path, args.delimiter, args.header).map_err(|e| t!("cmd.nsheet.load_fail", e))?;
    if sheet.headers.is_empty() {
        return Err(t!("cmd.nsheet.empty_sheet"));
    }
    let mut rows = sheet.rows;
    let total = rows.len();
    if let Some(ref expr) = args.filter {
        let (conds, logic) =
            parse_filter(expr, &sheet.headers).map_err(|e| t!("cmd.nsheet.filter_fail", e))?;
        rows.retain(|r| row_matches(r, &conds, logic));
    }
    if let Some(ref spec) = args.sort_col {
        let col = resolve_col(spec, &sheet.headers)
            .ok_or_else(|| t!("cmd.nsheet.unknown_column", spec))?;
        sort_rows(&mut rows, col, args.sort_desc);
    }
    Ok((sheet.headers, rows, total))
}

pub(super) fn resolve_sheet_path(cwd: &Path, file: &str) -> PathBuf {
    let expanded = crate::utils::paths::expand_path_str(file);
    let p = Path::new(&expanded);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}
fn run_export(
    headers: &[String],
    rows: &[Vec<String>],
    format: &str,
    output: Option<&str>,
    cwd: &Path,
) -> Result<bool> {
    let text = if format == "json" {
        to_json(headers, rows)
    } else {
        to_markdown(headers, rows)
    };
    match output {
        Some(out) => {
            let path = resolve_sheet_path(cwd, out);
            std::fs::write(&path, text)
                .map_err(|e| anyhow::anyhow!("{}", t!("cmd.nsheet.export_fail", e)))?;
            print_success(&t!("cmd.nsheet.exported", path.display()));
        }
        None => println!("{}", text),
    }
    Ok(false)
}

pub(super) fn cmd_view(nav: &mut Navigator, args: &NsheetArgs) -> Result<bool> {
    let cwd = nav.current_path().to_path_buf();
    let (headers, rows, total) = prepare_view(nav, args).map_err(|e| anyhow::anyhow!("{}", e))?;
    // Export bypasses the table printer.
    if let Some(ref format) = args.export {
        let mut export_rows = rows;
        if !args.all {
            let limit = args.limit.unwrap_or(DEFAULT_LIMIT);
            export_rows.truncate(limit);
        }
        return run_export(&headers, &export_rows, format, args.output.as_deref(), &cwd);
    }
    let limit = if args.all {
        usize::MAX
    } else {
        args.limit.unwrap_or(DEFAULT_LIMIT)
    };
    let shown: Vec<Vec<String>> = rows.iter().take(limit).cloned().collect();
    let width = headers.len();
    let boxed = format_box_table(&headers, &to_opt_rows(&shown, width), MAX_CELL_WIDTH);
    let theme = crate::utils::theme::ThemeManager::current();
    let painted = asciify(&boxed);
    let lines: Vec<&str> = painted.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if i == 1 {
            println!(
                "{}",
                theme
                    .shell
                    .paint_bold(crate::utils::theme::ShellRole::TableHeader, line)
            );
        } else if i == 0 || i + 1 == lines.len() || line.starts_with('├') || line.starts_with('+')
        {
            println!(
                "{}",
                theme
                    .shell
                    .paint(crate::utils::theme::ShellRole::TableBorder, line)
            );
        } else {
            println!("{}", line);
        }
    }
    drop(theme);
    if rows.is_empty() {
        print_warning(&t!("cmd.nsheet.no_rows"));
    } else {
        print_info(&t!(
            "cmd.nsheet.rows_shown",
            shown.len().to_string(),
            total.to_string(),
            width.to_string()
        ));
        if shown.len() < rows.len() {
            print_info(&t!("cmd.nsheet.rows_truncated", limit.to_string()));
        }
    }
    Ok(false)
}
