use crate::editor::{byte_to_col, gutter_width, sel_range, Editor};
use crate::t;

impl Editor {
    pub(crate) fn clamp(&mut self) {
        let max = self.lines.len().saturating_sub(1);
        self.cursor.y = self.cursor.y.min(max);
        let line_len = self.current().len();
        self.cursor.byte = self.cursor.byte.min(line_len);
        while self.cursor.byte > 0 && !self.current().is_char_boundary(self.cursor.byte) {
            self.cursor.byte -= 1;
        }
        for c in &mut self.cursor.extra_cursors {
            c.y = c.y.min(max);
            let clen = self.lines[c.y].len();
            c.byte = c.byte.min(clen);
            while c.byte > 0 && !self.lines[c.y].is_char_boundary(c.byte) {
                c.byte -= 1;
            }
            if let Some((ay, ab)) = c.anchor {
                let ay = ay.min(max);
                let ab = ab.min(self.lines[ay].len());
                c.anchor = Some((ay, ab));
            }
        }
    }

    pub(crate) fn mark_dirty(&mut self, line: usize) {
        if self.dirty_end == 0 {
            self.dirty_start = line;
            self.dirty_end = line + 1;
        } else {
            self.dirty_start = self.dirty_start.min(line);
            self.dirty_end = self.dirty_end.max(line + 1);
        }
    }

    pub(crate) fn mark_all_dirty(&mut self) {
        let rows = self.term_h.saturating_sub(4);
        self.dirty_start = self.sv.offset;
        self.dirty_end = self.sv.offset + rows;
    }

    pub(crate) fn max_scroll_x(&self) -> usize {
        let gw = gutter_width(self.lines.len());
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let max_vis = self
            .lines
            .iter()
            .map(|l| byte_to_col(l, l.len()))
            .max()
            .unwrap_or(0);
        max_vis.saturating_sub(text_cols)
    }

    pub(crate) fn scroll_visible(&mut self) {
        let rows = self.term_h.saturating_sub(5);
        if self.cursor.y < self.sv.offset {
            self.sv.offset = self.cursor.y;
        } else if self.cursor.y >= self.sv.offset + rows {
            self.sv.offset = self.cursor.y.saturating_sub(rows) + 1;
        }

        let gw = gutter_width(self.lines.len());
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let cursor_col = byte_to_col(self.current(), self.cursor.byte);
        if cursor_col >= self.sv.x + text_cols {
            self.sv.x = cursor_col.saturating_sub(text_cols) + 1;
        } else if cursor_col < self.sv.x {
            self.sv.x = cursor_col;
        }
    }

    pub(crate) fn clear_selection(&mut self) {
        if let Some((ay, _ab)) = self.cursor.selection_anchor {
            let (s, e) = if ay < self.cursor.y {
                (ay, self.cursor.y)
            } else {
                (self.cursor.y, ay)
            };
            for row in s..=e {
                self.mark_dirty(row);
            }
        }
        self.cursor.selection_anchor = None;
    }

    pub(crate) fn start_or_extend_selection(&mut self) {
        if self.cursor.selection_anchor.is_none() {
            if self.has_multiple_cursors() {
                self.clear_extra_cursors();
            }
            self.cursor.selection_anchor = Some((self.cursor.y, self.cursor.byte));
            self.mark_dirty(self.cursor.y);
        }
    }

    pub(crate) fn has_selection(&self) -> bool {
        if let Some((ay, ab)) = self.cursor.selection_anchor {
            (ay, ab) != (self.cursor.y, self.cursor.byte)
        } else {
            false
        }
    }

    pub(crate) fn selected_text(&self) -> String {
        let (ay, ab) = match self.cursor.selection_anchor {
            Some(a) => a,
            None => return String::new(),
        };
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        if sy == ey {
            self.lines[sy][sb..eb.min(self.lines[sy].len())].to_string()
        } else {
            let mut out = String::new();
            out.push_str(&self.lines[sy][sb..]);
            out.push('\n');
            for row in sy + 1..ey {
                out.push_str(&self.lines[row]);
                out.push('\n');
            }
            out.push_str(&self.lines[ey][..eb.min(self.lines[ey].len())]);
            out
        }
    }

    pub(crate) fn delete_selection(&mut self) {
        let (ay, ab) = match self.cursor.selection_anchor {
            Some(a) => a,
            None => return,
        };
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        let end_clamped = eb.min(self.lines[ey].len());
        if sy == ey {
            self.lines[sy].drain(sb..end_clamped);
            self.mark_dirty(sy);
            self.syntax.invalidate_line(sy);
        } else {
            let tail = self.lines[ey][end_clamped..].to_string();
            self.lines.drain(sy + 1..=ey);
            for _ in sy + 1..=ey {
                self.syntax.remove_line(sy + 1);
            }
            self.lines[sy].truncate(sb);
            self.lines[sy].push_str(&tail);
            self.syntax.invalidate_line(sy);
            self.mark_dirty(sy);
            self.dirty_end = self.dirty_end.max(self.lines.len());
        }
        self.cursor.y = sy;
        self.cursor.byte = sb;
        self.cursor.selection_anchor = None;
        self.modified = true;
    }

    pub(crate) fn split_primary_cursor(&mut self) {
        let cp = crate::editor::CursorPos {
            y: self.cursor.y,
            byte: self.cursor.byte,
            anchor: self.cursor.selection_anchor,
        };
        self.cursor.extra_cursors.push(cp);
        self.cursor.last_added_cursor_idx = Some(self.cursor.extra_cursors.len() - 1);
        self.mark_all_dirty();
        self.status_msg = Some(format!(
            "{} ({})",
            t!("editor.nav_split_cursor"),
            self.cursor.extra_cursors.len() + 1
        ));
    }

    pub(crate) fn select_all(&mut self) {
        self.cursor.selection_anchor = Some((0, 0));
        self.cursor.y = self.lines.len().saturating_sub(1);
        self.cursor.byte = self.lines[self.cursor.y].len();
        self.mark_all_dirty();
        self.status_msg = Some(t!("editor.nav_selected_all"));
    }

    pub(crate) fn select_line(&mut self) {
        self.clear_extra_cursors();
        self.cursor.selection_anchor = Some((self.cursor.y, 0));
        self.cursor.byte = self.current().len();
        self.mark_dirty(self.cursor.y);
        self.status_msg = Some(t!("editor.nav_line_selected"));
    }

    pub(crate) fn select_word_under_cursor(&mut self) -> Option<String> {
        let line = self.current();
        let byte = self.cursor.byte;
        if byte >= line.len() || line.is_empty() {
            return None;
        }
        let start = byte
            - line[..byte]
                .chars()
                .rev()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .map(|c| c.len_utf8())
                .sum::<usize>();
        let end = byte
            + line[byte..]
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .map(|c| c.len_utf8())
                .sum::<usize>();
        if start == end {
            return None;
        }
        let word = line[start..end].to_string();
        self.cursor.byte = end;
        self.cursor.selection_anchor = Some((self.cursor.y, start));
        Some(word)
    }
}
