use crate::editor::{Editor, Snapshot};

impl Editor {
    pub(crate) fn snapshot(&mut self) {
        let snap = Snapshot {
            lines: self.lines.clone(),
            cursor_y: self.cursor.y,
            cursor_byte: self.cursor.byte,
            extra_cursors: self.cursor.extra_cursors.clone(),
        };
        self.undo_stack.push(snap);
        if self.undo_stack.len() > self.editor_cfg.undo_depth {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();

        self.search.mark_dirty();

        self.edit_count += 1;
        if self.edit_count.is_multiple_of(30) {
            self.save_recovery_snapshot();
        }
    }

    pub(crate) fn undo(&mut self) {
        if let Some(snap) = self.undo_stack.pop() {
            let current = Snapshot {
                lines: self.lines.clone(),
                cursor_y: self.cursor.y,
                cursor_byte: self.cursor.byte,
                extra_cursors: self.cursor.extra_cursors.clone(),
            };
            self.redo_stack.push(current);
            self.lines = snap.lines;
            self.cursor.y = snap.cursor_y;
            self.cursor.byte = snap.cursor_byte;
            self.cursor.selection_anchor = None;
            self.cursor.extra_cursors = snap.extra_cursors;
            self.modified = true;
            self.syntax.invalidate_all();
            self.syntax.resize_cache(self.lines.len());
            self.mark_all_dirty();
            self.status_msg = Some("Undo".into());
        } else {
            self.status_msg = Some("Nothing to undo".into());
        }
    }

    pub(crate) fn redo(&mut self) {
        if let Some(snap) = self.redo_stack.pop() {
            let current = Snapshot {
                lines: self.lines.clone(),
                cursor_y: self.cursor.y,
                cursor_byte: self.cursor.byte,
                extra_cursors: self.cursor.extra_cursors.clone(),
            };
            self.undo_stack.push(current);
            self.lines = snap.lines;
            self.cursor.y = snap.cursor_y;
            self.cursor.byte = snap.cursor_byte;
            self.cursor.selection_anchor = None;
            self.cursor.extra_cursors = snap.extra_cursors;
            self.modified = true;
            self.syntax.invalidate_all();
            self.syntax.resize_cache(self.lines.len());
            self.mark_all_dirty();
            self.status_msg = Some("Redo".into());
        } else {
            self.status_msg = Some("Nothing to redo".into());
        }
    }
}
