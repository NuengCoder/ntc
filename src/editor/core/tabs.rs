use std::path::Path;

use anyhow::Context;

use crate::editor::{save_lines_to, Editor, EditorEncoding, Mode, Tab};
use crate::t;

/// Split decoded text into editor lines (one String per logical line; a
/// trailing newline yields a final empty line, and an empty buffer becomes a
/// single empty line).
fn content_to_lines(content: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in content.lines() {
        lines.push(line.to_string());
    }
    if content.ends_with('\n') {
        lines.push(String::new());
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Decode raw file bytes using an explicit encoding. The encoding's own BOM is
/// skipped when present so reloading a file with the same encoding is stable.
pub(crate) fn decode_bytes_to_lines(bytes: &[u8], enc: EditorEncoding) -> Vec<String> {
    let bom = enc.bom();
    let body = if !bom.is_empty() && bytes.starts_with(bom) {
        &bytes[bom.len()..]
    } else {
        bytes
    };
    let content = match enc {
        EditorEncoding::Utf8 => encoding_rs::UTF_8
            .decode_without_bom_handling(body)
            .0
            .into_owned(),
        EditorEncoding::Utf16Le => encoding_rs::UTF_16LE
            .decode_without_bom_handling(body)
            .0
            .into_owned(),
        EditorEncoding::Utf16Be => encoding_rs::UTF_16BE
            .decode_without_bom_handling(body)
            .0
            .into_owned(),
        EditorEncoding::Windows1252 => encoding_rs::WINDOWS_1252
            .decode_without_bom_handling(body)
            .0
            .into_owned(),
    };
    content_to_lines(&content)
}

/// Read a file into lines, decoding according to its byte order mark (falling
/// back to UTF-8 and then Windows-1252). Returns the lines plus the detected
/// encoding so the buffer can round-trip bytes losslessly on save.
pub(crate) fn read_file_lines(path: &Path) -> anyhow::Result<(Vec<String>, EditorEncoding)> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("{} {}", t!("editor.tab_cannot_open"), path.display()))?;
    if bytes.is_empty() {
        return Ok((vec![String::new()], EditorEncoding::Utf8));
    }
    let (skip, enc) = if bytes.starts_with(b"\xef\xbb\xbf") {
        (3, EditorEncoding::Utf8)
    } else if bytes.starts_with(b"\xff\xfe") {
        (2, EditorEncoding::Utf16Le)
    } else if bytes.starts_with(b"\xfe\xff") {
        (2, EditorEncoding::Utf16Be)
    } else {
        let (_, had_errors) = encoding_rs::UTF_8.decode_without_bom_handling(&bytes);
        if had_errors {
            (0, EditorEncoding::Windows1252)
        } else {
            (0, EditorEncoding::Utf8)
        }
    };
    Ok((decode_bytes_to_lines(&bytes[skip..], enc), enc))
}

impl Editor {
    /// Re-read the on-disk file with the active encoding and replace the
    /// buffer. Used when the user confirms an encoding change on a clean file.
    pub(crate) fn reload_with_encoding(&mut self) -> anyhow::Result<()> {
        if !self.path.exists() {
            return Ok(());
        }
        let bytes = std::fs::read(&self.path)?;
        self.lines = decode_bytes_to_lines(&bytes, self.encoding);
        self.modified = false;
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.cursor.selection_anchor = None;
        self.cursor.extra_cursors.clear();
        self.syntax.resize_cache(self.lines.len());
        self.syntax.invalidate_all();
        self.clamp();
        self.mark_all_dirty();
        Ok(())
    }
}

impl Editor {
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

    pub(crate) fn editor_offset(&self) -> usize {
        if self.sidebar.open {
            self.sidebar_cols() + 1
        } else {
            0
        }
    }

    /// Effective sidebar width in columns: percent of terminal width
    /// when configured (`nfe:size N%`), else the legacy 30 columns.
    pub(crate) fn sidebar_cols(&self) -> usize {
        match self.editor_cfg.sidebar_width_pct {
            Some(p) => {
                let cols = self.term_w * p.clamp(1, 100) as usize / 100;
                cols.clamp(8, self.term_w.saturating_sub(20).max(8))
            }
            None => crate::editor::DEFAULT_SIDEBAR_WIDTH,
        }
    }

    pub(crate) fn toggle_sidebar(&mut self) {
        self.sidebar.open = !self.sidebar.open;
        self.mark_all_dirty();
    }

    pub(crate) fn save_active_tab(&mut self) {
        if self.active_tab < self.tabs.len() {
            self.tabs[self.active_tab] = Tab::from_editor(self);
        }
    }

    pub(crate) fn restore_tab(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            let tab = self.tabs[idx].clone();
            tab.apply_to(self);
        }
    }

    pub(crate) fn switch_tab(&mut self, idx: usize) {
        if idx == self.active_tab || idx >= self.tabs.len() {
            return;
        }
        if self.mode == Mode::InsertMulti {
            self.clear_extra_cursors();
            self.mode = Mode::Insert;
        }
        self.save_active_tab();
        self.active_tab = idx;
        self.restore_tab(idx);
        // Different file on screen: re-arm watcher + fingerprint, then
        // pick up any disk changes that landed while this tab was in
        // the background.
        self.restart_file_watcher();
        self.check_file_changed();
        self.mark_all_dirty();
    }

    pub(crate) fn next_tab(&mut self) {
        if self.tabs.len() <= 1 {
            return;
        }
        let next = (self.active_tab + 1) % self.tabs.len();
        self.switch_tab(next);
    }

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

    pub(crate) fn is_scratch_path(&self, path: &std::path::Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == ".scratch" || n == "scratch" || n == "welcome.ntcEditor")
            .unwrap_or(false)
    }

    pub(crate) fn open_scratch_tab(&mut self) {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let welcome_path = cwd.join("welcome.ntcEditor");
        // Create when missing, refresh when generated-but-stale; a
        // user-edited copy never matches and is left alone.
        let _ = crate::editor::template::refresh_welcome_file(&welcome_path);
        for (i, tab) in self.tabs.iter().enumerate() {
            if self.is_scratch_path(&tab.path) {
                self.switch_tab(i);
                return;
            }
        }
        self.open_file(&welcome_path);
    }

    pub(crate) fn close_tab(&mut self, idx: usize) -> bool {
        if idx >= self.tabs.len() {
            return false;
        }
        if idx == self.active_tab {
            if self.tabs.len() <= 1 {
                if self.is_scratch_path(&self.path) {
                    return true;
                }
                // Last real tab: never discard unsaved work. Save (or abort the
                // close on failure) before replacing it with the scratch tab.
                if self.modified {
                    if let Err(e) = self.save() {
                        self.status_msg = Some(format!("{} {}", t!("editor.save_failed"), e));
                        return false;
                    }
                    self.clear_recovery_snapshot();
                }
                self.open_scratch_tab();
                self.tabs.remove(0);
                self.active_tab = 0;
                self.restore_tab(0);
                self.mark_all_dirty();
                return false;
            }
            if self.modified {
                if let Err(e) = self.save() {
                    self.status_msg = Some(format!("{} {}", t!("editor.save_failed"), e));
                    return false;
                }
                self.clear_recovery_snapshot();
            }
            self.tabs.remove(idx);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
            self.restore_tab(self.active_tab);
            self.mark_all_dirty();
            false
        } else {
            // Closing a background tab: persist any unsaved edits first.
            if self.tabs[idx].modified && !self.is_scratch_path(&self.tabs[idx].path) {
                let path = self.tabs[idx].path.clone();
                let lines = self.tabs[idx].lines.clone();
                let encoding = self.tabs[idx].encoding;
                if let Err(e) = save_lines_to(&path, &lines, encoding) {
                    self.status_msg = Some(format!("{} {}", t!("editor.save_failed"), e));
                    return false;
                }
            }
            if idx < self.active_tab {
                self.active_tab -= 1;
            }
            self.tabs.remove(idx);
            false
        }
    }

    pub(crate) fn close_current_tab(&mut self) -> bool {
        self.close_tab(self.active_tab)
    }

    pub(crate) fn load_file(&mut self, path: &Path) -> bool {
        let (content, enc) = if path.exists() {
            match read_file_lines(path) {
                Ok((content, enc)) => (content, enc),
                Err(e) => {
                    self.status_msg = Some(format!(
                        "{} {}: {}",
                        t!("editor.tab_error_reading"),
                        path.display(),
                        e
                    ));
                    return false;
                }
            }
        } else {
            (vec![], EditorEncoding::Utf8)
        };
        self.lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content
        };
        self.path = path.to_path_buf();
        self.encoding = enc;
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
        if self.editor_cfg.syntax_enabled && self.lines.len() <= 5000 {
            let lines = self.lines.clone();
            self.syntax.pre_tokenize_up_to(lines.len(), &lines);
        }
        self.mark_all_dirty();
        self.status_msg = Some(format!("{} {}", t!("editor.tab_opened"), path.display()));
        // The open file changed: re-arm the external-change watcher and
        // baseline the on-disk fingerprint for the new file.
        self.restart_file_watcher();
        true
    }

    pub(crate) fn open_file(&mut self, path: &Path) {
        if let Some(i) = self.find_tab_by_path(path) {
            self.switch_tab(i);
            return;
        }

        self.save_active_tab();

        if self.buffer_stack.last() != Some(&self.path) {
            self.buffer_stack.push(self.path.clone());
            if self.buffer_stack.len() > self.editor_cfg.max_buffers {
                self.buffer_stack.remove(0);
            }
        }
        self.buffer_idx = self.buffer_stack.len() - 1;

        if !self.load_file(path) {
            self.buffer_stack.pop();
            self.buffer_idx = self.buffer_stack.len().saturating_sub(1);
            return;
        }

        if self.tabs.len() >= self.editor_cfg.max_tabs {
            self.status_msg = Some(format!(
                "{} ({})",
                t!("editor.tab_max_reached"),
                self.editor_cfg.max_tabs
            ));
        } else {
            self.tabs.push(Tab::from_editor(self));
            self.active_tab = self.tabs.len() - 1;
        }
    }

    pub(crate) fn current(&self) -> &str {
        &self.lines[self.cursor.y]
    }

    pub(crate) fn current_mut(&mut self) -> &mut String {
        &mut self.lines[self.cursor.y]
    }
}
