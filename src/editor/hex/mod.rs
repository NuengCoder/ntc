use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use super::{Editor, HexViewer, Mode};
use crate::t;

const BYTES_PER_ROW: usize = 16;

impl Editor {
    pub(crate) fn enter_hex_mode(&mut self) {
        let path = self.path.clone();
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(e) => {
                self.status_msg = Some(format!("{} {}", t!("editor.hex_cannot_read"), e));
                return;
            }
        };
        self.hex = Some(HexViewer {
            data,
            path: self.path.clone(),
            cursor: 0,
            offset: 0,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            nibble: false,
            pending_byte: 0,
        });
        self.mode = Mode::Hex;
        self.mark_all_dirty();
        self.status_msg = Some(t!("editor.hex_mode_hint"));
    }

    fn exit_hex_mode(&mut self) {
        self.hex = None;
        self.mode = Mode::Normal;
        self.mark_all_dirty();
    }

    pub(crate) fn handle_hex_event(
        &mut self,
        ev: Event,
        _stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        let hex = match self.hex.as_mut() {
            Some(h) => h,
            None => return Ok(()),
        };

        match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    return Ok(());
                }
                let ctrl = ke.modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, ke.code) {
                    (false, KeyCode::Esc) => {
                        if hex.modified {
                            self.status_msg = Some(t!("editor.hex_unsaved_warning"));
                        } else {
                            self.exit_hex_mode();
                        }
                    }
                    (false, KeyCode::Char(':')) => {
                        self.cmd_buf.clear();
                        self.prev_mode = Mode::Hex;
                        self.mode = Mode::Command;
                    }
                    (false, KeyCode::Left) if hex.cursor > 0 => {
                        hex.cursor -= 1;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::Right) if hex.cursor + 1 < hex.data.len() => {
                        hex.cursor += 1;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::Up) => {
                        let n = BYTES_PER_ROW.min(hex.cursor);
                        hex.cursor -= n;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::Down) => {
                        let n = BYTES_PER_ROW.min(hex.data.len().saturating_sub(hex.cursor + 1));
                        hex.cursor += n;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::PageUp) => {
                        let rows = self.term_h.saturating_sub(4).max(1);
                        let n = (rows * BYTES_PER_ROW).min(hex.cursor);
                        hex.cursor -= n;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(4).max(1);
                        let n = (rows * BYTES_PER_ROW)
                            .min(hex.data.len().saturating_sub(hex.cursor + 1));
                        hex.cursor += n;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::Home) => {
                        hex.cursor = 0;
                        hex.offset = 0;
                        hex.nibble = false;
                        hex.pending_byte = 0;
                    }
                    (false, KeyCode::End) => {
                        hex.cursor = hex.data.len().saturating_sub(1);
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        hex_ensure_visible(hex, self.term_h);
                    }
                    (false, KeyCode::Char('u')) => {
                        hex_undo(hex);
                    }
                    (true, KeyCode::Char('r')) => {
                        hex_redo(hex);
                    }
                    (false, KeyCode::Char(c)) if c.is_ascii_hexdigit() => {
                        let val = c.to_digit(16).unwrap() as u8;
                        if !hex.nibble {
                            hex.pending_byte = val << 4;
                            hex.nibble = true;
                        } else {
                            let new_byte = hex.pending_byte | val;
                            let old = hex.data[hex.cursor];
                            if old != new_byte {
                                hex.undo_stack.push(vec![(hex.cursor, old)]);
                                hex.redo_stack.clear();
                                hex.data[hex.cursor] = new_byte;
                                hex.modified = true;
                                if hex.undo_stack.len() > 100 {
                                    hex.undo_stack.remove(0);
                                }
                            }
                            hex.nibble = false;
                            hex.pending_byte = 0;
                            // Advance cursor
                            if hex.cursor + 1 < hex.data.len() {
                                hex.cursor += 1;
                                hex_ensure_visible(hex, self.term_h);
                            }
                        }
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::Backspace) if hex.nibble => {
                        hex.nibble = false;
                        hex.pending_byte = 0;
                        self.mark_all_dirty();
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
                hex_ensure_visible(hex, self.term_h);
                self.mark_all_dirty();
            }
            _ => {}
        }

        Ok(())
    }
}

fn hex_ensure_visible(hex: &mut HexViewer, term_h: usize) {
    let rows = term_h.saturating_sub(4).max(1);
    let cur_row = hex.cursor / BYTES_PER_ROW;
    let off_row = hex.offset / BYTES_PER_ROW;
    if cur_row < off_row {
        hex.offset = cur_row * BYTES_PER_ROW;
    } else if cur_row >= off_row + rows {
        hex.offset = (cur_row.saturating_add(1).saturating_sub(rows)) * BYTES_PER_ROW;
    }
}

fn hex_undo(hex: &mut HexViewer) {
    if let Some(ops) = hex.undo_stack.pop() {
        hex.redo_stack.push(
            ops.iter()
                .map(|&(off, old)| {
                    let current = hex.data[off];
                    hex.data[off] = old;
                    (off, current)
                })
                .collect(),
        );
        hex.modified = !hex.undo_stack.is_empty();
        hex.nibble = false;
        hex.pending_byte = 0;
    }
}

fn hex_redo(hex: &mut HexViewer) {
    if let Some(ops) = hex.redo_stack.pop() {
        hex.undo_stack.push(
            ops.iter()
                .map(|&(off, old)| {
                    let current = hex.data[off];
                    hex.data[off] = old;
                    (off, current)
                })
                .collect(),
        );
        hex.modified = true;
        hex.nibble = false;
        hex.pending_byte = 0;
    }
}
