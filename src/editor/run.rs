use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseEvent,
};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

use super::{Editor, Focus, Mode};
use crate::editor::types::RunMessage;
use crate::t;

/// Drop guard that restores the terminal to a usable state on any exit path
/// (normal return, error, or panic).
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Self {
        TerminalGuard
    }
}

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

impl Editor {
    pub(crate) fn run(&mut self) -> Result<bool> {
        let mut stdout = stdout();
        let _guard = TerminalGuard::new();
        execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
        terminal::enable_raw_mode()?;

        // Start file watcher for live reload
        self.start_file_watcher();

        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.main_loop(&mut stdout)));

        drop(_guard);
        let _ = stdout.flush();
        result.map_err(|e| {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            anyhow::anyhow!("Editor panicked: {}", msg)
        })?
    }

    pub(crate) fn main_loop(&mut self, stdout: &mut std::io::Stdout) -> Result<bool> {
        self.mark_all_dirty();
        self.render(stdout)?;
        loop {
            // ── Check if theme changed ──
            if crate::utils::theme::ThemeManager::take_theme_changed() {
                // Force a full redraw
                self.mark_all_dirty();
                self.syntax.invalidate_all(); // Force syntax to refresh colors
                self.render(stdout)?;
            }
            // ── Check for external file changes ──
            if self.check_file_changed() {
                self.render(stdout)?;
            }
            // ── Check for sidebar directory changes ──
            if self.sidebar.check_changed() {
                self.file_index = None; // Invalidate file finder cache too
                self.render(stdout)?;
            }
            // ── Blink tick — runs even when no input arrives ──
            let blink_rate = self.editor_cfg.cursor_blink_rate_ms;
            if blink_rate > 0 && self.last_blink.elapsed().as_millis() as u64 >= blink_rate {
                self.cursor_visible = !self.cursor_visible;
                self.last_blink = Instant::now();
                self.mark_dirty(self.cursor.y);
                let ec_ys: Vec<usize> = self.cursor.extra_cursors.iter().map(|c| c.y).collect();
                for y in ec_ys {
                    self.mark_dirty(y);
                }
                self.render(stdout)?;
            }
            // ── Terminal cursor blink — only when focused ──
            if let Some(ref mut term) = self.terminal {
                if self.focus == Focus::Terminal {
                    term.tick_cursor(blink_rate);
                } else if !term.cursor_visible {
                    term.cursor_visible = true;
                    term.dirty = true;
                }
            }
            // ── Session auto-save ──
            if self.editor_cfg.session_auto_save_secs > 0
                && self.last_session_save.elapsed().as_secs()
                    >= self.editor_cfg.session_auto_save_secs
            {
                let snapshot = self.capture_full_session();
                let mut session = crate::session::SessionState::write_global();
                session.session_snapshot = Some(snapshot);
                session.save();
                self.last_session_save = Instant::now();
            }
            // ── wait for event, polling streaming output while process runs ──
            let poll_ms = self.editor_cfg.poll_rate_ms.min(50);
            let has_run = self.run_panel_open && self.run_executing.is_some();
            let has_term = self.terminal.is_some();
            let ev = if has_run {
                if !event::poll(Duration::from_millis(poll_ms))? {
                    self.poll_run_output(stdout);
                    if self.dirty_end > 0 {
                        self.render(stdout)?;
                    }
                    continue;
                }
                event::read()?
            } else if has_term {
                if !event::poll(Duration::from_millis(poll_ms))? {
                    let mut is_dirty = false;
                    let mut follow = false;
                    if let Some(ref mut term) = self.terminal {
                        term.poll_output();
                        is_dirty = term.dirty;
                        follow = term.follow_mode;
                    }
                    if is_dirty {
                        // Auto-scroll to cursor if follow mode is on
                        if follow {
                            if let Some(ref term) = self.terminal {
                                let panel_h = self.run_panel_height();
                                let total = term.total_lines();
                                let cursor_abs_y = term.scrollback.len() + term.cursor_y;
                                self.sv.run = (cursor_abs_y + 1)
                                    .saturating_sub(panel_h)
                                    .min(total.saturating_sub(panel_h));
                            }
                        }
                        // Horizontal follow — keep cursor in visible range
                        if let Some(ref term) = self.terminal {
                            let editor_cols = self.term_w.saturating_sub(self.editor_offset());
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                editor_cols.saturating_sub(1),
                            );
                        }
                        self.mark_all_dirty();
                        self.render(stdout)?;
                        if let Some(ref mut term) = self.terminal {
                            term.clear_dirty();
                        }
                    }
                    continue;
                }
                event::read()?
            } else {
                if !event::poll(Duration::from_millis(poll_ms))? {
                    continue;
                }
                event::read()?
            };

            let skip_render = matches!(
                &ev,
                Event::Mouse(MouseEvent {
                    kind: crossterm::event::MouseEventKind::Moved,
                    ..
                })
            );

            // Track previous state for dirty-region diffing
            self.cursor.prev_y = self.cursor.y;
            self.cursor.prev_byte = self.cursor.byte;
            let old_scroll = self.sv.offset;
            let old_scroll_x = self.sv.x;

            // Detect mouse scroll (should not affect cursor or be clamped by scroll_visible)
            let is_mouse_scroll = matches!(
                &ev,
                Event::Mouse(MouseEvent {
                    kind: crossterm::event::MouseEventKind::ScrollUp
                        | crossterm::event::MouseEventKind::ScrollDown
                        | crossterm::event::MouseEventKind::ScrollLeft
                        | crossterm::event::MouseEventKind::ScrollRight,
                    ..
                })
            );

            // Transient `gs -i` jump highlight: cleared by any key press or
            // mouse click, but NOT by mouse scroll / move / resize.
            if self.gs_jump_highlight.is_some() {
                let should_clear = match &ev {
                    Event::Key(_) => true,
                    Event::Mouse(m) => matches!(
                        m.kind,
                        crossterm::event::MouseEventKind::Down(_)
                            | crossterm::event::MouseEventKind::Up(_)
                            | crossterm::event::MouseEventKind::Drag(_)
                    ),
                    _ => false,
                };
                if should_clear {
                    self.clear_gs_jump_highlight();
                }
            }

            // Paste batching (Windows: rapid Char events in Insert mode)
            let mut handled = false;
            if matches!(self.mode, Mode::Insert | Mode::InsertMulti) {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers,
                    kind: KeyEventKind::Press,
                    ..
                }) = &ev
                {
                    // Only batch unmodified chars — Ctrl/Alt keys are commands, not paste
                    if !modifiers.contains(KeyModifiers::CONTROL)
                        && !modifiers.contains(KeyModifiers::ALT)
                    {
                        self.paste_buf.clear();
                        self.paste_buf.push(*c);
                        while event::poll(Duration::from_micros(500))? {
                            if let Event::Key(KeyEvent {
                                code: KeyCode::Char(c2),
                                kind: KeyEventKind::Press,
                                ..
                            }) = event::read()?
                            {
                                self.paste_buf.push(c2);
                            } else {
                                break;
                            }
                        }
                        if self.paste_buf.len() > 1 {
                            let buf = self.paste_buf.clone();
                            if self.has_multiple_cursors() {
                                self.multi_paste_fast(&buf);
                            } else {
                                if self.has_selection() {
                                    self.snapshot();
                                    self.delete_selection();
                                }
                                self.snapshot();
                                self.paste_fast(&buf);
                            }
                            handled = true;
                        }
                        self.paste_buf.clear();
                    }
                }
            }

            let is_resize = matches!(&ev, Event::Resize(..));

            if !handled {
                // Check pending quit from mouse tab-close
                if self.pending_quit {
                    self.pending_quit = false;
                    if self.exit_flow(stdout)? {
                        return Ok(true);
                    }
                }
                match self.focus {
                    Focus::Run => self.handle_run_event(ev, stdout)?,
                    Focus::Terminal => self.handle_terminal_event(ev, stdout)?,
                    Focus::Editor => match self.mode.clone() {
                        Mode::Search => self.handle_search_event(ev)?,
                        Mode::Visual => self.handle_visual_event(ev, stdout)?,
                        Mode::Command => self.handle_command_event(ev, stdout)?,
                        Mode::Help => self.handle_help_event(ev)?,
                        Mode::Insert => self.handle_insert_event(ev, stdout)?,
                        Mode::InsertMulti => self.handle_insert_multi_event(ev, stdout)?,
                        Mode::Jump => self.handle_jump_event(ev)?,
                        Mode::Normal => self.handle_normal_event(ev, stdout)?,
                        Mode::FileFinder => self.handle_file_finder_event(ev)?,
                        Mode::Gosc => self.handle_gosc_event(ev)?,
                        Mode::Gmode => self.handle_gmode_event(ev, stdout)?,
                        Mode::Nfe => self.handle_nfe_event(ev, stdout)?,
                        Mode::Query => self.handle_query_event(ev)?,
                        Mode::Encoding => self.handle_encoding_event(ev)?,
                        Mode::GitDiff => self.handle_git_diff_event(ev, stdout)?,
                    },
                }
            }

            if is_resize {
                self.mark_all_dirty();
                // Resize the embedded terminal if active
                if let Some(ref mut term) = self.terminal {
                    let (tw, th) = crate::editor::helpers::term_size();
                    let term_h = (th as f64 * 0.4) as u16;
                    term.resize(tw as u16, term_h);
                }
            }

            if !skip_render {
                if !is_mouse_scroll {
                    self.clamp();
                    self.scroll_visible();
                }
                if self.sv.offset != old_scroll || self.sv.x != old_scroll_x {
                    self.mark_all_dirty();
                }
                if self.cursor.prev_y != self.cursor.y || self.cursor.prev_byte != self.cursor.byte
                {
                    self.mark_dirty(self.cursor.y);
                    self.mark_dirty(self.cursor.prev_y);
                    self.last_blink = Instant::now();
                    self.cursor_visible = true;
                }
                self.render(stdout)?;

                // Poll streaming output after render too
                if self.run_panel_open {
                    self.poll_run_output(stdout);
                    if self.dirty_end > 0 {
                        self.render(stdout)?;
                    }
                }

                // Poll terminal output
                if self.terminal.is_some() {
                    let mut is_dirty = false;
                    let mut follow = false;
                    if let Some(ref mut term) = self.terminal {
                        term.poll_output();
                        is_dirty = term.dirty;
                        follow = term.follow_mode;
                    }
                    if is_dirty {
                        if follow {
                            if let Some(ref term) = self.terminal {
                                let panel_h = self.run_panel_height();
                                let total = term.total_lines();
                                let cursor_abs_y = term.scrollback.len() + term.cursor_y;
                                self.sv.run = (cursor_abs_y + 1)
                                    .saturating_sub(panel_h)
                                    .min(total.saturating_sub(panel_h));
                            }
                        }
                        // Horizontal follow — keep cursor in visible range
                        if let Some(ref term) = self.terminal {
                            let editor_cols = self.term_w.saturating_sub(self.editor_offset());
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                editor_cols.saturating_sub(1),
                            );
                        }
                        self.mark_all_dirty();
                        self.render(stdout)?;
                        if let Some(ref mut term) = self.terminal {
                            term.clear_dirty();
                        }
                    }
                }
            }
        }
    }

    /// Poll the run output channel for new lines and re-render if needed.
    fn poll_run_output(&mut self, stdout: &mut std::io::Stdout) {
        let mut changed = false;
        if let Some(rx) = self.run_rx.take() {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    RunMessage::Done => {
                        if let Some(mut child) = self.run_child.take() {
                            let exit_code = child.wait().ok().and_then(|s| s.code());
                            if let Some(code) = exit_code {
                                self.push_run_line(t!("editor.run_exited", code));
                            } else {
                                self.push_run_line(t!("editor.run_terminated"));
                            }
                            let panel_h = self.run_panel_height();
                            self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                        }
                        self.last_run_cmd = self.run_executing.take();
                        let _ = execute!(stdout, EnableMouseCapture);
                    }
                    RunMessage::Line(line) => {
                        self.push_run_line(line);
                        let panel_h = self.run_panel_height();
                        self.sv.run = self.run_lines.len().saturating_sub(panel_h);
                    }
                }
                changed = true;
            }
            if self.run_executing.is_some() {
                self.run_rx = Some(rx);
            }
        }
        if changed {
            self.mark_all_dirty();
        }
    }

    fn push_run_line(&mut self, line: String) {
        if self.run_lines.len() >= self.editor_cfg.max_run_lines {
            self.run_lines.remove(0);
        }
        self.run_lines.push(line);
    }
}
