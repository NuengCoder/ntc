use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};

impl Editor {
    pub(crate) fn handle_help_event(&mut self, ev: Event) -> Result<()> {
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
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
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
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mark_all_dirty();
                        self.mode = Mode::Normal;
                    }
                    _ => match code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mark_all_dirty();
                            self.mode = Mode::Normal;
                            self.status_msg = None;
                        }
                        KeyCode::Char('b') => {
                            self.toggle_sidebar();
                        }
                        KeyCode::Up => {
                            self.sv.help = self.sv.help.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            self.sv.help += 1;
                        }
                        _ => {}
                    },
                }
            },
            Event::Mouse(m) => {
                match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp => {
                        self.sv.help = self.sv.help.saturating_sub(3);
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        self.sv.help += 3;
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
}
