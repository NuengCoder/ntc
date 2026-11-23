use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use super::{display_col_to_byte, gutter_width, Editor, Focus};
use crate::t;

impl Editor {
    /// Sidebar scrollbar geometry: `(bar column, rows)` when the bar is
    /// visible (open sidebar with more nodes than rows). Mirrors the
    /// render math in `render_sidebar`.
    pub(crate) fn nfe_scrollbar_geom(&self) -> Option<(usize, usize)> {
        if !self.sidebar.open {
            return None;
        }
        let total = self.sidebar.nodes.len();
        let rows = self.term_h;
        if rows == 0 || total <= rows {
            return None;
        }
        Some((self.sidebar_cols().saturating_sub(1), rows))
    }

    /// Map a scrollbar row to a sidebar scroll offset (inverse of the
    /// render thumb math: `start = scroll·(rows−h)/(total−1)`).
    pub(crate) fn nfe_scroll_for_row(row: usize, total: usize, rows: usize) -> usize {
        let avail = rows.max(1);
        let height = (avail * rows / total.max(1)).max(1);
        let span = avail.saturating_sub(height);
        let max_scroll = total.saturating_sub(1);
        if span == 0 || max_scroll == 0 {
            return 0;
        }
        (row.min(avail.saturating_sub(1)) * max_scroll / span).min(max_scroll)
    }

    /// Geometry of the text area's vertical scrollbar: `(bar column, rows)`.
    /// Matches the thumb math in `render()`.
    pub(crate) fn editor_vscroll_geom(&self) -> Option<(usize, usize)> {
        let rows = self.visible_text_rows();
        if rows == 0 || self.term_w == 0 || self.lines.len() <= rows {
            return None;
        }
        Some((self.term_w.saturating_sub(1), rows))
    }

    /// Map a vertical scrollbar row to a text scroll offset. Content rows
    /// start at screen row 1 (row 0 is the tab bar), so the 0-based track
    /// row is `row - 1`. Uses the same direct mapping as the NFE sidebar
    /// bar: the thumb's top follows the pointer, the top row yields 0 and
    /// the bottom row yields `total - 1` (last line at the top).
    pub(crate) fn editor_scroll_for_row(row: usize, total: usize, rows: usize) -> usize {
        Self::nfe_scroll_for_row(row.saturating_sub(1), total, rows)
    }

    /// Geometry of the text area's horizontal scrollbar:
    /// `(bar row, left column, track width, thumb width)`, or `None` when
    /// horizontal scrolling is not needed.
    pub(crate) fn editor_hscroll_geom(&self) -> Option<(usize, usize, usize, usize)> {
        // The bar's row belongs to the run/terminal panel when one is open.
        if self.run_panel_open || self.terminal.is_some() {
            return None;
        }
        let rows = self.visible_text_rows();
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let gw = gutter_width(self.lines.len());
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let max_vis = self.max_line_width;
        if text_cols == 0 || max_vis <= text_cols {
            return None;
        }
        let track = editor_cols.saturating_sub(2);
        if track == 0 {
            return None;
        }
        let thumb_start = self.sv.x * track / max_vis;
        let thumb_end = (self.sv.x + text_cols).min(max_vis) * track / max_vis;
        let thumb = (thumb_end.saturating_sub(thumb_start)).clamp(1, track);
        Some((1 + rows, eo + 1, track, thumb))
    }

    /// Map a horizontal scrollbar column to a text horizontal offset.
    pub(crate) fn editor_scroll_x_for_col(&self, col: usize) -> usize {
        let Some((_, left, track, thumb)) = self.editor_hscroll_geom() else {
            return self.sv.x;
        };
        let span = track.saturating_sub(thumb);
        // Use the cached widest-line width, like the renderer; scanning the
        // whole buffer here would add O(file) work to every drag event.
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gutter_width(self.lines.len()) + 2);
        let max_scroll = self.max_line_width.saturating_sub(text_cols);
        if span == 0 || max_scroll == 0 {
            return 0;
        }
        let pos = col.saturating_sub(left).min(track.saturating_sub(1));
        let start = pos.saturating_sub(thumb / 2).min(span);
        (start * max_scroll + span / 2) / span
    }

    /// Scroll the sidebar with the wheel when the pointer is over it.
    /// Returns true when consumed (caller skips text-field scrolling).
    pub(crate) fn sidebar_wheel(&mut self, col: u16, row: u16, up: bool) -> bool {
        if !self.sidebar.open
            || (col as usize) >= self.editor_offset()
            || (row as usize) >= self.term_h
            || self.sidebar.nodes.is_empty()
        {
            return false;
        }
        let max = self.sidebar.nodes.len().saturating_sub(1);
        if up {
            self.sidebar.scroll = self.sidebar.scroll.saturating_sub(3);
        } else {
            self.sidebar.scroll = (self.sidebar.scroll + 3).min(max);
        }
        true
    }

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
        let byte = display_col_to_byte(
            &self.lines[line_idx],
            scroll_col,
            self.indent_width().max(1),
        );
        Some((line_idx, byte))
    }

    /// Clickable spans `(start, end)` of the tab bar in screen columns.
    /// The final cell of each span is the close button; the rest of the
    /// span selects that tab. Mirrors the tabs drawn by `render_tab_bar`.
    pub(crate) fn tab_hit_zones(&self) -> Vec<(usize, usize)> {
        let mut zones = Vec::new();
        let right = self.term_w;
        let mut x = self.editor_offset();
        for (i, _) in self.tabs.iter().enumerate() {
            let full_w = self.tab_label(i).chars().count() + 1;
            if full_w > right.saturating_sub(x) {
                break;
            }
            zones.push((x, x + full_w));
            x += full_w;
        }
        zones
    }

    pub(crate) fn handle_mouse(&mut self, m: MouseEvent) {
        // Mouse is optional and covers the text field, sidebar and tab
        // bar (tab click switches, X closes); panel borders ignore it.
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

                // Tab bar: far-right X closes all tabs, per-tab closes
                // that tab, and clicking the rest of a tab switches to it
                // without changing the current mode.
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
                    let hit = self
                        .tab_hit_zones()
                        .into_iter()
                        .enumerate()
                        .find(|(_, (start, end))| col >= *start && col < *end);
                    if let Some((i, (_, end))) = hit {
                        let close_start = end.saturating_sub(1);
                        if col >= close_start {
                            if self.close_tab(i) {
                                self.pending_quit = true;
                            }
                            self.mark_all_dirty();
                        } else {
                            self.switch_tab(i);
                        }
                    }
                    return;
                }

                // Sidebar scrollbar: click/drag scrolling. Hit-tested
                // before node select (the bar column would otherwise
                // select the node behind it).
                if (m.row as usize) < self.term_h {
                    if let Some((sb_col, rows)) = self.nfe_scrollbar_geom() {
                        if col == sb_col {
                            let total = self.sidebar.nodes.len();
                            self.sidebar.scroll =
                                Self::nfe_scroll_for_row(m.row as usize, total, rows);
                            self.sidebar.dragging_scroll = true;
                            self.mark_all_dirty();
                            return;
                        }
                    }
                }

                // Text-field vertical scrollbar: click/drag scrolling.
                // Hit-tested before the text click so the far-right column
                // does not move the cursor.
                if let Some((bar_col, rows)) = self.editor_vscroll_geom() {
                    let row = m.row as usize;
                    if col == bar_col && (1..1 + rows).contains(&row) {
                        self.sv.offset = Self::editor_scroll_for_row(row, self.lines.len(), rows);
                        self.sv.dragging_vscroll = true;
                        self.mark_all_dirty();
                        return;
                    }
                }

                // Text-field horizontal scrollbar (row below the text).
                if let Some((bar_row, left, track, _)) = self.editor_hscroll_geom() {
                    let row = m.row as usize;
                    if row == bar_row && col >= left && col < left + track {
                        self.sv.x = self.editor_scroll_x_for_col(col);
                        self.sv.dragging_hscroll = true;
                        self.mark_all_dirty();
                        return;
                    }
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
                // Sidebar scrollbar drag takes over the gesture.
                if self.sidebar.dragging_scroll {
                    let total = self.sidebar.nodes.len();
                    let rows = self.term_h;
                    self.sidebar.scroll = Self::nfe_scroll_for_row(m.row as usize, total, rows);
                    self.mark_all_dirty();
                    return;
                }
                if self.sv.dragging_vscroll {
                    let rows = self.visible_text_rows();
                    self.sv.offset =
                        Self::editor_scroll_for_row(m.row as usize, self.lines.len(), rows);
                    self.mark_all_dirty();
                    return;
                }
                if self.sv.dragging_hscroll {
                    self.sv.x = self.editor_scroll_x_for_col(m.column as usize);
                    self.mark_all_dirty();
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
            MouseEventKind::Up(MouseButton::Left) => {
                self.sv.dragging_vscroll = false;
                self.sv.dragging_hscroll = false;
                self.sidebar.dragging_border = false;
                self.sidebar.dragging_scroll = false;
            }
            MouseEventKind::ScrollUp => {
                if self.sidebar_wheel(m.column, m.row, true) {
                    self.mark_all_dirty();
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    self.sv.x = self.sv.x.saturating_sub(3);
                } else if self.sv.offset > 0 {
                    self.sv.offset = self.sv.offset.saturating_sub(3);
                    self.mark_all_dirty();
                }
            }
            MouseEventKind::ScrollDown => {
                if self.sidebar_wheel(m.column, m.row, false) {
                    self.mark_all_dirty();
                } else if m.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                    let max = self.max_scroll_x();
                    self.sv.x = (self.sv.x + 3).min(max);
                } else if self.sv.offset + 1 < self.lines.len() {
                    self.sv.offset = (self.sv.offset + 3).min(self.lines.len().saturating_sub(1));
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
