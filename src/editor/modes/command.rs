use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};

impl Editor {
    pub(crate) fn handle_command_event(
        &mut self,
        ev: Event,
        stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
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
                    (true, KeyCode::Char('n')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    _ => match code {
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
}
