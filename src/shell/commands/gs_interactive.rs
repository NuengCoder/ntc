use crate::navigator::Navigator;
use crate::output::print_info;
use crate::search::ContentMatch;
use crate::t;
use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const PREVIEW_LINES: usize = 4;
const MAX_CACHED_FILE_LINES: usize = 50_000;

struct GsInteractive {
    matches: Vec<ContentMatch>,
    filtered: Vec<usize>,
    selected: usize,
    scroll_offset: usize,
    term_h: usize,
    term_w: usize,
    file_cache: HashMap<PathBuf, Vec<String>>,
}

impl GsInteractive {
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

    fn ensure_visible(&mut self) {
        let rh = self.result_h().max(1);
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + rh {
            self.scroll_offset = self.selected.saturating_add(1).saturating_sub(rh);
        }
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
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
        let truncated = if header.len() > cols {
            &header[..cols]
        } else {
            &header
        };
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
            if row >= self.term_h {
                break;
            }
            if idx >= total {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
                    Clear(ClearType::UntilNewLine)
                )?;
                continue;
            }
            let m = &self.matches[self.filtered[idx]];
            let line = m.line.trim();
            let display = format!(
                "{}:{}: {}",
                m.file_path.display(),
                m.line_number,
                if line.len() > cols.saturating_sub(20) {
                    &line[..cols.saturating_sub(23)]
                } else {
                    line
                }
            );
            let truncated = if display.len() > cols {
                &display[..cols]
            } else {
                &display
            };

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
                queue!(stdout, MoveTo(0, row as u16), Print(truncated), ResetColor,)?;
            }
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
                let text = if text.len() > available {
                    &text[..available]
                } else {
                    text
                };
                format!("{}{}", prefix, text)
            } else {
                format!("{}~", prefix)
            };

            let truncated = if display.len() > cols {
                &display[..cols]
            } else {
                &display
            };

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

pub fn run_gs_interactive(matches: Vec<ContentMatch>, _root: &Path) -> Result<bool> {
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
        scroll_offset: 0,
        term_h: h,
        term_w: w,
        file_cache: HashMap::new(),
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
