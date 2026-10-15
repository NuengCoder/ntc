//! `nsheet` command: terminal sheet viewer for CSV/TSV/JSON/JSONL.
//!
//! ```text
//! nsheet open users.csv                  # fullscreen TUI (arrows, /, s, f, e, q)
//! nsheet view users.csv -l 20 --sort age --desc --filter "city~york"
//! nsheet users.csv --filter "age>30" --export json -o out.json
//! ```
//!
//! Parsing/filter/sort/export live in [`crate::nsheet`]; this module is the
//! shell front-end: argument parsing, the non-interactive `view` printer
//! (reusing `ndb`'s box table) and the fullscreen crossterm TUI.

use crate::navigator::Navigator;
use crate::ndb::render::{format_box_table, MAX_CELL_WIDTH};
use crate::nsheet::{
    col_name, load_file, parse_filter, resolve_col, row_matches, sort_rows, to_json, to_markdown,
    to_opt_rows, Sheet,
};
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::terminal::asciify;
use anyhow::Result;
use colored::*;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::{
    Attribute, Color as CxColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
    SetForegroundColor,
};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

/// Default rows printed by `view` (mirrors `ndb`'s 50-row default).
const DEFAULT_LIMIT: usize = 50;
/// Per-column width cap inside the TUI grid.
const TUI_MAX_COL: usize = 28;

fn tokenize(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut single = false;
    let mut double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            ' ' | '\t' if !single && !double => {
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
    // Strip one layer of surrounding quotes (tokenize kept them).
    out.into_iter().map(|t| strip_wrapping_quotes(&t)).collect()
}

fn strip_wrapping_quotes(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

struct NsheetArgs {
    file: String,
    delimiter: Option<char>,
    header: Option<bool>,
    limit: Option<usize>,
    all: bool,
    sort_col: Option<String>,
    sort_desc: bool,
    filter: Option<String>,
    export: Option<String>,
    output: Option<String>,
}

fn parse_delimiter_value(val: &str) -> Result<Option<char>, String> {
    let low = val.to_lowercase();
    match low.as_str() {
        "auto" => Ok(None),
        "tab" | "\\t" => Ok(Some('\t')),
        "comma" => Ok(Some(',')),
        "semicolon" => Ok(Some(';')),
        "pipe" => Ok(Some('|')),
        _ => {
            let mut chars = val.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Ok(Some(c)),
                _ => Err(t!("cmd.nsheet.invalid_delimiter", val)),
            }
        }
    }
}

fn parse_nsheet_args(tokens: &[String]) -> Result<NsheetArgs, String> {
    let mut args = NsheetArgs {
        file: String::new(),
        delimiter: None,
        header: None,
        limit: None,
        all: false,
        sort_col: None,
        sort_desc: false,
        filter: None,
        export: None,
        output: None,
    };
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-d" | "--delimiter" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.delimiter_usage"))?;
                args.delimiter = parse_delimiter_value(val)?;
                i += 2;
            }
            "--header" => {
                args.header = Some(true);
                i += 1;
            }
            "--no-header" => {
                args.header = Some(false);
                i += 1;
            }
            "-l" | "--limit" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.limit_usage"))?;
                match val.parse::<usize>() {
                    Ok(n) => args.limit = Some(n),
                    Err(_) => return Err(t!("cmd.nsheet.invalid_limit", val)),
                }
                i += 2;
            }
            "--all" => {
                args.all = true;
                i += 1;
            }
            "--sort" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.sort_usage"))?;
                args.sort_col = Some(val.clone());
                i += 2;
            }
            "--desc" => {
                args.sort_desc = true;
                i += 1;
            }
            "--filter" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.filter_usage"))?;
                args.filter = Some(val.clone());
                i += 2;
            }
            "--export" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.export_usage"))?;
                let low = val.to_lowercase();
                if low != "json" && low != "md" {
                    return Err(t!("cmd.nsheet.invalid_export", val));
                }
                args.export = Some(low);
                i += 2;
            }
            "-o" | "--output" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.output_usage"))?;
                args.output = Some(val.clone());
                i += 2;
            }
            _ => {
                let tok = &tokens[i];
                if tok.starts_with('-') {
                    return Err(t!("cmd.nsheet.unknown_option", tok));
                }
                if !args.file.is_empty() {
                    return Err(t!("cmd.nsheet.unknown_option", tok));
                }
                args.file = tok.clone();
                i += 1;
            }
        }
    }
    if args.file.is_empty() {
        return Err(t!("cmd.nsheet.missing_file"));
    }
    Ok(args)
}

/// Loaded + filtered + sorted view: (headers, rows, total pre-filter rows).
type SheetView = (Vec<String>, Vec<Vec<String>>, usize);

/// Load + filter + sort. Returns (headers, view rows, total rows).
fn prepare_view(nav: &Navigator, args: &NsheetArgs) -> Result<SheetView, String> {
    prepare_view_in(nav.current_path(), args)
}

fn prepare_view_in(cwd: &Path, args: &NsheetArgs) -> Result<SheetView, String> {
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

fn resolve_sheet_path(cwd: &Path, file: &str) -> PathBuf {
    let p = Path::new(file);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}

fn print_usage() {
    println!("{}", t!("cmd.nsheet.usage_title").cyan().bold());
    println!("  {}", t!("cmd.nsheet.help_open"));
    println!("  {}", t!("cmd.nsheet.help_view"));
    println!();
    println!("{}", t!("cmd.nsheet.options_title").cyan().bold());
    println!("  {}", t!("cmd.nsheet.help_delimiter"));
    println!("  {}", t!("cmd.nsheet.help_header"));
    println!("  {}", t!("cmd.nsheet.help_limit"));
    println!("  {}", t!("cmd.nsheet.help_sort"));
    println!("  {}", t!("cmd.nsheet.help_filter"));
    println!("  {}", t!("cmd.nsheet.help_export"));
    println!("  {}", t!("cmd.nsheet.help_output"));
    println!("  {}", t!("cmd.nsheet.help_help"));
    println!();
    println!("{}", t!("cmd.nsheet.examples_title").green());
    println!("    {}", t!("cmd.nsheet.example_open"));
    println!("    {}", t!("cmd.nsheet.example_view"));
    println!("    {}", t!("cmd.nsheet.example_filter"));
    println!("    {}", t!("cmd.nsheet.example_export"));
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

fn cmd_view(nav: &mut Navigator, args: &NsheetArgs) -> Result<bool> {
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
    #[cfg(not(target_os = "android"))]
    {
        for (i, line) in asciify(&boxed).lines().enumerate() {
            if i == 1 {
                println!("{}", line.cyan().bold());
            } else {
                println!("{}", line);
            }
        }
    }
    #[cfg(target_os = "android")]
    {
        println!("{}", asciify(&boxed));
    }
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

pub fn cmd_nsheet(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    let first = tokens[0].to_lowercase();
    if first == "help" || first == "-h" || first == "--help" {
        print_usage();
        return Ok(false);
    }
    let (is_view, rest) = if first == "open" {
        (false, &tokens[1..])
    } else if first == "view" {
        (true, &tokens[1..])
    } else if first.starts_with('-') {
        print_usage();
        return Ok(false);
    } else {
        // Bare `nsheet <file> [flags]` behaves like `open`.
        (false, &tokens[..])
    };
    let parsed = match parse_nsheet_args(rest) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            println!();
            print_usage();
            return Ok(false);
        }
    };
    if is_view {
        cmd_view(nav, &parsed)
    } else {
        cmd_open(nav, &parsed)
    }
}

// ============================================================================
// Fullscreen TUI
// ============================================================================

/// RAII terminal cleanup: leaves the alternate screen and restores the cursor.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }
}

struct NsheetTui {
    title: String,
    headers: Vec<String>,
    base: Vec<Vec<String>>,
    view: Vec<usize>,
    cur_r: usize,
    cur_c: usize,
    off_r: usize,
    off_c: usize,
    term_w: usize,
    term_h: usize,
    search: String,
    sort_col: Option<usize>,
    sort_desc: bool,
    filter: String,
    status: String,
}

fn display_width(s: &str) -> usize {
    s.chars().count()
}

fn one_line(s: &str) -> String {
    s.replace(['\n', '\r', '\t'], " ")
}

fn elide(s: &str, max: usize) -> String {
    if max == 0 || display_width(s) <= max {
        return s.to_string();
    }
    let keep = max.saturating_sub(1);
    let mut out: String = s.chars().take(keep).collect();
    out.push('…');
    out
}

impl NsheetTui {
    fn recompute(&mut self) {
        let mut idx: Vec<usize> = (0..self.base.len()).collect();
        if !self.filter.trim().is_empty() {
            match parse_filter(&self.filter, &self.headers) {
                Ok((conds, logic)) => {
                    idx.retain(|&i| row_matches(&self.base[i], &conds, logic));
                    self.status = String::new();
                }
                Err(e) => {
                    self.status = t!("cmd.nsheet.filter_fail", e);
                    self.view = idx;
                    self.clamp_cursor();
                    return;
                }
            }
        } else {
            self.status = String::new();
        }
        if let Some(col) = self.sort_col {
            let mut rows: Vec<Vec<String>> = idx.iter().map(|&i| self.base[i].clone()).collect();
            sort_rows(&mut rows, col, self.sort_desc);
            // Map sorted rows back to base indices. Identical duplicate rows
            // resolve to the first unused match — order among exact dupes is
            // irrelevant for display purposes.
            let mut used = vec![false; self.base.len()];
            let mut mapped = Vec::with_capacity(rows.len());
            for r in &rows {
                if let Some(bi) = self.base.iter().enumerate().find_map(|(bi, b)| {
                    if !used[bi] && b == r {
                        Some(bi)
                    } else {
                        None
                    }
                }) {
                    used[bi] = true;
                    mapped.push(bi);
                }
            }
            idx = mapped;
        }
        self.view = idx;
        self.clamp_cursor();
    }

    fn clamp_cursor(&mut self) {
        if self.view.is_empty() {
            self.cur_r = 0;
        } else {
            self.cur_r = self.cur_r.min(self.view.len() - 1);
        }
        if !self.headers.is_empty() {
            self.cur_c = self.cur_c.min(self.headers.len() - 1);
        }
    }

    /// Column widths capped at TUI_MAX_COL, sampled over the first 500 rows.
    fn col_widths(&self) -> Vec<usize> {
        let n = self.headers.len();
        let mut widths = vec![1usize; n];
        for (i, h) in self.headers.iter().enumerate() {
            widths[i] = display_width(&one_line(h)).clamp(1, TUI_MAX_COL);
        }
        for &ri in self.view.iter().take(500) {
            if let Some(row) = self.base.get(ri) {
                for (i, w) in widths.iter_mut().enumerate() {
                    let cw = display_width(&one_line(row.get(i).map(|s| s.as_str()).unwrap_or("")));
                    *w = (*w).max(cw.min(TUI_MAX_COL));
                }
            }
        }
        widths
    }

    fn ensure_visible(&mut self) {
        let data_h = self.term_h.saturating_sub(3);
        if data_h == 0 {
            return;
        }
        if self.cur_r < self.off_r {
            self.off_r = self.cur_r;
        } else if self.cur_r >= self.off_r + data_h {
            self.off_r = self.cur_r + 1 - data_h;
        }
        // Horizontal: keep cursor column on screen.
        let widths = self.col_widths();
        let avail = self.term_w;
        let mut first = self.off_c;
        // Walk from off_c; if cursor is left of it, jump back.
        if self.cur_c < self.off_c {
            first = self.cur_c;
        } else {
            let mut span = 0usize;
            let mut last = self.off_c;
            for (c, &w) in widths.iter().enumerate().skip(self.off_c) {
                let need = w + 3;
                if span + need > avail && c > self.off_c {
                    break;
                }
                span += need;
                last = c;
            }
            if self.cur_c > last {
                // Shift window so cursor is the last visible column.
                let mut span2 = 0usize;
                first = self.cur_c;
                for c in (0..=self.cur_c).rev() {
                    let need = widths[c] + 3;
                    if span2 + need > avail && c < self.cur_c {
                        break;
                    }
                    span2 += need;
                    first = c;
                }
            }
        }
        self.off_c = first;
    }

    fn search_hits(&self) -> Vec<usize> {
        if self.search.is_empty() {
            return vec![];
        }
        let needle = self.search.to_lowercase();
        self.view
            .iter()
            .enumerate()
            .filter(|(_, &ri)| {
                self.base
                    .get(ri)
                    .map(|row| row.iter().any(|c| c.to_lowercase().contains(&needle)))
                    .unwrap_or(false)
            })
            .map(|(vi, _)| vi)
            .collect()
    }

    fn render(&self, out: &mut impl Write) -> Result<()> {
        let widths = self.col_widths();
        let data_h = self.term_h.saturating_sub(3);
        let hits = self.search_hits();
        let in_hit = |vi: usize| hits.contains(&vi);

        queue!(out, MoveTo(0, 0))?;
        // Title bar.
        let mut title = format!(
            "nsheet {} — {}x{}",
            self.title,
            self.view.len(),
            self.headers.len()
        );
        if let Some(col) = self.sort_col {
            title.push_str(&format!(
                " | sort: {}{}",
                col_name(&self.headers, col),
                if self.sort_desc { "▼" } else { "▲" }
            ));
        }
        if !self.filter.trim().is_empty() {
            title.push_str(&format!(" | filter: {}", self.filter.trim()));
        }
        if !self.search.is_empty() {
            title.push_str(&format!(" | /{}", self.search));
        }
        let title = asciify(&elide(&title, self.term_w.max(1)));
        queue!(
            out,
            SetBackgroundColor(CxColor::DarkGrey),
            SetForegroundColor(CxColor::White),
            Print(format!("{:<width$}", title, width = self.term_w)),
            ResetColor
        )?;

        // Header row.
        queue!(out, MoveTo(0, 1))?;
        let mut x = 0usize;
        for (c, &w) in widths.iter().enumerate().skip(self.off_c) {
            let need = w + 3;
            if x + need > self.term_w && c > self.off_c {
                break;
            }
            let label = elide(&one_line(&self.headers[c]), w);
            let pad = w.saturating_sub(display_width(&label));
            queue!(out, Print(" "))?;
            if c == self.cur_c {
                queue!(
                    out,
                    SetAttribute(Attribute::Bold),
                    SetAttribute(Attribute::Reverse),
                    Print(format!("{}{}", label, " ".repeat(pad))),
                    SetAttribute(Attribute::NoReverse),
                    ResetColor
                )?;
            } else {
                queue!(
                    out,
                    SetForegroundColor(CxColor::Cyan),
                    SetAttribute(Attribute::Bold),
                    Print(format!("{}{}", label, " ".repeat(pad))),
                    ResetColor
                )?;
            }
            queue!(out, Print(" │"))?;
            x += need;
        }

        // Data rows.
        for (li, vi) in (self.off_r..self.view.len()).take(data_h).enumerate() {
            let y = (li + 2) as u16;
            queue!(out, MoveTo(0, y))?;
            let ri = self.view[vi];
            let row = self.base.get(ri);
            let mut x = 0usize;
            for (c, &w) in widths.iter().enumerate().skip(self.off_c) {
                let need = w + 3;
                if x + need > self.term_w && c > self.off_c {
                    break;
                }
                let raw = row.and_then(|r| r.get(c)).map(|s| s.as_str()).unwrap_or("");
                let label = elide(&one_line(raw), w);
                let pad = w.saturating_sub(display_width(&label));
                let is_cursor = vi == self.cur_r && c == self.cur_c;
                let is_hit = !self.search.is_empty() && in_hit(vi);
                queue!(out, Print(" "))?;
                if is_cursor {
                    queue!(
                        out,
                        SetAttribute(Attribute::Reverse),
                        Print(format!("{}{}", label, " ".repeat(pad))),
                        SetAttribute(Attribute::NoReverse),
                        ResetColor
                    )?;
                } else if is_hit {
                    queue!(
                        out,
                        SetBackgroundColor(CxColor::DarkYellow),
                        SetForegroundColor(CxColor::Black),
                        Print(format!("{}{}", label, " ".repeat(pad))),
                        ResetColor
                    )?;
                } else {
                    queue!(out, Print(format!("{}{}", label, " ".repeat(pad))))?;
                }
                queue!(out, Print(" │"))?;
                x += need;
            }
        }

        // Status bar.
        queue!(out, MoveTo(0, (self.term_h.saturating_sub(1)) as u16))?;
        let hint = if self.status.is_empty() {
            t!("cmd.nsheet.tui_hint")
        } else {
            self.status.clone()
        };
        let pos = if self.view.is_empty() {
            "0/0".to_string()
        } else {
            format!("{}/{}", self.cur_r + 1, self.view.len())
        };
        let bar = format!(
            "{}  {}",
            elide(&hint, self.term_w.saturating_sub(pos.len() + 3)),
            pos
        );
        queue!(
            out,
            SetForegroundColor(CxColor::DarkGrey),
            Print(elide(&asciify(&bar), self.term_w.max(1))),
            ResetColor
        )?;
        out.flush()?;
        Ok(())
    }

    /// Bottom-line text prompt. Returns None when cancelled with Esc.
    fn prompt(&self, out: &mut impl Write, msg: &str, initial: &str) -> Result<Option<String>> {
        let mut buf = initial.to_string();
        loop {
            queue!(out, MoveTo(0, (self.term_h.saturating_sub(1)) as u16))?;
            let line = format!("{}: {}", msg, buf);
            queue!(
                out,
                SetForegroundColor(CxColor::Yellow),
                Print(elide(&asciify(&line), self.term_w.max(1))),
                Print(" ".repeat(self.term_w.saturating_sub(display_width(&line)))),
                ResetColor
            )?;
            out.flush()?;
            match event::read()? {
                Event::Key(ke) if ke.kind == KeyEventKind::Press => match ke.code {
                    KeyCode::Enter => return Ok(Some(buf)),
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Backspace => {
                        buf.pop();
                    }
                    KeyCode::Char(ch) => {
                        if ke.modifiers.contains(KeyModifiers::CONTROL) {
                            if ch == 'c' || ch == 'g' {
                                return Ok(None);
                            }
                        } else {
                            buf.push(ch);
                        }
                    }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    let _ = (w, h);
                }
                _ => {}
            }
        }
    }

    /// Full-cell popup; any key closes it.
    fn popup(&self, out: &mut impl Write, text: &str) -> Result<()> {
        queue!(
            out,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
        queue!(out, MoveTo(0, 0))?;
        queue!(
            out,
            SetForegroundColor(CxColor::Cyan),
            SetAttribute(Attribute::Bold),
            Print(asciify(&t!("cmd.nsheet.tui_cell_title"))),
            ResetColor
        )?;
        let w = self.term_w.max(10);
        let mut y = 2u16;
        let mut line = String::new();
        for word in one_line(text).split(' ') {
            if display_width(&line) + display_width(word) + 1 > w {
                queue!(out, MoveTo(0, y))?;
                queue!(out, Print(line.clone()))?;
                line.clear();
                y += 1;
                if y as usize >= self.term_h.saturating_sub(1) {
                    break;
                }
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        if (y as usize) < self.term_h.saturating_sub(1) {
            queue!(out, MoveTo(0, y))?;
            queue!(out, Print(line))?;
            y += 1;
        }
        if (y as usize) < self.term_h {
            queue!(out, MoveTo(0, y))?;
            queue!(
                out,
                SetForegroundColor(CxColor::DarkGrey),
                Print(asciify(&t!("cmd.nsheet.tui_popup_hint"))),
                ResetColor
            )?;
        }
        out.flush()?;
        loop {
            if let Event::Key(ke) = event::read()? {
                if ke.kind == KeyEventKind::Press {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn terminal_size() -> (usize, usize) {
    terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

fn cmd_open(nav: &mut Navigator, args: &NsheetArgs) -> Result<bool> {
    let cwd = nav.current_path().to_path_buf();
    let path = resolve_sheet_path(&cwd, &args.file);
    if !path.is_file() {
        print_error(&t!("cmd.nsheet.file_not_found", args.file));
        return Ok(false);
    }
    let sheet: Sheet = load_file(&path, args.delimiter, args.header)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.nsheet.load_fail", e)))?;
    if sheet.headers.is_empty() {
        print_error(&t!("cmd.nsheet.empty_sheet"));
        return Ok(false);
    }
    if sheet.rows.is_empty() {
        print_warning(&t!("cmd.nsheet.no_rows"));
        return Ok(false);
    }

    let mut stdout = stdout();
    let _guard = TerminalGuard;
    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;

    let (w, h) = terminal_size();
    let mut tui = NsheetTui {
        title: sheet.source.clone(),
        headers: sheet.headers,
        base: sheet.rows,
        view: vec![],
        cur_r: 0,
        cur_c: 0,
        off_r: 0,
        off_c: 0,
        term_w: w,
        term_h: h,
        search: String::new(),
        sort_col: None,
        sort_desc: false,
        filter: args.filter.clone().unwrap_or_default(),
        status: String::new(),
    };
    // CLI-provided sort applies on entry.
    if let Some(ref spec) = args.sort_col {
        match resolve_col(spec, &tui.headers) {
            Some(col) => {
                tui.sort_col = Some(col);
                tui.sort_desc = args.sort_desc;
            }
            None => tui.status = t!("cmd.nsheet.unknown_column", spec),
        }
    }
    tui.recompute();
    if tui.base.len() > 20_000 {
        tui.status = t!("cmd.nsheet.tui_large", tui.base.len().to_string());
    }
    tui.ensure_visible();
    tui.render(&mut stdout)?;

    loop {
        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    continue;
                }
                if ke.modifiers.contains(KeyModifiers::CONTROL)
                    && matches!(ke.code, KeyCode::Char('c') | KeyCode::Char('q'))
                {
                    break;
                }
                match ke.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        tui.cur_r = tui.cur_r.saturating_sub(1);
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if !tui.view.is_empty() {
                            tui.cur_r = (tui.cur_r + 1).min(tui.view.len() - 1);
                        }
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        tui.cur_c = tui.cur_c.saturating_sub(1);
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if !tui.headers.is_empty() {
                            tui.cur_c = (tui.cur_c + 1).min(tui.headers.len() - 1);
                        }
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::PageUp => {
                        let step = tui.term_h.saturating_sub(3).max(1);
                        tui.cur_r = tui.cur_r.saturating_sub(step);
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::PageDown | KeyCode::Char(' ') => {
                        let step = tui.term_h.saturating_sub(3).max(1);
                        if !tui.view.is_empty() {
                            tui.cur_r = (tui.cur_r + step).min(tui.view.len() - 1);
                        }
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Home | KeyCode::Char('g') => {
                        tui.cur_r = 0;
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::End | KeyCode::Char('G') => {
                        if !tui.view.is_empty() {
                            tui.cur_r = tui.view.len() - 1;
                        }
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('/') => {
                        if let Some(s) =
                            tui.prompt(&mut stdout, &t!("cmd.nsheet.tui_search_prompt"), "")?
                        {
                            tui.search = s;
                            // Jump to first hit.
                            let hits = tui.search_hits();
                            if let Some(&first) = hits.first() {
                                tui.cur_r = first;
                                tui.ensure_visible();
                            } else if !tui.search.is_empty() {
                                tui.status = t!("cmd.nsheet.tui_no_matches");
                            }
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('n') => {
                        let hits = tui.search_hits();
                        if !hits.is_empty() {
                            let pos = hits.iter().position(|&v| v > tui.cur_r).unwrap_or(0);
                            tui.cur_r = hits[pos];
                            tui.ensure_visible();
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('N') => {
                        let hits = tui.search_hits();
                        if !hits.is_empty() {
                            let pos = hits
                                .iter()
                                .rposition(|&v| v < tui.cur_r)
                                .unwrap_or(hits.len() - 1);
                            tui.cur_r = hits[pos];
                            tui.ensure_visible();
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('s') => {
                        let cur = tui
                            .sort_col
                            .map(|c| col_name(&tui.headers, c))
                            .unwrap_or_default();
                        if let Some(spec) =
                            tui.prompt(&mut stdout, &t!("cmd.nsheet.tui_sort_prompt"), &cur)?
                        {
                            let spec = spec.trim();
                            if spec.is_empty() {
                                tui.sort_col = None;
                            } else {
                                let (name, desc) = match spec.strip_suffix('!') {
                                    Some(n) => (n.trim(), true),
                                    None => (spec, false),
                                };
                                match resolve_col(name, &tui.headers) {
                                    Some(col) => {
                                        tui.sort_col = Some(col);
                                        tui.sort_desc = desc;
                                    }
                                    None => tui.status = t!("cmd.nsheet.unknown_column", name),
                                }
                            }
                            tui.recompute();
                            tui.ensure_visible();
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('f') => {
                        let cur = tui.filter.clone();
                        if let Some(expr) =
                            tui.prompt(&mut stdout, &t!("cmd.nsheet.tui_filter_prompt"), &cur)?
                        {
                            tui.filter = expr;
                            tui.recompute();
                            tui.ensure_visible();
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('c') => {
                        tui.search.clear();
                        tui.filter.clear();
                        tui.sort_col = None;
                        tui.recompute();
                        tui.ensure_visible();
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Char('e') => {
                        if let Some(dest) =
                            tui.prompt(&mut stdout, &t!("cmd.nsheet.tui_export_prompt"), "")?
                        {
                            let dest = dest.trim();
                            if !dest.is_empty() {
                                let low = dest.to_lowercase();
                                let format = if low.ends_with(".json") {
                                    Some("json")
                                } else if low.ends_with(".md") {
                                    Some("md")
                                } else {
                                    None
                                };
                                match format {
                                    Some(fmt) => {
                                        let rows: Vec<Vec<String>> = tui
                                            .view
                                            .iter()
                                            .filter_map(|&i| tui.base.get(i).cloned())
                                            .collect();
                                        let text = if fmt == "json" {
                                            to_json(&tui.headers, &rows)
                                        } else {
                                            to_markdown(&tui.headers, &rows)
                                        };
                                        let out_path = resolve_sheet_path(&cwd, dest);
                                        match std::fs::write(&out_path, text) {
                                            Ok(_) => {
                                                tui.status =
                                                    t!("cmd.nsheet.exported", out_path.display())
                                            }
                                            Err(e) => {
                                                tui.status =
                                                    t!("cmd.nsheet.export_fail", e.to_string())
                                            }
                                        }
                                    }
                                    None => {
                                        tui.status = t!("cmd.nsheet.invalid_export", dest);
                                    }
                                }
                            }
                        }
                        tui.render(&mut stdout)?;
                    }
                    KeyCode::Enter => {
                        if let Some(&ri) = tui.view.get(tui.cur_r) {
                            let text = tui
                                .base
                                .get(ri)
                                .and_then(|r| r.get(tui.cur_c))
                                .map(|s| s.as_str())
                                .unwrap_or("");
                            tui.popup(&mut stdout, text)?;
                        }
                        tui.render(&mut stdout)?;
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                tui.term_w = w as usize;
                tui.term_h = h as usize;
                tui.ensure_visible();
                tui.render(&mut stdout)?;
            }
            _ => {}
        }
    }
    drop(_guard);
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_keeps_quoted_filter() {
        let t = tokenize(r#"view a.csv --filter "age>30 && city~york" -l 5"#);
        assert_eq!(
            t,
            vec![
                "view",
                "a.csv",
                "--filter",
                "age>30 && city~york",
                "-l",
                "5"
            ]
        );
    }

    #[test]
    fn args_parse_view_with_flags() {
        let toks = tokenize("data.csv --sort age --desc --filter age>30 -l 10");
        let a = parse_nsheet_args(&toks).unwrap();
        assert_eq!(a.file, "data.csv");
        assert_eq!(a.sort_col, Some("age".into()));
        assert!(a.sort_desc);
        assert_eq!(a.filter, Some("age>30".into()));
        assert_eq!(a.limit, Some(10));
    }

    #[test]
    fn args_reject_unknown_and_missing_file() {
        assert!(parse_nsheet_args(&tokenize("--sort age")).is_err());
        assert!(parse_nsheet_args(&tokenize("a.csv --bogus")).is_err());
        assert!(parse_nsheet_args(&tokenize("a.csv --export xml")).is_err());
    }

    #[test]
    fn delimiter_values() {
        assert_eq!(parse_delimiter_value("auto").unwrap(), None);
        assert_eq!(parse_delimiter_value("tab").unwrap(), Some('\t'));
        assert_eq!(parse_delimiter_value(";").unwrap(), Some(';'));
        assert!(parse_delimiter_value("xx").is_err());
    }

    #[test]
    fn prepare_view_filters_and_sorts() {
        use std::io::Write as _;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.csv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "name,age\nbob,50\nal,20\ncid,30").unwrap();
        let args = parse_nsheet_args(&tokenize("t.csv --filter age>25 --sort age")).unwrap();
        let (headers, rows, total) = prepare_view_in(dir.path(), &args).unwrap();
        assert_eq!(headers, vec!["name", "age"]);
        assert_eq!(total, 3);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], "cid");
        assert_eq!(rows[1][0], "bob");
    }

    #[test]
    fn prepare_view_missing_file_errors() {
        let dir = tempfile::tempdir().unwrap();
        let args = parse_nsheet_args(&tokenize("nope.csv")).unwrap();
        assert!(prepare_view_in(dir.path(), &args).is_err());
    }
}
