use super::{
    prev_char_byte, is_ctrl_char, sel_range,
    CursorPos, Editor,
};

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
        let n = self.extra_cursors.len();
        if n == 0 {
            f(self);
            return;
        }

        // Collect (slot, position) where slot==0 means primary.
        let mut slots: Vec<(usize, CursorPos)> = Vec::with_capacity(n + 1);
        slots.push((
            0,
            CursorPos {
                y: self.cursor_y,
                byte: self.cursor_byte,
                anchor: self.selection_anchor,
            },
        ));
        for (i, c) in self.extra_cursors.iter().enumerate() {
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
            self.cursor_y = cp.y;
            self.cursor_byte = cp.byte;
            self.selection_anchor = cp.anchor;

            f(self);

            slot.1 = CursorPos {
                y: self.cursor_y,
                byte: self.cursor_byte,
                anchor: self.selection_anchor,
            };
        }

        // Write back
        for (slot, cp) in slots {
            if slot == 0 {
                self.cursor_y = cp.y;
                self.cursor_byte = cp.byte;
                self.selection_anchor = cp.anchor;
            } else {
                self.extra_cursors[slot - 1] = cp;
            }
        }
    }

    // ── mutations ────────────────────────────────────────────────────────────

    /// Insert without snapshot (used for paste/batch ops).
    pub(crate) fn insert_at_raw(&mut self, c: char) {
        if is_ctrl_char(c) {
            return;
        }
        let byte = self.cursor_byte.min(self.current().len());
        self.current_mut().insert(byte, c);
        self.cursor_byte = byte + c.len_utf8();
        self.modified = true;
        self.mark_dirty(self.cursor_y);
        self.syntax.invalidate_line(self.cursor_y);
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
    }

    pub(crate) fn backspace(&mut self) {
        if self.has_selection() {
            self.snapshot();
            self.delete_selection();
            return;
        }
        let line = self.current();
        if self.cursor_byte > 0 {
            let prev = prev_char_byte(line, self.cursor_byte);
            self.snapshot();
            self.current_mut().remove(prev);
            self.cursor_byte = prev;
            self.modified = true;
            self.mark_dirty(self.cursor_y);
            self.syntax.invalidate_line(self.cursor_y);
        } else if self.cursor_y > 0 {
            self.snapshot();
            let prev_len = self.lines[self.cursor_y - 1].len();
            let tail = self.lines.remove(self.cursor_y);
            self.lines[self.cursor_y - 1].push_str(&tail);
            self.syntax.invalidate_line(self.cursor_y - 1);
            self.syntax.remove_line(self.cursor_y);
            self.mark_dirty(self.cursor_y - 1);
            self.mark_dirty(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_byte = prev_len;
            self.modified = true;
        }
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
            let byte = self.cursor_byte.min(byte_len);
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
            self.mark_dirty(self.cursor_y);
            self.syntax.invalidate_line(self.cursor_y);
        } else if self.cursor_y + 1 < self.lines.len() {
            self.snapshot();
            let next = self.lines.remove(self.cursor_y + 1);
            self.current_mut().push_str(&next);
            self.modified = true;
            self.mark_dirty(self.cursor_y);
            self.syntax.invalidate_line(self.cursor_y);
            self.syntax.remove_line(self.cursor_y + 1);
        }
    }

    pub(crate) fn split_line(&mut self) {
        let byte = self.cursor_byte.min(self.current().len());
        let rest = self.current_mut().split_off(byte);
        self.lines.insert(self.cursor_y + 1, rest);
        self.syntax.invalidate_line(self.cursor_y);
        self.syntax.insert_line(self.cursor_y + 1);
        self.mark_dirty(self.cursor_y);
        self.dirty_end = self.dirty_end.max(self.lines.len());
        self.cursor_y += 1;
        self.cursor_byte = 0;
        self.modified = true;
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
            if e.has_selection() {
                e.delete_selection();
            }
            e.insert_at_raw(c);
        });
    }

    pub(crate) fn multi_backspace(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.has_selection() {
                e.delete_selection();
                return;
            }
            let line = e.current();
            if e.cursor_byte > 0 {
                let prev = prev_char_byte(line, e.cursor_byte);
                e.current_mut().remove(prev);
                e.cursor_byte = prev;
                e.modified = true;
                e.mark_dirty(e.cursor_y);
            } else if e.cursor_y > 0 {
                let prev_len = e.lines[e.cursor_y - 1].len();
                let tail = e.lines.remove(e.cursor_y);
                e.lines[e.cursor_y - 1].push_str(&tail);
                e.mark_dirty(e.cursor_y - 1);
                e.mark_dirty(e.cursor_y);
                e.cursor_y -= 1;
                e.cursor_byte = prev_len;
                e.modified = true;
            }
        });
    }

    pub(crate) fn multi_delete_forward(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.has_selection() {
                e.delete_selection();
                return;
            }
            let (byte, next_byte) = {
                let line = e.current();
                let byte_len = line.len();
                let byte = e.cursor_byte.min(byte_len);
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
                e.mark_dirty(e.cursor_y);
            } else if e.cursor_y + 1 < e.lines.len() {
                let next = e.lines.remove(e.cursor_y + 1);
                e.current_mut().push_str(&next);
                e.modified = true;
                e.mark_dirty(e.cursor_y);
            }
        });
    }

    pub(crate) fn multi_paste_fast(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.snapshot();
        self.for_each_cursor(true, |e| {
            if e.has_selection() {
                e.delete_selection();
            }
            e.paste_fast(text);
        });
    }

    pub(crate) fn multi_split_line(&mut self) {
        self.snapshot();
        self.for_each_cursor(true, |e| {
            let byte = e.cursor_byte.min(e.current().len());
            let rest = e.current_mut().split_off(byte);
            e.lines.insert(e.cursor_y + 1, rest);
            e.mark_dirty(e.cursor_y);
            e.cursor_y += 1;
            e.cursor_byte = 0;
            e.modified = true;
        });
        self.mark_all_dirty();
    }

    /// Duplicate the current line (Ctrl+D, VSCode style).
    /// Delete the entire current line (Ctrl+K).
    pub(crate) fn kill_line(&mut self) {
        self.snapshot();
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor_y);
            self.syntax.remove_line(self.cursor_y);
            self.cursor_y = self.cursor_y.min(self.lines.len().saturating_sub(1));
        } else {
            self.lines[0].clear();
            self.cursor_byte = 0;
        }
        self.mark_dirty(self.cursor_y);
        self.dirty_end = self.dirty_end.max(self.lines.len());
        self.cursor_byte = self.cursor_byte.min(self.current().len());
        self.modified = true;
        self.status_msg = Some("Line deleted".into());
    }

    /// Jump to the most recently added cursor (like VSCode Ctrl+G with multi-cursor).
    /// Swaps the primary cursor with the last added extra cursor.
    pub(crate) fn jump_to_last_cursor(&mut self) {
        let idx = match self.last_added_cursor_idx {
            Some(i) if i < self.extra_cursors.len() => i,
            _ => {
                self.status_msg = Some("No extra cursors to jump to".into());
                return;
            }
        };
        let target = self.extra_cursors[idx];
        let current = CursorPos {
            y: self.cursor_y,
            byte: self.cursor_byte,
            anchor: self.selection_anchor,
        };
        self.cursor_y = target.y;
        self.cursor_byte = target.byte;
        self.selection_anchor = target.anchor;
        self.extra_cursors[idx] = current;
        self.last_added_cursor_idx = None;
        self.mark_all_dirty();
        self.status_msg = Some(format!("Jumped to cursor {}", idx + 2));
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
        let (search_y, search_byte) = (self.cursor_y, self.cursor_byte);

        let q = query.to_lowercase();
        let q_len = q.len();

        // Inline helper: check if (y, byte) already has a cursor
        let pos_has_cursor = |extra: &[CursorPos], py: usize, pb: usize, y: usize, byte: usize| -> bool {
            (py == y && pb == byte) || extra.iter().any(|c| c.y == y && c.byte == byte)
        };

        // Find next occurrence at or after search point
        for li in search_y..self.lines.len() {
            let lower = self.lines[li].to_lowercase();
            let start = if li == search_y { search_byte } else { 0 };
            if let Some(pos) = lower[start..].find(&q) {
                let abs_pos = start + pos;
                let already_exists = pos_has_cursor(&self.extra_cursors, self.cursor_y, self.cursor_byte, li, abs_pos + q_len);
                if !already_exists {
                    let old_primary = CursorPos {
                        y: self.cursor_y,
                        byte: self.cursor_byte,
                        anchor: self.selection_anchor,
                    };
                    self.extra_cursors.push(old_primary);
                    self.cursor_y = li;
                    self.cursor_byte = abs_pos + q_len;
                    self.selection_anchor = Some((li, abs_pos));
                    self.last_added_cursor_idx = Some(self.extra_cursors.len() - 1);
                    self.mark_all_dirty();
                    self.status_msg = Some(format!(
                        "Added cursor ({} total)",
                        self.extra_cursors.len() + 1
                    ));
                } else {
                    self.status_msg = Some("Already selected".into());
                }
                return;
            }
        }
        // Wrap around from top
        for li in 0..=search_y {
            let lower = self.lines[li].to_lowercase();
            let start = 0;
            if let Some(pos) = lower[start..].find(&q) {
                let abs_pos = start + pos;
                if li < search_y || (li == search_y && abs_pos < search_byte) {
                    let already_exists = pos_has_cursor(&self.extra_cursors, self.cursor_y, self.cursor_byte, li, abs_pos + q_len);
                    if !already_exists {
                        let old_primary = CursorPos {
                            y: self.cursor_y,
                            byte: self.cursor_byte,
                            anchor: self.selection_anchor,
                        };
                        self.extra_cursors.push(old_primary);
                        self.cursor_y = li;
                        self.cursor_byte = abs_pos + q_len;
                        self.selection_anchor = Some((li, abs_pos));
                        self.last_added_cursor_idx = Some(self.extra_cursors.len() - 1);
                        self.mark_all_dirty();
                        self.status_msg = Some(format!(
                            "Added cursor ({} total) (wrap)",
                            self.extra_cursors.len() + 1
                        ));
                    } else {
                        self.status_msg = Some("Already selected".into());
                    }
                    return;
                }
            }
        }
        self.status_msg = Some("No more occurrences".into());
    }

    /// Move current line up (Alt+Up).
    pub(crate) fn move_line_up(&mut self) {
        if self.cursor_y == 0 {
            return;
        }
        self.snapshot();
        self.lines.swap(self.cursor_y, self.cursor_y - 1);
        self.syntax.invalidate_line(self.cursor_y);
        self.syntax.invalidate_line(self.cursor_y - 1);
        self.mark_dirty(self.cursor_y);
        self.mark_dirty(self.cursor_y - 1);
        self.cursor_y -= 1;
        self.modified = true;
    }

    pub(crate) fn move_line_down(&mut self) {
        if self.cursor_y + 1 >= self.lines.len() {
            return;
        }
        self.snapshot();
        self.lines.swap(self.cursor_y, self.cursor_y + 1);
        self.syntax.invalidate_line(self.cursor_y);
        self.syntax.invalidate_line(self.cursor_y + 1);
        self.mark_dirty(self.cursor_y);
        self.mark_dirty(self.cursor_y + 1);
        self.cursor_y += 1;
        self.modified = true;
    }

    /// Indent selected lines or current line (Tab with selection).
    pub(crate) fn indent_lines(&mut self) {
        if let Some((ay, _ab)) = self.selection_anchor {
            self.snapshot();
            let ((sy, _), (ey, _)) = sel_range(self.cursor_y, self.cursor_byte, ay, 0);
            for row in sy..=ey.min(self.lines.len().saturating_sub(1)) {
                self.lines[row].insert_str(0, "    ");
                self.mark_dirty(row);
                self.syntax.invalidate_line(row);
            }
            self.modified = true;
        } else {
            self.insert_at(' ');
            self.insert_at(' ');
            self.insert_at(' ');
            self.insert_at(' ');
        }
    }

    /// Dedent selected lines or current line (Shift+Tab).
    pub(crate) fn dedent_lines(&mut self) {
        if let Some((ay, _ab)) = self.selection_anchor {
            self.snapshot();
            let ((sy, _), (ey, _)) = sel_range(self.cursor_y, self.cursor_byte, ay, 0);
            for row in sy..=ey.min(self.lines.len().saturating_sub(1)) {
                let spaces = self.lines[row]
                    .chars()
                    .take(4)
                    .filter(|&c| c == ' ')
                    .count();
                self.lines[row].drain(..spaces);
                self.mark_dirty(row);
                self.syntax.invalidate_line(row);
            }
            self.modified = true;
        } else {
            // Dedent current line
            let spaces = self.lines[self.cursor_y]
                .chars()
                .take(4)
                .filter(|&c| c == ' ')
                .count();
            if spaces > 0 {
                self.snapshot();
                self.lines[self.cursor_y].drain(..spaces);
                self.mark_dirty(self.cursor_y);
                self.syntax.invalidate_line(self.cursor_y);
                self.cursor_byte = self.cursor_byte.saturating_sub(spaces);
                self.modified = true;
            }
        }
    }
}
