use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::editor::{prev_char_byte, next_char_byte, Editor, Mode};

impl Editor {
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
                    (true, KeyCode::Char('a')) => {
                        self.select_all();
                    }
                    (true, KeyCode::Char('o')) => {
                        self.mode = Mode::Insert;
                        self.clear_selection();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('e')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    (true, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('b')) => {
                        self.toggle_sidebar();
                    }
                    (true, KeyCode::Char('f')) => {
                        let sel = self.selected_text();
                        self.clear_selection();
                        self.mode = Mode::Search;
                        if !sel.is_empty() {
                            self.search.query = sel;
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
                    (true, KeyCode::Char('n')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('g')) => {
                        self.clear_selection();
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (false, KeyCode::Left) => {
                        self.start_or_extend_selection();
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
                        self.start_or_extend_selection();
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
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Zoom: Ctrl++ / Ctrl+- in terminal emulator".into());
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
}
