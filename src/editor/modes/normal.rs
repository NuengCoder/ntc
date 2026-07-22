use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{prev_char_byte, next_char_byte, prev_word_byte, next_word_byte, Editor, Mode};

impl Editor {
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
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                match (ctrl, code) {
                    (true, KeyCode::Char('o')) => {
                        self.mode = Mode::Insert;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg =
                            Some("Visual mode — arrow keys to select, Esc to exit".into());
                    }
                    (true, KeyCode::Char('a')) => {
                        self.select_all();
                        self.mode = Mode::Visual;
                    }
                    (true, KeyCode::Char('c')) => {
                        self.copy_selection();
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        if self.has_selection() {
                            self.search.query = self.selected_text();
                        } else {
                            self.search.query.clear();
                        }
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            self.jump_to_match(0);
                        }
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Zoom: Ctrl++ / Ctrl+- in terminal emulator".into());
                    }
                    (false, KeyCode::Char(':')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    (true, KeyCode::Char('e')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
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
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                    }
                    (false, KeyCode::Esc) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                            self.status_msg = Some("Cleared extra cursors".into());
                        } else if self.exit_flow(stdout)? {
                            return Err(anyhow::anyhow!("__exit__"));
                        }
                    }
                    (true, KeyCode::Char('b')) => {
                        self.toggle_sidebar();
                    }
                    (true, KeyCode::Char('p')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('t')) => {
                        if self.terminal.is_some() {
                            self.terminal = None;
                            self.focus = crate::editor::Focus::Editor;
                            self.mark_all_dirty();
                        } else {
                            let (tw, th) = crate::editor::helpers::term_size();
                            let term_h = (th as f64 * 0.4) as u16;
                            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                            let shell = if cfg!(windows) { "powershell" } else { "sh" };
                            match crate::editor::terminal::TerminalState::spawn(shell, tw as u16, term_h, &cwd) {
                                Ok(term) => {
                                    self.terminal = Some(term);
                                    self.run_panel_open = false;
                                    self.focus = crate::editor::Focus::Editor;
                                    self.mark_all_dirty();
                                }
                                Err(e) => {
                                    self.status_msg = Some(format!("Terminal error: {}", e));
                                }
                            }
                        }
                    }
                    (true, KeyCode::Char('n')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::PageDown) => {
                        self.next_tab();
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::PageUp) => {
                        self.prev_tab();
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Tab) if !modifiers.contains(KeyModifiers::SHIFT) => {
                        self.next_buffer();
                    }
                    (true, KeyCode::BackTab) | (true, KeyCode::Tab) => {
                        self.prev_buffer();
                    }
                    (true, KeyCode::Char('d')) if !alt => {
                        self.add_cursor_at_next_occurrence();
                    }
                    (true, KeyCode::Char('d')) if alt => {
                        self.split_primary_cursor();
                    }
                    (true, KeyCode::Char('g')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (false, KeyCode::Left) if alt => {
                        self.sv.x = self.sv.x.saturating_sub(3);
                    }
                    (false, KeyCode::Right) if alt => {
                        self.sv.x = (self.sv.x + 3).min(self.max_scroll_x());
                    }
                    (false, KeyCode::Left) if shift && !alt => {
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        }
                    }
                    (false, KeyCode::Right) if shift && !alt => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_char_byte(line, self.cursor.byte);
                        }
                    }
                    (false, KeyCode::Left) if alt && shift => {
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_word_byte(self.current(), self.cursor.byte);
                        } else if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                            self.cursor.byte = self.current().len();
                        }
                    }
                    (false, KeyCode::Right) if alt && shift => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_word_byte(line, self.cursor.byte);
                        } else if self.cursor.y + 1 < self.lines.len() {
                            self.cursor.y += 1;
                            self.cursor.byte = 0;
                        }
                    }
                    (false, KeyCode::Up) if self.cursor.y > 0 => {
                        self.cursor.y -= 1;
                    }
                    (false, KeyCode::Down) if self.cursor.y + 1 < self.lines.len() => {
                        self.cursor.y += 1;
                    }
                    (false, KeyCode::Left) => {
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        } else if self.cursor.y > 0 {
                            let mut prev = self.cursor.y - 1;
                            loop {
                                if !self.lines[prev].is_empty() {
                                    self.cursor.y = prev;
                                    self.cursor.byte = self.lines[prev].len();
                                    break;
                                }
                                if prev == 0 { break; }
                                prev -= 1;
                            }
                        }
                    }
                    (false, KeyCode::Right) => {
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_char_byte(line, self.cursor.byte);
                        } else {
                            let mut next = self.cursor.y + 1;
                            while next < self.lines.len() && self.lines[next].is_empty() {
                                next += 1;
                            }
                            if next < self.lines.len() {
                                self.cursor.y = next;
                                self.cursor.byte = 0;
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
        self.update_hover();
        Ok(())
    }
}
