use std::io::BufRead;
use std::path::Path;

use crate::editor::{Editor, Mode, Tab};

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
        if self.sidebar.open { self.sidebar.width + 1 } else { 0 }
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
        if !welcome_path.exists() {
            let content = crate::editor::template::generate_welcome_content();
            let _ = std::fs::write(&welcome_path, content);
        }
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
                self.open_scratch_tab();
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
        let content: Vec<String> = if path.exists() {
            let file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    self.status_msg = Some(format!("Cannot open {}: {}", path.display(), e));
                    return false;
                }
            };
            match std::io::BufReader::new(file).lines().collect::<std::io::Result<Vec<_>>>() {
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
        if self.editor_cfg.syntax_enabled && self.lines.len() <= 5000 {
            let lines = self.lines.clone();
            self.syntax.pre_tokenize_up_to(lines.len(), &lines);
        }
        self.mark_all_dirty();
        self.status_msg = Some(format!("Opened {}", path.display()));
        true
    }

    pub(crate) fn open_file(&mut self, path: &Path) {
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.path == path {
                self.switch_tab(i);
                return;
            }
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
            self.status_msg = Some(format!("Max tabs ({}) reached", self.editor_cfg.max_tabs));
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
}
