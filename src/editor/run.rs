use std::io::{stdout, Write};
use std::time::Duration;

use anyhow::Result;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseEvent,
};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;

use super::{Editor, Mode};
use crate::editor::types::RunMessage;

impl Editor {
    pub(crate) fn run(&mut self) -> Result<bool> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
        terminal::enable_raw_mode()?;

        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.main_loop(&mut stdout)));

        terminal::disable_raw_mode()?;
        let _ = execute!(stdout, DisableMouseCapture, Show, LeaveAlternateScreen);
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
            // ── wait for event, polling streaming output while process runs ──
            let ev = if self.mode == Mode::Run && self.run_executing.is_some() {
                if !event::poll(Duration::from_millis(50))? {
                    self.poll_run_output(stdout);
                    if self.dirty_end > 0 {
                        self.render(stdout)?;
                    }
                    continue;
                }
                event::read()?
            } else {
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
            self.prev_cursor_y = self.cursor_y;
            self.prev_cursor_byte = self.cursor_byte;
            let old_scroll = self.scroll;
            let old_scroll_x = self.scroll_x;

            // Detect mouse scroll (should not affect cursor or be clamped by scroll_visible)
            let is_mouse_scroll = matches!(
                &ev,
                Event::Mouse(MouseEvent {
                    kind:
                        crossterm::event::MouseEventKind::ScrollUp
                        | crossterm::event::MouseEventKind::ScrollDown
                        | crossterm::event::MouseEventKind::ScrollLeft
                        | crossterm::event::MouseEventKind::ScrollRight,
                    ..
                })
            );

            // Paste batching (Windows: rapid Char events in Insert mode)
            let mut handled = false;
            if matches!(self.mode, Mode::Insert) {
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
                match self.mode.clone() {
                    Mode::Search => self.handle_search_event(ev)?,
                    Mode::Visual => self.handle_visual_event(ev, stdout)?,
                    Mode::Command => self.handle_command_event(ev, stdout)?,
                    Mode::Help => self.handle_help_event(ev)?,
                    Mode::Insert => self.handle_insert_event(ev, stdout)?,
                    Mode::Normal => self.handle_normal_event(ev, stdout)?,
                    Mode::FileFinder => self.handle_file_finder_event(ev)?,
                    Mode::Gosc => self.handle_gosc_event(ev)?,
                    Mode::Run => self.handle_run_event(ev, stdout)?,
                }
            }

            if is_resize {
                self.mark_all_dirty();
            }

            if !skip_render {
                if !is_mouse_scroll {
                    self.clamp();
                    self.scroll_visible();
                }
                if self.scroll != old_scroll || self.scroll_x != old_scroll_x {
                    self.mark_all_dirty();
                }
                if self.prev_cursor_y != self.cursor_y || self.prev_cursor_byte != self.cursor_byte
                {
                    self.mark_dirty(self.cursor_y);
                    self.mark_dirty(self.prev_cursor_y);
                }
                // Ensure extra cursor lines are always dirty for redraw
                let extra_lines: Vec<usize> = self.extra_cursors.iter().map(|c| c.y).collect();
                for y in extra_lines {
                    self.mark_dirty(y);
                }
                self.render(stdout)?;

                // Poll streaming output after render too
                if self.mode == Mode::Run {
                    self.poll_run_output(stdout);
                    if self.dirty_end > 0 {
                        self.render(stdout)?;
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
                                self.push_run_line(format!("Process exited with code {}", code));
                            } else {
                                self.push_run_line("Process terminated".into());
                            }
                            let panel_h = self.run_panel_height();
                            self.run_scroll = self.run_lines.len().saturating_sub(panel_h);
                        }
                        self.run_executing = None;
                        let _ = execute!(stdout, EnableMouseCapture);
                    }
                    RunMessage::Line(line) => {
                        self.push_run_line(line);
                        let panel_h = self.run_panel_height();
                        self.run_scroll = self.run_lines.len().saturating_sub(panel_h);
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
        use crate::editor::types::MAX_RUN_LINES;
        if self.run_lines.len() >= MAX_RUN_LINES {
            self.run_lines.remove(0);
        }
        self.run_lines.push(line);
    }
}
