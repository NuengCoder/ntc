use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEventKind};

use crate::editor::{Editor, Focus};
use crate::t;

impl Editor {
    pub(crate) fn handle_terminal_event(
        &mut self,
        ev: Event,
        _stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        match ev {
            Event::Mouse(m) => {
                let eo = self.editor_offset();
                let split = self.run_split_row();

                // Fixed 50% split: the separator row is not draggable and
                // consumes no mouse action.
                if (m.row as usize) == split {
                    return Ok(());
                }

                // Click in editor area → switch focus to editor
                if (m.row as usize) < split {
                    if matches!(
                        m.kind,
                        MouseEventKind::Down(crossterm::event::MouseButton::Left)
                    ) {
                        self.focus = Focus::Editor;
                    }
                    self.handle_mouse(m);
                    return Ok(());
                }

                let panel_top = split + 1;
                let panel_h = self.terminal_content_rows();
                let content_cols = self.terminal_content_cols();
                let (_, _, bright, _) = self.terminal_box();
                // Bottom box border consumes no mouse action.
                let (_, _, _, bbottom) = self.terminal_box();
                if (m.row as usize) == bbottom {
                    return Ok(());
                }
                if let Some(ref mut term) = self.terminal {
                    let total = term.total_lines();
                    let max_scroll = total.saturating_sub(panel_h);
                    let max_scroll_x = term.width.saturating_sub(content_cols);
                    match m.kind {
                        MouseEventKind::ScrollUp => {
                            self.sv.run = self.sv.run.saturating_sub(3);
                            term.follow_mode = false;
                            term.dirty = true;
                        }
                        MouseEventKind::ScrollDown => {
                            let new_scroll = (self.sv.run + 3).min(max_scroll);
                            self.sv.run = new_scroll;
                            if new_scroll >= max_scroll {
                                term.follow_mode = true;
                            }
                            term.dirty = true;
                        }
                        MouseEventKind::ScrollLeft => {
                            self.sv.terminal_scroll_x = self.sv.terminal_scroll_x.saturating_sub(3);
                            term.dirty = true;
                        }
                        MouseEventKind::ScrollRight => {
                            self.sv.terminal_scroll_x =
                                (self.sv.terminal_scroll_x + 3).min(max_scroll_x);
                            term.dirty = true;
                        }
                        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                            let r = m.row as usize;
                            let mcol = m.column as usize;
                            // Content starts one column past the left wall;
                            // clicks on either wall are ignored.
                            if mcol <= eo || mcol >= bright {
                                return Ok(());
                            }
                            let c = mcol - (eo + 1);
                            if r >= panel_top && r < panel_top + panel_h {
                                let line_idx = self.sv.run + (r - panel_top);
                                if line_idx < total {
                                    term.selection_anchor = Some((line_idx, c));
                                    term.selection_active = Some((line_idx, c));
                                    term.dirty = true;
                                }
                            }
                        }
                        MouseEventKind::Drag(crossterm::event::MouseButton::Left) => {
                            let r = m.row as usize;
                            let mcol = m.column as usize;
                            if mcol <= eo || mcol >= bright {
                                return Ok(());
                            }
                            let c = mcol - (eo + 1);
                            if r >= panel_top && r < panel_top + panel_h {
                                let line_idx = self.sv.run + (r - panel_top);
                                if line_idx < total {
                                    term.selection_active = Some((line_idx, c));
                                    term.dirty = true;
                                }
                            }
                        }
                        MouseEventKind::Up(crossterm::event::MouseButton::Left) => {
                            term.dirty = true;
                        }
                        MouseEventKind::Down(crossterm::event::MouseButton::Right) => {
                            if let Some(text) = Self::pull_from_system_clipboard() {
                                if !text.is_empty() {
                                    term.send_input(text.as_bytes());
                                    term.follow_mode = true;
                                    let total = term.total_lines();
                                    let cursor_abs_y = term.cursor_abs_y();
                                    self.sv.run = (cursor_abs_y + 1)
                                        .saturating_sub(panel_h)
                                        .min(total.saturating_sub(panel_h));
                                    term.ensure_cursor_visible_x(
                                        &mut self.sv.terminal_scroll_x,
                                        content_cols,
                                    );
                                    term.dirty = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return Ok(());
            }
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                let alt = modifiers.contains(KeyModifiers::ALT);
                match (ctrl, shift, code) {
                    // Esc / Ctrl+Q fully hide the panel (not just unfocus).
                    (false, false, KeyCode::Esc) | (true, false, KeyCode::Char('q')) => {
                        self.terminal = None;
                        self.focus = Focus::Editor;
                        self.mark_all_dirty();
                    }
                    (true, false, KeyCode::Char('c')) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            if term.has_selection() {
                                if let Some(text) = term.selected_text() {
                                    self.clipboard = text.clone();
                                    Self::push_to_system_clipboard(&text);
                                    self.status_msg =
                                        Some(t!("editor.mode_term_copied", self.clipboard.len()));
                                }
                                term.clear_selection();
                            } else {
                                term.send_input(&[3]);
                                term.follow_mode = true;
                                let total = term.total_lines();
                                let cursor_abs_y = term.cursor_abs_y();
                                self.sv.run = (cursor_abs_y + 1)
                                    .saturating_sub(panel_h)
                                    .min(total.saturating_sub(panel_h));
                                term.ensure_cursor_visible_x(
                                    &mut self.sv.terminal_scroll_x,
                                    content_cols,
                                );
                                term.dirty = true;
                            }
                        }
                    }
                    (true, true, KeyCode::Char('c')) => {
                        if let Some(ref term) = self.terminal {
                            let panel_h = self.terminal_content_rows();
                            let text = term.visible_text(self.sv.run, panel_h);
                            self.clipboard = text.clone();
                            Self::push_to_system_clipboard(&text);
                            self.status_msg =
                                Some(t!("editor.mode_term_copied", self.clipboard.len()));
                        }
                    }
                    (true, false, KeyCode::Char('t')) => {
                        self.terminal = None;
                        self.focus = Focus::Editor;
                        self.mark_all_dirty();
                    }
                    (false, false, KeyCode::Enter) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\r");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Backspace) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(&[127]);
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Tab) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\t");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, _, KeyCode::Char(c)) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            let mut buf = [0u8; 4];
                            let s = c.encode_utf8(&mut buf);
                            term.send_input(s.as_bytes());
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Up) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\x1b[A");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Down) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\x1b[B");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Right) if alt => {
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            let max_scroll_x = term.width.saturating_sub(content_cols);
                            self.sv.terminal_scroll_x =
                                (self.sv.terminal_scroll_x + 3).min(max_scroll_x);
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Left) if alt => {
                        if let Some(ref mut term) = self.terminal {
                            self.sv.terminal_scroll_x = self.sv.terminal_scroll_x.saturating_sub(3);
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Right) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\x1b[C");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Left) => {
                        let panel_h = self.terminal_content_rows();
                        let content_cols = self.terminal_content_cols();
                        if let Some(ref mut term) = self.terminal {
                            term.send_input(b"\x1b[D");
                            term.follow_mode = true;
                            let total = term.total_lines();
                            let cursor_abs_y = term.cursor_abs_y();
                            self.sv.run = (cursor_abs_y + 1)
                                .saturating_sub(panel_h)
                                .min(total.saturating_sub(panel_h));
                            term.ensure_cursor_visible_x(
                                &mut self.sv.terminal_scroll_x,
                                content_cols,
                            );
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::PageUp) => {
                        let panel_h = self.terminal_content_rows();
                        if let Some(ref mut term) = self.terminal {
                            let total = term.total_lines();
                            let max_scroll = total.saturating_sub(panel_h);
                            self.sv.run = self.sv.run.saturating_sub(panel_h);
                            if self.sv.run > max_scroll {
                                self.sv.run = max_scroll;
                            }
                            term.follow_mode = false;
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::PageDown) => {
                        let panel_h = self.terminal_content_rows();
                        if let Some(ref mut term) = self.terminal {
                            let total = term.total_lines();
                            let max_scroll = total.saturating_sub(panel_h);
                            let new_scroll = (self.sv.run + panel_h).min(max_scroll);
                            self.sv.run = new_scroll;
                            if new_scroll >= max_scroll {
                                term.follow_mode = true;
                            }
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::Home) => {
                        self.sv.run = 0;
                        if let Some(ref mut term) = self.terminal {
                            term.follow_mode = false;
                            term.dirty = true;
                        }
                    }
                    (false, false, KeyCode::End) => {
                        let panel_h = self.terminal_content_rows();
                        if let Some(ref mut term) = self.terminal {
                            let total = term.total_lines();
                            self.sv.run = total.saturating_sub(panel_h);
                            term.follow_mode = true;
                            term.dirty = true;
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
                let panel_h = self.terminal_content_rows();
                let (max_scroll, max_scroll_x) = match self.terminal.as_ref() {
                    Some(t) => (
                        t.total_lines().saturating_sub(panel_h),
                        t.width.saturating_sub(panel_h),
                    ),
                    None => (0, 0),
                };
                self.sv.run = self.sv.run.min(max_scroll);
                self.sv.terminal_scroll_x = self.sv.terminal_scroll_x.min(max_scroll_x);
                if let Some(ref mut term) = self.terminal {
                    term.dirty = true;
                }
                self.mark_all_dirty();
            }
            _ => {}
        }
        Ok(())
    }
}
