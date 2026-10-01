use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};
use crate::t;

impl Editor {
    pub(crate) fn handle_search_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
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
                            self.status_msg = Some(t!(
                                "editor.mode_search_count",
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
                                self.search.match_idx = self
                                    .search
                                    .match_idx
                                    .min(self.search.matches.len().saturating_sub(1));
                                self.jump_to_match(self.search.match_idx);
                            }
                        }
                        KeyCode::Char(c) => {
                            self.search.query.push(c);
                            self.rebuild_search_matches();
                            if !self.search.matches.is_empty() {
                                let pos = (self.cursor.y, self.cursor.byte);
                                let idx = self
                                    .search
                                    .matches
                                    .iter()
                                    .position(|&(ly, sb, _)| (ly, sb) >= pos)
                                    .unwrap_or(0);
                                self.search.match_idx = idx;
                                self.jump_to_match(idx);
                            }
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
