use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode, NfeBox, NfeBoxKind, NfeConfirm};
use crate::t;

impl Editor {
    /// Enter the nfe sidebar explorer (via `e`): opens the sidebar when
    /// closed and takes keyboard focus. Caller sets `prev_mode` first.
    pub(crate) fn enter_nfe(&mut self) {
        if !self.sidebar.open {
            self.toggle_sidebar();
        }
        self.mode = Mode::Nfe;
        self.status_msg = None;
        self.nfe_confirm = None;
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Leave nfe back to the invoking mode. The sidebar always hides:
    /// `e` is a toggle, never a one-way door.
    fn exit_nfe(&mut self) {
        let prev = std::mem::replace(&mut self.prev_mode, Mode::Normal);
        self.mode = prev;
        self.sidebar.open = false;
        self.status_msg = None;
        self.nfe_confirm = None;
        self.mark_all_dirty();
    }

    /// `i` from nfe (or `:i` from the prompt): hide the sidebar and
    /// enter Insert mode, mirroring Normal-mode `i`.
    fn nfe_to_insert(&mut self) {
        self.exit_nfe();
        self.mode = Mode::Insert;
        self.completion = None;
        self.sighelp = None;
        self.nfe_cmd = None;
        self.status_msg = None;
        self.mark_all_dirty();
    }

    /// `v` from nfe (or `:v` from the prompt): hide the sidebar and
    /// enter Visual mode, mirroring Normal-mode `v`.
    fn nfe_to_visual(&mut self) {
        self.exit_nfe();
        self.mode = Mode::Visual;
        self.clear_selection();
        self.nfe_cmd = None;
        self.status_msg = Some(t!("editor.mode_visual_start"));
        self.mark_all_dirty();
    }

    /// Keep the keyboard cursor inside the visible sidebar window.
    pub(crate) fn nfe_ensure_visible(&mut self) {
        let total = self.sidebar.nodes.len();
        if total == 0 {
            return;
        }
        let rows = self.term_h.max(1);
        self.sidebar.selected = self.sidebar.selected.min(total - 1);
        if self.sidebar.selected < self.sidebar.scroll {
            self.sidebar.scroll = self.sidebar.selected;
        } else if self.sidebar.selected + 1 > self.sidebar.scroll + rows {
            self.sidebar.scroll = self.sidebar.selected + 1 - rows;
        }
    }

    /// Run one `nfe:` prompt line: `size full|def|N%`, `n <file>`,
    /// `m <dir>`, `rmd`/`rmf`, `i` (insert), `v` (visual), `qt`/`aqt`
    /// (close tabs, like gmode), `t`/`T`/`t<n>`/`T<n>` (switch tabs), or a
    /// bare number to jump to the sidebar entry with that number. Clears
    /// the prompt on success so navigation resumes; leaves it open on error.
    fn run_nfe_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "qt" => {
                self.close_current_tab();
                self.nfe_cmd = None;
                self.mark_all_dirty();
                return;
            }
            "aqt" => {
                self.close_all_tabs();
                self.nfe_cmd = None;
                self.mark_all_dirty();
                return;
            }
            _ => {}
        }
        for (prefix, back) in [('t', false), ('T', true)] {
            let Some(n) = Editor::parse_tab_command(cmd, prefix) else {
                continue;
            };
            let jumped = if n == 0 {
                if back {
                    self.prev_tab();
                } else {
                    self.next_tab();
                }
                true
            } else {
                self.goto_tab(n)
            };
            if jumped {
                self.nfe_cmd = None;
            }
            self.mark_all_dirty();
            return;
        }
        if let Ok(n) = cmd.parse::<usize>() {
            let total = self.sidebar.nodes.len();
            if n >= 1 && n <= total {
                self.sidebar.selected = n - 1;
                let name = self.sidebar.nodes[n - 1].name.clone();
                self.status_msg = Some(t!("editor.nfe_jumped", n, name));
                self.nfe_cmd = None;
                self.nfe_ensure_visible();
            } else {
                self.status_msg = Some(t!("editor.nfe_jump_range", n, total));
            }
            self.mark_all_dirty();
            return;
        }
        let mut parts = cmd.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some("size"), Some(arg), None) => match parse_nfe_size(arg) {
                Some(pct) => {
                    self.editor_cfg.sidebar_width_pct = pct;
                    self.editor_cfg.save();
                    let label = match pct {
                        None => t!("editor.nfe_size_default"),
                        Some(100) => t!("editor.nfe_size_full"),
                        Some(n) => format!("{}%", n),
                    };
                    self.status_msg = Some(t!("editor.nfe_size_set", label));
                    self.nfe_cmd = None;
                    self.nfe_ensure_visible();
                    self.mark_all_dirty();
                }
                None => {
                    self.status_msg = Some(t!("editor.nfe_size_usage", cmd));
                }
            },
            (Some("n"), Some(name), None) => self.nfe_create_file(name),
            (Some("m"), Some(name), None) => self.nfe_create_dir(name),
            (Some("i") | Some("I"), None, None) => self.nfe_to_insert(),
            (Some("v") | Some("V"), None, None) => self.nfe_to_visual(),
            (Some("rmd"), Some(name), None) => self.nfe_remove_dir(name),
            (Some("rmf"), Some(name), None) => self.nfe_remove_file(name),
            (Some("n") | Some("m"), _, _) => {
                self.status_msg = Some(t!("editor.nfe_new_usage"));
            }
            (Some("rmd") | Some("rmf"), _, _) => {
                self.status_msg = Some(t!("editor.nfe_rm_usage"));
            }
            _ => {
                self.status_msg = Some(t!("editor.nfe_unknown", cmd));
            }
        }
    }

    /// Create a file relative to the sidebar root and open it in the
    /// field (NFE focus is kept). Errors leave the prompt open.
    fn nfe_create_file(&mut self, name: &str) {
        let root = self.sidebar.root.clone();
        match resolve_new_path(&root, name) {
            Err(e) => {
                self.status_msg = Some(e);
                return;
            }
            Ok(path) => {
                if path.exists() {
                    self.status_msg = Some(t!("editor.nfe_create_exists", name));
                    return;
                }
                match crate::editor::init_file(&path) {
                    Ok(true) => {}
                    Ok(false) => {
                        self.status_msg = Some(t!("editor.nfe_create_failed", name));
                        return;
                    }
                    Err(e) => {
                        self.status_msg = Some(t!("editor.nfe_create_failed", e));
                        return;
                    }
                }
                self.mode = Mode::Nfe;
                self.sidebar.invalidate_cache();
                self.sidebar.rebuild_tree();
                self.nfe_select_path(&path);
                self.status_msg = Some(t!("editor.nfe_created_file", name));
                self.nfe_cmd = None;
                self.nfe_ensure_visible();
                self.mark_all_dirty();
            }
        }
    }

    /// Create a directory relative to the sidebar root. Errors leave
    /// the prompt open.
    fn nfe_create_dir(&mut self, name: &str) {
        let root = self.sidebar.root.clone();
        match resolve_new_path(&root, name) {
            Err(e) => {
                self.status_msg = Some(e);
                return;
            }
            Ok(path) => {
                if path.exists() {
                    self.status_msg = Some(t!("editor.nfe_create_exists", name));
                    return;
                }
                if let Err(e) = std::fs::create_dir_all(&path) {
                    self.status_msg = Some(t!("editor.nfe_create_failed", e));
                    return;
                }
                self.sidebar.invalidate_cache();
                self.sidebar.rebuild_tree();
                self.nfe_select_path(&path);
                self.status_msg = Some(t!("editor.nfe_created_dir", name));
                self.nfe_cmd = None;
                self.nfe_ensure_visible();
                self.mark_all_dirty();
            }
        }
    }

    /// Delete the selected sidebar entry: sets a `y`/`n` confirmation state.
    /// While a confirm is pending, all keys route through
    /// [`Editor::handle_nfe_confirm_key`].
    fn request_nfe_delete(&mut self) -> bool {
        if self.sidebar.nodes.is_empty() {
            return false;
        }
        let idx = self.sidebar.selected.min(self.sidebar.nodes.len() - 1);
        let node = self.sidebar.nodes[idx].clone();
        if node.name == ".." {
            self.status_msg = Some(t!("editor.nfe_rm_root"));
            self.mark_all_dirty();
            return false;
        }
        self.nfe_confirm = Some(NfeConfirm {
            path: node.path.clone(),
            is_dir: node.is_dir,
        });
        self.status_msg = Some(t!("editor.nfe_confirm", node.name));
        self.mark_all_dirty();
        true
    }

    /// Keys while a delete confirmation is pending: `y` deletes, `n`/Esc
    /// cancels, anything else is ignored.
    fn handle_nfe_confirm_key(&mut self, code: KeyCode, ctrl: bool) {
        if ctrl {
            return;
        }
        match code {
            KeyCode::Esc => {
                self.nfe_confirm = None;
                self.status_msg = None;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(c) = self.nfe_confirm.take() {
                    self.nfe_remove_path(&c.path, c.is_dir);
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.nfe_confirm = None;
                self.status_msg = None;
            }
            _ => {}
        }
    }

    /// Remove a file or directory tree, closing tabs with unsaved edits.
    fn nfe_remove_path(&mut self, path: &std::path::Path, is_dir: bool) {
        let root = self.sidebar.root.clone();
        let name = path
            .strip_prefix(&root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| {
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string())
            });
        if !path.exists() {
            self.status_msg = Some(t!("editor.nfe_rm_not_exist", &name));
            return;
        }
        if is_dir && !path.is_dir() {
            self.status_msg = Some(t!("editor.nfe_rmd_not_dir", &name));
            return;
        }
        if !is_dir && path.is_dir() {
            self.status_msg = Some(t!("editor.nfe_rmf_is_dir", &name));
            return;
        }
        if !self.nfe_close_affected_tabs(path, is_dir) {
            return;
        }
        let result = if is_dir {
            std::fs::remove_dir_all(path)
        } else {
            std::fs::remove_file(path)
        };
        if let Err(e) = result {
            self.status_msg = Some(t!("editor.nfe_rm_failed", e));
            return;
        }
        self.sidebar.invalidate_cache();
        self.sidebar.rebuild_tree();
        self.clamp_nfe_selection();
        self.status_msg = Some(t!("editor.nfe_removed", &name));
        self.nfe_cmd = None;
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Remove a directory tree relative to the sidebar root (`rmd`).
    /// Strict: the target must be an existing dir. Open tabs with
    /// unsaved edits under it abort the delete. Tabs showing the
    /// removed tree are closed. Errors leave the prompt open.
    fn nfe_remove_dir(&mut self, name: &str) {
        let root = self.sidebar.root.clone();
        let path = match resolve_new_path(&root, name) {
            Err(e) => {
                self.status_msg = Some(e);
                return;
            }
            Ok(p) => p,
        };
        if !path.exists() {
            self.status_msg = Some(t!("editor.nfe_rm_not_exist", name));
            return;
        }
        if !path.is_dir() {
            self.status_msg = Some(t!("editor.nfe_rmd_not_dir", name));
            return;
        }
        if !self.nfe_close_affected_tabs(&path, true) {
            return;
        }
        if let Err(e) = std::fs::remove_dir_all(&path) {
            self.status_msg = Some(t!("editor.nfe_rm_failed", e));
            return;
        }
        self.sidebar.invalidate_cache();
        self.sidebar.rebuild_tree();
        self.clamp_nfe_selection();
        self.status_msg = Some(t!("editor.nfe_removed", name));
        self.nfe_cmd = None;
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Remove a file relative to the sidebar root (`rmf`).
    /// Strict: the target must be an existing file (never a dir).
    /// Same tab/unsaved handling as [`nfe_remove_dir`].
    fn nfe_remove_file(&mut self, name: &str) {
        let root = self.sidebar.root.clone();
        let path = match resolve_new_path(&root, name) {
            Err(e) => {
                self.status_msg = Some(e);
                return;
            }
            Ok(p) => p,
        };
        if !path.exists() {
            self.status_msg = Some(t!("editor.nfe_rm_not_exist", name));
            return;
        }
        if path.is_dir() {
            self.status_msg = Some(t!("editor.nfe_rmf_is_dir", name));
            return;
        }
        if !self.nfe_close_affected_tabs(&path, false) {
            return;
        }
        if let Err(e) = std::fs::remove_file(&path) {
            self.status_msg = Some(t!("editor.nfe_rm_failed", e));
            return;
        }
        self.sidebar.invalidate_cache();
        self.sidebar.rebuild_tree();
        self.clamp_nfe_selection();
        self.status_msg = Some(t!("editor.nfe_removed", name));
        self.nfe_cmd = None;
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Close tabs showing `target` (`is_dir`: the whole subtree).
    /// Returns false (leaving everything untouched) when an affected
    /// tab has unsaved edits. Clean tabs are closed without saving so
    /// the delete is not resurrected by `close_tab`'s save path.
    fn nfe_close_affected_tabs(&mut self, target: &std::path::Path, is_dir: bool) -> bool {
        self.save_active_tab();
        let affected: Vec<usize> = self
            .tabs
            .iter()
            .enumerate()
            .filter(|(_, tab)| {
                if is_dir {
                    tab.path.as_path() == target || tab.path.starts_with(target)
                } else {
                    tab.path.as_path() == target
                }
            })
            .map(|(i, _)| i)
            .collect();
        for &i in &affected {
            if self.tabs[i].modified && !self.is_scratch_path(&self.tabs[i].path) {
                let label = self.tabs[i]
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
                    .to_string();
                self.status_msg = Some(t!("editor.nfe_rm_has_unsaved", label));
                return false;
            }
        }
        for &i in affected.iter().rev() {
            self.tabs[i].modified = false;
            self.close_tab(i);
        }
        // The live buffer may point at the removed target while `tabs`
        // is empty (pre-`init_tabs` flows): fall back to scratch.
        if self.tabs.is_empty() {
            self.open_scratch_tab();
        }
        true
    }

    /// Clamp the sidebar selection after the tree changed.
    fn clamp_nfe_selection(&mut self) {
        if self.sidebar.nodes.is_empty() {
            self.sidebar.selected = 0;
        } else {
            self.sidebar.selected = self.sidebar.selected.min(self.sidebar.nodes.len() - 1);
        }
    }

    /// Move the sidebar selection to `path` when it is visible.
    fn nfe_select_path(&mut self, path: &std::path::Path) {
        if let Some(idx) = self.sidebar.nodes.iter().position(|n| n.path == path) {
            self.sidebar.selected = idx;
        }
    }
    fn nfe_activate(&mut self) {
        // Activate the selected node: dirs expand/collapse, files open
        // in the field while staying in nfe mode.
        if self.sidebar.nodes.is_empty() {
            return;
        }
        let idx = self.sidebar.selected.min(self.sidebar.nodes.len() - 1);
        let node = self.sidebar.nodes[idx].clone();
        if node.is_dir {
            self.sidebar.toggle_expand(idx);
        } else {
            self.open_file(&node.path);
        }
        // Opening a file must not drop nfe focus.
        self.mode = Mode::Nfe;
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Keyboard-driven sidebar explorer: hjkl/awsd/arrows move, Enter
    /// activates, `i`/`v` jump to insert/visual (sidebar hides), `:`
    /// opens the prompt (`:i`/`:v` also switch modes), Esc returns.
    pub(crate) fn handle_nfe_event(
        &mut self,
        ev: Event,
        _stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                // The input box for `n`/`m`/`r` takes every key while open.
                if self.nfe_box.is_some() {
                    let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                    self.handle_nfe_box_key(code, ctrl);
                    self.mark_all_dirty();
                    return Ok(());
                }
                // Command-prompt input capture takes over all keys.
                if self.nfe_cmd.is_some() {
                    // Ctrl+E accepts the ghost hint, like End.
                    if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('e') {
                        self.prompt_accept_or_end();
                        self.mark_all_dirty();
                        return Ok(());
                    }
                    match code {
                        KeyCode::Esc => {
                            self.nfe_cmd = None;
                            self.status_msg = None;
                        }
                        KeyCode::Enter => {
                            let cmd = self.nfe_cmd.clone().unwrap_or_default();
                            if cmd.trim().is_empty() {
                                self.nfe_cmd = None;
                            } else {
                                // run_nfe_command clears the prompt on
                                // success and leaves it open on error.
                                self.run_nfe_command(cmd.trim());
                            }
                        }
                        KeyCode::Backspace => self.prompt_backspace(),
                        KeyCode::Delete => self.prompt_delete(),
                        KeyCode::Left => self.prompt_move_left(),
                        KeyCode::Right => self.prompt_accept_or_right(),
                        KeyCode::Home => self.prompt_home(),
                        KeyCode::End => self.prompt_accept_or_end(),
                        KeyCode::Tab => self.prompt_tab(),
                        KeyCode::Char(c) => self.prompt_push(c),
                        _ => {}
                    }
                    self.mark_all_dirty();
                    return Ok(());
                }
                // Pending delete confirmation takes over all keys.
                if self.nfe_confirm.is_some() {
                    let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                    self.handle_nfe_confirm_key(code, ctrl);
                    self.mark_all_dirty();
                    return Ok(());
                }
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let total = self.sidebar.nodes.len();
                let step = |sel: usize, delta: isize| -> usize {
                    if total == 0 {
                        return 0;
                    }
                    ((sel as isize + delta).clamp(0, total as isize - 1)) as usize
                };
                match (ctrl, code) {
                    (false, KeyCode::Esc) => self.exit_nfe(),
                    (true, KeyCode::Char('q')) => self.exit_nfe(),
                    (false, KeyCode::Char('e')) => self.exit_nfe(),
                    (false, KeyCode::Char('i')) | (false, KeyCode::Char('I')) => {
                        self.nfe_to_insert()
                    }
                    (false, KeyCode::Char('v')) | (false, KeyCode::Char('V')) => {
                        self.nfe_to_visual()
                    }
                    (false, KeyCode::Char(':')) | (false, KeyCode::Char(';')) => {
                        self.nfe_cmd = Some(String::new());
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::Enter) => self.nfe_activate(),
                    (false, KeyCode::Char('n')) | (false, KeyCode::Char('N')) => {
                        let base = self.nfe_box_base_dir();
                        self.open_nfe_box(NfeBox::new_file(base));
                    }
                    (false, KeyCode::Char('m')) | (false, KeyCode::Char('M')) => {
                        let base = self.nfe_box_base_dir();
                        self.open_nfe_box(NfeBox::new_dir(base));
                    }
                    (false, KeyCode::Char('r')) | (false, KeyCode::Char('R')) => {
                        match self.sidebar.nodes.get(self.sidebar.selected) {
                            Some(node) if node.name != ".." => {
                                let path = node.path.clone();
                                self.open_nfe_box(NfeBox::rename(path));
                            }
                            _ => {
                                self.status_msg = Some(t!("editor.nfe_rename_root"));
                                self.mark_all_dirty();
                            }
                        }
                    }
                    (false, KeyCode::Char('k'))
                    | (false, KeyCode::Char('K'))
                    | (false, KeyCode::Char('w'))
                    | (false, KeyCode::Char('W'))
                    | (false, KeyCode::Up) => {
                        self.sidebar.selected = step(self.sidebar.selected, -1);
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::Char('j'))
                    | (false, KeyCode::Char('J'))
                    | (false, KeyCode::Char('s'))
                    | (false, KeyCode::Char('S'))
                    | (false, KeyCode::Down) => {
                        self.sidebar.selected = step(self.sidebar.selected, 1);
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::Char('h'))
                    | (false, KeyCode::Char('H'))
                    | (false, KeyCode::Char('a'))
                    | (false, KeyCode::Char('A'))
                    | (false, KeyCode::Left) => {
                        // Collapse: move to parent dir when possible.
                        self.nfe_collapse();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::Char('l'))
                    | (false, KeyCode::Char('L'))
                    | (false, KeyCode::Right) => {
                        self.nfe_activate();
                    }
                    (false, KeyCode::Char('d')) | (false, KeyCode::Char('D')) => {
                        // Delete selected entry (with confirm; `l` moves).
                        self.request_nfe_delete();
                    }
                    (false, KeyCode::Home) => {
                        self.sidebar.selected = 0;
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::End) => {
                        if total > 0 {
                            self.sidebar.selected = total - 1;
                            self.nfe_ensure_visible();
                            self.mark_all_dirty();
                        }
                    }
                    (false, KeyCode::PageUp) => {
                        let rows = self.term_h.saturating_sub(4).max(1);
                        self.sidebar.selected = step(self.sidebar.selected, -(rows as isize));
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(4).max(1);
                        self.sidebar.selected = step(self.sidebar.selected, rows as isize);
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
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

    /// Directory a box-created entry goes into: the selected directory,
    /// else the selected file's parent, else the sidebar root.
    fn nfe_box_base_dir(&self) -> std::path::PathBuf {
        if let Some(node) = self.sidebar.nodes.get(self.sidebar.selected) {
            // The `..` pseudo-entry creates in the sidebar root, not above it.
            if node.name == ".." {
                return self.sidebar.root.clone();
            }
            if node.is_dir {
                return node.path.clone();
            }
            if let Some(parent) = node.path.parent() {
                return parent.to_path_buf();
            }
        }
        self.sidebar.root.clone()
    }

    /// Show the `n`/`m`/`r` input box.
    fn open_nfe_box(&mut self, b: NfeBox) {
        self.nfe_box = Some(b);
        self.status_msg = None;
        self.mark_all_dirty();
    }

    /// Keys while the input box is open: Esc cancels, Enter applies (or
    /// cancels when the line is blank), the rest edit the line.
    fn handle_nfe_box_key(&mut self, code: KeyCode, ctrl: bool) {
        match code {
            KeyCode::Esc => {
                self.nfe_box = None;
                self.status_msg = None;
                return;
            }
            KeyCode::Enter => {
                let blank = self
                    .nfe_box
                    .as_ref()
                    .map(|b| b.buf.trim().is_empty())
                    .unwrap_or(true);
                if blank {
                    self.nfe_box = None;
                    self.status_msg = None;
                } else {
                    self.accept_nfe_box();
                }
                return;
            }
            _ => {}
        }
        let Some(b) = self.nfe_box.as_mut() else {
            return;
        };
        match code {
            KeyCode::Backspace => b.backspace(),
            KeyCode::Delete => b.delete(),
            KeyCode::Left => b.move_left(),
            KeyCode::Right => b.move_right(),
            KeyCode::Home => b.cursor = 0,
            KeyCode::End => b.cursor = b.buf.len(),
            KeyCode::Char(c) if !ctrl => {
                let mut small = [0u8; 4];
                b.insert(c.encode_utf8(&mut small));
            }
            _ => {}
        }
    }

    /// Apply the input box: create a file/dir (files get the `ne --init`
    /// template) or rename the selected entry. Failures leave the box open
    /// so the name can be fixed.
    fn accept_nfe_box(&mut self) {
        let Some(b) = self.nfe_box.take() else {
            return;
        };
        let name = b.buf.trim().to_string();
        match b.kind {
            NfeBoxKind::NewFile | NfeBoxKind::NewDir => {
                let path = match resolve_new_path(&b.base, &name) {
                    Ok(p) => p,
                    Err(msg) => {
                        self.status_msg = Some(msg);
                        self.nfe_box = Some(b);
                        return;
                    }
                };
                if path.exists() {
                    self.status_msg = Some(t!("editor.nfe_create_exists", name));
                    self.nfe_box = Some(b);
                    return;
                }
                let is_file = b.kind == NfeBoxKind::NewFile;
                let created = if is_file {
                    matches!(crate::editor::init_file(&path), Ok(true))
                } else {
                    std::fs::create_dir_all(&path).is_ok()
                };
                if !created {
                    self.status_msg = Some(t!("editor.nfe_create_failed", name));
                    self.nfe_box = Some(b);
                    return;
                }
                self.sidebar.invalidate_cache();
                self.sidebar.rebuild_tree();
                self.nfe_select_path(&path);
                self.status_msg = Some(if is_file {
                    t!("editor.nfe_created_file", name)
                } else {
                    t!("editor.nfe_created_dir", name)
                });
            }
            NfeBoxKind::Rename => {
                let Some(old) = b.target.clone() else {
                    return;
                };
                if !old.exists() {
                    self.status_msg = Some(t!("editor.nfe_rm_not_exist", name));
                    return;
                }
                let new = match resolve_new_path(&b.base, &name) {
                    Ok(p) => p,
                    Err(msg) => {
                        self.status_msg = Some(msg);
                        self.nfe_box = Some(b);
                        return;
                    }
                };
                if new == old {
                    return;
                }
                if new.exists() {
                    self.status_msg = Some(t!("editor.nfe_create_exists", name));
                    self.nfe_box = Some(b);
                    return;
                }
                if let Err(e) = std::fs::rename(&old, &new) {
                    self.status_msg = Some(t!("editor.nfe_rename_failed", e));
                    self.nfe_box = Some(b);
                    return;
                }
                self.rename_tab_paths(&old, &new);
                self.sidebar.invalidate_cache();
                self.sidebar.rebuild_tree();
                self.nfe_select_path(&new);
                self.status_msg = Some(t!("editor.nfe_renamed", name));
            }
        }
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }

    /// Keep open tabs pointing at a renamed entry (and everything below a
    /// renamed directory). Buffers, cursors and undo stacks are untouched.
    fn rename_tab_paths(&mut self, old: &std::path::Path, new: &std::path::Path) {
        fn rewrite(
            p: &std::path::Path,
            old: &std::path::Path,
            new: &std::path::Path,
        ) -> Option<std::path::PathBuf> {
            if p == old {
                return Some(new.to_path_buf());
            }
            p.strip_prefix(old).ok().map(|rest| new.join(rest))
        }
        let mut active_moved = false;
        if let Some(p) = rewrite(&self.path, old, new) {
            self.path = p;
            active_moved = true;
        }
        for tab in &mut self.tabs {
            if let Some(p) = rewrite(&tab.path, old, new) {
                tab.path = p;
            }
        }
        // A rename can change the extension: re-pick the syntax language.
        if active_moved {
            self.syntax = crate::syntax::SyntaxHighlighter::for_path(&self.path);
        }
    }

    /// Collapse the current node (or jump to its parent directory).
    fn nfe_collapse(&mut self) {
        if self.sidebar.nodes.is_empty() {
            return;
        }
        let idx = self.sidebar.selected.min(self.sidebar.nodes.len() - 1);
        let node = self.sidebar.nodes[idx].clone();
        if node.is_dir && node.expanded {
            self.sidebar.toggle_expand(idx);
        } else if node.depth > 0 {
            // Walk up to the nearest ancestor directory node.
            for i in (0..idx).rev() {
                if self.sidebar.nodes[i].is_dir && self.sidebar.nodes[i].depth < node.depth {
                    self.sidebar.selected = i;
                    break;
                }
            }
        }
        self.nfe_ensure_visible();
        self.mark_all_dirty();
    }
}

/// Resolve an `n`/`m` name against the sidebar root. Rejects empty
/// names and absolute paths (everything stays root-relative).
/// Returns the localized usage/error message on failure.
pub(crate) fn resolve_new_path(
    root: &std::path::Path,
    name: &str,
) -> Result<std::path::PathBuf, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(t!("editor.nfe_new_usage"));
    }
    let rel = std::path::Path::new(name);
    if rel.is_absolute() {
        return Err(t!("editor.nfe_new_usage"));
    }
    Ok(root.join(rel))
}

/// Parse an `nfe:size` argument: `full` (=100%), `def` (=default width),
/// or `N%` / `N` (1-100). Returns the percent to store (`None` = default).
fn parse_nfe_size(arg: &str) -> Option<Option<u8>> {
    let low = arg.trim().to_lowercase();
    if low == "full" {
        return Some(Some(100));
    }
    if low == "def" || low == "default" {
        return Some(None);
    }
    let num = low.strip_suffix('%').unwrap_or(&low);
    match num.trim().parse::<u8>() {
        Ok(n) if (1..=100).contains(&n) => Some(Some(n)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_nfe_size, resolve_new_path};
    use crate::editor::{Mode, NfeBox};
    use crossterm::event::{
        Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    };

    fn nfe_test_editor(root: &std::path::Path) -> crate::editor::Editor {
        let path = root.join("probe.txt");
        std::fs::write(&path, "probe").unwrap();
        let mut editor = crate::editor::Editor::new(&path).unwrap();
        editor.sidebar.root = root.to_path_buf();
        editor
    }

    fn temp_root(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "ntc_nfe_test_{}_{}",
            tag,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn resolve_new_path_rules() {
        let root = std::path::PathBuf::from("/tmp/ntc-root");
        assert_eq!(
            resolve_new_path(&root, "a.txt").unwrap(),
            root.join("a.txt")
        );
        assert_eq!(
            resolve_new_path(&root, "sub/dir").unwrap(),
            root.join("sub/dir")
        );
        assert!(resolve_new_path(&root, "").is_err());
        assert!(resolve_new_path(&root, "   ").is_err());
        let abs = std::env::temp_dir();
        assert!(resolve_new_path(&root, &abs.to_string_lossy()).is_err());
    }

    #[test]
    fn nfe_prompt_creates_template_without_opening_tab() {
        let root = temp_root("file");
        let mut editor = nfe_test_editor(&root);
        let original_path = editor.path.clone();
        editor.nfe_cmd = Some("n hello.rs".to_string());
        editor.run_nfe_command("n hello.rs");
        assert!(root.join("hello.rs").is_file());
        assert!(std::fs::read_to_string(root.join("hello.rs"))
            .unwrap()
            .contains("fn main"));
        assert_eq!(editor.path, original_path);
        assert!(editor.tabs.is_empty());
        assert!(editor.nfe_cmd.is_none());
    }

    #[test]
    fn nfe_prompt_creates_nested_parents() {
        let root = temp_root("nested");
        let mut editor = nfe_test_editor(&root);
        editor.run_nfe_command("n sub/dir/f.txt");
        assert!(root.join("sub/dir/f.txt").is_file());
        assert!(editor.nfe_cmd.is_none());
    }

    #[test]
    fn nfe_prompt_creates_dir() {
        let root = temp_root("dir");
        let mut editor = nfe_test_editor(&root);
        editor.run_nfe_command("m assets/sfx");
        assert!(root.join("assets/sfx").is_dir());
        assert!(editor.nfe_cmd.is_none());
    }

    #[test]
    fn nfe_prompt_existing_name_stays_open() {
        let root = temp_root("exists");
        // probe.txt exists → error, prompt stays open.
        let mut editor = nfe_test_editor(&root);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("n probe.txt");
        assert!(editor.nfe_cmd.is_some());
        assert!(editor.status_msg.is_some());
        let mut editor = nfe_test_editor(&root);
        std::fs::create_dir_all(root.join("taken")).unwrap();
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("m taken");
        assert!(editor.nfe_cmd.is_some());
    }

    #[test]
    fn nfe_prompt_rmf_removes_file_and_closes_tab() {
        let root = temp_root("rmf-close");
        let mut editor = nfe_test_editor(&root);
        editor.init_tabs();
        let target = root.join("gone.txt");
        std::fs::write(&target, "x").unwrap();
        editor.open_file(&target);
        assert!(editor.tabs.iter().any(|t| t.path.as_path() == target));
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("rmf gone.txt");
        assert!(!target.exists());
        assert!(editor.tabs.iter().all(|t| t.path.as_path() != target));
        assert!(editor.nfe_cmd.is_none());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_prompt_rmd_removes_tree_and_open_tabs_inside() {
        let root = temp_root("rmd-tree");
        let mut editor = nfe_test_editor(&root);
        editor.init_tabs();
        let inner = root.join("sub").join("f.txt");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(&inner, "x").unwrap();
        editor.open_file(&inner);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("rmd sub");
        assert!(!root.join("sub").exists());
        assert!(editor
            .tabs
            .iter()
            .all(|t| !t.path.starts_with(&root.join("sub"))));
        assert!(editor.nfe_cmd.is_none());
    }

    #[test]
    fn nfe_prompt_rm_strictness_errors() {
        let root = temp_root("rm-strict");
        let mut editor = nfe_test_editor(&root);
        std::fs::create_dir_all(root.join("adir")).unwrap();
        // rmd on a file → error, file stays.
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("rmd probe.txt");
        assert!(root.join("probe.txt").is_file());
        assert!(editor.nfe_cmd.is_some());
        // rmf on a dir → error, dir stays.
        editor.run_nfe_command("rmf adir");
        assert!(root.join("adir").is_dir());
        assert!(editor.nfe_cmd.is_some());
        // Missing target → error.
        editor.run_nfe_command("rmf ghost.txt");
        assert!(editor.nfe_cmd.is_some());
        editor.run_nfe_command("rmd ghost");
        assert!(editor.nfe_cmd.is_some());
        // Bad arity → usage.
        editor.run_nfe_command("rmd");
        assert!(editor.status_msg.is_some());
        editor.run_nfe_command("rmf a b");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_prompt_rm_aborts_on_unsaved_edits() {
        let root = temp_root("rm-unsaved");
        let mut editor = nfe_test_editor(&root);
        editor.init_tabs();
        let target = root.join("dirty.txt");
        std::fs::write(&target, "clean").unwrap();
        editor.open_file(&target);
        editor.lines = vec!["dirty edit".to_string()];
        editor.modified = true;
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("rmf dirty.txt");
        assert!(target.is_file());
        assert!(editor.nfe_cmd.is_some());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_prompt_bad_arity_shows_usage() {
        let root = temp_root("arity");
        let mut editor = nfe_test_editor(&root);
        editor.run_nfe_command("n");
        assert!(editor.status_msg.is_some());
        editor.run_nfe_command("n a b");
        assert!(editor.status_msg.is_some());
        editor.run_nfe_command("zzz nope");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn size_grammar() {
        assert_eq!(parse_nfe_size("full"), Some(Some(100)));
        assert_eq!(parse_nfe_size("def"), Some(None));
        assert_eq!(parse_nfe_size("default"), Some(None));
        assert_eq!(parse_nfe_size("50%"), Some(Some(50)));
        assert_eq!(parse_nfe_size("1"), Some(Some(1)));
        assert_eq!(parse_nfe_size("100"), Some(Some(100)));
        assert_eq!(parse_nfe_size("0"), None);
        assert_eq!(parse_nfe_size("101"), None);
        assert_eq!(parse_nfe_size("abc"), None);
        assert_eq!(parse_nfe_size(""), None);
    }

    fn nfe_key(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    fn nfe_focused_editor(root: &std::path::Path, files: usize) -> crate::editor::Editor {
        for i in 0..files {
            std::fs::write(root.join(format!("f{:02}.txt", i)), "x").unwrap();
        }
        let mut editor = nfe_test_editor(root);
        editor.sidebar.root = root.to_path_buf();
        editor.sidebar.rebuild_tree();
        editor.sidebar.open = true;
        editor.mode = Mode::Nfe;
        editor.term_h = 10;
        editor.term_w = 40;
        editor
    }

    #[test]
    fn nfe_keys_jump_to_insert_and_visual() {
        let root = temp_root("keys");
        let mut editor = nfe_focused_editor(&root, 2);
        let mut out = std::io::stdout();
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('i')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Insert);
        assert!(!editor.sidebar.open);

        let mut editor = nfe_focused_editor(&root, 2);
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('V')), &mut out)
            .unwrap();
        assert_eq!(editor.mode, Mode::Visual);
        assert!(!editor.sidebar.open);
    }

    #[test]
    fn nfe_prompt_number_jumps_to_entry() {
        let root = temp_root("jump-number");
        let mut editor = nfe_focused_editor(&root, 4);
        let total = editor.sidebar.nodes.len();
        assert!(total > 2);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("2");
        assert_eq!(editor.sidebar.selected, 1);
        assert!(editor.nfe_cmd.is_none());
        assert!(editor.status_msg.is_some());
        // Out-of-range numbers keep the prompt open for a fix.
        editor.nfe_cmd = Some(String::new());
        let too_big = (total + 1).to_string();
        editor.run_nfe_command(&too_big);
        assert!(editor.nfe_cmd.is_some());
        assert_eq!(editor.sidebar.selected, 1);
    }

    #[test]
    fn nfe_prompt_tab_cycles_and_right_accepts_the_hint() {
        let root = temp_root("tab-hint");
        let mut editor = nfe_focused_editor(&root, 2);
        let mut out = std::io::stdout();
        editor.nfe_cmd = Some("si".to_string());
        editor.prompt_cursor = 2;
        editor
            .handle_nfe_event(nfe_key(KeyCode::Tab), &mut out)
            .unwrap();
        // Tab only moves the hint; Right commits it.
        assert_eq!(editor.nfe_cmd.as_deref(), Some("si"));
        assert_eq!(editor.prompt_ghost(), Some("ze ".to_string()));
        editor
            .handle_nfe_event(nfe_key(KeyCode::Right), &mut out)
            .unwrap();
        assert_eq!(editor.nfe_cmd.as_deref(), Some("size "));
    }

    /// The sidebar tree rebuilds from CWD (a process-global the rest of the
    /// suite shares), so box tests build the box directly against a temp
    /// root instead of relying on the selected row.
    fn box_editor() -> (std::path::PathBuf, crate::editor::Editor) {
        let root = temp_root("box");
        let mut editor = nfe_test_editor(&root);
        std::fs::write(root.join("f00.txt"), "x").unwrap();
        editor.sidebar.root = root.clone();
        editor.mode = Mode::Nfe;
        editor.sidebar.open = true;
        (root, editor)
    }

    #[test]
    fn nfe_box_edits_multibyte_names() {
        let mut b = NfeBox::new_file(std::path::PathBuf::from("."));
        b.insert("αβ");
        b.insert("γ");
        assert_eq!(b.buf, "αβγ");
        b.move_left();
        b.backspace();
        assert_eq!(b.buf, "αγ");
        b.delete();
        assert_eq!(b.buf, "α");
        b.insert("x");
        assert_eq!(b.buf, "αx");
    }

    #[test]
    fn nfe_box_keys_open_the_right_box() {
        let root = temp_root("box-keys");
        let mut editor = nfe_focused_editor(&root, 2);
        let mut out = std::io::stdout();
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('n')), &mut out)
            .unwrap();
        assert_eq!(
            editor.nfe_box.as_ref().map(|b| b.kind),
            Some(crate::editor::NfeBoxKind::NewFile)
        );
        // While the box is open, letters edit it instead of opening another.
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('m')), &mut out)
            .unwrap();
        assert_eq!(
            editor.nfe_box.as_ref().map(|b| b.kind),
            Some(crate::editor::NfeBoxKind::NewFile)
        );
        assert_eq!(editor.nfe_box.as_ref().map(|b| b.buf.as_str()), Some("m"));
        editor.nfe_box = None;
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('m')), &mut out)
            .unwrap();
        assert_eq!(
            editor.nfe_box.as_ref().map(|b| b.kind),
            Some(crate::editor::NfeBoxKind::NewDir)
        );
        // The `..` row cannot be renamed.
        editor.nfe_box = None;
        editor.sidebar.selected = 0;
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('r')), &mut out)
            .unwrap();
        assert!(editor.nfe_box.is_none());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_box_creates_file_with_template() {
        let (root, mut editor) = box_editor();
        let mut out = std::io::stdout();
        editor.nfe_box = Some(NfeBox::new_file(root.clone()));
        for c in "new_box.rs".chars() {
            editor
                .handle_nfe_event(nfe_key(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        assert_eq!(
            editor.nfe_box.as_ref().map(|b| b.buf.as_str()),
            Some("new_box.rs")
        );
        editor
            .handle_nfe_event(nfe_key(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(
            editor.nfe_box.is_none(),
            "box: {:?} status: {:?}",
            editor
                .nfe_box
                .as_ref()
                .map(|b| (b.base.clone(), b.buf.clone())),
            editor.status_msg
        );
        let created = root.join("new_box.rs");
        assert!(created.is_file(), "status: {:?}", editor.status_msg);
        // `ne --init` template behavior: the file is not empty.
        assert!(!std::fs::read_to_string(&created).unwrap().is_empty());
    }

    #[test]
    fn nfe_box_cancel_paths_and_errors() {
        let (root, mut editor) = box_editor();
        let mut out = std::io::stdout();
        // Esc cancels.
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('n')), &mut out)
            .unwrap();
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('x')), &mut out)
            .unwrap();
        editor
            .handle_nfe_event(nfe_key(KeyCode::Esc), &mut out)
            .unwrap();
        assert!(editor.nfe_box.is_none());
        assert!(!root.join("x").exists());
        // An empty Enter cancels too.
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('m')), &mut out)
            .unwrap();
        editor
            .handle_nfe_event(nfe_key(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(editor.nfe_box.is_none());
        assert!(!root.join("mydir").exists());
        // A name that already exists keeps the box open with a message.
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('n')), &mut out)
            .unwrap();
        for c in "f00.txt".chars() {
            editor
                .handle_nfe_event(nfe_key(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_nfe_event(nfe_key(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(editor.nfe_box.is_some());
        assert!(editor.status_msg.is_some());
        // Typing a dir name creates a directory.
        editor.nfe_box = None;
        editor
            .handle_nfe_event(nfe_key(KeyCode::Char('m')), &mut out)
            .unwrap();
        for c in "mydir".chars() {
            editor
                .handle_nfe_event(nfe_key(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_nfe_event(nfe_key(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(root.join("mydir").is_dir());
    }

    #[test]
    fn nfe_box_rename_updates_files_and_tabs() {
        let (root, mut editor) = box_editor();
        editor.init_tabs();
        editor.open_file(&root.join("f00.txt"));
        let old = root.join("f00.txt");
        let mut out = std::io::stdout();
        editor.nfe_box = Some(NfeBox::rename(old.clone()));
        let b = editor.nfe_box.as_ref().expect("rename box open");
        // The box starts with the current name, caret at the end.
        assert_eq!(b.buf, "f00.txt");
        assert_eq!(b.cursor, 7);
        for _ in 0..7 {
            editor
                .handle_nfe_event(nfe_key(KeyCode::Backspace), &mut out)
                .unwrap();
        }
        for c in "renamed.txt".chars() {
            editor
                .handle_nfe_event(nfe_key(KeyCode::Char(c)), &mut out)
                .unwrap();
        }
        editor
            .handle_nfe_event(nfe_key(KeyCode::Enter), &mut out)
            .unwrap();
        assert!(editor.nfe_box.is_none());
        assert!(!old.exists());
        let new = root.join("renamed.txt");
        assert!(new.is_file());
        // Open tabs follow the rename; the buffer keeps its contents.
        assert_eq!(editor.path, new);
        assert!(editor.tabs.iter().any(|t| t.path == new));
        assert!(editor.tabs.iter().all(|t| t.path != old));
    }

    #[test]
    fn nfe_prompt_tab_commands_and_close() {
        let root = temp_root("tab-cmds");
        let mut editor = nfe_test_editor(&root);
        std::fs::write(root.join("t1.txt"), "1").unwrap();
        std::fs::write(root.join("t2.txt"), "2").unwrap();
        editor.init_tabs();
        editor.open_file(&root.join("t1.txt"));
        editor.open_file(&root.join("t2.txt"));
        let last = editor.tabs.len() - 1;
        assert_eq!(editor.active_tab, last);

        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("t1");
        assert_eq!(editor.active_tab, 0);
        assert!(editor.nfe_cmd.is_none());

        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("t");
        assert_eq!(editor.active_tab, 1);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("T");
        assert_eq!(editor.active_tab, 0);

        // Out of range keeps the prompt open for a fix.
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("t9");
        assert!(editor.nfe_cmd.is_some());
        assert_eq!(editor.active_tab, 0);

        // `qt` closes the current tab, like gmode.
        let before = editor.tabs.len();
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("qt");
        assert_eq!(editor.tabs.len(), before - 1);
        assert!(editor.nfe_cmd.is_none());
    }

    #[test]
    fn nfe_prompt_i_v_switch_modes() {
        let root = temp_root("prompt-modes");
        let mut editor = nfe_focused_editor(&root, 2);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("i");
        assert_eq!(editor.mode, Mode::Insert);
        assert!(!editor.sidebar.open);
        assert!(editor.nfe_cmd.is_none());

        let mut editor = nfe_focused_editor(&root, 2);
        editor.nfe_cmd = Some(String::new());
        editor.run_nfe_command("V");
        assert_eq!(editor.mode, Mode::Visual);
        assert!(!editor.sidebar.open);
        assert!(editor.nfe_cmd.is_none());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn nfe_scrollbar_math() {
        use crate::editor::Editor;
        // 100 nodes over 10 rows: thumb height 1, full span.
        assert_eq!(Editor::nfe_scroll_for_row(0, 100, 10), 0);
        assert_eq!(Editor::nfe_scroll_for_row(9, 100, 10), 99);
        assert_eq!(Editor::nfe_scroll_for_row(4, 100, 10), 44);
        // Rows beyond the track clamp; degenerate inputs yield 0.
        assert_eq!(Editor::nfe_scroll_for_row(99, 100, 10), 99);
        assert_eq!(Editor::nfe_scroll_for_row(5, 3, 10), 0);
        assert_eq!(Editor::nfe_scroll_for_row(0, 0, 10), 0);
    }

    #[test]
    fn nfe_scrollbar_mouse_click_drag_release() {
        let root = temp_root("mouse-bar");
        let mut editor = nfe_focused_editor(&root, 30);
        let total = editor.sidebar.nodes.len();
        assert!(total > editor.term_h);
        let (sb_col, rows) = editor.nfe_scrollbar_geom().unwrap();
        assert_eq!(rows, 10);
        let before = editor.sidebar.selected;

        let click = |col: usize, row: usize| MouseEvent {
            kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: col as u16,
            row: row as u16,
            modifiers: KeyModifiers::NONE,
        };
        // Click near the bottom of the bar jumps deep into the tree...
        editor.handle_mouse(click(sb_col, 9));
        let deep = crate::editor::Editor::nfe_scroll_for_row(9, total, rows);
        assert_eq!(editor.sidebar.scroll, deep);
        assert!(deep > 0);
        assert!(editor.sidebar.dragging_scroll);
        // ...without moving the keyboard selection.
        assert_eq!(editor.sidebar.selected, before);
        // Drag continues the gesture.
        editor.handle_mouse(MouseEvent {
            kind: MouseEventKind::Drag(crossterm::event::MouseButton::Left),
            column: sb_col as u16,
            row: 2,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            editor.sidebar.scroll,
            crate::editor::Editor::nfe_scroll_for_row(2, total, rows)
        );
        // Release ends the gesture.
        editor.handle_mouse(MouseEvent {
            kind: MouseEventKind::Up(crossterm::event::MouseButton::Left),
            column: sb_col as u16,
            row: 2,
            modifiers: KeyModifiers::NONE,
        });
        assert!(!editor.sidebar.dragging_scroll);
    }

    #[test]
    fn nfe_wheel_over_sidebar_scrolls_tree() {
        let root = temp_root("mouse-wheel");
        let mut editor = nfe_focused_editor(&root, 30);
        assert_eq!(editor.sidebar.scroll, 0);
        // ScrollDown over the sidebar steps the tree, not the text field.
        let text_off = editor.sv.offset;
        editor.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 0,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(editor.sidebar.scroll, 3);
        assert_eq!(editor.sv.offset, text_off);
        editor.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(editor.sidebar.scroll, 0);
        // Wheel over the text field leaves the sidebar alone.
        let eo = editor.editor_offset();
        editor.handle_mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: (eo + 5) as u16,
            row: 5,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(editor.sidebar.scroll, 0);
    }
}
