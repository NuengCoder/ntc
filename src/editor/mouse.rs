use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use super::{col_to_byte, gutter_width, Editor, Focus};
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
        let rows = self.visible_text_rows();
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

    pub(crate) fn handle_mouse(&mut self, m: MouseEvent) {
        // Mouse is optional and limited to the text field and the
        // terminal panel: tab bar, sidebar, scroll bars and panel
        // borders intentionally ignore mouse input.
        if !self.editor_cfg.mouse_enabled {
            return;
        }
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let eo = self.editor_offset();
                let col = m.column as usize;
                let tab_bar_h = 1;
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

                // Tab bar close zones only: far-right X closes all tabs,
                // per-tab ✕ closes that tab. Tab switching stays
                // keyboard-only (gmode); name clicks do nothing.
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
                        let tab_end = x + self.term_w.saturating_sub(eo);
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

                        // Close button area: the single "✕" character
                        let name_end = x + label_chars;
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

                // Sidebar click: select + open (dirs expand/collapse,
                // files open in place). Works in any mode while open.
                if self.sidebar.open && col < eo && (m.row as usize) < self.term_h {
                    let idx = self.sidebar.scroll + m.row as usize;
                    if idx < self.sidebar.nodes.len() {
                        self.sidebar.selected = idx;
                        let node = self.sidebar.nodes[idx].clone();
                        if node.is_dir {
                            self.sidebar.toggle_expand(idx);
                        } else {
                            self.open_file(&node.path);
                        }
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    return;
                }

                // Text-field click moves the cursor
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
            MouseEventKind::Up(MouseButton::Left) => {
                self.sv.dragging_vscroll = false;
                self.sv.dragging_hscroll = false;
                self.sidebar.dragging_border = false;
                self.dragging_run_border = false;
            }
            MouseEventKind::ScrollUp => {
                if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    self.sv.x = self.sv.x.saturating_sub(3);
                } else if self.sv.offset > 0 {
                    self.sv.offset = self.sv.offset.saturating_sub(3);
                    self.mark_all_dirty();
                }
            }
            MouseEventKind::ScrollDown => {
                if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    let max = self.max_scroll_x();
                    self.sv.x = (self.sv.x + 3).min(max);
                } else if self.sv.offset + 1 < self.lines.len() {
                    self.sv.offset =
                        (self.sv.offset + 3).min(self.lines.len().saturating_sub(1));
                    self.mark_all_dirty();
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

}
