use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use super::{byte_to_col, col_to_byte, gutter_width, Editor, Focus, MIN_SIDEBAR_WIDTH};
use crate::t;

impl Editor {
    fn screen_to_pos(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let r = if row >= 1 {
            (row - 1) as usize
        } else {
            return None;
        };
        let c = col as usize;
        let eo = self.editor_offset();
        let rows = self.term_h.saturating_sub(4);
        let gw = gutter_width(self.lines.len());
        if r >= rows {
            return None;
        }
        let line_idx = self.sv.offset + r;
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
        let scroll_col = self.sv.x + text_col;
        let byte = col_to_byte(&self.lines[line_idx], scroll_col);
        Some((line_idx, byte))
    }

    fn scroll_from_vclick(&self, row: u16) -> usize {
        let rows = self.term_h.saturating_sub(4);
        let r = if row >= 1 {
            (row - 1) as usize
        } else {
            return self.sv.offset;
        };
        let total = self.lines.len();
        if total <= rows || r >= rows {
            return self.sv.offset;
        }
        let max_scroll = total - rows;
        r * max_scroll / rows
    }

    fn scroll_x_from_hclick(&self, col: u16) -> usize {
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let rel_col = (col as usize).saturating_sub(eo);
        let max_vis = self
            .lines
            .iter()
            .map(|l| byte_to_col(l, l.len()))
            .max()
            .unwrap_or(0);
        let avail = editor_cols.saturating_sub(2);
        if max_vis == 0 || rel_col == 0 || rel_col > avail {
            return self.sv.x;
        }
        let gw = gutter_width(self.lines.len());
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let max = max_vis.saturating_sub(text_cols);
        if max == 0 {
            return self.sv.x;
        }
        let target = (rel_col as f64 / avail as f64 * max as f64) as usize;
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

                // Auto-switch focus based on click location
                let split = self.run_split_row();
                if (m.row as usize) < split {
                    self.focus = Focus::Editor;
                } else if (m.row as usize) > split
                    && (self.run_panel_open || self.terminal.is_some())
                {
                    if self.run_panel_open {
                        self.focus = Focus::Run;
                    } else {
                        self.focus = Focus::Terminal;
                    }
                }

                // Tab bar click — switch, close tab, or close-all
                if (m.row as usize) < tab_bar_h && col >= eo {
                    // Close-all X button (far right of tab bar)
                    let close_all_col = self.term_w.saturating_sub(2);
                    if col >= close_all_col {
                        while self.tabs.len() > 1 {
                            self.close_tab(self.tabs.len() - 1);
                        }
                        self.switch_tab(0);
                        self.mark_all_dirty();
                        self.status_msg = Some(t!("editor.mouse_closed_all"));
                        return;
                    }
                    let mut x = eo;
                    for (i, tab) in self.tabs.iter().enumerate() {
                        let tab_end = x + (self.term_w - eo);
                        if x >= tab_end {
                            break;
                        }
                        let name = if self.is_scratch_path(&tab.path) {
                            t!("editor.scratch_label")
                        } else {
                            tab.path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| t!("editor.untitled_label"))
                        };
                        let mark = if tab.modified { " ●" } else { "" };
                        let label = format!(" {} {} ", name, mark);
                        let label_chars = label.chars().count();
                        let close_btn_w = 1usize;
                        let full_w = label_chars + close_btn_w;

                        let remaining = tab_end.saturating_sub(x);
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
                if col == sb_col
                    && (m.row as usize) < 1 + content_rows
                    && (m.row as usize) >= tab_bar_h
                {
                    self.sv.dragging_vscroll = true;
                    let target = self.scroll_from_vclick(m.row);
                    self.sv.offset = target.min(self.lines.len().saturating_sub(1));
                    self.cursor.y = self.sv.offset;
                    self.cursor.byte = 0;
                    self.clear_selection();
                    self.mark_all_dirty();
                    return;
                }

                // Horizontal scroll bar click
                if m.row as usize == hrow && col >= eo && col < self.term_w {
                    let new_x = self.scroll_x_from_hclick(m.column);
                    if new_x != self.sv.x {
                        self.sv.x = new_x;
                        self.sv.dragging_hscroll = true;
                        self.mark_all_dirty();
                    }
                    return;
                }

                // Sidebar vertical scroll bar click
                if self.sidebar.open
                    && col == self.sidebar.width - 1
                    && (m.row as usize) < self.term_h
                {
                    let r = m.row as usize;
                    let total = self.sidebar.nodes.len();
                    if total > 0 {
                        let max_scroll = total.saturating_sub(1);
                        self.sidebar.scroll = r * max_scroll / self.term_h.saturating_sub(1);
                        self.mark_all_dirty();
                    }
                    return;
                }

                // Sidebar border drag — click on the separator column
                if self.sidebar.open && col == self.sidebar.width && (m.row as usize) < self.term_h
                {
                    self.sidebar.dragging_border = true;
                    return;
                }

                // Run/terminal panel border drag — click on the separator row
                if (self.run_panel_open || self.terminal.is_some())
                    && (m.row as usize) == split
                    && col >= eo
                {
                    self.dragging_run_border = true;
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
                    if y != self.cursor.y || byte != self.cursor.byte {
                        self.cursor.y = y;
                        self.cursor.byte = byte;
                        self.clear_selection();
                    }
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.sv.dragging_vscroll {
                    let content_rows = self.term_h.saturating_sub(4);
                    let tab_bar_h = 1;
                    let r = if (m.row as usize) >= tab_bar_h {
                        (m.row as usize) - tab_bar_h
                    } else {
                        return;
                    };
                    if r < content_rows {
                        let total = self.lines.len();
                        let max_scroll = total.saturating_sub(1);
                        let target = r * max_scroll / content_rows;
                        self.sv.offset = target.min(max_scroll);
                        self.cursor.y = self.sv.offset;
                        self.cursor.byte = 0;
                        self.mark_all_dirty();
                    }
                    return;
                }
                if self.sv.dragging_hscroll {
                    let new_x = self.scroll_x_from_hclick(m.column);
                    if new_x != self.sv.x {
                        self.sv.x = new_x;
                        self.mark_all_dirty();
                    }
                    return;
                }
                if self.sidebar.dragging_border {
                    let col = m.column as usize;
                    let new_width = col.clamp(MIN_SIDEBAR_WIDTH, self.term_w.saturating_sub(10));
                    if new_width != self.sidebar.width {
                        self.sidebar.width = new_width;
                        self.mark_all_dirty();
                    }
                    return;
                }
                if self.dragging_run_border {
                    let new_split = (m.row as usize).clamp(5, self.term_h.saturating_sub(7));
                    if new_split != self.stored_run_split {
                        self.stored_run_split = new_split;
                        self.mark_all_dirty();
                    }
                    return;
                }
                if self.cursor.selection_anchor.is_none() {
                    if self.has_multiple_cursors() {
                        self.clear_extra_cursors();
                    }
                    self.cursor.selection_anchor = Some((self.cursor.y, self.cursor.byte));
                }
                if let Some((y, byte)) = self.screen_to_pos(m.column, m.row) {
                    self.cursor.y = y;
                    self.cursor.byte = byte;
                }
            }
            MouseEventKind::Down(MouseButton::Middle) => {
                let col = m.column as usize;
                // Sidebar vertical scroll bar middle-click → jump
                if self.sidebar.open
                    && col == self.sidebar.width - 1
                    && (m.row as usize) < self.term_h
                {
                    let r = m.row as usize;
                    let total = self.sidebar.nodes.len();
                    if total > 0 {
                        let max_scroll = total.saturating_sub(1);
                        self.sidebar.scroll = r * max_scroll / self.term_h.saturating_sub(1);
                        self.mark_all_dirty();
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.sv.dragging_vscroll = false;
                self.sv.dragging_hscroll = false;
                self.sidebar.dragging_border = false;
                self.dragging_run_border = false;
            }
            MouseEventKind::ScrollUp => {
                let col = m.column as usize;
                let eo = self.editor_offset();
                // Sidebar scroll (sidebar spans full height, including bottom bar rows)
                if self.sidebar.open && col < eo {
                    if self.sidebar.scroll > 0 {
                        self.sidebar.scroll = self.sidebar.scroll.saturating_sub(3);
                        self.mark_all_dirty();
                    }
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    self.sv.x = self.sv.x.saturating_sub(3);
                } else if self.sv.offset > 0 {
                    self.sv.offset = self.sv.offset.saturating_sub(3);
                    self.mark_all_dirty();
                }
            }
            MouseEventKind::ScrollDown => {
                let col = m.column as usize;
                let eo = self.editor_offset();
                // Sidebar scroll (sidebar spans full height)
                if self.sidebar.open && col < eo {
                    let total = self.sidebar.nodes.len();
                    if self.sidebar.scroll + 1 < total {
                        self.sidebar.scroll =
                            (self.sidebar.scroll + 3).min(total.saturating_sub(1));
                        self.mark_all_dirty();
                    }
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    let max = self.max_scroll_x();
                    self.sv.x = (self.sv.x + 3).min(max);
                } else {
                    if self.sv.offset + 1 < self.lines.len() {
                        self.sv.offset =
                            (self.sv.offset + 3).min(self.lines.len().saturating_sub(1));
                        self.mark_all_dirty();
                    }
                }
            }
            MouseEventKind::ScrollLeft => {
                self.sv.x = self.sv.x.saturating_sub(3);
            }
            MouseEventKind::ScrollRight => {
                let max = self.max_scroll_x();
                self.sv.x = (self.sv.x + 3).min(max);
            }
            _ => {}
        }
    }

    fn handle_sidebar_click(&mut self, row: u16) {
        let r = row as usize;
        let rows = self.term_h;
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
