use crate::navigator::Navigator;
use crate::output::print_info;
use crate::search::{display_content_path, ContentMatch};
use crate::t;
use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::style::{
    Color as CxColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const PREVIEW_LINES: usize = 4;
const MAX_CACHED_FILE_LINES: usize = 50_000;
/// Each interactive page holds exactly 10 matches.
pub const GS_PAGE_SIZE: usize = 10;

struct GsInteractive {
    matches: Vec<ContentMatch>,
    filtered: Vec<usize>,
    selected: usize,
    term_h: usize,
    term_w: usize,
    file_cache: HashMap<PathBuf, Vec<String>>,
    root: PathBuf,
    pattern: String,
}

impl GsInteractive {
    fn total(&self) -> usize {
        self.filtered.len()
    }

    fn page_count(&self) -> usize {
        (self.total() + GS_PAGE_SIZE - 1) / GS_PAGE_SIZE.max(1)
    }

    fn page(&self) -> usize {
        if self.total() == 0 {
            0
        } else {
            self.selected / GS_PAGE_SIZE
        }
    }

    fn page_range(&self) -> (usize, usize) {
        let start = self.page() * GS_PAGE_SIZE;
        let end = (start + GS_PAGE_SIZE).min(self.total());
        (start, end)
    }

    fn result_h(&self) -> usize {
        // Reserve header(2) + separators/preview; list area capped at page size.
        let preview_h = (PREVIEW_LINES * 2 + 2).min(self.term_h.saturating_sub(4));
        let avail = self.term_h.saturating_sub(4).saturating_sub(preview_h);
        avail.clamp(1, GS_PAGE_SIZE).max(1)
    }

    fn selected_match(&self) -> Option<&ContentMatch> {
        self.filtered.get(self.selected).map(|&i| &self.matches[i])
    }

    fn get_file_lines(&mut self, path: &Path) -> Vec<String> {
        if !self.file_cache.contains_key(path) {
            let mut lines = Vec::new();
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines().take(MAX_CACHED_FILE_LINES) {
                    match line {
                        Ok(l) => lines.push(l),
                        Err(_) => break,
                    }
                }
            }
            self.file_cache.insert(path.to_path_buf(), lines);
        }
        self.file_cache.get(path).cloned().unwrap_or_default()
    }

    fn goto_page(&mut self, page: usize) {
        let pc = self.page_count();
        if pc == 0 {
            self.selected = 0;
            return;
        }
        let p = page.min(pc - 1);
        self.selected = (p * GS_PAGE_SIZE).min(self.total().saturating_sub(1));
    }

    fn next_page(&mut self) {
        self.goto_page(self.page() + 1);
    }

    fn prev_page(&mut self) {
        self.goto_page(self.page().saturating_sub(1));
    }

    fn ensure_visible(&mut self) {
        // Selection drives the page; nothing else to sync.
        if self.total() == 0 {
            self.selected = 0;
            return;
        }
        self.selected = self.selected.min(self.total() - 1);
    }

    /// Split `text` into (segment, is_match) for red pattern highlight.
    fn match_segments<'a>(&self, text: &'a str) -> Vec<(&'a str, bool)> {
        if self.pattern.is_empty() {
            return vec![(text, false)];
        }
        let lower = text.to_lowercase();
        let pat = self.pattern.to_lowercase();
        let mut segs = Vec::new();
        let mut i = 0usize;
        while i < text.len() {
            if lower[i..].starts_with(pat.as_str()) {
                let mut e = (i + self.pattern.len()).min(text.len());
                while e < text.len() && !text.is_char_boundary(e) {
                    e += 1;
                }
                let mut s = i;
                while s < e && !text.is_char_boundary(s) {
                    s += 1;
                }
                segs.push((&text[s..e], true));
                i = e;
            } else {
                // Extend current non-match run by one char.
                let ch_len = text[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                if let Some((_, true)) = segs.last() {
                    segs.push((&text[i..i + ch_len], false));
                } else if let Some(last) = segs.last_mut() {
                    let start = text.as_ptr() as usize;
                    let cur_end = last.0.as_ptr() as usize + last.0.len() - start;
                    last.0 = &text[cur_end - last.0.len()..i + ch_len];
                } else {
                    segs.push((&text[i..i + ch_len], false));
                }
                i += ch_len;
            }
        }
        segs
    }

    fn truncate_chars(s: &str, max: usize) -> &str {
        if s.len() <= max {
            return s;
        }
        let mut end = max.min(s.len());
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }

    fn render_row_highlighted(
        &self,
        stdout: &mut std::io::Stdout,
        row: usize,
        prefix: &str,
        content: &str,
        is_selected: bool,
    ) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cols = self.term_w;
        queue!(stdout, MoveTo(0, row as u16))?;
        if is_selected {
            queue!(
                stdout,
                SetForegroundColor(theme.editor.selection_bg.to_crossterm()),
                SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
            )?;
            queue!(stdout, Print(prefix))?;
            for (seg, is_hit) in self.match_segments(content) {
                if is_hit {
                    queue!(
                        stdout,
                        SetForegroundColor(CxColor::Red),
                        SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                        Print(seg),
                        SetForegroundColor(theme.editor.selection_bg.to_crossterm()),
                    )?;
                } else {
                    queue!(stdout, Print(seg))?;
                }
            }
            queue!(stdout, ResetColor)?;
        } else {
            queue!(stdout, Print(prefix))?;
            for (seg, is_hit) in self.match_segments(content) {
                if is_hit {
                    queue!(
                        stdout,
                        SetForegroundColor(CxColor::Red),
                        Print(seg),
                        ResetColor,
                    )?;
                } else {
                    queue!(stdout, Print(seg))?;
                }
            }
            queue!(stdout, ResetColor)?;
        }
        // Clear rest of line without counting ANSI widths precisely.
        queue!(stdout, Clear(ClearType::UntilNewLine))?;
        let _ = cols;
        Ok(())
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cols = self.term_w;
        let rh = self.result_h().max(1);
        let total = self.total();
        let global_total = self.matches.len();
        let (page_start, page_end) = self.page_range();
        let pc = self.page_count();

        queue!(stdout, terminal::Clear(ClearType::All))?;

        // ── Header ──
        let header = format!(
            " gs: {} of {} matches | page {}/{} (10/page)  |  j/k navigate  |  n/p page  |  Enter=open  |  q=quit ",
            if total == 0 { 0 } else { self.selected + 1 },
            global_total,
            if pc == 0 { 0 } else { self.page() + 1 },
            pc.max(1),
        );
        let truncated = Self::truncate_chars(&header, cols);
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(theme.shell.info.to_crossterm()),
            SetBackgroundColor(theme.editor.status_bg.to_crossterm()),
            Print(truncated),
            ResetColor,
        )?;

        // separator
        let sep = "\u{2500}".repeat(cols);
        queue!(stdout, MoveTo(0, 1), Print(&sep), ResetColor)?;

        // ── Results list (current page only) ──
        let preview_top = 2 + rh.max(page_end.saturating_sub(page_start).max(1));
        for i in 0..rh {
            let idx = page_start + i;
            let row = 2 + i;
            if row >= self.term_h {
                break;
            }
            if idx >= page_end {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
                    Clear(ClearType::UntilNewLine)
                )?;
                continue;
            }
            let m = &self.matches[self.filtered[idx]];
            let rel = display_content_path(&m.file_path, &self.root);
            let prefix = format!("{}:{}: ", rel, m.line_number);
            let content = m.line.trim();
            let avail = cols.saturating_sub(prefix.len() + 1);
            let shown = Self::truncate_chars(content, avail.max(10));
            let is_selected = idx == self.selected;
            self.render_row_highlighted(stdout, row, &prefix, shown, is_selected)?;
        }

        // ── Footer / page bar ──
        let footer_row = 2 + rh;
        if footer_row < self.term_h {
            let footer = format!(
                " ── page {}/{} · showing {}–{} of {} · n/p or PgUp/PgDn ── ",
                if pc == 0 { 0 } else { self.page() + 1 },
                pc.max(1),
                if total == 0 { 0 } else { page_start + 1 },
                page_end,
                total,
            );
            let ft = Self::truncate_chars(&footer, cols);
            queue!(stdout, MoveTo(0, footer_row as u16), Print(ft), ResetColor)?;
        }

        // ── Preview separator ──
        if preview_top < self.term_h {
            queue!(
                stdout,
                MoveTo(0, preview_top as u16),
                Print(&sep),
                ResetColor
            )?;
        }

        // ── Preview panel ──
        let (file_path, line_num) = match self.selected_match() {
            Some(m) => (m.file_path.clone(), m.line_number),
            None => return Ok(()),
        };
        let start = line_num.saturating_sub(PREVIEW_LINES);
        let lines = self.get_file_lines(&file_path);

        for i in 0..=PREVIEW_LINES * 2 {
            let row = preview_top + 1 + i;
            if row >= self.term_h {
                break;
            }
            let src_line = start + i;
            let prefix = format!("{:>6} ", src_line);

            let display = if src_line >= 1 && src_line <= lines.len() {
                let text = &lines[src_line - 1];
                let available = cols.saturating_sub(7);
                let text = Self::truncate_chars(text, available);
                format!("{}{}", prefix, text)
            } else {
                format!("{}~", prefix)
            };

            let truncated = Self::truncate_chars(&display, cols);

            if src_line == line_num {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
                    SetForegroundColor(theme.editor.selection_bg.to_crossterm()),
                    SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                    Print(truncated),
                    ResetColor,
                )?;
            } else {
                queue!(stdout, MoveTo(0, row as u16), Print(truncated), ResetColor,)?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}

fn terminal_size() -> (usize, usize) {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
    (w as usize, h as usize)
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            DisableMouseCapture,
            Show,
            LeaveAlternateScreen
        );
    }
}

pub fn run_gs_interactive(
    matches: Vec<ContentMatch>,
    root: &Path,
    pattern: &str,
) -> Result<bool> {
    if matches.is_empty() {
        print_info(&t!("cmd.gs.no_matches"));
        return Ok(false);
    }

    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;

    // Re-query terminal size after entering alternate screen
    let (w, h) = terminal_size();
    let all: Vec<usize> = (0..matches.len()).collect();
    let total = matches.len();
    if total > 10_000 {
        print_info(&format!(
            "Large result set ({} matches). Performance may degrade.",
            total
        ));
    }
    let mut gs = GsInteractive {
        matches,
        filtered: all,
        selected: 0,
        term_h: h,
        term_w: w,
        file_cache: HashMap::new(),
        root: root.to_path_buf(),
        pattern: pattern.to_string(),
    };

    gs.ensure_visible();
    gs.render(&mut stdout)?;

    let result = loop {
        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }
        let ev = event::read()?;
        match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    continue;
                }
                match ke.code {
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(false),
                    KeyCode::Enter => {
                        if let Some(m) = gs.selected_match() {
                            let path = m.file_path.clone();
                            let line = m.line_number;
                            let pat = gs.pattern.clone();
                            drop(_guard);
                            // Flag editor to center + transiently highlight target line.
                            crate::editor::GOTO_HIGHLIGHT.store(true, std::sync::atomic::Ordering::SeqCst);
                            crate::editor::set_goto_pattern(&pat);
                            // Open ntcEditor at the file and line
                            crate::shell::commands::misc::cmd_ne(
                                &format!("{}:{}", path.display(), line),
                                &mut Navigator::new()?,
                            )?;
                        }
                        break Ok(false);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if gs.selected > 0 {
                            gs.selected -= 1;
                        }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if gs.selected + 1 < gs.total() {
                            gs.selected += 1;
                        }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::PageUp => {
                        gs.prev_page();
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::PageDown | KeyCode::Char(' ') => {
                        gs.next_page();
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Char('n') | KeyCode::Right => {
                        gs.next_page();
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Char('p') | KeyCode::Left => {
                        gs.prev_page();
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Home => {
                        gs.selected = 0;
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::End => {
                        let total = gs.total();
                        if total > 0 {
                            gs.selected = total - 1;
                            gs.ensure_visible();
                            gs.render(&mut stdout)?;
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                gs.term_w = w as usize;
                gs.term_h = h as usize;
                gs.ensure_visible();
                gs.render(&mut stdout)?;
            }
            Event::Mouse(m) => match m.kind {
                crossterm::event::MouseEventKind::ScrollUp => {
                    if gs.selected > 0 {
                        gs.selected -= 1;
                    }
                    gs.ensure_visible();
                    gs.render(&mut stdout)?;
                }
                crossterm::event::MouseEventKind::ScrollDown => {
                    if gs.selected + 1 < gs.total() {
                        gs.selected += 1;
                    }
                    gs.ensure_visible();
                    gs.render(&mut stdout)?;
                }
                _ => {}
            },
            _ => {}
        }
    };

    result
}
