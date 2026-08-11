use crate::editor::Editor;
use crate::t;

impl Editor {
    pub(crate) fn push_to_system_clipboard(text: &str) {
        crate::shell::commands::clipboard::push_clip(text);
        #[cfg(not(target_os = "android"))]
        {
            if let Ok(mut cb) = arboard::Clipboard::new() {
                let _ = cb.set_text(text);
            }
        }
        #[cfg(target_os = "android")]
        {
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

    pub(crate) fn pull_from_system_clipboard() -> Option<String> {
        #[cfg(not(target_os = "android"))]
        {
            arboard::Clipboard::new()
                .ok()
                .and_then(|mut cb| cb.get_text().ok())
        }
        #[cfg(target_os = "android")]
        {
            std::process::Command::new("termux-clipboard-get")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .filter(|s| !s.is_empty())
        }
    }

    pub(crate) fn copy_selection(&mut self) {
        if !self.has_selection() {
            let text = self.current().to_string() + "\n";
            Self::push_to_system_clipboard(&text);
            self.clipboard = text;
            self.status_msg = Some(t!("editor.clip_copied_line"));
            return;
        }
        let text = self.selected_text();
        Self::push_to_system_clipboard(&text);
        self.clipboard = text.clone();
        self.status_msg = Some(t!("editor.clip_copied_chars", text.len()));
    }

    pub(crate) fn cut_selection(&mut self) {
        if !self.has_selection() {
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
            self.status_msg = Some(t!("editor.clip_cut_line"));
            return;
        }
        self.snapshot();
        let text = self.selected_text();
        Self::push_to_system_clipboard(&text);
        self.clipboard = text.clone();
        self.delete_selection();
        self.status_msg = Some(t!("editor.clip_cut_chars", text.len()));
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
        self.save_if_auto();
    }

    pub(crate) fn paste(&mut self) {
        let text = Self::pull_from_system_clipboard().unwrap_or_else(|| self.clipboard.clone());

        if text.is_empty() {
            self.status_msg = Some(t!("editor.clip_empty"));
            return;
        }
        self.snapshot();

        if self.has_selection() {
            self.delete_selection();
        }
        self.paste_fast(&text);
    }

    pub(crate) fn yank_line(&mut self) {
        let text = self.current().to_string() + "\n";
        Self::push_to_system_clipboard(&text);
        self.clipboard = text;
        self.status_msg = Some(t!("editor.clip_yanked"));
    }
}
