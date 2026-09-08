use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode, ALL_ENCODINGS};
use crate::t;

impl Editor {
    /// Open the encoding picker (Ctrl+U in normal mode), preselecting the
    /// active buffer's current encoding.
    pub(crate) fn open_encoding_picker(&mut self) {
        self.encoding_sel = ALL_ENCODINGS
            .iter()
            .position(|&e| e == self.encoding)
            .unwrap_or(0);
        self.mode = Mode::Encoding;
        self.status_msg = None;
        self.mark_all_dirty();
    }

    pub(crate) fn handle_encoding_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
                    (true, KeyCode::Char('g')) => {
                        self.mode = Mode::Normal;
                        self.mark_all_dirty();
                    }
                    _ => match code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.mode = Mode::Normal;
                            self.mark_all_dirty();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if self.encoding_sel == 0 {
                                self.encoding_sel = ALL_ENCODINGS.len() - 1;
                            } else {
                                self.encoding_sel -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.encoding_sel = (self.encoding_sel + 1) % ALL_ENCODINGS.len();
                        }
                        KeyCode::Enter => {
                            self.confirm_encoding();
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

    fn confirm_encoding(&mut self) {
        let selected = ALL_ENCODINGS[self.encoding_sel];
        let changed = selected != self.encoding;
        self.encoding = selected;
        self.mode = Mode::Normal;
        if !changed {
            self.mark_all_dirty();
            return;
        }
        if !self.modified {
            match self.reload_with_encoding() {
                Ok(()) => {
                    self.status_msg = Some(format!(
                        "{}: {}",
                        t!("editor.encoding_changed"),
                        selected.label()
                    ));
                }
                Err(e) => {
                    self.status_msg =
                        Some(format!("{} {}", t!("editor.encoding_reload_error"), e));
                }
            }
        } else {
            self.status_msg = Some(format!(
                "{}: {} ({})",
                t!("editor.encoding_changed"),
                selected.label(),
                t!("editor.encoding_applied_on_save")
            ));
        }
        self.mark_all_dirty();
    }
}