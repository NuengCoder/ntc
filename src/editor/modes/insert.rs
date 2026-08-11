use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Color;

use crate::t;

use crate::editor::{
    next_char_byte, next_word_byte, prev_char_byte, prev_word_byte, sel_range, set_status_bar,
    Editor, Mode,
};

impl Editor {
    pub(crate) fn handle_insert_event(
        &mut self,
        ev: Event,
        stdout: &mut std::io::Stdout,
    ) -> Result<()> {
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
                    (true, _, _, KeyCode::Char('s')) => match self.save() {
                        Ok(()) => {
                            let _ = set_status_bar(
                                stdout,
                                Color::Green,
                                Color::White,
                                &t!("editor.saved_full"),
                            );
                        }
                        Err(e) => {
                            let _ = set_status_bar(
                                stdout,
                                Color::Red,
                                Color::White,
                                &t!("editor.cmd_save_failed", e),
                            );
                        }
                    },
                    (true, true, _, KeyCode::Char('q')) => {
                        self.prev_mode = Mode::Insert;
                        self.mode = Mode::Query;
                        self.query_buf.clear();
                        self.mark_all_dirty();
                    }
                    (true, _, _, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    (true, _, _, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (false, false, false, KeyCode::Esc) => {
                        self.clear_selection();
                        self.clear_extra_cursors();
                        self.mode = Mode::Normal;
                        self.status_msg = None;
                        self.hover_sig_active = false;
                        self.hover_text = None;
                    }
                    (true, _, _, KeyCode::Char('t')) => {
                        if self.terminal.is_some() {
                            self.terminal = None;
                            self.focus = crate::editor::Focus::Editor;
                            self.mark_all_dirty();
                        } else {
                            let (tw, th) = crate::editor::helpers::term_size();
                            let term_h = (th as f64 * 0.4) as u16;
                            let cwd = std::env::current_dir()
                                .unwrap_or_else(|_| std::path::PathBuf::from("."));
                            let shell = if cfg!(windows) { "powershell" } else { "sh" };
                            match crate::editor::terminal::TerminalState::spawn(
                                shell, tw as u16, term_h, &cwd,
                            ) {
                                Ok(term) => {
                                    self.terminal = Some(term);
                                    self.run_panel_open = false;
                                    self.focus = crate::editor::Focus::Editor;
                                    self.mark_all_dirty();
                                }
                                Err(e) => {
                                    self.status_msg = Some(t!("editor.mode_terminal_err", e));
                                }
                            }
                        }
                    }
                    (true, _, _, KeyCode::Char('e')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }

                    (true, _, _, KeyCode::Char('z')) => self.undo(),
                    (true, _, _, KeyCode::Char('r')) => self.redo(),

                    (true, _, _, KeyCode::Char('a')) => {
                        self.clear_extra_cursors();
                        self.select_all();
                    }
                    (true, _, _, KeyCode::Char('c')) => self.copy_selection(),
                    (true, _, _, KeyCode::Char('x')) => self.cut_selection(),
                    (true, _, _, KeyCode::Char('v')) => {}
                    (true, _, _, KeyCode::Char('p')) => self.paste(),

                    (true, _, _, KeyCode::Char('=')) | (true, _, _, KeyCode::Char('-')) => {
                        self.status_msg = Some(t!("editor.mode_zoom_hint"));
                    }

                    (true, false, _, KeyCode::Char('d')) => {
                        self.add_cursor_at_next_occurrence();
                        if self.has_multiple_cursors() {
                            self.mode = Mode::InsertMulti;
                        }
                    }
                    (true, true, _, KeyCode::Char('d')) => {
                        self.split_primary_cursor();
                        if self.has_multiple_cursors() {
                            self.mode = Mode::InsertMulti;
                        }
                    }
                    (true, _, _, KeyCode::Char('g')) => {
                        self.prev_mode = Mode::Insert;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, _, _, KeyCode::Char('k')) => self.kill_line(),
                    (true, _, _, KeyCode::Char('l')) => self.select_line(),
                    (true, _, _, KeyCode::Char('m')) => {
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
                    (false, true, _, KeyCode::Up) => self.move_line_up(),
                    (false, true, _, KeyCode::Down) => self.move_line_down(),

                    (true, _, _, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, _, _, KeyCode::Char('n')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (false, false, false, KeyCode::F(3)) if !self.search.query.is_empty() => {
                        self.search_next();
                    }
                    (false, false, true, KeyCode::F(3)) if !self.search.query.is_empty() => {
                        self.search_prev();
                    }

                    (true, _, _, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg = Some(t!("editor.mode_visual_start"));
                    }
                    (true, _, _, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, _, _, KeyCode::Char('y')) => self.yank_line(),

                    (true, false, false, KeyCode::Char('b')) => self.toggle_sidebar(),
                    (true, false, false, KeyCode::Tab) => self.next_buffer(),
                    (true, false, true, KeyCode::Tab) => self.prev_buffer(),

                    (false, true, false, KeyCode::Left) => {
                        self.sv.x = self.sv.x.saturating_sub(3);
                    }
                    (false, true, false, KeyCode::Right) => {
                        self.sv.x = (self.sv.x + 3).min(self.max_scroll_x());
                    }

                    (false, false, false, KeyCode::Up) if self.cursor.y > 0 => {
                        self.cursor.y -= 1;
                    }
                    (false, false, false, KeyCode::Down)
                        if self.cursor.y + 1 < self.lines.len() =>
                    {
                        self.cursor.y += 1;
                    }
                    (false, false, false, KeyCode::Left) => {
                        if let Some((ay, ab)) = self.cursor.selection_anchor {
                            if (ay, ab) != (self.cursor.y, self.cursor.byte) {
                                let ((sy, sb), _) =
                                    sel_range(self.cursor.y, self.cursor.byte, ay, ab);
                                self.cursor.y = sy;
                                self.cursor.byte = sb;
                            }
                            self.clear_selection();
                        } else if self.cursor.byte > 0 {
                            self.cursor.byte = prev_char_byte(self.current(), self.cursor.byte);
                        } else if self.cursor.y > 0 {
                            let mut prev = self.cursor.y - 1;
                            loop {
                                if !self.lines[prev].is_empty() {
                                    self.cursor.y = prev;
                                    self.cursor.byte = self.lines[prev].len();
                                    break;
                                }
                                if prev == 0 {
                                    break;
                                }
                                prev -= 1;
                            }
                        }
                    }
                    (false, false, false, KeyCode::Right) => {
                        if let Some((ay, ab)) = self.cursor.selection_anchor {
                            if (ay, ab) != (self.cursor.y, self.cursor.byte) {
                                let (_, (ey, eb)) =
                                    sel_range(self.cursor.y, self.cursor.byte, ay, ab);
                                self.cursor.y = ey;
                                self.cursor.byte = eb;
                            }
                            self.clear_selection();
                        } else {
                            let line = self.current();
                            let byte_len = line.len();
                            if self.cursor.byte < byte_len {
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
                    }

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
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_char_byte(line, self.cursor.byte);
                        }
                    }
                    (false, true, true, KeyCode::Left) => {
                        self.start_or_extend_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_word_byte(self.current(), self.cursor.byte);
                        } else if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                            self.cursor.byte = self.current().len();
                        }
                    }
                    (false, true, true, KeyCode::Right) => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_word_byte(line, self.cursor.byte);
                        } else if self.cursor.y + 1 < self.lines.len() {
                            self.cursor.y += 1;
                            self.cursor.byte = 0;
                        }
                    }

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

                    (false, false, false, KeyCode::Tab) => self.indent_lines(),
                    (false, false, _, KeyCode::BackTab) => self.dedent_lines(),

                    (false, false, false, KeyCode::Enter) | (true, _, _, KeyCode::Enter) => {
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

                    (false, false, _, KeyCode::Backspace) => {
                        if self.has_multiple_cursors() {
                            self.multi_backspace();
                        } else {
                            self.backspace();
                        }
                        self.trigger_completion();
                        self.trigger_function_signature();
                    }
                    (false, false, _, KeyCode::Delete) => {
                        if self.has_multiple_cursors() {
                            self.multi_delete_forward();
                        } else {
                            self.delete_forward();
                        }
                        self.trigger_completion();
                        self.trigger_function_signature();
                    }

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

        self.trigger_function_signature();
        Ok(())
    }

    pub(crate) fn handle_insert_multi_event(
        &mut self,
        ev: Event,
        stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        if let Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = &ev
        {
            if !modifiers.contains(KeyModifiers::CONTROL) {
                self.clear_extra_cursors();
                self.mode = Mode::Insert;
                self.status_msg = Some(t!("editor.mode_exited_multi"));
                return Ok(());
            }
        }
        self.handle_insert_event(ev, stdout)
    }
}
