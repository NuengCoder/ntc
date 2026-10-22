use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};
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
    /// `m <dir>`, `rmd`/`rmf`, `i` (insert), `v` (visual). Clears the
    /// prompt on success so navigation resumes; leaves it open on error.
    fn run_nfe_command(&mut self, cmd: &str) {
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
                self.sidebar.rebuild_tree();
                self.nfe_select_path(&path);
                self.status_msg = Some(t!("editor.nfe_created_dir", name));
                self.nfe_cmd = None;
                self.nfe_ensure_visible();
                self.mark_all_dirty();
            }
        }
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
                // Command-prompt input capture takes over all keys.
                if self.nfe_cmd.is_some() {
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
                        KeyCode::Backspace => {
                            if let Some(buf) = self.nfe_cmd.as_mut() {
                                buf.pop();
                            }
                        }
                        KeyCode::Char(c) => {
                            if let Some(buf) = self.nfe_cmd.as_mut() {
                                buf.push(c);
                            }
                        }
                        _ => {}
                    }
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
                    | (false, KeyCode::Char('d'))
                    | (false, KeyCode::Char('D'))
                    | (false, KeyCode::Right) => {
                        self.nfe_activate();
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
    use crate::editor::Mode;
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
