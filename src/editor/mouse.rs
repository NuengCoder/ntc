use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use super::{byte_to_col, col_to_byte, gutter_width, Editor, SIDEBAR_WIDTH};

impl Editor {
    fn screen_to_pos(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let r = if row >= 1 { (row - 1) as usize } else { return None; };
        let c = col as usize;
        let eo = self.editor_offset();
        let rows = self.term_h.saturating_sub(4);
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

    fn scroll_from_vclick(&self, row: u16) -> usize {
        let rows = self.term_h.saturating_sub(4);
        let r = if row >= 1 { (row - 1) as usize } else { return self.scroll; };
        let total = self.lines.len();
        if total <= rows || r >= rows {
            return self.scroll;
        }
        let max_scroll = total - rows;
        r * max_scroll / rows
    }

    fn scroll_x_from_hclick(&self, col: u16) -> usize {
        let col = col as usize;
        let max_vis = self
            .lines
            .iter()
            .map(|l| byte_to_col(l, l.len()))
            .max()
            .unwrap_or(0);
        let avail = self.term_w.saturating_sub(2);
        if max_vis == 0 || col == 0 || col > avail {
            return self.scroll_x;
        }
        let gw = gutter_width(self.lines.len());
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let max = max_vis.saturating_sub(text_cols);
        if max == 0 {
            return self.scroll_x;
        }
        let target = (col as f64 / avail as f64 * max as f64) as usize;
        target.min(max)
    }

    pub(crate) fn handle_mouse(&mut self, m: MouseEvent) {
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let eo = self.editor_offset();
                let col = m.column as usize;
                let content_rows = self.term_h.saturating_sub(4);
                let hrow = 1 + content_rows; // horizontal scroll bar row
                let tab_bar_h = 1;
                let sb_col = self.term_w.saturating_sub(1);

                // Tab bar click — switch or close tab
                if (m.row as usize) < tab_bar_h {
                    let mut x = 0usize;
                    for (i, tab) in self.tabs.iter().enumerate() {
                        if x >= self.term_w {
                            break;
                        }
                        let name = if self.is_scratch_path(&tab.path) {
                            "[scratch]".to_string()
                        } else {
                            tab.path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "untitled".to_string())
                        };
                        let mark = if tab.modified { " ●" } else { "" };
                        let label = format!(" {} {} ", name, mark);
                        let label_chars = label.chars().count();
                        let close_btn_w = 1usize;
                        let full_w = label_chars + close_btn_w;

                        let remaining = self.term_w.saturating_sub(x);
                        if full_w > remaining {
                            break;
                        }

                        // Name area: from x to label end
                        let name_end = x + label_chars;
                        if col < name_end {
                            self.switch_tab(i);
                            self.mark_all_dirty();
                            return;
                        }
                        // Close button area: the single "✕" character
                        if col >= name_end && col < x + full_w {
                            if self.close_tab(i) {
                                self.pending_quit = true;
                            }
                            self.mark_all_dirty();
                            return;
                        }
                        x += full_w;
                    }
                    return;
                }

                // Vertical scroll bar click
                if col == sb_col && (m.row as usize) < 1 + content_rows && (m.row as usize) >= tab_bar_h {
                    self.dragging_vscroll = true;
                    let target = self.scroll_from_vclick(m.row);
                    self.scroll = target.min(self.lines.len().saturating_sub(content_rows));
                    self.cursor_y = self.scroll;
                    self.cursor_byte = 0;
                    self.clear_selection();
                    self.mark_all_dirty();
                    return;
                }

                // Horizontal scroll bar click
                if m.row as usize == hrow && col < self.term_w {
                    let new_x = self.scroll_x_from_hclick(m.column);
                    if new_x != self.scroll_x {
                        self.scroll_x = new_x;
                        self.dragging_hscroll = true;
                        self.mark_all_dirty();
                    }
                    return;
                }

                // Sidebar vertical scroll bar click
                if self.sidebar.open && col == SIDEBAR_WIDTH - 1 && (m.row as usize) >= tab_bar_h && (m.row as usize) < 1 + content_rows {
                    let r = (m.row as usize).saturating_sub(tab_bar_h);
                    if r < content_rows {
                        let total = self.sidebar.nodes.len();
                        if total > content_rows {
                            let max_scroll = total - content_rows;
                            self.sidebar.scroll = r * max_scroll / content_rows;
                            self.mark_all_dirty();
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
                if self.dragging_vscroll {
                    let content_rows = self.term_h.saturating_sub(4);
                    let tab_bar_h = 1;
                    let r = if (m.row as usize) >= tab_bar_h { (m.row as usize) - tab_bar_h } else { return; };
                    if r < content_rows {
                        let total = self.lines.len();
                        let max_scroll = total.saturating_sub(content_rows);
                        let target = r * max_scroll / content_rows;
                        self.scroll = target.min(max_scroll);
                        self.cursor_y = self.scroll;
                        self.cursor_byte = 0;
                        self.mark_all_dirty();
                    }
                    return;
                }
                if self.dragging_hscroll {
                    let new_x = self.scroll_x_from_hclick(m.column);
                    if new_x != self.scroll_x {
                        self.scroll_x = new_x;
                        self.mark_all_dirty();
                    }
                    return;
                }
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
            MouseEventKind::Down(MouseButton::Middle) => {
                let col = m.column as usize;
                let content_rows = self.term_h.saturating_sub(4);
                let tab_bar_h = 1;
                // Sidebar vertical scroll bar middle-click → jump
                if self.sidebar.open && col == SIDEBAR_WIDTH - 1
                    && (m.row as usize) >= tab_bar_h && (m.row as usize) < 1 + content_rows
                {
                    let r = (m.row as usize).saturating_sub(tab_bar_h);
                    if r < content_rows {
                        let total = self.sidebar.nodes.len();
                        if total > content_rows {
                            let max_scroll = total - content_rows;
                            self.sidebar.scroll = r * max_scroll / content_rows;
                            self.mark_all_dirty();
                        }
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.dragging_vscroll = false;
                self.dragging_hscroll = false;
            }
            MouseEventKind::ScrollUp => {
                let content_rows = self.term_h.saturating_sub(4);
                let col = m.column as usize;
                let eo = self.editor_offset();
                // Sidebar scroll
                if self.sidebar.open && col < eo && (m.row as usize) >= 1 && (m.row as usize) < 1 + content_rows {
                    if self.sidebar.scroll > 0 {
                        self.sidebar.scroll = self.sidebar.scroll.saturating_sub(3);
                        self.mark_all_dirty();
                    }
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    self.scroll_x = self.scroll_x.saturating_sub(3);
                } else if self.scroll > 0 {
                    self.scroll = self.scroll.saturating_sub(3);
                }
            }
            MouseEventKind::ScrollDown => {
                let content_rows = self.term_h.saturating_sub(4);
                let col = m.column as usize;
                let eo = self.editor_offset();
                // Sidebar scroll
                if self.sidebar.open && col < eo && (m.row as usize) >= 1 && (m.row as usize) < 1 + content_rows {
                    let total = self.sidebar.nodes.len();
                    if self.sidebar.scroll + content_rows < total {
                        self.sidebar.scroll += 3;
                        self.mark_all_dirty();
                    }
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    let max = self.max_scroll_x();
                    self.scroll_x = (self.scroll_x + 3).min(max);
                } else {
                    if self.scroll + content_rows < self.lines.len() {
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
        let content_top = 1;
        let r = if (row as usize) >= content_top { (row as usize) - content_top } else { return; };
        let rows = self.term_h.saturating_sub(4);
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
