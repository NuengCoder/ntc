use crate::navigator::Navigator;
use crate::output::{print_error, print_warning};
use crate::shell::commands::builtin::nsheet::nsheet::{
    col_name, load_file, parse_filter, resolve_col, row_matches, sort_rows, to_json, to_markdown,
    Sheet,
};
use crate::t;
use crate::utils::terminal::asciify;
use crate::utils::theme::{ansi_enabled, ShellRole, ThemeManager};
use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::{
    Attribute, Color as CxColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
    SetForegroundColor,
};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{stdout, Write};

use super::args::NsheetArgs;
use super::view::resolve_sheet_path;

/// Per-column width cap inside the TUI grid.
const TUI_MAX_COL: usize = 28;
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

pub(super) struct NsheetTui {
    pub(super) title: String,
    pub(super) headers: Vec<String>,
    pub(super) base: Vec<Vec<String>>,
    pub(super) view: Vec<usize>,
    pub(super) cur_r: usize,
    pub(super) cur_c: usize,
    pub(super) off_r: usize,
    pub(super) off_c: usize,
    pub(super) term_w: usize,
    pub(super) term_h: usize,
    pub(super) search: String,
    pub(super) sort_col: Option<usize>,
    pub(super) sort_desc: bool,
    pub(super) filter: String,
    pub(super) status: String,
}

fn display_width(s: &str) -> usize {
    s.chars().count()
}

fn one_line(s: &str) -> String {
    s.replace(['\n', '\r', '\t'], " ")
}

/// Theme colours used by the fullscreen table TUI. All painting is skipped
/// when `cfg color off` (or the terminal cannot render ANSI).
struct TuiColors {
    enabled: bool,
    header: CxColor,
    border: CxColor,
    muted: CxColor,
    line_number: CxColor,
    warning: CxColor,
}

impl TuiColors {
    fn current() -> Self {
        let enabled = ansi_enabled();
        let theme = ThemeManager::current();
        Self {
            enabled,
            header: theme.shell.role(ShellRole::TableHeader).to_crossterm(),
            border: theme.shell.role(ShellRole::TableBorder).to_crossterm(),
            muted: theme.shell.role(ShellRole::Muted).to_crossterm(),
            line_number: theme.shell.role(ShellRole::LineNumber).to_crossterm(),
            warning: theme.shell.role(ShellRole::Warning).to_crossterm(),
        }
    }
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
    pub(super) fn recompute(&mut self) {
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

    pub(super) fn render(&self, out: &mut impl Write) -> Result<()> {
        let widths = self.col_widths();
        let data_h = self.term_h.saturating_sub(3);
        let hits = self.search_hits();
        let in_hit = |vi: usize| hits.contains(&vi);
        let colors = TuiColors::current();

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
                if self.sort_desc { "v" } else { "^" }
            ));
        }
        if !self.filter.trim().is_empty() {
            title.push_str(&format!(" | filter: {}", self.filter.trim()));
        }
        if !self.search.is_empty() {
            title.push_str(&format!(" | /{}", self.search));
        }
        let title = asciify(&elide(&title, self.term_w.max(1)));
        if colors.enabled {
            queue!(
                out,
                SetAttribute(Attribute::Reverse),
                SetForegroundColor(colors.header)
            )?;
        }
        queue!(
            out,
            Print(format!("{:<width$}", title, width = self.term_w))
        )?;
        if colors.enabled {
            queue!(out, SetAttribute(Attribute::NoReverse), ResetColor)?;
        }

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
                    SetAttribute(Attribute::NoBold)
                )?;
            } else {
                if colors.enabled {
                    queue!(out, SetForegroundColor(colors.header))?;
                }
                queue!(
                    out,
                    SetAttribute(Attribute::Bold),
                    Print(format!("{}{}", label, " ".repeat(pad))),
                    SetAttribute(Attribute::NoBold)
                )?;
                if colors.enabled {
                    queue!(out, ResetColor)?;
                }
            }
            if colors.enabled {
                queue!(out, SetForegroundColor(colors.border))?;
            }
            queue!(out, Print(" │"))?;
            if colors.enabled {
                queue!(out, ResetColor)?;
            }
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
                        SetAttribute(Attribute::NoReverse)
                    )?;
                } else if is_hit {
                    if colors.enabled {
                        queue!(
                            out,
                            SetBackgroundColor(colors.warning),
                            SetForegroundColor(colors.muted)
                        )?;
                    }
                    queue!(out, Print(format!("{}{}", label, " ".repeat(pad))))?;
                    if colors.enabled {
                        queue!(out, ResetColor)?;
                    }
                } else {
                    queue!(out, Print(format!("{}{}", label, " ".repeat(pad))))?;
                }
                if colors.enabled {
                    queue!(out, SetForegroundColor(colors.border))?;
                }
                queue!(out, Print(" │"))?;
                if colors.enabled {
                    queue!(out, ResetColor)?;
                }
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
        if colors.enabled {
            queue!(out, SetForegroundColor(colors.muted))?;
        }
        queue!(out, Print(elide(&asciify(&bar), self.term_w.max(1))))?;
        if colors.enabled {
            queue!(out, ResetColor)?;
        }
        out.flush()?;
        Ok(())
    }

    /// Bottom-line text prompt. Returns None when cancelled with Esc.
    fn prompt(&self, out: &mut impl Write, msg: &str, initial: &str) -> Result<Option<String>> {
        let mut buf = initial.to_string();
        loop {
            queue!(out, MoveTo(0, (self.term_h.saturating_sub(1)) as u16))?;
            let line = format!("{}: {}", msg, buf);
            let colors = TuiColors::current();
            if colors.enabled {
                queue!(out, SetForegroundColor(colors.line_number))?;
            }
            queue!(
                out,
                Print(elide(&asciify(&line), self.term_w.max(1))),
                Print(" ".repeat(self.term_w.saturating_sub(display_width(&line))))
            )?;
            if colors.enabled {
                queue!(out, ResetColor)?;
            }
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
        let colors = TuiColors::current();
        if colors.enabled {
            queue!(
                out,
                SetForegroundColor(colors.header),
                SetAttribute(Attribute::Bold)
            )?;
        }
        queue!(out, Print(asciify(&t!("cmd.nsheet.tui_cell_title"))))?;
        if colors.enabled {
            queue!(out, SetAttribute(Attribute::NoBold), ResetColor)?;
        }
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
            if colors.enabled {
                queue!(out, SetForegroundColor(colors.muted))?;
            }
            queue!(out, Print(asciify(&t!("cmd.nsheet.tui_popup_hint"))))?;
            if colors.enabled {
                queue!(out, ResetColor)?;
            }
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

pub(super) fn cmd_open(nav: &mut Navigator, args: &NsheetArgs) -> Result<bool> {
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
