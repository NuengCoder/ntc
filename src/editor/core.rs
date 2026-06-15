use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;
use crossterm::style::Color;

use super::{
    byte_to_col, auto_pair,
    gutter_width, set_status_bar, prompt_yes_no, sel_range,
    Editor, Snapshot, Tab,
    MAX_UNDO, MAX_BUFFERS, MAX_TABS,
};

impl Editor {
    // ── multi-cursor helpers ────────────────────────────────────────────────

    pub(crate) fn has_multiple_cursors(&self) -> bool {
        !self.cursor.extra_cursors.is_empty()
    }

    pub(crate) fn clear_extra_cursors(&mut self) {
        if !self.cursor.extra_cursors.is_empty() {
            self.cursor.extra_cursors.clear();
            self.cursor.last_added_cursor_idx = None;
            self.mark_all_dirty();
        }
    }

    // ── sidebar / file helpers ──────────────────────────────────────────────

    pub(crate) fn editor_offset(&self) -> usize {
        if self.sidebar.open { self.sidebar.width + 1 } else { 0 }
    }

    pub(crate) fn toggle_sidebar(&mut self) {
        self.sidebar.open = !self.sidebar.open;
        self.mark_all_dirty();
    }

    /// Save the current editor state into the active tab slot.
    pub(crate) fn save_active_tab(&mut self) {
        if self.active_tab < self.tabs.len() {
            self.tabs[self.active_tab] = Tab::from_editor(self);
        }
    }

    /// Restore editor state from a tab slot.
    pub(crate) fn restore_tab(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            let tab = self.tabs[idx].clone();
            tab.apply_to(self);
        }
    }

    /// Switch to a different tab by index.
    pub(crate) fn switch_tab(&mut self, idx: usize) {
        if idx == self.active_tab || idx >= self.tabs.len() {
            return;
        }
        self.save_active_tab();
        self.active_tab = idx;
        self.restore_tab(idx);
        self.mark_all_dirty();
    }

    /// Switch to the next tab (cycle forward).
    pub(crate) fn next_tab(&mut self) {
        if self.tabs.len() <= 1 {
            return;
        }
        let next = (self.active_tab + 1) % self.tabs.len();
        self.switch_tab(next);
    }

    /// Switch to the previous tab (cycle backward).
    pub(crate) fn prev_tab(&mut self) {
        if self.tabs.len() <= 1 {
            return;
        }
        let prev = if self.active_tab == 0 {
            self.tabs.len() - 1
        } else {
            self.active_tab - 1
        };
        self.switch_tab(prev);
    }

    /// Check if a path is a scratch/welcome buffer.
    pub(crate) fn is_scratch_path(&self, path: &std::path::Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == ".scratch" || n == "scratch" || n == "welcome.ntcEditor")
            .unwrap_or(false)
    }

    /// Create a welcome tab: opens a welcome.ntcEditor file in the current directory.
    pub(crate) fn open_scratch_tab(&mut self) {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let welcome_path = cwd.join("welcome.ntcEditor");
        if !welcome_path.exists() {
            let content = super::template::generate_welcome_content();
            let _ = std::fs::write(&welcome_path, content);
        }
        // If welcome.ntcEditor is already open in some tab, switch to it
        for (i, tab) in self.tabs.iter().enumerate() {
            if self.is_scratch_path(&tab.path) {
                self.switch_tab(i);
                return;
            }
        }
        self.open_file(&welcome_path);
    }

    /// Close a specific tab by index.
    /// Returns true if the editor should quit (last scratch tab closed).
    pub(crate) fn close_tab(&mut self, idx: usize) -> bool {
        if idx >= self.tabs.len() {
            return false;
        }
        if idx == self.active_tab {
            // Closing the active tab
            if self.tabs.len() <= 1 {
                // Last tab — if scratch, quit; otherwise create scratch then quit
                if self.is_scratch_path(&self.path) {
                    return true;
                }
                self.open_scratch_tab();
                // Now there are 2+ tabs. Remove old one.
                self.tabs.remove(0);
                self.active_tab = 0;
                self.restore_tab(0);
                self.mark_all_dirty();
                return false;
            }
            if self.modified {
                let _ = self.save();
            }
            self.tabs.remove(idx);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
            self.restore_tab(self.active_tab);
            self.mark_all_dirty();
            false
        } else {
            // Closing an inactive tab — just remove it
            if idx < self.active_tab {
                self.active_tab -= 1;
            }
            self.tabs.remove(idx);
            // No editor state change needed
            false
        }
    }

    /// Close the current tab.
    /// Returns true if the editor should quit.
    pub(crate) fn close_current_tab(&mut self) -> bool {
        self.close_tab(self.active_tab)
    }

    pub(crate) fn load_file(&mut self, path: &Path) -> bool {
        let content: Vec<String> = if path.exists() {
            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    self.status_msg = Some(format!("Cannot open {}: {}", path.display(), e));
                    return false;
                }
            };
            match BufReader::new(file).lines().collect::<std::io::Result<Vec<_>>>() {
                Ok(content) => content,
                Err(e) => {
                    self.status_msg = Some(format!("Error reading {}: {}", path.display(), e));
                    return false;
                }
            }
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
        self.cursor.y = 0;
        self.cursor.byte = 0;
        self.sv.offset = 0;
        self.sv.x = 0;
        self.cursor.selection_anchor = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.cursor.extra_cursors.clear();
        self.syntax
            .set_language(path.extension().and_then(|e| e.to_str()));
        self.syntax.resize_cache(self.lines.len());
        self.mark_all_dirty();
        self.status_msg = Some(format!("Opened {}", path.display()));
        true
    }

    /// Open a file in a new tab (or switch to existing tab if already open).
    pub(crate) fn open_file(&mut self, path: &Path) {
        // Check if the file is already open in another tab
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.path == path {
                self.switch_tab(i);
                return;
            }
        }

        // Save current tab state
        self.save_active_tab();

        // Remember current file in buffer stack (before loading new file)
        if self.buffer_stack.last() != Some(&self.path) {
            self.buffer_stack.push(self.path.clone());
            if self.buffer_stack.len() > MAX_BUFFERS {
                self.buffer_stack.remove(0);
            }
        }
        self.buffer_idx = self.buffer_stack.len() - 1;

        // Load the new file into current editor fields
        if !self.load_file(path) {
            // Restore the previous buffer idx since load failed
            self.buffer_stack.pop();
            self.buffer_idx = self.buffer_stack.len().saturating_sub(1);
            return;
        }

        // Register as a new tab (cap at MAX_TABS)
        if self.tabs.len() >= MAX_TABS {
            self.status_msg = Some(format!("Max tabs ({}) reached", MAX_TABS));
        } else {
            self.tabs.push(Tab::from_editor(self));
            self.active_tab = self.tabs.len() - 1;
        }
    }

    pub(crate) fn next_buffer(&mut self) {
        if self.buffer_stack.len() <= 1 {
            return;
        }
        let next = (self.buffer_idx + 1) % self.buffer_stack.len();
        let path = self.buffer_stack[next].clone();
        self.save_active_tab();
        if self.load_file(&path) {
            self.buffer_idx = next;
        }
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
        self.save_active_tab();
        if self.load_file(&path) {
            self.buffer_idx = prev;
        }
    }

    pub(crate) fn current(&self) -> &str {
        &self.lines[self.cursor.y]
    }

    pub(crate) fn current_mut(&mut self) -> &mut String {
        &mut self.lines[self.cursor.y]
    }

    // ── snapshot / undo / redo ───────────────────────────────────────────────

    pub(crate) fn snapshot(&mut self) {
        let snap = Snapshot {
            lines: self.lines.clone(),
            cursor_y: self.cursor.y,
            cursor_byte: self.cursor.byte,
            extra_cursors: self.cursor.extra_cursors.clone(),
        };
        self.undo_stack.push(snap);
        if self.undo_stack.len() > MAX_UNDO {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();

        // Periodically save crash recovery snapshot
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

    // ── cursor / scroll ──────────────────────────────────────────────────────

    pub(crate) fn clamp(&mut self) {
        let max = self.lines.len().saturating_sub(1);
        self.cursor.y = self.cursor.y.min(max);
        let line_len = self.current().len();
        self.cursor.byte = self.cursor.byte.min(line_len);
        // ensure cursor_byte is on a char boundary
        while self.cursor.byte > 0 && !self.current().is_char_boundary(self.cursor.byte) {
            self.cursor.byte -= 1;
        }
        // Clamp extra cursors too
        for c in &mut self.cursor.extra_cursors {
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
        let rows = self.term_h.saturating_sub(4);
        self.dirty_start = self.sv.offset;
        self.dirty_end = self.sv.offset + rows;
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
        let rows = self.term_h.saturating_sub(5); // tab bar + hscroll + cwd + status + hint
        // Only scroll when cursor goes outside visible area
        if self.cursor.y < self.sv.offset {
            self.sv.offset = self.cursor.y;
        } else if self.cursor.y >= self.sv.offset + rows {
            self.sv.offset = self.cursor.y.saturating_sub(rows) + 1;
        }

        // Horizontal scroll — only auto-scroll RIGHT when cursor exceeds the right edge
        let gw = gutter_width(self.lines.len());
        let eo = self.editor_offset();
        let editor_cols = self.term_w.saturating_sub(eo);
        let text_cols = editor_cols.saturating_sub(gw + 2);
        let cursor_col = byte_to_col(self.current(), self.cursor.byte);
        if cursor_col >= self.sv.x + text_cols {
            self.sv.x = cursor_col.saturating_sub(text_cols) + 1;
        }
        // Leftward scroll is left to explicit user action (Alt+Left / scroll bar click)
    }

    // ── selection ────────────────────────────────────────────────────────────

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

    /// Get the selected text as a String.
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

    /// Delete the selected region; leaves cursor at selection start.
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
                self.lines.remove(self.cursor.y);
                self.cursor.y = self.cursor.y.min(self.lines.len().saturating_sub(1));
            } else {
                self.lines[0].clear();
                self.cursor.byte = 0;
            }
            self.mark_dirty(self.cursor.y);
            self.dirty_end = self.dirty_end.max(self.lines.len());
            self.cursor.byte = self.cursor.byte.min(self.current().len());
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

        let byte = self.cursor.byte;
        let line_idx = self.cursor.y;

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
            self.cursor.byte = byte + parts[0].len();
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

            let last_part = parts.last().copied().unwrap_or("");
            self.lines
                .insert(insert_pos, format!("{}{}", last_part, right));
            self.syntax.insert_line(insert_pos);

            self.dirty_end = self.dirty_end.max(self.lines.len());
            self.cursor.y = insert_pos;
            self.cursor.byte = last_part.len();
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
        self.search.matches.clear();
        if self.search.query.is_empty() {
            self.mark_all_dirty();
            return;
        }
        let q = self.search.query.to_lowercase();
        for (li, line) in self.lines.iter().enumerate() {
            let lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = lower[start..].find(&q) {
                let abs = start + pos;
                self.search.matches.push((li, abs, abs + q.len()));
                start = abs + 1;
                if start >= lower.len() {
                    break;
                }
            }
        }
        self.mark_all_dirty();
    }

    pub(crate) fn jump_to_match(&mut self, idx: usize) {
        if self.search.matches.is_empty() {
            return;
        }
        let (ly, sb, _eb) = self.search.matches[idx];
        self.cursor.y = ly;
        self.cursor.byte = sb;
    }

    pub(crate) fn search_next(&mut self) {
        if self.search.matches.is_empty() {
            self.status_msg = Some(format!("Pattern not found: {}", self.search.query));
            return;
        }
        self.search.match_idx = (self.search.match_idx + 1) % self.search.matches.len();
        self.jump_to_match(self.search.match_idx);
    }

    pub(crate) fn search_prev(&mut self) {
        if self.search.matches.is_empty() {
            self.status_msg = Some(format!("Pattern not found: {}", self.search.query));
            return;
        }
        if self.search.match_idx == 0 {
            self.search.match_idx = self.search.matches.len() - 1;
        } else {
            self.search.match_idx -= 1;
        }
        self.jump_to_match(self.search.match_idx);
    }

    // ── common character input with auto-pair ────────────────────────────────

    /// Handle a regular character press, applying auto-pair and triggering
    /// completion for .ntc.math files. Shared between shifted and non-shifted
    /// insert paths to eliminate duplicate code.
    pub(crate) fn handle_char_input(&mut self, c: char) {
        if self.has_multiple_cursors() {
            self.multi_insert_at(c);
        } else {
            self.snapshot();
            if let Some(closing) = auto_pair(c) {
                self.insert_at(c);
                self.insert_at_raw(closing);
                self.cursor.byte = self.cursor.byte.saturating_sub(closing.len_utf8());
            } else {
                self.insert_at(c);
            }
        }
        if self.syntax.language == Some(crate::syntax::SyntaxLanguage::NtcMath) {
            self.trigger_completion();
        }
    }

    // ── select all / line operations ─────────────────────────────────────────

    pub(crate) fn select_all(&mut self) {
        self.cursor.selection_anchor = Some((0, 0));
        self.cursor.y = self.lines.len().saturating_sub(1);
        self.cursor.byte = self.lines[self.cursor.y].len();
        self.mark_all_dirty();
        self.status_msg = Some("Selected all".into());
    }

    pub(crate) fn select_line(&mut self) {
        self.clear_extra_cursors();
        self.cursor.selection_anchor = Some((self.cursor.y, 0));
        self.cursor.byte = self.current().len();
        self.mark_dirty(self.cursor.y);
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
            self.clear_recovery_snapshot();
            return Ok(true);
        }
        if !self.modified {
            self.clear_recovery_snapshot();
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
                    self.clear_recovery_snapshot();
                    return Ok(true);
                }
                Some(false) => {
                    self.clear_recovery_snapshot();
                    return Ok(true);
                }
                None => return Ok(false),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::CursorPos;

    fn test_editor(content: &[&str]) -> Editor {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_editor.txt");
        let lines: Vec<String> = content.iter().map(|s| s.to_string()).collect();
        std::fs::write(&path, lines.join("\n")).unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines = lines.clone();
        if lines.is_empty() || lines[0].is_empty() {
            editor.lines = lines;
        }
        editor
    }

    #[test]
    fn test_snapshot_undo_redo() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.snapshot();
        editor.lines[0] = "modified".to_string();
        assert_eq!(editor.lines[0], "modified");

        editor.undo();
        assert_eq!(editor.lines[0], "line1");

        editor.redo();
        assert_eq!(editor.lines[0], "modified");
    }

    #[test]
    fn test_undo_empty_stack() {
        let mut editor = test_editor(&["line1"]);
        // No snapshot taken yet
        let before = editor.lines[0].clone();
        editor.undo();
        assert_eq!(editor.lines[0], before);
    }

    #[test]
    fn test_redo_cleared_on_new_snapshot() {
        let mut editor = test_editor(&["line1"]);
        editor.snapshot();
        editor.lines[0] = "change1".to_string();
        editor.undo();
        assert_eq!(editor.lines[0], "line1");

        // New edit should clear redo
        editor.snapshot();
        editor.lines[0] = "change2".to_string();
        editor.redo();
        // Redo should do nothing since redo was cleared
        assert_eq!(editor.lines[0], "change2");
    }

    #[test]
    fn test_max_undo_capped() {
        let mut editor = test_editor(&["line"]);
        for i in 0..MAX_UNDO + 50 {
            editor.snapshot();
            editor.lines[0] = format!("change{}", i);
        }
        // Undo stack should be at most MAX_UNDO
        assert!(editor.undo_stack.len() <= MAX_UNDO);
    }

    #[test]
    fn test_has_selection_no_anchor() {
        let editor = test_editor(&["hello world"]);
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_has_selection_with_anchor() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert!(editor.has_selection());
    }

    #[test]
    fn test_has_selection_identical_positions() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 0;
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_selected_text() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert_eq!(editor.selected_text(), "hello");
    }

    #[test]
    fn test_selected_text_multi_line() {
        let mut editor = test_editor(&["hello", "world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.y = 1;
        editor.cursor.byte = 3;
        assert_eq!(editor.selected_text(), "hello\nwor");
    }

    #[test]
    fn test_select_all() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.select_all();
        assert!(editor.has_selection());
        let (ay, _) = editor.cursor.selection_anchor.unwrap();
        assert_eq!(ay, 0);
        assert_eq!(editor.cursor.y, 2);
    }

    #[test]
    fn test_delete_selection_single_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.modified = true;
        editor.delete_selection();
        assert_eq!(editor.lines[0], " world");
    }

    #[test]
    fn test_delete_selection_multi_line() {
        let mut editor = test_editor(&["hello", "cruel", "world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.y = 2;
        editor.cursor.byte = 3; // Select from start of file to "wor" on line 2
        editor.modified = true;
        editor.delete_selection();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "ld"); // remaining: "ld" from "world"[3..]
    }

    #[test]
    fn test_clamp_cursor() {
        let mut editor = test_editor(&["short"]);
        editor.cursor.y = 100;
        editor.cursor.byte = 100;
        editor.clamp();
        assert_eq!(editor.cursor.y, 0);
        assert_eq!(editor.cursor.byte, 5); // "short".len()
    }

    #[test]
    fn test_yank_line() {
        let mut editor = test_editor(&["yank me"]);
        editor.yank_line();
        assert!(editor.clipboard.contains("yank me"));
    }

    #[test]
    fn test_copy_selection_no_selection() {
        let mut editor = test_editor(&["copy line"]);
        editor.copy_selection();
        assert!(editor.clipboard.contains("copy line"));
    }

    #[test]
    fn test_copy_selection_with_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.copy_selection();
        assert_eq!(editor.clipboard, "hello");
    }

    #[test]
    fn test_paste_fast_single_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 5;
        editor.paste_fast(" beautiful");
        assert_eq!(editor.lines[0], "hello beautiful world");
    }

    #[test]
    fn test_paste_fast_multi_line() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 5;
        editor.paste_fast("line1\nline2\nline3");
        assert_eq!(editor.lines[0], "helloline1");
        assert_eq!(editor.lines[1], "line2");
        assert_eq!(editor.lines[2], "line3 world");
    }

    #[test]
    fn test_paste_fast_empty() {
        let mut editor = test_editor(&["hello"]);
        editor.paste_fast("");
        assert_eq!(editor.lines[0], "hello");
    }

    #[test]
    fn test_select_word_under_cursor() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.byte = 6; // at 'w' in "world"
        let word = editor.select_word_under_cursor();
        assert_eq!(word, Some("world".to_string()));
    }

    #[test]
    fn test_clear_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        assert!(editor.has_selection());
        editor.clear_selection();
        assert!(!editor.has_selection());
    }

    #[test]
    fn test_clear_extra_cursors() {
        let mut editor = test_editor(&["line1", "line2", "line3"]);
        editor.cursor.extra_cursors.push(CursorPos {
            y: 1,
            byte: 0,
            anchor: None,
        });
        assert!(editor.has_multiple_cursors());
        editor.clear_extra_cursors();
        assert!(!editor.has_multiple_cursors());
    }

    #[test]
    fn test_cut_selection() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cursor.byte = 5;
        editor.cut_selection();
        assert_eq!(editor.lines[0], " world");
    }
}