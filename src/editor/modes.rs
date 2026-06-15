use anyhow::Result;
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, EnableMouseCapture,
};
use crossterm::execute;
use crossterm::style::Color;

use super::{
    prev_char_byte, next_char_byte, prev_word_byte, next_word_byte,
    set_status_bar, sel_range,
    Editor, Mode,
};
use crate::editor::types::RunMessage;
use crate::search::search_files;

impl Editor {
    // ── search mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_search_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                match code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.status_msg = Some(format!(
                            "Search: {} ({} matches)",
                            self.search.query,
                            self.search.matches.len()
                        ));
                    }
                    KeyCode::Enter => {
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            self.search_prev();
                        } else {
                            self.search_next();
                        }
                    }
                    KeyCode::Tab => self.search_next(),
                    KeyCode::BackTab => self.search_prev(),
                    KeyCode::Backspace => {
                        self.search.query.pop();
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            // Clamp idx
                            self.search.match_idx = self
                                .search.match_idx
                                .min(self.search.matches.len().saturating_sub(1));
                            self.jump_to_match(self.search.match_idx);
                        }
                    }
                    KeyCode::Char(c) => {
                        self.search.query.push(c);
                        self.rebuild_search_matches();
                        // Jump to first match at or after cursor
                        if !self.search.matches.is_empty() {
                            let pos = (self.cursor.y, self.cursor.byte);
                            let idx = self
                                .search.matches
                                .iter()
                                .position(|&(ly, sb, _)| (ly, sb) >= pos)
                                .unwrap_or(0);
                            self.search.match_idx = idx;
                            self.jump_to_match(idx);
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── file finder mode (Ctrl+P) ────────────────────────────────────────────

    pub(crate) fn handle_file_finder_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.ff_query.clear();
                    self.ff_results.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    if !self.ff_results.is_empty() {
                        let path = self.ff_results[self.ff_idx].1.clone();
                        self.open_file(&path);
                    }
                    self.ff_query.clear();
                    self.ff_results.clear();
                    self.mode = Mode::Normal;
                    self.mark_all_dirty();
                }
                KeyCode::Up
                    if self.ff_idx > 0 =>
                {
                    self.ff_idx -= 1;
                    self.mark_all_dirty();
                }
                KeyCode::Down
                    if self.ff_idx + 1 < self.ff_results.len() =>
                {
                    self.ff_idx += 1;
                    self.mark_all_dirty();
                }
                KeyCode::Backspace => {
                    self.ff_query.pop();
                    self.rebuild_ff_results();
                    self.mark_all_dirty();
                }
                KeyCode::Char(c) => {
                    self.ff_query.push(c);
                    self.rebuild_ff_results();
                    self.mark_all_dirty();
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    fn rebuild_ff_results(&mut self) {
        if self.ff_query.is_empty() {
            self.ff_results.clear();
            return;
        }
        let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let results = search_files(&root, &self.ff_query, 10);
        self.ff_results = results
            .into_iter()
            .map(|r| (r.name, r.full_path, r.score))
            .collect();
        self.ff_idx = 0;
    }

    // ── run mode ──────────────────────────────────────────────────────────────

    pub(crate) fn handle_run_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        // Forward resize events
        if let Event::Resize(w, h) = ev {
            self.term_w = w as usize;
            self.term_h = h as usize;
            self.mark_all_dirty();
            return Ok(());
        }

        // Forward mouse events (sidebar, scrollbar, editor clicks)
        if let Event::Mouse(m) = ev {
            let eo = self.editor_offset();
            let col = m.column as usize;
            let split = self.run_split_row();

            // Sidebar click — always handled (sidebar spans full height)
            if self.sidebar.open && col < eo {
                self.handle_mouse(m);
                return Ok(());
            }

            // Scroll wheel in Run panel area
            if m.row as usize >= split {
                match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp
                        if self.sv.run > 0 => {
                            self.sv.run = self.sv.run.saturating_sub(3);
                            self.mark_all_dirty();
                        }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        let panel_h = self.run_panel_height();
                        let max = self.run_lines.len().saturating_sub(panel_h);
                        self.sv.run = (self.sv.run + 3).min(max);
                        self.mark_all_dirty();
                    }
                    _ => {}
                }
                return Ok(());
            }

            // In editor area — forward to standard mouse handler
            self.handle_mouse(m);
            return Ok(());
        }

        // If user is typing a command (run_cmd_buf non-empty), handle that first
        if !self.run_cmd_buf.is_empty() {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = ev { match code {
                KeyCode::Esc => {
                    self.run_cmd_buf.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    let cmd = self.run_cmd_buf.clone();
                    self.run_cmd_buf.clear();
                    if let Some(rest) = cmd.strip_prefix(":run ") {
                        let rest = rest.trim();
                        if !rest.is_empty() {
                            self.start_run(rest);
                        }
                    } else if let Some(rest) = cmd.strip_prefix(":") {
                        let rest = rest.trim();
                        if !rest.is_empty() {
                            self.run_lines.clear();
                            self.sv.run = 0;
                            self.start_run(rest);
                        }
                    }
                    self.mark_all_dirty();
                }
                KeyCode::Backspace => {
                    self.run_cmd_buf.pop();
                    self.mark_all_dirty();
                }
                KeyCode::Char(c) => {
                    self.run_cmd_buf.push(c);
                    self.mark_all_dirty();
                }
                _ => {}
            } }
            return Ok(());
        }

        // Normal Run mode key handling
        match ev {
            Event::Key(KeyEvent { code: KeyCode::Esc, kind: KeyEventKind::Press, .. }) => {
                // Kill running process if any
                if let Some(mut child) = self.run_child.take() {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                self.run_executing = None;
                self.run_rx = None;
                self.run_lines.clear();
                self.sv.run = 0;
                self.mode = Mode::Normal;
                let _ = execute!(stdout, EnableMouseCapture);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::Char(':'), kind: KeyEventKind::Press, .. }) => {
                self.run_cmd_buf = ":".into();
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if let Some(mut child) = self.run_child.take() {
                    let _ = child.kill();
                    let _ = child.wait();
                    self.run_executing = None;
                    self.run_rx = None;
                    self.run_lines.push("^C".into());
                    let panel_h = self.run_panel_height();
                    self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                    let _ = execute!(stdout, EnableMouseCapture);
                    self.mark_all_dirty();
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. })
            | Event::Key(KeyEvent { code: KeyCode::Char('k'), kind: KeyEventKind::Press, .. })
                if self.sv.run > 0 => {
                    self.sv.run -= 1;
                    self.mark_all_dirty();
                }
            Event::Key(KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. })
            | Event::Key(KeyEvent { code: KeyCode::Char('j'), kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                if self.sv.run + panel_h < self.run_lines.len() {
                    self.sv.run += 1;
                    self.mark_all_dirty();
                }
            }
            Event::Key(KeyEvent { code: KeyCode::PageUp, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                self.sv.run = self.sv.run.saturating_sub(panel_h);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::PageDown, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                let max = self.run_lines.len().saturating_sub(panel_h);
                self.sv.run = (self.sv.run + panel_h).min(max);
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::Home, kind: KeyEventKind::Press, .. }) => {
                self.sv.run = 0;
                self.mark_all_dirty();
            }
            Event::Key(KeyEvent { code: KeyCode::End, kind: KeyEventKind::Press, .. }) => {
                let panel_h = self.run_panel_height();
                self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                self.mark_all_dirty();
            }
            _ => {}
        }
        Ok(())
    }

    /// Spawn a shell command with real-time streaming output.
    pub(super) fn start_run(&mut self, cmd_line: &str) {
        // Kill any previous child process
        if let Some(mut old_child) = self.run_child.take() {
            let _ = old_child.kill();
            let _ = old_child.wait();
        }
        self.run_rx = None;
        self.run_executing = None;

        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let cmd_line = cmd_line.to_string();

        let child_result = {
                #[cfg(windows)]
                let shell = "cmd";
                #[cfg(windows)]
                let flag = "/C";
                #[cfg(not(windows))]
                let shell = "sh";
                #[cfg(not(windows))]
                let flag = "-c";
                std::process::Command::new(shell)
                    .args([flag, &cmd_line])
                    .current_dir(&cwd)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
            };

        match child_result {
            Ok(mut child) => {
                let (tx, rx) = std::sync::mpsc::channel::<RunMessage>();

                // Take stdout/stderr pipes before moving them into the thread
                let stdout_pipe = child.stdout.take();
                let stderr_pipe = child.stderr.take();

                // Spawn reader thread (only takes pipes, not child itself)
                std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};

                    let read_pipe = |reader: Option<Box<dyn std::io::Read + Send>>, tx: &std::sync::mpsc::Sender<RunMessage>| {
                        if let Some(pipe) = reader {
                            let buf = BufReader::new(pipe);
                            for l in buf.lines().map_while(Result::ok) {
                                if tx.send(RunMessage::Line(l)).is_err() {
                                    break;
                                }
                            }
                        }
                    };

                    read_pipe(stdout_pipe.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>), &tx);
                    read_pipe(stderr_pipe.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>), &tx);

                    let _ = tx.send(RunMessage::Done);
                });

                self.run_rx = Some(rx);
                self.run_child = Some(child);
                self.run_executing = Some(cmd_line);
                self.run_lines.clear();
                self.sv.run = 0;
                self.mode = Mode::Run;
                self.status_msg = None;
                self.mark_all_dirty();
            }
            Err(e) => {
                self.status_msg = Some(format!("run: {}", e));
            }
        }
    }

    pub(crate) fn run_panel_height(&self) -> usize {
        let split = self.run_split_row();
        self.term_h.saturating_sub(4).saturating_sub(split).max(3)
    }

    pub(crate) fn run_split_row(&self) -> usize {
        (self.term_h * 2 / 3).max(5)
    }

    // ── gosc mode ─────────────────────────────────────────────────────────────

    pub(crate) fn handle_gosc_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.mode = Mode::Normal;
                    self.gosc_buf.clear();
                    self.gosc_dirs.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    let input = self.gosc_buf.trim().to_string();
                    self.gosc_buf.clear();
                    if input.is_empty() || input == "0" {
                        self.mode = Mode::Normal;
                        self.gosc_dirs.clear();
                        self.mark_all_dirty();
                    } else if let Some(back_str) = input.strip_prefix('-') {
                        let n: usize = back_str.parse().unwrap_or(1);
                        let mut target =
                            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                        let mut ok = true;
                        for _ in 0..n {
                            match target.parent() {
                                Some(p) => target = p.to_path_buf(),
                                None => {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                        if ok {
                            self.cd_to(&target);
                        } else {
                            self.status_msg = Some("Already at root".into());
                        }
                        self.gosc_dirs = self.list_subdirs();
                        self.mark_all_dirty();
                    } else if let Ok(n) = input.parse::<usize>() {
                        if n > 0 && n <= self.gosc_dirs.len() {
                            let cwd = std::env::current_dir()
                                .unwrap_or_else(|_| std::path::PathBuf::from("."));
                            let target = cwd.join(&self.gosc_dirs[n - 1]);
                            self.cd_to(&target);
                            self.gosc_dirs = self.list_subdirs();
                            self.mark_all_dirty();
                        }
                    }
                }
                KeyCode::Backspace => {
                    self.gosc_buf.pop();
                }
                KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
                    self.gosc_buf.push(c);
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── normal mode (read-only) ────────────────────────────────────────────────

    pub(crate) fn handle_normal_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);
                match (ctrl, code) {
                    (true, KeyCode::Char('i')) => {
                        self.mode = Mode::Insert;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg =
                            Some("Visual mode — arrow keys to select, Esc to exit".into());
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.mark_all_dirty();
                    }
                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }
                    (false, KeyCode::Char(':')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    // ── Ctrl+Q: close tab / quit ──────────────────────────────
                    (true, KeyCode::Char('q')) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                            self.status_msg = Some("Cleared extra cursors".into());
                        } else {
                            let should_quit = self.close_current_tab();
                            if should_quit && self.exit_flow(stdout)? {
                                return Err(anyhow::anyhow!("__exit__"));
                            }
                        }
                    }
                    // ── Ctrl+Shift+Q: close all tabs and quit ──────────────────
                    (true, KeyCode::Char('Q'))
                        if self.exit_flow(stdout)? => {
                            return Err(anyhow::anyhow!("__exit__"));
                        }
                    // ── Esc: clear extra cursors or exit if last tab ───────────
                    (false, KeyCode::Esc) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                            self.status_msg = Some("Cleared extra cursors".into());
                        } else if self.exit_flow(stdout)? {
                            return Err(anyhow::anyhow!("__exit__"));
                        }
                    }
                    // ── Ctrl+B: toggle sidebar ─────────────────────────────────
                    (true, KeyCode::Char('b')) => {
                        self.toggle_sidebar();
                    }
                    // ── Ctrl+P: file finder (opens in new tab) ────────────────
                    (true, KeyCode::Char('p')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    // ── Ctrl+T: also file finder (opens in new tab) ────────────
                    (true, KeyCode::Char('t')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    // ── Ctrl+PgDn: next tab ───────────────────────────────────
                    (true, KeyCode::PageDown) => {
                        self.next_tab();
                        self.mark_all_dirty();
                    }
                    // ── Ctrl+PgUp: prev tab ───────────────────────────────────
                    (true, KeyCode::PageUp) => {
                        self.prev_tab();
                        self.mark_all_dirty();
                    }
                    // ── Ctrl+Tab / Ctrl+Shift+Tab: buffer switching ────────────
                    (true, KeyCode::Tab) if !modifiers.contains(KeyModifiers::SHIFT) => {
                        self.next_buffer();
                    }
                    (true, KeyCode::BackTab) | (true, KeyCode::Tab) => {
                        self.prev_buffer();
                    }
                    // ── Ctrl+D: add cursor at next occurrence ──────────────────
                    (true, KeyCode::Char('d')) => {
                        self.add_cursor_at_next_occurrence();
                    }
                    // ── Ctrl+G: jump to last added cursor (multi-cursor) ─────
                    (true, KeyCode::Char('g')) => {
                        self.jump_to_last_cursor();
                    }
                    // ── Horizontal scroll with Alt+Left/Right ─────────────────
                    (false, KeyCode::Left) if alt => {
                        self.sv.x = self.sv.x.saturating_sub(3);
                    }
                    (false, KeyCode::Right) if alt => {
                        self.sv.x = (self.sv.x + 3).min(self.max_scroll_x());
                    }
                    // Navigation — with multi-cursor support
                    (false, KeyCode::Up) if self.cursor.y > 0 => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor.y > 0 {
                                    e.cursor.y -= 1;
                                }
                            });
                        } else {
                            self.cursor.y -= 1;
                        }
                    }
                    (false, KeyCode::Down) if self.cursor.y + 1 < self.lines.len() => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor.y + 1 < e.lines.len() {
                                    e.cursor.y += 1;
                                }
                            });
                        } else {
                            self.cursor.y += 1;
                        }
                    }
                    (false, KeyCode::Left) if self.cursor.byte > 0 => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor.byte > 0 {
                                    e.cursor.byte = prev_char_byte(e.current(), e.cursor.byte);
                                }
                            });
                        } else {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        }
                    }
                    (false, KeyCode::Right) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                let line = e.current();
                                if e.cursor.byte < line.len() {
                                    e.cursor.byte = next_char_byte(line, e.cursor.byte);
                                }
                            });
                        } else {
                            let line = self.current();
                            if self.cursor.byte < line.len() {
                                self.cursor.byte = next_char_byte(line, self.cursor.byte);
                            }
                        }
                    }
                    (false, KeyCode::Home) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                let first_non_ws = e
                                    .current()
                                    .char_indices()
                                    .find(|(_, c)| !c.is_whitespace())
                                    .map(|(b, _)| b)
                                    .unwrap_or(0);
                                e.cursor.byte = if e.cursor.byte != first_non_ws {
                                    first_non_ws
                                } else {
                                    0
                                };
                                e.sv.x = 0;
                            });
                        } else {
                            let first_non_ws = self
                                .current()
                                .char_indices()
                                .find(|(_, c)| !c.is_whitespace())
                                .map(|(b, _)| b)
                                .unwrap_or(0);
                            self.cursor.byte = if self.cursor.byte != first_non_ws {
                                first_non_ws
                            } else {
                                0
                            };
                            self.sv.x = 0;
                        }
                    }
                    (false, KeyCode::End) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor.byte = e.current().len();
                            });
                        } else {
                            self.cursor.byte = self.current().len();
                        }
                    }
                    (false, KeyCode::PageUp) => {
                        let rows = self.term_h.saturating_sub(4);
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor.y = e.cursor.y.saturating_sub(rows);
                            });
                        } else {
                            self.cursor.y = self.cursor.y.saturating_sub(rows);
                        }
                    }
                    (false, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(4);
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor.y = (e.cursor.y + rows).min(e.lines.len().saturating_sub(1));
                            });
                        } else {
                            self.cursor.y =
                                (self.cursor.y + rows).min(self.lines.len().saturating_sub(1));
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(m) => self.handle_mouse(m),
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── command mode ────────────────────────────────────────────────────────────

    pub(crate) fn handle_command_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.cmd_buf.clear();
                }
                KeyCode::Enter => {
                    let cmd = self.cmd_buf.clone();
                    self.cmd_buf.clear();
                    if self.execute_command(&cmd, stdout)? {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    if self.mode == Mode::Command {
                        self.mode = Mode::Normal;
                    }
                }
                KeyCode::Backspace => {
                    self.cmd_buf.pop();
                }
                KeyCode::Char(c) => {
                    self.cmd_buf.push(c);
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }


    // ── help mode ───────────────────────────────────────────────────────────────

    pub(crate) fn handle_help_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.mark_all_dirty();
                    self.mode = Mode::Normal;
                    self.status_msg = None;
                }
                KeyCode::Up => {
                    self.sv.help = self.sv.help.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.sv.help += 1;
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── visual mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_visual_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
                    (_, KeyCode::Esc) => {
                        self.clear_selection();
                        self.mode = Mode::Normal;
                    }
                    (true, KeyCode::Char('i')) => {
                        self.mode = Mode::Insert;
                        self.clear_selection();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (false, KeyCode::Left) => {
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        }
                    }
                    (false, KeyCode::Right) => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_char_byte(line, self.cursor.byte);
                        }
                    }
                    (false, KeyCode::Up) => {
                        self.start_or_extend_selection();
                        if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                        }
                    }
                    (false, KeyCode::Down) => {
                        self.start_or_extend_selection();
                        if self.cursor.y + 1 < self.lines.len() {
                            self.cursor.y += 1;
                        }
                    }
                    (false, KeyCode::Home) => {
                        self.start_or_extend_selection();
                        let first_non_ws = self
                            .current()
                            .char_indices()
                            .find(|(_, c)| !c.is_whitespace())
                            .map(|(b, _)| b)
                            .unwrap_or(0);
                        self.cursor.byte = if self.cursor.byte != first_non_ws {
                            first_non_ws
                        } else {
                            0
                        };
                        self.sv.x = 0;
                    }
                    (false, KeyCode::End) => {
                        self.start_or_extend_selection();
                        self.cursor.byte = self.current().len();
                    }
                    (true, KeyCode::Char('c')) => {
                        self.copy_selection();
                        self.mode = Mode::Normal;
                    }
                    (true, KeyCode::Char('x')) => {
                        self.cut_selection();
                        self.mode = Mode::Normal;
                    }
                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }
                    _ => {}
                }
            }
            Event::Mouse(m) => self.handle_mouse(m),
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── insert mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_insert_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        // Clear transient status message on any keypress
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        }) = &ev
        {
            self.status_msg = None;
        }

        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);

                // ── Completion handling ──────────────────────────────────
                if self.completion_visible {
                    match (ctrl, code) {
                        (false, KeyCode::Esc) => {
                            self.dismiss_completion();
                            return Ok(());
                        }
                        (false, KeyCode::Tab) => {
                            self.select_next_completion();
                            return Ok(());
                        }
                        (true, KeyCode::Char('n')) => {
                            self.select_next_completion();
                            return Ok(());
                        }
                        (true, KeyCode::Char('p')) => {
                            self.select_prev_completion();
                            return Ok(());
                        }
                        (false, KeyCode::Down) => {
                            self.select_next_completion();
                            return Ok(());
                        }
                        (false, KeyCode::Enter) => {
                            self.apply_completion();
                            return Ok(());
                        }
                        _ => {}
                    }
                }

                match (ctrl, alt, shift, code) {
                    // ── File ops ──────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('s')) => match self.save() {
                        Ok(()) => {
                            let _ = set_status_bar(stdout, Color::Green, Color::White, " Saved ✓");
                        }
                        Err(e) => {
                            let _ = set_status_bar(
                                stdout,
                                Color::Red,
                                Color::White,
                                &format!(" Save failed: {}", e),
                            );
                        }
                    },
                    (true, _, _, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (false, false, false, KeyCode::Esc) => {
                        self.clear_selection();
                        self.clear_extra_cursors();
                        self.mode = Mode::Normal;
                        self.status_msg = None;
                    }

                    // ── Undo / Redo ───────────────────────────────────────────
                    (true, _, _, KeyCode::Char('z')) => self.undo(),
                    (true, _, _, KeyCode::Char('r')) => self.redo(),

                    // ── Clipboard ─────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('a')) => {
                        self.clear_extra_cursors();
                        self.select_all();
                    }
                    (true, _, _, KeyCode::Char('c')) => self.copy_selection(),
                    (true, _, _, KeyCode::Char('x')) => self.cut_selection(),
                    // ── Paste ──────────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('v')) => {} // block ^V (Windows system paste)
                    (true, _, _, KeyCode::Char('p')) => self.paste(),

                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, _, _, KeyCode::Char('='))
                    | (true, _, _, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }

                    // ── Line operations / multi-cursor ─────────────────────────
                    (true, _, _, KeyCode::Char('d')) => self.add_cursor_at_next_occurrence(),
                    (true, _, _, KeyCode::Char('g')) => self.jump_to_last_cursor(),
                    (true, _, _, KeyCode::Char('k')) => self.kill_line(),
                    (true, _, _, KeyCode::Char('l')) => self.select_line(),
                    (true, _, _, KeyCode::Char('m')) => {
                        // Ctrl+M: move cursor to middle of current line
                        let mid = self.current().len() / 2;
                        self.cursor.byte = mid;
                    }
                    (false, true, _, KeyCode::Up) => self.move_line_up(),
                    (false, true, _, KeyCode::Down) => self.move_line_down(),

                    // ── Search ────────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.mark_all_dirty();
                    }
                    // Continue search with n / N (when not in search mode)
                    (false, false, false, KeyCode::F(3)) if !self.search.query.is_empty() => {
                        self.search_next();
                    }
                    (false, false, true, KeyCode::F(3)) if !self.search.query.is_empty() => {
                        self.search_prev();
                    }

                    // ── Help / Visual / Yank ──────────────────────────────────
                    (true, _, _, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg =
                            Some("Visual mode — arrow keys to select, Esc to exit".into());
                    }
                    (true, _, _, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, _, _, KeyCode::Char('y')) => self.yank_line(),

                    // ── Sidebar / buffers ───────────────────────────────────────
                    (true, false, false, KeyCode::Char('b')) => self.toggle_sidebar(),
                    (true, false, false, KeyCode::Tab) => self.next_buffer(),
                    (true, false, true, KeyCode::Tab) => self.prev_buffer(),

                    // ── Horizontal scroll with Alt+Left/Right ─────────────────
                    (false, true, false, KeyCode::Left) => {
                        self.sv.x = self.sv.x.saturating_sub(3);
                    }
                    (false, true, false, KeyCode::Right) => {
                        self.sv.x = (self.sv.x + 3).min(self.max_scroll_x());
                    }

                    // ── Cursor movement (plain) ───────────────────────────────
                    (false, false, false, KeyCode::Up) => {
                        self.move_cursors(|e| {
                            if e.cursor.y > 0 {
                                e.cursor.y -= 1;
                            }
                        });
                    }
                    (false, false, false, KeyCode::Down) => {
                        self.move_cursors(|e| {
                            if e.cursor.y + 1 < e.lines.len() {
                                e.cursor.y += 1;
                            }
                        });
                    }
                    (false, false, false, KeyCode::Left) => {
                        self.move_cursors(|e| {
                            if let Some((ay, ab)) = e.cursor.selection_anchor {
                                if (ay, ab) != (e.cursor.y, e.cursor.byte) {
                                    let ((sy, sb), _) = sel_range(e.cursor.y, e.cursor.byte, ay, ab);
                                    e.cursor.y = sy;
                                    e.cursor.byte = sb;
                                }
                                e.clear_selection();
                            } else if e.cursor.byte > 0 {
                                e.cursor.byte = prev_char_byte(e.current(), e.cursor.byte);
                            }
                        });
                    }
                    (false, false, false, KeyCode::Right) => {
                        self.move_cursors(|e| {
                            if let Some((ay, ab)) = e.cursor.selection_anchor {
                                if (ay, ab) != (e.cursor.y, e.cursor.byte) {
                                    let (_, (ey, eb)) = sel_range(e.cursor.y, e.cursor.byte, ay, ab);
                                    e.cursor.y = ey;
                                    e.cursor.byte = eb;
                                }
                                e.clear_selection();
                            } else {
                                let line = e.current();
                                let byte_len = line.len();
                                if e.cursor.byte < byte_len {
                                    e.cursor.byte = next_char_byte(line, e.cursor.byte);
                                }
                            }
                        });
                    }

                    // ── Shift+Arrow = extend selection ────────────────────────
                    (false, false, true, KeyCode::Up) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                        }
                    }
                    (false, false, true, KeyCode::Down) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        if self.cursor.y + 1 < self.lines.len() {
                            self.cursor.y += 1;
                        }
                    }
                    (false, false, true, KeyCode::Left) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        }
                    }
                    (false, false, true, KeyCode::Right) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        let line = self.current();
                        let byte_len = line.len();
                        if self.cursor.byte < byte_len {
                            self.cursor.byte = next_char_byte(line, self.cursor.byte);
                        }
                    }

                    // ── Ctrl+Arrow = word jump ────────────────────────────────
                    (true, _, false, KeyCode::Left) => {
                        self.move_cursors(|e| {
                            if e.cursor.byte > 0 {
                                e.cursor.byte = prev_word_byte(e.current(), e.cursor.byte);
                            } else if e.cursor.y > 0 {
                                e.cursor.y -= 1;
                                e.cursor.byte = e.current().len();
                            }
                        });
                    }
                    (true, _, false, KeyCode::Right) => {
                        self.move_cursors(|e| {
                            let line = e.current();
                            if e.cursor.byte < line.len() {
                                e.cursor.byte = next_word_byte(line, e.cursor.byte);
                            } else if e.cursor.y + 1 < e.lines.len() {
                                e.cursor.y += 1;
                                e.cursor.byte = 0;
                            }
                        });
                    }

                    // ── Ctrl+Shift+Arrow = word-select ────────────────────────
                    (true, _, true, KeyCode::Left) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_word_byte(self.current(), self.cursor.byte);
                        }
                    }
                    (true, _, true, KeyCode::Right) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_word_byte(line, self.cursor.byte);
                        }
                    }

                    // ── Home / End ────────────────────────────────────────────
                    (false, _, false, KeyCode::Home) => {
                        self.move_cursors(|e| {
                            let first_non_ws = e
                                .current()
                                .char_indices()
                                .find(|(_, c)| !c.is_whitespace())
                                .map(|(b, _)| b)
                                .unwrap_or(0);
                            e.cursor.byte = if e.cursor.byte != first_non_ws {
                                first_non_ws
                            } else {
                                0
                            };
                            e.sv.x = 0;
                        });
                    }
                    (false, _, false, KeyCode::End) => {
                        self.move_cursors(|e| {
                            e.cursor.byte = e.current().len();
                        });
                    }
                    (false, _, true, KeyCode::Home) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        self.cursor.byte = 0;
                        self.sv.x = 0;
                    }
                    (false, _, true, KeyCode::End) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                        }
                        self.start_or_extend_selection();
                        self.cursor.byte = self.current().len();
                    }
                    // Ctrl+Home / Ctrl+End = file start/end
                    (true, _, _, KeyCode::Home) => {
                        self.move_cursors(|e| {
                            e.cursor.y = 0;
                            e.cursor.byte = 0;
                        });
                    }
                    (true, _, _, KeyCode::End) => {
                        self.move_cursors(|e| {
                            e.cursor.y = e.lines.len().saturating_sub(1);
                            e.cursor.byte = e.current().len();
                        });
                    }

                    // ── Page up / down ────────────────────────────────────────
                    (false, _, _, KeyCode::PageUp) => {
                        let rows = self.term_h.saturating_sub(4);
                        self.move_cursors(|e| {
                            e.cursor.y = e.cursor.y.saturating_sub(rows);
                        });
                    }
                    (false, _, _, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(4);
                        self.move_cursors(|e| {
                            e.cursor.y = (e.cursor.y + rows).min(e.lines.len().saturating_sub(1));
                        });
                    }

                    // ── Tab / Shift+Tab ───────────────────────────────────────
                    (false, false, false, KeyCode::Tab) => self.indent_lines(),
                    (false, false, _, KeyCode::BackTab) => self.dedent_lines(),

                    // ── Enter ─────────────────────────────────────────────────
                    (false, false, false, KeyCode::Enter) => {
                        if self.has_multiple_cursors() {
                            self.multi_split_line();
                        } else {
                            if self.has_selection() {
                                self.snapshot();
                                self.delete_selection();
                            }
                            self.snapshot();
                            self.split_line();
                        }
                    }

                    // ── Backspace / Delete ────────────────────────────────────
                    (false, false, _, KeyCode::Backspace) => {
                        if self.completion_visible {
                            self.dismiss_completion();
                        }
                        if self.has_multiple_cursors() {
                            self.multi_backspace();
                        } else {
                            self.backspace();
                        }
                    }
                    (false, false, _, KeyCode::Delete) => {
                        if self.completion_visible {
                            self.dismiss_completion();
                        }
                        if self.has_multiple_cursors() {
                            self.multi_delete_forward();
                        } else {
                            self.delete_forward();
                        }
                    }

                    // ── Regular character input with auto-pair ─────────────────
                    (false, false, false, KeyCode::Char(c))
                    | (false, false, true, KeyCode::Char(c)) => {
                        self.handle_char_input(c);
                    }

                    _ => {}
                }
            }

            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }

            Event::Mouse(m) => self.handle_mouse(m),

            _ => {}
        }

        Ok(())
    }
}
