use anyhow::Result;
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use crossterm::style::Color;

use super::{
    prev_char_byte, next_char_byte, prev_word_byte, next_word_byte, auto_pair,
    set_status_bar, sel_range,
    Editor, Mode,
};
use crate::navigator::Navigator;
use crate::search::search_files;
use crate::teleport::TeleportManager;

impl Editor {
    // ── search mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_search_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                match code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.status_msg = Some(format!(
                            "Search: {} ({} matches)",
                            self.search_query,
                            self.search_matches.len()
                        ));
                    }
                    KeyCode::Enter => {
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            self.search_prev();
                        } else {
                            self.search_next();
                        }
                    }
                    KeyCode::Tab => self.search_next(),
                    KeyCode::BackTab => self.search_prev(),
                    KeyCode::Backspace => {
                        self.search_query.pop();
                        self.rebuild_search_matches();
                        if !self.search_matches.is_empty() {
                            // Clamp idx
                            self.search_match_idx = self
                                .search_match_idx
                                .min(self.search_matches.len().saturating_sub(1));
                            self.jump_to_match(self.search_match_idx);
                        }
                    }
                    KeyCode::Char(c) => {
                        self.search_query.push(c);
                        self.rebuild_search_matches();
                        // Jump to first match at or after cursor
                        if !self.search_matches.is_empty() {
                            let pos = (self.cursor_y, self.cursor_byte);
                            let idx = self
                                .search_matches
                                .iter()
                                .position(|&(ly, sb, _)| (ly, sb) >= pos)
                                .unwrap_or(0);
                            self.search_match_idx = idx;
                            self.jump_to_match(idx);
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── file finder mode (Ctrl+P) ────────────────────────────────────────────

    pub(crate) fn handle_file_finder_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.ff_query.clear();
                    self.ff_results.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    if !self.ff_results.is_empty() {
                        let path = self.ff_results[self.ff_idx].1.clone();
                        self.open_file(&path);
                    }
                    self.ff_query.clear();
                    self.ff_results.clear();
                    self.mode = Mode::Normal;
                    self.mark_all_dirty();
                }
                KeyCode::Up
                    if self.ff_idx > 0 =>
                {
                    self.ff_idx -= 1;
                    self.mark_all_dirty();
                }
                KeyCode::Down
                    if self.ff_idx + 1 < self.ff_results.len() =>
                {
                    self.ff_idx += 1;
                    self.mark_all_dirty();
                }
                KeyCode::Backspace => {
                    self.ff_query.pop();
                    self.rebuild_ff_results();
                    self.mark_all_dirty();
                }
                KeyCode::Char(c) => {
                    self.ff_query.push(c);
                    self.rebuild_ff_results();
                    self.mark_all_dirty();
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    fn rebuild_ff_results(&mut self) {
        if self.ff_query.is_empty() {
            self.ff_results.clear();
            return;
        }
        let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let results = search_files(&root, &self.ff_query, 10);
        self.ff_results = results
            .into_iter()
            .map(|r| (r.name, r.full_path, r.score))
            .collect();
        self.ff_idx = 0;
    }

    // ── gosc mode ─────────────────────────────────────────────────────────────

    pub(crate) fn handle_gosc_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.mode = Mode::Normal;
                    self.gosc_buf.clear();
                    self.gosc_dirs.clear();
                    self.mark_all_dirty();
                }
                KeyCode::Enter => {
                    let input = self.gosc_buf.trim().to_string();
                    self.gosc_buf.clear();
                    if input.is_empty() || input == "0" {
                        self.mode = Mode::Normal;
                        self.gosc_dirs.clear();
                        self.mark_all_dirty();
                    } else if let Some(back_str) = input.strip_prefix('-') {
                        let n: usize = back_str.parse().unwrap_or(1);
                        let mut target =
                            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                        let mut ok = true;
                        for _ in 0..n {
                            match target.parent() {
                                Some(p) => target = p.to_path_buf(),
                                None => {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                        if ok {
                            self.cd_to(&target);
                        } else {
                            self.status_msg = Some("Already at root".into());
                        }
                        self.gosc_dirs = self.list_subdirs();
                        self.mark_all_dirty();
                    } else if let Ok(n) = input.parse::<usize>() {
                        if n > 0 && n <= self.gosc_dirs.len() {
                            let cwd = std::env::current_dir()
                                .unwrap_or_else(|_| std::path::PathBuf::from("."));
                            let target = cwd.join(&self.gosc_dirs[n - 1]);
                            self.cd_to(&target);
                            self.gosc_dirs = self.list_subdirs();
                            self.mark_all_dirty();
                        }
                    }
                }
                KeyCode::Backspace => {
                    self.gosc_buf.pop();
                }
                KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
                    self.gosc_buf.push(c);
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── normal mode (read-only) ────────────────────────────────────────────────

    pub(crate) fn handle_normal_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);
                match (ctrl, code) {
                    (true, KeyCode::Char('i')) => {
                        self.mode = Mode::Insert;
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg =
                            Some("Visual mode — arrow keys to select, Esc to exit".into());
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }
                    (false, KeyCode::Char(':')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    // ── Multi-cursor: Esc clears extras, then exit only if none ──
                    (false, KeyCode::Esc) | (true, KeyCode::Char('q')) => {
                        if self.has_multiple_cursors() {
                            self.clear_extra_cursors();
                            self.status_msg = Some("Cleared extra cursors".into());
                        } else if self.exit_flow(stdout)? {
                            return Err(anyhow::anyhow!("__exit__"));
                        }
                    }
                    // ── Ctrl+B: toggle sidebar ─────────────────────────────────
                    (true, KeyCode::Char('b')) => {
                        self.toggle_sidebar();
                    }
                    // ── Ctrl+P: file finder ────────────────────────────────────
                    (true, KeyCode::Char('p')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    // ── Ctrl+Tab / Ctrl+Shift+Tab: buffer switching ────────────
                    (true, KeyCode::Tab) if !modifiers.contains(KeyModifiers::SHIFT) => {
                        self.next_buffer();
                    }
                    (true, KeyCode::BackTab) | (true, KeyCode::Tab) => {
                        self.prev_buffer();
                    }
                    // ── Ctrl+D: add cursor at next occurrence ──────────────────
                    (true, KeyCode::Char('d')) => {
                        self.add_cursor_at_next_occurrence();
                    }
                    // ── Horizontal scroll with Alt+Left/Right ─────────────────
                    (false, KeyCode::Left) if alt => {
                        self.scroll_x = self.scroll_x.saturating_sub(3);
                    }
                    (false, KeyCode::Right) if alt => {
                        self.scroll_x = (self.scroll_x + 3).min(self.max_scroll_x());
                    }
                    // Navigation — with multi-cursor support
                    (false, KeyCode::Up) if self.cursor_y > 0 => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor_y > 0 {
                                    e.cursor_y -= 1;
                                }
                            });
                        } else {
                            self.cursor_y -= 1;
                        }
                    }
                    (false, KeyCode::Down) if self.cursor_y + 1 < self.lines.len() => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor_y + 1 < e.lines.len() {
                                    e.cursor_y += 1;
                                }
                            });
                        } else {
                            self.cursor_y += 1;
                        }
                    }
                    (false, KeyCode::Left) if self.cursor_byte > 0 => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                if e.cursor_byte > 0 {
                                    e.cursor_byte = prev_char_byte(e.current(), e.cursor_byte);
                                }
                            });
                        } else {
                            self.cursor_byte = prev_char_byte(self.current(), self.cursor_byte);
                        }
                    }
                    (false, KeyCode::Right) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                let line = e.current();
                                if e.cursor_byte < line.len() {
                                    e.cursor_byte = next_char_byte(line, e.cursor_byte);
                                }
                            });
                        } else {
                            let line = self.current();
                            if self.cursor_byte < line.len() {
                                self.cursor_byte = next_char_byte(line, self.cursor_byte);
                            }
                        }
                    }
                    (false, KeyCode::Home) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                let first_non_ws = e
                                    .current()
                                    .char_indices()
                                    .find(|(_, c)| !c.is_whitespace())
                                    .map(|(b, _)| b)
                                    .unwrap_or(0);
                                e.cursor_byte = if e.cursor_byte != first_non_ws {
                                    first_non_ws
                                } else {
                                    0
                                };
                            });
                        } else {
                            let first_non_ws = self
                                .current()
                                .char_indices()
                                .find(|(_, c)| !c.is_whitespace())
                                .map(|(b, _)| b)
                                .unwrap_or(0);
                            self.cursor_byte = if self.cursor_byte != first_non_ws {
                                first_non_ws
                            } else {
                                0
                            };
                        }
                    }
                    (false, KeyCode::End) => {
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor_byte = e.current().len();
                            });
                        } else {
                            self.cursor_byte = self.current().len();
                        }
                    }
                    (false, KeyCode::PageUp) => {
                        let rows = self.term_h.saturating_sub(3);
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor_y = e.cursor_y.saturating_sub(rows);
                            });
                        } else {
                            self.cursor_y = self.cursor_y.saturating_sub(rows);
                        }
                    }
                    (false, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(3);
                        if self.has_multiple_cursors() {
                            self.for_each_cursor(false, |e| {
                                e.cursor_y = (e.cursor_y + rows).min(e.lines.len().saturating_sub(1));
                            });
                        } else {
                            self.cursor_y =
                                (self.cursor_y + rows).min(self.lines.len().saturating_sub(1));
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(m) => self.handle_mouse(m),
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── command mode ────────────────────────────────────────────────────────────

    pub(crate) fn handle_command_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.cmd_buf.clear();
                }
                KeyCode::Enter => {
                    let cmd = self.cmd_buf.clone();
                    self.cmd_buf.clear();
                    if self.execute_command(&cmd, stdout)? {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    if self.mode == Mode::Command {
                        self.mode = Mode::Normal;
                    }
                }
                KeyCode::Backspace => {
                    self.cmd_buf.pop();
                }
                KeyCode::Char(c) => {
                    self.cmd_buf.push(c);
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    fn list_subdirs(&self) -> Vec<String> {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let mut dirs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&cwd) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !name.starts_with('.') {
                            dirs.push(name.to_string());
                        }
                    }
                }
            }
        }
        dirs.sort_by_key(|a| a.to_lowercase());
        dirs
    }

    fn cd_to(&mut self, path: &std::path::Path) {
        match Navigator::new() {
            Ok(mut nav) => {
                let target = if path.to_string_lossy() == "-" {
                    // :cd - goes to parent
                    match nav.current_path().parent() {
                        Some(p) => p.to_path_buf(),
                        None => {
                            self.status_msg = Some("Already at root".into());
                            return;
                        }
                    }
                } else {
                    path.to_path_buf()
                };
                match nav.go_to(&target) {
                    Ok(()) => {
                        self.sidebar.rebuild_tree();
                        self.mark_all_dirty();
                        self.status_msg = Some(format!("CWD: {}", nav.display_path()));
                    }
                    Err(e) => {
                        self.status_msg = Some(format!("cd: {}", e));
                    }
                }
            }
            Err(_) => {
                self.status_msg = Some("Cannot get current directory".into());
            }
        }
    }

    fn execute_command(&mut self, cmd: &str, stdout: &mut std::io::Stdout) -> Result<bool> {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return Ok(false);
        }

        // Parse bare number: "42" → jump to line 42
        if let Ok(n) = cmd.parse::<usize>() {
            if n > 0 {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor_y = target;
                self.cursor_byte = 0;
                self.status_msg = Some(format!("Jumped to line {}", n));
                return Ok(false);
            }
        }

        // Parse "l N" or "line N"
        if let Some(rest) = cmd.strip_prefix("l ") {
            if let Ok(n) = rest.trim().parse::<usize>() {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor_y = target;
                self.cursor_byte = 0;
                self.status_msg = Some(format!("Jumped to line {}", n));
                return Ok(false);
            }
        }
        if let Some(rest) = cmd.strip_prefix("line ") {
            if let Ok(n) = rest.trim().parse::<usize>() {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor_y = target;
                self.cursor_byte = 0;
                self.status_msg = Some(format!("Jumped to line {}", n));
                return Ok(false);
            }
        }

        if cmd == "q" || cmd == "quit" {
            self.mode = Mode::Normal;
            return self.exit_flow(stdout);
        }

        if cmd == "wq" {
            self.mode = Mode::Normal;
            if let Err(e) = self.save() {
                self.status_msg = Some(format!("Save failed: {}", e));
                return Ok(false);
            }
            return self.exit_flow(stdout);
        }

        if cmd == "w" || cmd == "write" {
            match self.save() {
                Ok(()) => {
                    self.status_msg = Some("Saved ✓".into());
                }
                Err(e) => {
                    self.status_msg = Some(format!("Save failed: {}", e));
                }
            }
            return Ok(false);
        }

        if cmd == "auto on" {
            self.editor_cfg.auto_save = true;
            self.editor_cfg.save();
            self.status_msg = Some("Auto-save on exit: ON".into());
            return Ok(false);
        }
        if cmd == "auto off" {
            self.editor_cfg.auto_save = false;
            self.editor_cfg.save();
            self.status_msg = Some("Auto-save on exit: OFF".into());
            return Ok(false);
        }

        if cmd == "syntax on" {
            self.editor_cfg.syntax_enabled = true;
            self.editor_cfg.save();
            self.syntax.invalidate_all();
            self.mark_all_dirty();
            self.status_msg = Some("Syntax highlighting: ON".into());
            return Ok(false);
        }
        if cmd == "syntax off" {
            self.editor_cfg.syntax_enabled = false;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some("Syntax highlighting: OFF".into());
            return Ok(false);
        }

        if cmd == "color on" {
            self.editor_cfg.color_enabled = true;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some("Color: ON".into());
            return Ok(false);
        }
        if cmd == "color off" {
            self.editor_cfg.color_enabled = false;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some("Color: OFF".into());
            return Ok(false);
        }

        // ── cd <path> — change CWD via Navigator ────────────────────────────
        if let Some(path) = cmd.strip_prefix("cd ") {
            let path = path.trim();
            if path.is_empty() {
                self.status_msg = Some(format!("CWD: {}", std::env::current_dir().unwrap_or_default().display()));
            } else {
                self.cd_to(std::path::Path::new(path));
            }
            return Ok(false);
        }

        // ── back [N] — go to parent directory ──────────────────────────────
        if cmd == "back" || cmd.starts_with("back ") {
            let count = if let Some(n) = cmd.strip_prefix("back ") {
                n.trim().parse::<usize>().unwrap_or(1).max(1)
            } else {
                1usize
            };
            let mut target = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut ok = true;
            for _ in 0..count {
                match target.parent() {
                    Some(p) => target = p.to_path_buf(),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                self.cd_to(&target);
            } else {
                self.status_msg = Some("Already at root".into());
            }
            return Ok(false);
        }

        // ── gos / gosc — list & navigate subdirectories by number ──────────
        if cmd == "gosc" || cmd == "gos" {
            let dirs = self.list_subdirs();
            if dirs.is_empty() {
                self.status_msg = Some("No subdirectories".into());
            } else {
                self.gosc_dirs = dirs;
                self.gosc_buf.clear();
                self.mode = Mode::Gosc;
                self.mark_all_dirty();
            }
            return Ok(false);
        }
        if let Some(n_str) = cmd.strip_prefix("gos ") {
            let n: usize = match n_str.trim().parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    self.status_msg = Some("gos: usage gos <N>".into());
                    return Ok(false);
                }
            };
            let dirs = self.list_subdirs();
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            if n > dirs.len() {
                self.status_msg = Some(format!("gos: only {} subdirectories", dirs.len()));
            } else {
                let target = cwd.join(&dirs[n - 1]);
                self.cd_to(&target);
            }
            return Ok(false);
        }

        // ── mkdir <path> — create directory ─────────────────────────────────
        if let Some(dir) = cmd.strip_prefix("mkdir ") {
            let dir = dir.trim();
            if dir.is_empty() {
                self.status_msg = Some("mkdir: missing operand".into());
            } else if let Err(e) = std::fs::create_dir_all(std::path::Path::new(dir)) {
                self.status_msg = Some(format!("mkdir: {}", e));
            } else {
                self.sidebar.rebuild_tree();
                self.mark_all_dirty();
                self.status_msg = Some(format!("Created directory: {}", dir));
            }
            return Ok(false);
        }

        // ── touch / new <path> — create file ────────────────────────────────
        if let Some(file) = cmd.strip_prefix("touch ").or_else(|| cmd.strip_prefix("new ")) {
            let file = file.trim();
            if file.is_empty() {
                self.status_msg = Some("touch: missing operand".into());
            } else {
                let path = std::path::Path::new(file);
                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        if !parent.as_os_str().is_empty() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                    }
                    match std::fs::write(path, "") {
                        Ok(()) => {
                            self.sidebar.rebuild_tree();
                            self.mark_all_dirty();
                        }
                        Err(e) => {
                            self.status_msg = Some(format!("touch: {}", e));
                            return Ok(false);
                        }
                    }
                }
                self.open_file(path);
            }
            return Ok(false);
        }

        // ── ne <path> — navigate & edit (open/create file) ──────────────────
        if let Some(file) = cmd.strip_prefix("ne ") {
            let file = file.trim();
            if file.is_empty() || file == "." {
                self.sidebar.rebuild_tree();
                self.mark_all_dirty();
                self.status_msg = Some(format!("CWD: {}", std::env::current_dir().unwrap_or_default().display()));
            } else {
                let path = std::path::Path::new(file);
                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        if !parent.as_os_str().is_empty() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                    }
                    let _ = std::fs::write(path, "");
                }
                self.open_file(path);
            }
            return Ok(false);
        }
        if cmd == "ne" {
            self.sidebar.rebuild_tree();
            self.mark_all_dirty();
            self.status_msg = Some(format!("CWD: {}", std::env::current_dir().unwrap_or_default().display()));
            return Ok(false);
        }

        // ── tp <name> — jump to teleport savepoint ──────────────────────────
        if let Some(name) = cmd.strip_prefix("tp ") {
            let name = name.trim();
            if name.is_empty() {
                let tp_list = TeleportManager::get_all();
                if tp_list.is_empty() {
                    self.status_msg = Some("No teleport savepoints".into());
                } else {
                    let mut msg = String::from("Teleports:");
                    let mut sorted: Vec<_> = tp_list.into_iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(&b.0));
                    for (n, p) in &sorted {
                        msg.push_str(&format!(" {}->{}", n, p.display()));
                    }
                    self.status_msg = Some(msg);
                }
            } else {
                match Navigator::new() {
                    Ok(mut nav) => {
                        match TeleportManager::get_path(name) {
                            Some(path) => match nav.go_to(&path) {
                                Ok(()) => {
                                    self.sidebar.rebuild_tree();
                                    self.mark_all_dirty();
                                    self.status_msg =
                                        Some(format!("Teleported to '{}' -> {}", name, nav.display_path()));
                                }
                                Err(e) => {
                                    self.status_msg = Some(format!("tp: {}", e));
                                }
                            },
                            None => {
                                self.status_msg = Some(format!("Savepoint not found: '{}'", name));
                            }
                        }
                    }
                    Err(_) => {
                        self.status_msg = Some("Cannot get current directory".into());
                    }
                }
            }
            return Ok(false);
        }

        // ── tp-add <name> — save CWD as teleport ────────────────────────────
        if let Some(name) = cmd.strip_prefix("tp-add ") {
            let name = name.trim();
            if name.is_empty() {
                self.status_msg = Some("tp-add: missing name".into());
            } else {
                match Navigator::new() {
                    Ok(nav) => {
                        match TeleportManager::add_current(&nav, name) {
                            Ok(()) => {
                                self.status_msg = Some(format!("Savepoint '{}' created", name));
                            }
                            Err(e) => {
                                self.status_msg = Some(format!("tp-add: {}", e));
                            }
                        }
                    }
                    Err(_) => {
                        self.status_msg = Some("Cannot get current directory".into());
                    }
                }
            }
            return Ok(false);
        }

        self.status_msg = Some(format!("Unknown command: {}", cmd));
        Ok(false)
    }

    // ── help mode ───────────────────────────────────────────────────────────────

    pub(crate) fn handle_help_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.mark_all_dirty();
                    self.mode = Mode::Normal;
                    self.status_msg = None;
                }
                KeyCode::Up => {
                    self.help_scroll = self.help_scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.help_scroll += 1;
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── visual mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_visual_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
                    (_, KeyCode::Esc) => {
                        self.clear_selection();
                        self.mode = Mode::Normal;
                    }
                    (true, KeyCode::Char('i')) => {
                        self.mode = Mode::Insert;
                        self.clear_selection();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (true, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (false, KeyCode::Left) => {
                        self.start_or_extend_selection();
                        if self.cursor_byte > 0 {
                            self.cursor_byte = prev_char_byte(self.current(), self.cursor_byte);
                        }
                    }
                    (false, KeyCode::Right) => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor_byte < line.len() {
                            self.cursor_byte = next_char_byte(line, self.cursor_byte);
                        }
                    }
                    (false, KeyCode::Up) => {
                        self.start_or_extend_selection();
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    (false, KeyCode::Down) => {
                        self.start_or_extend_selection();
                        if self.cursor_y + 1 < self.lines.len() {
                            self.cursor_y += 1;
                        }
                    }
                    (false, KeyCode::Home) => {
                        self.start_or_extend_selection();
                        let first_non_ws = self
                            .current()
                            .char_indices()
                            .find(|(_, c)| !c.is_whitespace())
                            .map(|(b, _)| b)
                            .unwrap_or(0);
                        self.cursor_byte = if self.cursor_byte != first_non_ws {
                            first_non_ws
                        } else {
                            0
                        };
                    }
                    (false, KeyCode::End) => {
                        self.start_or_extend_selection();
                        self.cursor_byte = self.current().len();
                    }
                    (true, KeyCode::Char('c')) => {
                        self.copy_selection();
                        self.mode = Mode::Normal;
                    }
                    (true, KeyCode::Char('x')) => {
                        self.cut_selection();
                        self.mode = Mode::Normal;
                    }
                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, KeyCode::Char('='))
                    | (true, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }
                    _ => {}
                }
            }
            Event::Mouse(m) => self.handle_mouse(m),
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    // ── insert mode ──────────────────────────────────────────────────────────

    pub(crate) fn handle_insert_event(&mut self, ev: Event, stdout: &mut std::io::Stdout) -> Result<()> {
        // Clear transient status message on any keypress
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        }) = &ev
        {
            self.status_msg = None;
        }

        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);

                match (ctrl, alt, shift, code) {
                    // ── File ops ──────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('s')) => match self.save() {
                        Ok(()) => {
                            let _ = set_status_bar(stdout, Color::Green, Color::White, " Saved ✓");
                        }
                        Err(e) => {
                            let _ = set_status_bar(
                                stdout,
                                Color::Red,
                                Color::White,
                                &format!(" Save failed: {}", e),
                            );
                        }
                    },
                    (true, _, _, KeyCode::Char('q')) if self.exit_flow(stdout)? => {
                        return Err(anyhow::anyhow!("__exit__"));
                    }
                    (false, false, false, KeyCode::Esc) => {
                        self.clear_selection();
                        self.clear_extra_cursors();
                        self.mode = Mode::Normal;
                        self.status_msg = None;
                    }

                    // ── Undo / Redo ───────────────────────────────────────────
                    (true, _, _, KeyCode::Char('z')) => self.undo(),
                    (true, _, _, KeyCode::Char('r')) => self.redo(),

                    // ── Clipboard ─────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('a')) => {
                        self.clear_extra_cursors();
                        self.select_all();
                    }
                    (true, _, _, KeyCode::Char('c')) => self.copy_selection(),
                    (true, _, _, KeyCode::Char('x')) => self.cut_selection(),
                    // ── Paste ──────────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('v')) => {} // block ^V (Windows system paste)
                    (true, _, _, KeyCode::Char('p')) => self.paste(),

                    // ── Terminal zoom — block locally ─────────────────────────
                    (true, _, _, KeyCode::Char('='))
                    | (true, _, _, KeyCode::Char('-')) => {
                        self.status_msg = Some("Terminal zoom: Ctrl++ / Ctrl+-".into());
                    }

                    // ── Line operations / multi-cursor ─────────────────────────
                    (true, _, _, KeyCode::Char('d')) => self.add_cursor_at_next_occurrence(),
                    (true, _, _, KeyCode::Char('k')) => self.kill_line(),
                    (true, _, _, KeyCode::Char('l')) => self.select_line(),
                    (false, true, _, KeyCode::Up) => self.move_line_up(),
                    (false, true, _, KeyCode::Down) => self.move_line_down(),

                    // ── Search ────────────────────────────────────────────────
                    (true, _, _, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search_query.clear();
                        self.search_matches.clear();
                        self.search_match_idx = 0;
                        self.mark_all_dirty();
                    }
                    // Continue search with n / N (when not in search mode)
                    (false, false, false, KeyCode::F(3)) if !self.search_query.is_empty() => {
                        self.search_next();
                    }
                    (false, false, true, KeyCode::F(3)) if !self.search_query.is_empty() => {
                        self.search_prev();
                    }

                    // ── Help / Visual / Yank ──────────────────────────────────
                    (true, _, _, KeyCode::Char('w')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg =
                            Some("Visual mode — arrow keys to select, Esc to exit".into());
                    }
                    (true, _, _, KeyCode::Char('h')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (true, _, _, KeyCode::Char('y')) => self.yank_line(),

                    // ── Sidebar / buffers ───────────────────────────────────────
                    (true, false, false, KeyCode::Char('b')) => self.toggle_sidebar(),
                    (true, false, false, KeyCode::Tab) => self.next_buffer(),
                    (true, false, true, KeyCode::Tab) => self.prev_buffer(),

                    // ── Horizontal scroll with Alt+Left/Right ─────────────────
                    (false, true, false, KeyCode::Left) => {
                        self.scroll_x = self.scroll_x.saturating_sub(3);
                    }
                    (false, true, false, KeyCode::Right) => {
                        self.scroll_x = (self.scroll_x + 3).min(self.max_scroll_x());
                    }

                    // ── Cursor movement (plain) ───────────────────────────────
                    (false, false, false, KeyCode::Up) => {
                        self.move_cursors(|e| {
                            if e.cursor_y > 0 {
                                e.cursor_y -= 1;
                            }
                        });
                    }
                    (false, false, false, KeyCode::Down) => {
                        self.move_cursors(|e| {
                            if e.cursor_y + 1 < e.lines.len() {
                                e.cursor_y += 1;
                            }
                        });
                    }
                    (false, false, false, KeyCode::Left) => {
                        self.move_cursors(|e| {
                            if e.has_selection() {
                                let (ay, ab) = e.selection_anchor.unwrap();
                                let ((sy, sb), _) = sel_range(e.cursor_y, e.cursor_byte, ay, ab);
                                e.cursor_y = sy;
                                e.cursor_byte = sb;
                                e.clear_selection();
                            } else if e.cursor_byte > 0 {
                                e.cursor_byte = prev_char_byte(e.current(), e.cursor_byte);
                            }
                        });
                    }
                    (false, false, false, KeyCode::Right) => {
                        self.move_cursors(|e| {
                            if e.has_selection() {
                                let (ay, ab) = e.selection_anchor.unwrap();
                                let (_, (ey, eb)) = sel_range(e.cursor_y, e.cursor_byte, ay, ab);
                                e.cursor_y = ey;
                                e.cursor_byte = eb;
                                e.clear_selection();
                            } else {
                                let line = e.current();
                                let byte_len = line.len();
                                if e.cursor_byte < byte_len {
                                    e.cursor_byte = next_char_byte(line, e.cursor_byte);
                                }
                            }
                        });
                    }

                    // ── Shift+Arrow = extend selection ────────────────────────
                    (false, false, true, KeyCode::Up) => {
                        self.start_or_extend_selection();
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    (false, false, true, KeyCode::Down) => {
                        self.start_or_extend_selection();
                        if self.cursor_y + 1 < self.lines.len() {
                            self.cursor_y += 1;
                        }
                    }
                    (false, false, true, KeyCode::Left) => {
                        self.start_or_extend_selection();
                        if self.cursor_byte > 0 {
                            self.cursor_byte = prev_char_byte(self.current(), self.cursor_byte);
                        }
                    }
                    (false, false, true, KeyCode::Right) => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        let byte_len = line.len();
                        if self.cursor_byte < byte_len {
                            self.cursor_byte = next_char_byte(line, self.cursor_byte);
                        }
                    }

                    // ── Ctrl+Arrow = word jump ────────────────────────────────
                    (true, _, false, KeyCode::Left) => {
                        self.clear_selection();
                        if self.cursor_byte > 0 {
                            self.cursor_byte = prev_word_byte(self.current(), self.cursor_byte);
                        } else if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                            self.cursor_byte = self.current().len();
                        }
                    }
                    (true, _, false, KeyCode::Right) => {
                        self.clear_selection();
                        let line = self.current();
                        if self.cursor_byte < line.len() {
                            self.cursor_byte = next_word_byte(line, self.cursor_byte);
                        } else if self.cursor_y + 1 < self.lines.len() {
                            self.cursor_y += 1;
                            self.cursor_byte = 0;
                        }
                    }

                    // ── Ctrl+Shift+Arrow = word-select ────────────────────────
                    (true, _, true, KeyCode::Left) => {
                        self.start_or_extend_selection();
                        if self.cursor_byte > 0 {
                            self.cursor_byte = prev_word_byte(self.current(), self.cursor_byte);
                        }
                    }
                    (true, _, true, KeyCode::Right) => {
                        self.start_or_extend_selection();
                        let line = self.current();
                        if self.cursor_byte < line.len() {
                            self.cursor_byte = next_word_byte(line, self.cursor_byte);
                        }
                    }

                    // ── Home / End ────────────────────────────────────────────
                    (false, _, false, KeyCode::Home) => {
                        self.clear_selection();
                        // Smart home: jump to first non-whitespace, then to 0
                        let first_non_ws = self
                            .current()
                            .char_indices()
                            .find(|(_, c)| !c.is_whitespace())
                            .map(|(b, _)| b)
                            .unwrap_or(0);
                        self.cursor_byte = if self.cursor_byte != first_non_ws {
                            first_non_ws
                        } else {
                            0
                        };
                    }
                    (false, _, false, KeyCode::End) => {
                        self.clear_selection();
                        self.cursor_byte = self.current().len();
                    }
                    (false, _, true, KeyCode::Home) => {
                        self.start_or_extend_selection();
                        self.cursor_byte = 0;
                    }
                    (false, _, true, KeyCode::End) => {
                        self.start_or_extend_selection();
                        self.cursor_byte = self.current().len();
                    }
                    // Ctrl+Home / Ctrl+End = file start/end
                    (true, _, _, KeyCode::Home) => {
                        self.clear_selection();
                        self.cursor_y = 0;
                        self.cursor_byte = 0;
                    }
                    (true, _, _, KeyCode::End) => {
                        self.clear_selection();
                        self.cursor_y = self.lines.len().saturating_sub(1);
                        self.cursor_byte = self.current().len();
                    }

                    // ── Page up / down ────────────────────────────────────────
                    (false, _, _, KeyCode::PageUp) => {
                        self.clear_selection();
                        let rows = self.term_h.saturating_sub(3);
                        self.cursor_y = self.cursor_y.saturating_sub(rows);
                    }
                    (false, _, _, KeyCode::PageDown) => {
                        self.clear_selection();
                        let rows = self.term_h.saturating_sub(3);
                        self.cursor_y =
                            (self.cursor_y + rows).min(self.lines.len().saturating_sub(1));
                    }

                    // ── Tab / Shift+Tab ───────────────────────────────────────
                    (false, false, false, KeyCode::Tab) => self.indent_lines(),
                    (false, false, _, KeyCode::BackTab) => self.dedent_lines(),

                    // ── Enter ─────────────────────────────────────────────────
                    (false, false, false, KeyCode::Enter) => {
                        if self.has_multiple_cursors() {
                            self.multi_split_line();
                        } else {
                            if self.has_selection() {
                                self.snapshot();
                                self.delete_selection();
                            }
                            self.snapshot();
                            self.split_line();
                        }
                    }

                    // ── Backspace / Delete ────────────────────────────────────
                    (false, false, _, KeyCode::Backspace) => {
                        if self.has_multiple_cursors() {
                            self.multi_backspace();
                        } else {
                            self.backspace();
                        }
                    }
                    (false, false, _, KeyCode::Delete) => {
                        if self.has_multiple_cursors() {
                            self.multi_delete_forward();
                        } else {
                            self.delete_forward();
                        }
                    }

                    // ── Regular character input with auto-pair ─────────────────
                    (false, false, false, KeyCode::Char(c)) => {
                        if self.has_multiple_cursors() {
                            self.multi_insert_at(c);
                        } else {
                            self.snapshot();
                            if let Some(closing) = auto_pair(c) {
                                self.insert_at(c);
                                self.insert_at_raw(closing);
                                self.cursor_byte = self.cursor_byte.saturating_sub(closing.len_utf8());
                            } else {
                                self.insert_at(c);
                            }
                        }
                    }
                    (false, false, true, KeyCode::Char(c)) => {
                        if self.has_multiple_cursors() {
                            self.multi_insert_at(c);
                        } else {
                            self.snapshot();
                            if let Some(closing) = auto_pair(c) {
                                self.insert_at(c);
                                self.insert_at_raw(closing);
                                self.cursor_byte = self.cursor_byte.saturating_sub(closing.len_utf8());
                            } else {
                                self.insert_at(c);
                            }
                        }
                    }

                    _ => {}
                }
            }

            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }

            Event::Mouse(m) => self.handle_mouse(m),

            _ => {}
        }

        Ok(())
    }
}
