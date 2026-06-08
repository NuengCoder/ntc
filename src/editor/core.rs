use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;
use crossterm::style::Color;

use super::{
    byte_to_col,
    gutter_width, set_status_bar, prompt_yes_no, sel_range,
    Editor, Snapshot,
    MAX_UNDO, MAX_BUFFERS,
};

impl Editor {
    // ── multi-cursor helpers ────────────────────────────────────────────────

    pub(crate) fn has_multiple_cursors(&self) -> bool {
        !self.extra_cursors.is_empty()
    }

    pub(crate) fn clear_extra_cursors(&mut self) {
        if !self.extra_cursors.is_empty() {
            self.extra_cursors.clear();
            self.last_added_cursor_idx = None;
            self.mark_all_dirty();
        }
    }

    // ── sidebar / file helpers ──────────────────────────────────────────────

    pub(crate) fn editor_offset(&self) -> usize {
        if self.sidebar.open { super::SIDEBAR_WIDTH + 1 } else { 0 }
    }

    pub(crate) fn toggle_sidebar(&mut self) {
        self.sidebar.open = !self.sidebar.open;
        self.mark_all_dirty();
    }

    pub(crate) fn load_file(&mut self, path: &Path) {
        let content: Vec<String> = if path.exists() {
            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    self.status_msg = Some(format!("Cannot open: {}", e));
                    return;
                }
            };
            BufReader::new(file)
                .lines()
                .collect::<std::io::Result<Vec<_>>>()
                .unwrap_or_default()
        } else {
            vec![]
        };
        self.lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content
        };
        self.path = path.to_path_buf();
        self.modified = false;
        self.cursor_y = 0;
        self.cursor_byte = 0;
        self.scroll = 0;
        self.scroll_x = 0;
        self.selection_anchor = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.extra_cursors.clear();
        self.syntax
            .set_language(path.extension().and_then(|e| e.to_str()));
        self.syntax.resize_cache(self.lines.len());
        self.mark_all_dirty();
        self.status_msg = Some(format!("Opened {}", path.display()));
    }

    pub(crate) fn open_file(&mut self, path: &Path) {
        if path == self.path {
            return;
        }
        if self.buffer_stack.last() != Some(&self.path) {
            self.buffer_stack.push(self.path.clone());
            if self.buffer_stack.len() > MAX_BUFFERS {
                self.buffer_stack.remove(0);
            }
        }
        self.buffer_idx = self.buffer_stack.len() - 1;
        self.load_file(path);
    }

    pub(crate) fn next_buffer(&mut self) {
        if self.buffer_stack.len() <= 1 {
            return;
        }
        let next = (self.buffer_idx + 1) % self.buffer_stack.len();
        let path = self.buffer_stack[next].clone();
        self.load_file(&path);
        self.buffer_idx = next;
    }

    pub(crate) fn prev_buffer(&mut self) {
        if self.buffer_stack.len() <= 1 {
            return;
        }
        let prev = if self.buffer_idx == 0 {
            self.buffer_stack.len() - 1
        } else {
            self.buffer_idx - 1
        };
        let path = self.buffer_stack[prev].clone();
        self.load_file(&path);
        self.buffer_idx = prev;
    }

    pub(crate) fn current(&self) -> &str {
        &self.lines[self.cursor_y]
    }

    pub(crate) fn current_mut(&mut self) -> &mut String {
        &mut self.lines[self.cursor_y]
    }

    // ── snapshot / undo / redo ───────────────────────────────────────────────

    pub(crate) fn snapshot(&mut self) {
        let snap = Snapshot {
            lines: self.lines.clone(),
            cursor_y: self.cursor_y,
            cursor_byte: self.cursor_byte,
            extra_cursors: self.extra_cursors.clone(),
        };
        self.undo_stack.push(snap);
        if self.undo_stack.len() > MAX_UNDO {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub(crate) fn undo(&mut self) {
        if let Some(snap) = self.undo_stack.pop() {
            let current = Snapshot {
                lines: self.lines.clone(),
                cursor_y: self.cursor_y,
                cursor_byte: self.cursor_byte,
                extra_cursors: self.extra_cursors.clone(),
            };
            self.redo_stack.push(current);
            self.lines = snap.lines;
            self.cursor_y = snap.cursor_y;
            self.cursor_byte = snap.cursor_byte;
            self.selection_anchor = None;
            self.extra_cursors = snap.extra_cursors;
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
                cursor_y: self.cursor_y,
                cursor_byte: self.cursor_byte,
                extra_cursors: self.extra_cursors.clone(),
            };
            self.undo_stack.push(current);
            self.lines = snap.lines;
            self.cursor_y = snap.cursor_y;
            self.cursor_byte = snap.cursor_byte;
            self.selection_anchor = None;
            self.extra_cursors = snap.extra_cursors;
            self.modified = true;
            self.syntax.invalidate_all();
            self.syntax.resize_cache(self.lines.len());
            self.mark_all_dirty();
            self.status_msg = Some("Redo".into());
        } else {
            self.status_msg = Some("Nothing to redo".into());
        }
    }

    // ── cursor / scroll ──────────────────────────────────────────────────────

    pub(crate) fn clamp(&mut self) {
        let max = self.lines.len().saturating_sub(1);
        self.cursor_y = self.cursor_y.min(max);
        let line_len = self.current().len();
        self.cursor_byte = self.cursor_byte.min(line_len);
        // ensure cursor_byte is on a char boundary
        while self.cursor_byte > 0 && !self.current().is_char_boundary(self.cursor_byte) {
            self.cursor_byte -= 1;
        }
        // Clamp extra cursors too
        for c in &mut self.extra_cursors {
            c.y = c.y.min(max);
            let clen = self.lines[c.y].len();
            c.byte = c.byte.min(clen);
            while c.byte > 0 && !self.lines[c.y].is_char_boundary(c.byte) {
                c.byte -= 1;
            }
            // Also clamp anchor
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
        let rows = self.term_h.saturating_sub(3);
        self.dirty_start = 0;
        self.dirty_end = self.lines.len().max(rows);
    }

    /// Returns the maximum sensible horizontal scroll offset.
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
        let rows = self.term_h.saturating_sub(3); // -3: horizontal scroll bar + status + hint
        if self.cursor_y < self.scroll {
            self.scroll = self.cursor_y;
        } else if self.cursor_y >= self.scroll + rows {
            self.scroll = self.cursor_y.saturating_sub(rows) + 1;
        }

        // Horizontal scroll — only auto-scroll RIGHT when cursor exceeds the right edge
        let gw = gutter_width(self.lines.len());
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let cursor_col = byte_to_col(self.current(), self.cursor_byte);
        if cursor_col >= self.scroll_x + text_cols {
            self.scroll_x = cursor_col.saturating_sub(text_cols) + 1;
        }
        // Leftward scroll is left to explicit user action (Alt+Left / scroll bar click)
    }

    // ── selection ────────────────────────────────────────────────────────────

    pub(crate) fn clear_selection(&mut self) {
        if let Some((ay, _ab)) = self.selection_anchor {
            let (s, e) = if ay < self.cursor_y {
                (ay, self.cursor_y)
            } else {
                (self.cursor_y, ay)
            };
            for row in s..=e {
                self.mark_dirty(row);
            }
        }
        self.selection_anchor = None;
    }

    pub(crate) fn start_or_extend_selection(&mut self) {
        if self.selection_anchor.is_none() {
            if self.has_multiple_cursors() {
                self.clear_extra_cursors();
            }
            self.selection_anchor = Some((self.cursor_y, self.cursor_byte));
            self.mark_dirty(self.cursor_y);
        }
    }

    pub(crate) fn has_selection(&self) -> bool {
        if let Some((ay, ab)) = self.selection_anchor {
            (ay, ab) != (self.cursor_y, self.cursor_byte)
        } else {
            false
        }
    }

    /// Get the selected text as a String.
    pub(crate) fn selected_text(&self) -> String {
        let (ay, ab) = match self.selection_anchor {
            Some(a) => a,
            None => return String::new(),
        };
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor_y, self.cursor_byte, ay, ab);
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

    /// Delete the selected region; leaves cursor at selection start.
    pub(crate) fn delete_selection(&mut self) {
        let (ay, ab) = match self.selection_anchor {
            Some(a) => a,
            None => return,
        };
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor_y, self.cursor_byte, ay, ab);
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
        self.cursor_y = sy;
        self.cursor_byte = sb;
        self.selection_anchor = None;
        self.modified = true;
    }

    /// Push `text` to the system clipboard, silently ignoring any failure.
    fn push_to_system_clipboard(text: &str) {
        #[cfg(not(target_os = "android"))]
        {
            if let Ok(mut cb) = arboard::Clipboard::new() {
                let _ = cb.set_text(text);
            }
        }
        #[cfg(target_os = "android")]
        {
            // Termux: pipe through termux-clipboard-set
            use std::io::Write as _;
            let mut child = std::process::Command::new("termux-clipboard-set")
                .stdin(std::process::Stdio::piped())
                .spawn();
            if let Ok(ref mut c) = child {
                if let Some(ref mut stdin) = c.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                let _ = c.wait();
            }
        }
    }

    /// Pull text from the system clipboard; returns `None` on any failure.
    fn pull_from_system_clipboard() -> Option<String> {
        #[cfg(not(target_os = "android"))]
        {
            arboard::Clipboard::new()
                .ok()
                .and_then(|mut cb| cb.get_text().ok())
        }
        #[cfg(target_os = "android")]
        {
            // Termux: termux-clipboard-get
            std::process::Command::new("termux-clipboard-get")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .filter(|s| !s.is_empty())
        }
    }

    /// Copy selection to internal clipboard (+ system clipboard).
    pub(crate) fn copy_selection(&mut self) {
        if !self.has_selection() {
            // No selection: copy whole line (VSCode behaviour)
            let text = self.current().to_string() + "\n";
            Self::push_to_system_clipboard(&text);
            self.clipboard = text;
            self.status_msg = Some("Copied line".into());
            return;
        }
        let text = self.selected_text();
        Self::push_to_system_clipboard(&text);
        self.clipboard = text.clone();
        self.status_msg = Some(format!("Copied {} chars", text.len()));
    }

    pub(crate) fn cut_selection(&mut self) {
        if !self.has_selection() {
            // No selection: cut whole line
            self.snapshot();
            let text = self.current().to_string() + "\n";
            Self::push_to_system_clipboard(&text);
            self.clipboard = text;
            if self.lines.len() > 1 {
                self.lines.remove(self.cursor_y);
                self.cursor_y = self.cursor_y.min(self.lines.len().saturating_sub(1));
            } else {
                self.lines[0].clear();
                self.cursor_byte = 0;
            }
            self.mark_dirty(self.cursor_y);
            self.dirty_end = self.dirty_end.max(self.lines.len());
            self.cursor_byte = self.cursor_byte.min(self.current().len());
            self.modified = true;
            self.status_msg = Some("Cut line".into());
            return;
        }
        self.snapshot();
        let text = self.selected_text();
        Self::push_to_system_clipboard(&text);
        self.clipboard = text.clone();
        self.delete_selection();
        self.status_msg = Some(format!("Cut {} chars", text.len()));
    }

    pub(crate) fn paste_fast(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let byte = self.cursor_byte;
        let line_idx = self.cursor_y;

        let current = self.lines[line_idx].clone();

        let left = current[..byte].to_string();
        let right = current[byte..].to_string();

        // Normalize line endings: \r\n → \n, then strip lone \r
        let normalized = text.replace("\r\n", "\n").replace('\r', "");
        let parts: Vec<&str> = normalized.split('\n').collect();

        if parts.len() == 1 {
            self.lines[line_idx] = left + parts[0] + &right;
            self.mark_dirty(line_idx);
            self.syntax.invalidate_line(line_idx);
            self.cursor_byte = byte + parts[0].len();
        } else {
            self.lines[line_idx] = left + parts[0];
            self.mark_dirty(line_idx);
            self.syntax.invalidate_line(line_idx);

            let mut insert_pos = line_idx + 1;

            for middle in &parts[1..parts.len() - 1] {
                self.lines.insert(insert_pos, (*middle).to_string());
                self.syntax.insert_line(insert_pos);
                insert_pos += 1;
            }

            self.lines
                .insert(insert_pos, format!("{}{}", parts.last().unwrap(), right));
            self.syntax.insert_line(insert_pos);

            self.dirty_end = self.dirty_end.max(self.lines.len());
            self.cursor_y = insert_pos;
            self.cursor_byte = parts.last().unwrap().len();
        }

        self.modified = true;
    }

    pub(crate) fn paste(&mut self) {
        // Prefer system clipboard; fall back to internal clipboard.
        let text = Self::pull_from_system_clipboard().unwrap_or_else(|| self.clipboard.clone());

        if text.is_empty() {
            self.status_msg = Some("Clipboard is empty".into());
            return;
        }
        self.snapshot();

        if self.has_selection() {
            self.delete_selection();
        }
        self.paste_fast(&text);
    }

    // ── search ───────────────────────────────────────────────────────────────

    pub(crate) fn rebuild_search_matches(&mut self) {
        self.search_matches.clear();
        if self.search_query.is_empty() {
            self.mark_all_dirty();
            return;
        }
        let q = self.search_query.to_lowercase();
        for (li, line) in self.lines.iter().enumerate() {
            let lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = lower[start..].find(&q) {
                let abs = start + pos;
                self.search_matches.push((li, abs, abs + q.len()));
                start = abs + 1;
                if start >= lower.len() {
                    break;
                }
            }
        }
        self.mark_all_dirty();
    }

    pub(crate) fn jump_to_match(&mut self, idx: usize) {
        if self.search_matches.is_empty() {
            return;
        }
        let (ly, sb, _eb) = self.search_matches[idx];
        self.cursor_y = ly;
        self.cursor_byte = sb;
    }

    pub(crate) fn search_next(&mut self) {
        if self.search_matches.is_empty() {
            self.status_msg = Some(format!("Pattern not found: {}", self.search_query));
            return;
        }
        self.search_match_idx = (self.search_match_idx + 1) % self.search_matches.len();
        self.jump_to_match(self.search_match_idx);
    }

    pub(crate) fn search_prev(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.search_match_idx == 0 {
            self.search_match_idx = self.search_matches.len() - 1;
        } else {
            self.search_match_idx -= 1;
        }
        self.jump_to_match(self.search_match_idx);
    }

    // ── select all / line operations ─────────────────────────────────────────

    pub(crate) fn select_all(&mut self) {
        self.selection_anchor = Some((0, 0));
        self.cursor_y = self.lines.len().saturating_sub(1);
        self.cursor_byte = self.lines[self.cursor_y].len();
        self.mark_all_dirty();
        self.status_msg = Some("Selected all".into());
    }

    pub(crate) fn select_line(&mut self) {
        self.clear_extra_cursors();
        self.selection_anchor = Some((self.cursor_y, 0));
        self.cursor_byte = self.current().len();
        self.mark_dirty(self.cursor_y);
        self.status_msg = Some("Line selected".into());
    }

    pub(crate) fn yank_line(&mut self) {
        let text = self.current().to_string() + "\n";
        Self::push_to_system_clipboard(&text);
        self.clipboard = text;
        self.status_msg = Some("Line yanked".into());
    }

    pub(crate) fn select_word_under_cursor(&mut self) -> Option<String> {
        let line = self.current();
        let byte = self.cursor_byte;
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
        self.cursor_byte = end;
        self.selection_anchor = Some((self.cursor_y, start));
        Some(word)
    }

    // ── save / exit ─────────────────────────────────────────────────────────

    pub(crate) fn save(&mut self) -> Result<()> {
        if let Some(p) = self.path.parent() {
            std::fs::create_dir_all(p)?;
        }
        let mut out = String::new();
        for (i, l) in self.lines.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str(l);
        }
        std::fs::write(&self.path, out)?;
        self.modified = false;
        Ok(())
    }

    pub(crate) fn exit_flow(&mut self, stdout: &mut std::io::Stdout) -> Result<bool> {
        if self.editor_cfg.auto_save && self.modified {
            if let Err(e) = self.save() {
                let _ = set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                return Ok(false);
            }
            return Ok(true);
        }
        if !self.modified {
            return Ok(true);
        }
        loop {
            match prompt_yes_no(
                stdout,
                Color::Rgb {
                    r: 160,
                    g: 100,
                    b: 20,
                },
                "Save file [Y|N]?",
            )? {
                Some(true) => {
                    if let Err(e) = self.save() {
                        let _ =
                            set_status_bar(stdout, Color::Red, Color::White, &format!(" {}", e));
                        continue;
                    }
                    let _ = set_status_bar(stdout, Color::Green, Color::White, " Saved ✓");
                    return Ok(true);
                }
                Some(false) => return Ok(true),
                None => return Ok(false),
            }
        }
    }
}