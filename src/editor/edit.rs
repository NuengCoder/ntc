use super::{ci_find, is_ctrl_char, prev_char_byte, sel_range, CursorPos, Editor};
use crate::t;

impl Editor {
    /// Move all cursors by applying `f` to each. Clears selections first.
    pub(crate) fn move_cursors<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Editor),
    {
        if self.has_multiple_cursors() {
            self.clear_selection();
            self.for_each_cursor(false, |e| {
                e.clear_selection();
                f(e);
            });
        } else {
            self.clear_selection();
            f(self);
        }
    }

    /// Run `f` for every cursor position (primary + extra).
    /// `reverse=true`: process bottom-to-top (use for text mutations).
    /// `reverse=false`: process top-to-bottom (use for read/navigation).
    /// Temporarily swaps `cursor_y`/`cursor_byte`/`selection_anchor` to each
    /// cursor so that single-cursor operations like `insert_at_raw` work unchanged.
    pub(crate) fn for_each_cursor<F>(&mut self, reverse: bool, mut f: F)
    where
        F: FnMut(&mut Editor),
    {
        let n = self.cursor.extra_cursors.len();
        if n == 0 {
            f(self);
            return;
        }

        // Collect (slot, position) where slot==0 means primary.
        let mut slots: Vec<(usize, CursorPos)> = Vec::with_capacity(n + 1);
        slots.push((
            0,
            CursorPos {
                y: self.cursor.y,
                byte: self.cursor.byte,
                anchor: self.cursor.selection_anchor,
            },
        ));
        for (i, c) in self.cursor.extra_cursors.iter().enumerate() {
            slots.push((i + 1, *c));
        }

        // Sort: for mutations, process bottom-to-top so earlier inserts don't
        // shift positions for later cursors.
        if reverse {
            slots.sort_by_key(|b| std::cmp::Reverse((b.1.y, b.1.byte)));
        } else {
            slots.sort_by_key(|a| (a.1.y, a.1.byte));
        }

        for slot in slots.iter_mut() {
            let (_, cp) = slot;
            self.cursor.y = cp.y;
            self.cursor.byte = cp.byte;
            self.cursor.selection_anchor = cp.anchor;

            f(self);

            slot.1 = CursorPos {
                y: self.cursor.y,
                byte: self.cursor.byte,
                anchor: self.cursor.selection_anchor,
            };
        }

        // Write back
        for (slot, cp) in slots {
            if slot == 0 {
                self.cursor.y = cp.y;
                self.cursor.byte = cp.byte;
                self.cursor.selection_anchor = cp.anchor;
            } else {
                self.cursor.extra_cursors[slot - 1] = cp;
            }
        }
    }

    // ── mutations ────────────────────────────────────────────────────────────

    /// Insert without snapshot (used for paste/batch ops).
    pub(crate) fn insert_at_raw(&mut self, c: char) {
        if is_ctrl_char(c) {
            return;
        }
        let byte = self.cursor.byte.min(self.current().len());
        self.current_mut().insert(byte, c);
        self.cursor.byte = byte + c.len_utf8();
        self.modified = true;
        self.mark_dirty(self.cursor.y);
        self.syntax.invalidate_line(self.cursor.y);
        self.search.mark_dirty();
    }

    pub(crate) fn insert_at(&mut self, c: char) {
        if is_ctrl_char(c) {
            return;
        }
        // If selection active, replace it
        if self.has_selection() {
            self.snapshot();
            self.delete_selection();
        }
        self.insert_at_raw(c);
        self.modified = true;
        self.save_if_auto();
    }

    pub(crate) fn backspace(&mut self) {
        if self.has_selection() {
            self.snapshot();
            self.delete_selection();
            return;
        }
        let line = self.current();
        if self.cursor.byte > 0 {
            let prev = prev_char_byte(line, self.cursor.byte);
            self.snapshot();
            self.current_mut().remove(prev);
            self.cursor.byte = prev;
            self.modified = true;
            self.mark_dirty(self.cursor.y);
            self.syntax.invalidate_line(self.cursor.y);
        } else if self.cursor.y > 0 {
            self.snapshot();
            let prev_len = self.lines[self.cursor.y - 1].len();
            let tail = self.lines.remove(self.cursor.y);
            self.lines[self.cursor.y - 1].push_str(&tail);
            self.syntax.invalidate_line(self.cursor.y - 1);
            self.syntax.remove_line(self.cursor.y);
            self.mark_dirty(self.cursor.y - 1);
            self.mark_dirty(self.cursor.y);
            self.dirty_end = self.dirty_end.max(self.lines.len());
            self.cursor.y -= 1;
            self.cursor.byte = prev_len;
            self.modified = true;
        }
        self.search.mark_dirty();
        self.save_if_auto();
    }

    pub(crate) fn delete_forward(&mut self) {
        if self.has_selection() {
            self.snapshot();
            self.delete_selection();
            return;
        }
        let (byte, next_byte) = {
            let line = self.current();
            let byte_len = line.len();
            let byte = self.cursor.byte.min(byte_len);
            if byte < byte_len {
                let nxt = (byte + 1..=byte_len)
                    .find(|&b| line.is_char_boundary(b))
                    .unwrap_or(byte_len);
                (byte, Some(nxt))
            } else {
                (byte, None)
            }
        };
        if let Some(next) = next_byte {
            self.snapshot();
            self.current_mut().drain(byte..next);
            self.modified = true;
            self.mark_dirty(self.cursor.y);
            self.syntax.invalidate_line(self.cursor.y);
        } else if self.cursor.y + 1 < self.lines.len() {
            self.snapshot();
            let next = self.lines.remove(self.cursor.y + 1);
            self.current_mut().push_str(&next);
            self.modified = true;
            self.mark_dirty(self.cursor.y);
            self.syntax.invalidate_line(self.cursor.y);
            self.syntax.remove_line(self.cursor.y + 1);
        }
        self.search.mark_dirty();
        self.save_if_auto();
    }

    pub(crate) fn split_line(&mut self) {
        let byte = self.cursor.byte.min(self.current().len());
        let rest = self.current_mut().split_off(byte);
        self.lines.insert(self.cursor.y + 1, rest);
        self.syntax.invalidate_line(self.cursor.y);
        self.syntax.insert_line(self.cursor.y + 1);
        self.mark_dirty(self.cursor.y);
        self.dirty_end = self.dirty_end.max(self.lines.len());

        // Smart indent: copy leading whitespace from previous line
        let prev_line = &self.lines[self.cursor.y];
        let indent_len = prev_line
            .bytes()
            .take_while(|b| *b == b' ' || *b == b'\t')
            .count();
        if indent_len > 0 {
            let indent: String = prev_line.chars().take(indent_len).collect();
            self.lines[self.cursor.y + 1].insert_str(0, &indent);
        }

        self.cursor.y += 1;
        self.cursor.byte = indent_len;
        self.modified = true;
        self.search.mark_dirty();
        self.save_if_auto();
    }

    // ── multi-cursor text operations ─────────────────────────────────────────

    /// Insert a character at every cursor (reverse order).
    /// Deletes any active selection at each cursor first.
    pub(crate) fn multi_insert_at(&mut self, c: char) {
        if is_ctrl_char(c) {
            return;
        }
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.cursor.y >= e.lines.len() {
                return;
            }
            if e.has_selection() {
                e.delete_selection();
            }
            e.insert_at_raw(c);
        });
        self.save_if_auto();
    }

    pub(crate) fn multi_backspace(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.cursor.y >= e.lines.len() {
                return;
            }
            if e.has_selection() {
                e.delete_selection();
                return;
            }
            let line = e.current();
            if e.cursor.byte > 0 {
                let prev = prev_char_byte(line, e.cursor.byte);
                e.current_mut().remove(prev);
                e.cursor.byte = prev;
                e.modified = true;
                e.mark_dirty(e.cursor.y);
            } else if e.cursor.y > 0 {
                let prev_len = e.lines[e.cursor.y - 1].len();
                let tail = e.lines.remove(e.cursor.y);
                e.lines[e.cursor.y - 1].push_str(&tail);
                e.mark_dirty(e.cursor.y - 1);
                e.mark_dirty(e.cursor.y);
                e.cursor.y -= 1;
                e.cursor.byte = prev_len;
                e.modified = true;
            }
        });
        self.save_if_auto();
    }

    pub(crate) fn multi_delete_forward(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.cursor.y >= e.lines.len() {
                return;
            }
            if e.has_selection() {
                e.delete_selection();
                return;
            }
            let (byte, next_byte) = {
                let line = e.current();
                let byte_len = line.len();
                let byte = e.cursor.byte.min(byte_len);
                if byte < byte_len {
                    let nxt = (byte + 1..=byte_len)
                        .find(|&b| line.is_char_boundary(b))
                        .unwrap_or(byte_len);
                    (byte, Some(nxt))
                } else {
                    (byte, None)
                }
            };
            if let Some(next) = next_byte {
                e.current_mut().drain(byte..next);
                e.modified = true;
                e.mark_dirty(e.cursor.y);
            } else if e.cursor.y + 1 < e.lines.len() {
                let next = e.lines.remove(e.cursor.y + 1);
                e.current_mut().push_str(&next);
                e.modified = true;
                e.mark_dirty(e.cursor.y);
            }
        });
        self.save_if_auto();
    }

    pub(crate) fn multi_paste_fast(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.cursor.y >= e.lines.len() {
                return;
            }
            if e.has_selection() {
                e.delete_selection();
            }
            e.paste_fast(text);
        });
        self.save_if_auto();
    }

    pub(crate) fn multi_split_line(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            let byte = e.cursor.byte.min(e.current().len());
            let rest = e.current_mut().split_off(byte);
            e.lines.insert(e.cursor.y + 1, rest);
            e.mark_dirty(e.cursor.y);
            e.cursor.y += 1;
            e.cursor.byte = 0;
            e.modified = true;
        });
        self.mark_all_dirty();
        self.save_if_auto();
    }

    /// Duplicate the current line (Ctrl+D, VSCode style).
    /// Delete the entire current line (Ctrl+K).
    pub(crate) fn kill_line(&mut self) {
        self.snapshot();
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor.y);
            self.syntax.remove_line(self.cursor.y);
            self.cursor.y = self.cursor.y.min(self.lines.len().saturating_sub(1));
        } else {
            self.lines[0].clear();
            self.cursor.byte = 0;
        }
        self.mark_dirty(self.cursor.y);
        self.dirty_end = self.dirty_end.max(self.lines.len());
        self.cursor.byte = self.cursor.byte.min(self.current().len());
        self.modified = true;
        self.status_msg = Some("Line deleted".into());
        self.search.mark_dirty();
        self.save_if_auto();
    }

    /// Like VSCode Ctrl+D:
    /// - If no selection, select the word under the primary cursor.
    /// - If selection exists, add a new cursor at the *next* occurrence of that
    ///   word (searching from the last extra cursor, or primary if none).
    pub(crate) fn add_cursor_at_next_occurrence(&mut self) {
        // Determine the search text from primary cursor's selection
        let query = if self.has_selection() {
            self.selected_text()
        } else {
            match self.select_word_under_cursor() {
                Some(w) => w,
                None => return,
            }
        };
        if query.is_empty() {
            return;
        }

        // Search from the primary cursor position (which is always the most recently
        // found occurrence). Searching from here skips the already-selected occurrence.
        let (search_y, search_byte) = (self.cursor.y, self.cursor.byte);

        // Inline helper: check if (y, byte) already has a cursor
        let pos_has_cursor =
            |extra: &[CursorPos], py: usize, pb: usize, y: usize, byte: usize| -> bool {
                (py == y && pb == byte) || extra.iter().any(|c| c.y == y && c.byte == byte)
            };

        // Find next occurrence at or after search point
        for li in search_y..self.lines.len() {
            let line = &self.lines[li];
            let start = if li == search_y { search_byte } else { 0 };
            if let Some((s, e)) = ci_find(line, &query, start) {
                let already_exists = pos_has_cursor(
                    &self.cursor.extra_cursors,
                    self.cursor.y,
                    self.cursor.byte,
                    li,
                    e,
                );
                if !already_exists {
                    let old_primary = CursorPos {
                        y: self.cursor.y,
                        byte: self.cursor.byte,
                        anchor: self.cursor.selection_anchor,
                    };
                    self.cursor.extra_cursors.push(old_primary);
                    self.cursor.y = li;
                    self.cursor.byte = e;
                    self.cursor.selection_anchor = Some((li, s));
                    self.cursor.last_added_cursor_idx = Some(self.cursor.extra_cursors.len() - 1);
                    self.mark_all_dirty();
                    self.status_msg = Some(format!(
                        "{} ({})",
                        t!("editor.edit_added_cursor"),
                        self.cursor.extra_cursors.len() + 1
                    ));
                } else {
                    self.status_msg = Some(t!("editor.edit_already_selected"));
                }
                return;
            }
        }
        // Wrap around from top
        for li in 0..=search_y {
            let line = &self.lines[li];
            let start = 0;
            if let Some((s, e)) = ci_find(line, &query, start) {
                if li < search_y || (li == search_y && s < search_byte) {
                    let already_exists = pos_has_cursor(
                        &self.cursor.extra_cursors,
                        self.cursor.y,
                        self.cursor.byte,
                        li,
                        e,
                    );
                    if !already_exists {
                        let old_primary = CursorPos {
                            y: self.cursor.y,
                            byte: self.cursor.byte,
                            anchor: self.cursor.selection_anchor,
                        };
                        self.cursor.extra_cursors.push(old_primary);
                        self.cursor.y = li;
                        self.cursor.byte = e;
                        self.cursor.selection_anchor = Some((li, s));
                        self.cursor.last_added_cursor_idx =
                            Some(self.cursor.extra_cursors.len() - 1);
                        self.mark_all_dirty();
                        self.status_msg = Some(format!(
                            "{} ({}) ({})",
                            t!("editor.edit_added_cursor"),
                            self.cursor.extra_cursors.len() + 1,
                            t!("editor.edit_wrap")
                        ));
                    } else {
                        self.status_msg = Some(t!("editor.edit_already_selected"));
                    }
                    return;
                }
            }
        }
        self.status_msg = Some(t!("editor.edit_no_more_occurrences"));
    }

    /// Move current line up (Alt+Up).
    pub(crate) fn move_line_up(&mut self) {
        if self.cursor.y == 0 {
            return;
        }
        self.snapshot();
        self.lines.swap(self.cursor.y, self.cursor.y - 1);
        self.syntax.invalidate_line(self.cursor.y);
        self.syntax.invalidate_line(self.cursor.y - 1);
        self.mark_dirty(self.cursor.y);
        self.mark_dirty(self.cursor.y - 1);
        self.cursor.y -= 1;
        self.modified = true;
        self.search.mark_dirty();
        self.save_if_auto();
    }

    pub(crate) fn move_line_down(&mut self) {
        if self.cursor.y + 1 >= self.lines.len() {
            return;
        }
        self.snapshot();
        self.lines.swap(self.cursor.y, self.cursor.y + 1);
        self.syntax.invalidate_line(self.cursor.y);
        self.syntax.invalidate_line(self.cursor.y + 1);
        self.mark_dirty(self.cursor.y);
        self.mark_dirty(self.cursor.y + 1);
        self.cursor.y += 1;
        self.modified = true;
        self.save_if_auto();
    }

    /// Indent selected lines or current line (Tab with selection).
    pub(crate) fn indent_lines(&mut self) {
        let tab = " ".repeat(self.editor_cfg.tab_size);
        if let Some((ay, _ab)) = self.cursor.selection_anchor {
            self.snapshot();
            let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, 0);
            for row in sy..=ey.min(self.lines.len().saturating_sub(1)) {
                self.lines[row].insert_str(0, &tab);
                self.mark_dirty(row);
                self.syntax.invalidate_line(row);
            }
            self.modified = true;
        } else {
            for _ in 0..self.editor_cfg.tab_size {
                self.insert_at(' ');
            }
        }
        self.save_if_auto();
    }

    /// Dedent selected lines or current line (Shift+Tab).
    pub(crate) fn dedent_lines(&mut self) {
        let tab_size = self.editor_cfg.tab_size;
        if let Some((ay, _ab)) = self.cursor.selection_anchor {
            self.snapshot();
            let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, 0);
            for row in sy..=ey.min(self.lines.len().saturating_sub(1)) {
                let spaces = self.lines[row]
                    .chars()
                    .take(tab_size)
                    .filter(|&c| c == ' ' || c == '\t')
                    .count();
                let to_remove = spaces.min(tab_size);
                self.lines[row].drain(..to_remove);
                self.mark_dirty(row);
                self.syntax.invalidate_line(row);
            }
            self.modified = true;
        } else {
            // Dedent current line
            let spaces = self.lines[self.cursor.y]
                .chars()
                .take(tab_size)
                .filter(|&c| c == ' ' || c == '\t')
                .count();
            let to_remove = spaces.min(tab_size);
            if to_remove > 0 {
                self.snapshot();
                self.lines[self.cursor.y].drain(..to_remove);
                self.mark_dirty(self.cursor.y);
                self.syntax.invalidate_line(self.cursor.y);
                self.cursor.byte = self.cursor.byte.saturating_sub(to_remove);
                self.modified = true;
            }
        }
        self.save_if_auto();
    }
}
