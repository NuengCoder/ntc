use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use super::{byte_to_col, col_to_byte, gutter_width, Editor};

impl Editor {
    fn screen_to_pos(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let r = row as usize;
        let c = col as usize;
        let eo = self.editor_offset();
        let rows = self.term_h.saturating_sub(3);
        let gw = gutter_width(self.lines.len());
        if r >= rows {
            return None;
        }
        let line_idx = self.scroll + r;
        if line_idx >= self.lines.len() {
            return None;
        }
        if c < eo {
            return None; // clicked in sidebar / separator
        }
        let editor_c = c - eo;
        if editor_c <= gw {
            return Some((line_idx, 0));
        }
        let text_col = editor_c - gw;
        let scroll_col = self.scroll_x + text_col;
        let byte = col_to_byte(&self.lines[line_idx], scroll_col);
        Some((line_idx, byte))
    }

    pub(crate) fn handle_mouse(&mut self, m: MouseEvent) {
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let eo = self.editor_offset();
                let col = m.column as usize;
                let hrow = self.term_h.saturating_sub(3); // horizontal scroll bar row

                // Horizontal scroll bar click
                if m.row as usize == hrow && col < self.term_w {
                    let max_vis = self
                        .lines
                        .iter()
                        .map(|l| byte_to_col(l, l.len()))
                        .max()
                        .unwrap_or(0);
                    let avail = self.term_w.saturating_sub(2);
                    if max_vis > 0 && col > 0 && col <= avail {
                        let gw = gutter_width(self.lines.len());
                        let eo2 = self.editor_offset();
                        let editor_cols = self.term_w.saturating_sub(eo2);
                        let text_cols = editor_cols.saturating_sub(gw + 2);
                        let max = max_vis.saturating_sub(text_cols);
                        if max > 0 {
                            let target = (col as f64 / avail as f64 * max as f64) as usize;
                            self.scroll_x = target.min(max);
                        }
                    }
                    return;
                }

                // Sidebar click
                if self.sidebar.open && col < eo {
                    self.handle_sidebar_click(m.row);
                    return;
                }
                if self.has_multiple_cursors() {
                    self.clear_extra_cursors();
                }
                if let Some((y, byte)) = self.screen_to_pos(m.column, m.row) {
                    if y != self.cursor_y || byte != self.cursor_byte {
                        self.cursor_y = y;
                        self.cursor_byte = byte;
                        self.clear_selection();
                    }
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.selection_anchor.is_none() {
                    if self.has_multiple_cursors() {
                        self.clear_extra_cursors();
                    }
                    self.selection_anchor = Some((self.cursor_y, self.cursor_byte));
                }
                if let Some((y, byte)) = self.screen_to_pos(m.column, m.row) {
                    self.cursor_y = y;
                    self.cursor_byte = byte;
                }
            }
            MouseEventKind::ScrollUp => {
                if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    self.scroll_x = self.scroll_x.saturating_sub(3);
                } else if self.scroll > 0 {
                    self.scroll = self.scroll.saturating_sub(3);
                }
            }
            MouseEventKind::ScrollDown => {
                if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    let max = self.max_scroll_x();
                    self.scroll_x = (self.scroll_x + 3).min(max);
                } else {
                    let rows = self.term_h.saturating_sub(3);
                    if self.scroll + rows < self.lines.len() {
                        self.scroll += 3;
                    }
                }
            }
            MouseEventKind::ScrollLeft => {
                self.scroll_x = self.scroll_x.saturating_sub(3);
            }
            MouseEventKind::ScrollRight => {
                let max = self.max_scroll_x();
                self.scroll_x = (self.scroll_x + 3).min(max);
            }
            _ => {}
        }
    }

    fn handle_sidebar_click(&mut self, row: u16) {
        let r = row as usize;
        let rows = self.term_h.saturating_sub(3);
        if r >= rows {
            return;
        }
        let idx = self.sidebar.scroll + r;
        if idx >= self.sidebar.nodes.len() {
            return;
        }
        self.sidebar.selected = idx;
        let node = self.sidebar.nodes[idx].clone();
        if node.is_dir {
            self.sidebar.toggle_expand(idx);
        } else {
            self.open_file(&node.path);
        }
        self.mark_all_dirty();
    }
}
