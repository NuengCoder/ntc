use crate::search::ContentMatch;
use crate::output::print_info;
use crate::navigator::Navigator;
use anyhow::Result;
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::style::{Print, ResetColor, SetForegroundColor, SetBackgroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{stdout, Write};
use std::path::Path;

const PREVIEW_LINES: usize = 4;

struct GsInteractive {
    matches: Vec<ContentMatch>,
    filtered: Vec<usize>,
    selected: usize,
    scroll_offset: usize,
    term_h: usize,
    term_w: usize,
}

impl GsInteractive {
    fn new(matches: Vec<ContentMatch>) -> Self {
        let all: Vec<usize> = (0..matches.len()).collect();
        let (w, h) = terminal_size();
        Self {
            matches,
            filtered: all,
            selected: 0,
            scroll_offset: 0,
            term_h: h,
            term_w: w,
        }
    }

    fn total(&self) -> usize {
        self.filtered.len()
    }

    fn result_h(&self) -> usize {
        let preview_h = (PREVIEW_LINES + 2).min(self.term_h.saturating_sub(4));
        self.term_h.saturating_sub(4).saturating_sub(preview_h)
    }

    fn selected_match(&self) -> Option<&ContentMatch> {
        self.filtered.get(self.selected).map(|&i| &self.matches[i])
    }

    fn ensure_visible(&mut self) {
        let rh = self.result_h().max(1);
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + rh {
            self.scroll_offset = self.selected.saturating_add(1).saturating_sub(rh);
        }
    }

    fn render(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cols = self.term_w;
        let rh = self.result_h().max(1);
        let total = self.total();
        let display_idx = self.filtered.get(self.selected).copied().unwrap_or(0);
        let global_total = self.matches.len();

        queue!(stdout, terminal::Clear(ClearType::All))?;

        // ── Header ──
        let header = format!(
            " gs: {} of {} matches (in {} files)  |  j/k ↑↓ navigate  |  Enter=open  |  q=quit ",
            display_idx + 1,
            global_total,
            total,
        );
        let truncated = if header.len() > cols { &header[..cols] } else { &header };
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

        // ── Results list ──
        let preview_top = 2 + rh;
        for i in 0..rh {
            let idx = self.scroll_offset + i;
            let row = 2 + i;
            if row >= self.term_h { break; }
            if idx >= total {
                queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::UntilNewLine))?;
                continue;
            }
            let m = &self.matches[self.filtered[idx]];
            let line = m.line.trim();
            let display = format!(
                "{}:{}: {}",
                m.file_path.display(),
                m.line_number,
                if line.len() > cols.saturating_sub(20) { &line[..cols.saturating_sub(23)] } else { line }
            );
            let truncated = if display.len() > cols { &display[..cols] } else { &display };

            let is_selected = idx == self.selected;
            if is_selected {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
            SetForegroundColor(theme.editor.selection_bg.to_crossterm()),
                        SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                        Print(truncated),
                        ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        MoveTo(0, row as u16),
                        Print(truncated),
                        ResetColor,
                    )?;
                }
            }

            // ── Preview separator ──
            if preview_top < self.term_h {
                queue!(stdout, MoveTo(0, preview_top as u16), Print(&sep), ResetColor)?;
            }

            // ── Preview panel ──
            if let Some(m) = self.selected_match() {
                let file_path = &m.file_path;
                let line_num = m.line_number;
                let start = line_num.saturating_sub(PREVIEW_LINES);
                let _end = line_num + PREVIEW_LINES;
                let content = std::fs::read_to_string(file_path).unwrap_or_default();
                let lines: Vec<&str> = content.lines().collect();

                for i in 0..=PREVIEW_LINES * 2 {
                    let row = preview_top + 1 + i;
                    if row >= self.term_h { break; }
                    let src_line = start + i;
                    let prefix = format!("{:>6} ", src_line);

                    let display = if src_line >= 1 && src_line <= lines.len() {
                        let text = lines[src_line - 1];
                        let available = cols.saturating_sub(7);
                        let text = if text.len() > available { &text[..available] } else { text };
                        format!("{}{}", prefix, text)
                    } else {
                        format!("{}~", prefix)
                    };

                    let truncated = if display.len() > cols { &display[..cols] } else { &display };

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
                    queue!(
                        stdout,
                        MoveTo(0, row as u16),
                        Print(truncated),
                        ResetColor,
                    )?;
                }
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
        let _ = execute!(std::io::stdout(), DisableMouseCapture, Show, LeaveAlternateScreen);
    }
}

pub fn run_gs_interactive(matches: Vec<ContentMatch>, _root: &Path) -> Result<bool> {
    if matches.is_empty() {
        print_info("No matches found.");
        return Ok(false);
    }

    let mut gs = GsInteractive::new(matches);
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;

    gs.ensure_visible();
    gs.render(&mut stdout)?;

    let result = loop {
        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }
        let ev = event::read()?;
        match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press { continue; }
                match ke.code {
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(false),
                    KeyCode::Enter => {
                        if let Some(m) = gs.selected_match() {
                            let path = m.file_path.clone();
                            let line = m.line_number;
                            drop(_guard);
                            // Open ntcEditor at the file and line
                            crate::shell::commands::misc::cmd_ne(
                                &format!("{}:{}", path.display(), line),
                                &mut Navigator::new()?,
                            )?;
                        }
                        break Ok(false);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if gs.selected > 0 { gs.selected -= 1; }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if gs.selected + 1 < gs.total() { gs.selected += 1; }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::PageUp => {
                        let page = gs.result_h().max(1);
                        gs.selected = gs.selected.saturating_sub(page);
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::PageDown => {
                        let page = gs.result_h().max(1);
                        gs.selected = (gs.selected + page).min(gs.total().saturating_sub(1));
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    KeyCode::Home => {
                        gs.selected = 0;
                        gs.scroll_offset = 0;
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
            Event::Mouse(m) => {
                match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp => {
                        if gs.selected > 0 { gs.selected -= 1; }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        if gs.selected + 1 < gs.total() { gs.selected += 1; }
                        gs.ensure_visible();
                        gs.render(&mut stdout)?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    };

    result
}
