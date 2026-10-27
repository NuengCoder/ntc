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
                let alt = modifiers.contains(KeyModifiers::ALT);
                match (ctrl, alt, code) {
                    (true, _, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, _, KeyCode::Char('n')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, _, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, _, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mark_all_dirty();
                        self.mode = Mode::Normal;
                    }
                    // Alt+Up/Down keep scrolling vertically; Alt+Left/Right
                    // pan sideways by 3 columns, exactly like Normal and
                    // Insert do.
                    (false, true, KeyCode::Up) => {
                        self.sv.help = self.sv.help.saturating_sub(1);
                    }
                    (false, true, KeyCode::Down) => {
                        self.sv.help += 1;
                    }
                    (false, true, KeyCode::Left) => {
                        self.sv.help_x = self.sv.help_x.saturating_sub(3);
                    }
                    (false, true, KeyCode::Right) => {
                        self.sv.help_x += 3;
                    }
                    _ => match code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mark_all_dirty();
                            self.mode = Mode::Normal;
                            self.status_msg = None;
                        }
                        KeyCode::Char(':') | KeyCode::Char(';') => {
                            self.mode = Mode::Command;
                            self.cmd_buf.clear();
                            self.mark_all_dirty();
                        }
                        KeyCode::Char('b') => {
                            self.toggle_sidebar();
                        }
                        // Motion mirrors the editor: k/w/Up scroll up,
                        // j/s/Down scroll down, h/a/Left and l/d/Right pan
                        // the text one column sideways.
                        KeyCode::Up
                        | KeyCode::Char('k')
                        | KeyCode::Char('K')
                        | KeyCode::Char('w')
                        | KeyCode::Char('W') => {
                            self.sv.help = self.sv.help.saturating_sub(1);
                        }
                        KeyCode::Down
                        | KeyCode::Char('j')
                        | KeyCode::Char('J')
                        | KeyCode::Char('s')
                        | KeyCode::Char('S') => {
                            self.sv.help += 1;
                        }
                        KeyCode::Left
                        | KeyCode::Char('h')
                        | KeyCode::Char('H')
                        | KeyCode::Char('a')
                        | KeyCode::Char('A') => {
                            self.sv.help_x = self.sv.help_x.saturating_sub(1);
                        }
                        KeyCode::Right
                        | KeyCode::Char('l')
                        | KeyCode::Char('L')
                        | KeyCode::Char('d')
                        | KeyCode::Char('D') => {
                            self.sv.help_x += 1;
                        }
                        _ => {}
                    },
                }
            }
            Event::Mouse(m) => match m.kind {
                crossterm::event::MouseEventKind::ScrollUp => {
                    self.sv.help = self.sv.help.saturating_sub(3);
                }
                crossterm::event::MouseEventKind::ScrollDown => {
                    self.sv.help += 3;
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
}
