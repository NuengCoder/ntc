use super::{Editor, Mode};
use crate::navigator::Navigator;
use crate::t;
use crate::teleport::TeleportManager;
use anyhow::Result;

impl Editor {
    pub(super) fn list_subdirs(&self) -> Vec<String> {
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

    pub(super) fn cd_to(&mut self, path: &std::path::Path) {
        match Navigator::new() {
            Ok(mut nav) => {
                let target = if path.to_string_lossy() == "-" {
                    match nav.current_path().parent() {
                        Some(p) => p.to_path_buf(),
                        None => {
                            self.status_msg = Some(t!("editor.cmd_already_at_root").into());
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
                    }
                    Err(e) => {
                        self.status_msg = Some(format!("{} {}", t!("editor.cmd_cd_prefix"), e));
                    }
                }
            }
            Err(_) => {
                self.status_msg = Some(t!("editor.cmd_no_cwd").into());
            }
        }
    }

    pub(super) fn execute_command(
        &mut self,
        cmd: &str,
        stdout: &mut std::io::Stdout,
    ) -> Result<bool> {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return Ok(false);
        }

        if let Ok(n) = cmd.parse::<usize>() {
            if n > 0 {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor.y = target;
                self.cursor.byte = 0;
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_jumped_to_line"), n));
                return Ok(false);
            }
        }

        if let Some(rest) = cmd.strip_prefix("l ") {
            if let Ok(n) = rest.trim().parse::<usize>() {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor.y = target;
                self.cursor.byte = 0;
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_jumped_to_line"), n));
                return Ok(false);
            }
        }
        if let Some(rest) = cmd.strip_prefix("line ") {
            if let Ok(n) = rest.trim().parse::<usize>() {
                let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                self.cursor.y = target;
                self.cursor.byte = 0;
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_jumped_to_line"), n));
                return Ok(false);
            }
        }

        if cmd == "git diff" || cmd == "gd" {
            if crate::config::Config::global_get_git_integration_enabled() {
                let dir = self
                    .path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let lines = crate::utils::git::diff_of_file(&dir, &self.path);
                match lines {
                    Some(text) if !text.is_empty() => {
                        self.git_diff = Some(crate::editor::types::GitDiffState {
                            lines: text.lines().map(|l| l.to_string()).collect(),
                            scroll: 0,
                        });
                        self.mode = Mode::GitDiff;
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    _ => self.status_msg = Some(t!("editor.git_no_diff").into()),
                }
            } else {
                self.status_msg = Some(t!("editor.git_disabled").into());
            }
            return Ok(false);
        }

        if cmd == "git blame" || cmd == "blame" {
            if crate::config::Config::global_get_git_integration_enabled() {
                let dir = self
                    .path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                match crate::utils::git::blame_line(&dir, &self.path, self.cursor.y + 1) {
                    Some(info) => {
                        self.status_msg = Some(format!(
                            "{} {}  {}  {}",
                            info.commit, info.author, info.time, info.summary
                        ))
                    }
                    None => self.status_msg = Some(t!("editor.git_no_blame").into()),
                }
            } else {
                self.status_msg = Some(t!("editor.git_disabled").into());
            }
            return Ok(false);
        }

        if let Some(rest) = cmd.strip_prefix("goto ") {
            let rest = rest.trim();
            let addr = if let Some(h) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X"))
            {
                usize::from_str_radix(h, 16).ok()
            } else {
                rest.parse::<usize>().ok()
            };
            match addr {
                Some(a) => {
                    // Normal line jump for decimal
                    let target = a.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                    self.cursor.y = target;
                    self.cursor.byte = 0;
                    self.status_msg = Some(format!("{} {}", t!("editor.cmd_jumped_to_line"), a));
                }
                None => {
                    self.status_msg = Some(t!("editor.cmd_goto_usage").into());
                }
            }
            return Ok(false);
        }

        if cmd == "q!" || cmd == "quit!" {
            return self.exit_flow(stdout);
        }

        if cmd == "q" || cmd == "quit" {
            self.mode = Mode::Normal;
            return self.exit_flow(stdout);
        }

        if cmd == "wq" {
            self.mode = Mode::Normal;
            if let Err(e) = self.save() {
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_save_failed"), e));
                return Ok(false);
            }
            return self.exit_flow(stdout);
        }

        if cmd == "w" || cmd == "write" {
            match self.save() {
                Ok(()) => {
                    self.status_msg = Some(t!("editor.cmd_saved").into());
                }
                Err(e) => {
                    self.status_msg = Some(format!("{} {}", t!("editor.cmd_save_failed"), e));
                }
            }
            return Ok(false);
        }

        if cmd == "u" {
            self.undo();
            return Ok(false);
        }

        if cmd == "r" {
            self.redo();
            return Ok(false);
        }

        if let Some(rest) = cmd.strip_prefix('!') {
            self.run_in_terminal(rest);
            return Ok(false);
        }

        if cmd == "e!" || cmd == "reload!" || cmd == "reload" {
            if self.reload_from_disk() {
                self.status_msg = Some("Reloaded from disk".into());
            } else {
                self.status_msg = Some("Reload failed — file unreadable".into());
            }
            return Ok(false);
        }

        if cmd == "auto on" {
            self.editor_cfg.auto_save = true;
            self.editor_cfg.save();
            self.status_msg = Some(t!("editor.cmd_auto_on").into());
            return Ok(false);
        }
        if cmd == "auto off" {
            self.editor_cfg.auto_save = false;
            self.editor_cfg.save();
            self.status_msg = Some(t!("editor.cmd_auto_off").into());
            return Ok(false);
        }

        if cmd == "syntax on" {
            self.editor_cfg.syntax_enabled = true;
            self.editor_cfg.save();
            self.syntax.invalidate_all();
            self.mark_all_dirty();
            self.status_msg = Some(t!("editor.cmd_syntax_on").into());
            return Ok(false);
        }
        if cmd == "syntax off" {
            self.editor_cfg.syntax_enabled = false;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some(t!("editor.cmd_syntax_off").into());
            return Ok(false);
        }

        if cmd == "color on" {
            self.editor_cfg.color_enabled = true;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some(t!("editor.cmd_color_on").into());
            return Ok(false);
        }
        if cmd == "color off" {
            self.editor_cfg.color_enabled = false;
            self.editor_cfg.save();
            self.mark_all_dirty();
            self.status_msg = Some(t!("editor.cmd_color_off").into());
            return Ok(false);
        }

        if let Some(path) = cmd.strip_prefix("cd ") {
            let path = path.trim();
            if path.is_empty() {
            } else {
                self.cd_to(std::path::Path::new(path));
            }
            return Ok(false);
        }

        if cmd == "back" || cmd.starts_with("back ") {
            let count = if let Some(n) = cmd.strip_prefix("back ") {
                n.trim().parse::<usize>().unwrap_or(1)
            } else {
                1usize
            };
            let mut target =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
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
                self.status_msg = Some(t!("editor.cmd_already_at_root").into());
            }
            return Ok(false);
        }

        if cmd == "gosc" || cmd == "gos" {
            let dirs = self.list_subdirs();
            if dirs.is_empty() {
                self.status_msg = Some(t!("editor.cmd_no_subdirs").into());
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
                    self.status_msg = Some(t!("editor.cmd_gos_usage").into());
                    return Ok(false);
                }
            };
            let dirs = self.list_subdirs();
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            if n > dirs.len() {
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_gos_only"), dirs.len()));
            } else {
                let target = cwd.join(&dirs[n - 1]);
                self.cd_to(&target);
            }
            return Ok(false);
        }

        if let Some(pattern) = cmd.strip_prefix("gs") {
            let pattern = pattern.trim();
            if pattern.is_empty() {
                self.ff_query = "gs:".to_string();
                self.gs_state.query.clear();
                self.gs_state.results.clear();
                self.gs_state.loading = false;
                self.gs_state.loading_pattern.clear();
                self.mode = Mode::FileFinder;
                self.mark_all_dirty();
            } else {
                self.ff_query = format!("gs:{}", pattern);
                self.gs_state.query = pattern.to_string();
                self.gs_state.loading = true;
                self.gs_state.loading_pattern = pattern.to_string();
                self.gs_state.results.clear();
                self.mode = Mode::FileFinder;
                self.mark_all_dirty();
                let root =
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let matches = crate::utils::search::search_content(&root, pattern, 10);
                let mut list = crate::editor::types::GsResultList::new();
                for m in &matches {
                    let display = format!(
                        "{}:{}: {}",
                        m.file_path
                            .file_name()
                            .map(|n| n.to_string_lossy())
                            .unwrap_or_default(),
                        m.line_number,
                        m.line.trim()
                    );
                    list.push_back(display, m.file_path.clone(), m.line_number);
                }
                list.build_circular();
                self.gs_state.results = list;
                self.gs_state.loading = false;
                self.mark_all_dirty();
            }
            return Ok(false);
        }

        if let Some(dir) = cmd.strip_prefix("mkdir ") {
            let dir = dir.trim();
            if dir.is_empty() {
                self.status_msg = Some(t!("editor.cmd_mkdir_missing").into());
            } else if let Err(e) = std::fs::create_dir_all(std::path::Path::new(dir)) {
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_mkdir_failed"), e));
            } else {
                self.sidebar.rebuild_tree();
                self.mark_all_dirty();
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_created_dir"), dir));
            }
            return Ok(false);
        }

        if let Some(file) = cmd
            .strip_prefix("touch ")
            .or_else(|| cmd.strip_prefix("new "))
        {
            let file = file.trim();
            if file.is_empty() {
                self.status_msg = Some(t!("editor.cmd_touch_missing").into());
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
                            self.status_msg =
                                Some(format!("{} {}", t!("editor.cmd_touch_failed"), e));
                            return Ok(false);
                        }
                    }
                }
                self.open_file(path);
            }
            return Ok(false);
        }

        if let Some(file) = cmd.strip_prefix("mkf ") {
            let file = file.trim();
            if file.is_empty() {
                self.status_msg = Some(t!("editor.cmd_mkf_missing").into());
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
                            self.status_msg =
                                Some(format!("{} {}", t!("editor.cmd_mkf_failed"), e));
                            return Ok(false);
                        }
                    }
                }
                self.open_file(path);
            }
            return Ok(false);
        }

        if let Some(dir) = cmd.strip_prefix("mkd ") {
            let dir = dir.trim();
            if dir.is_empty() {
                self.status_msg = Some(t!("editor.cmd_mkd_missing").into());
            } else if let Err(e) = std::fs::create_dir_all(std::path::Path::new(dir)) {
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_mkd_failed"), e));
            } else {
                self.sidebar.rebuild_tree();
                self.mark_all_dirty();
                self.status_msg = Some(format!("{} {}", t!("editor.cmd_created_dir"), dir));
            }
            return Ok(false);
        }

        if let Some(target) = cmd.strip_prefix("rmd ") {
            let target = target.trim();
            if target.is_empty() {
                self.status_msg = Some(t!("editor.cmd_rmd_missing").into());
            } else {
                let path = std::path::Path::new(target);
                if !path.exists() {
                    self.status_msg = Some(format!(
                        "{} '{}' {}",
                        t!("editor.cmd_rmd"),
                        target,
                        t!("editor.cmd_not_exist")
                    ));
                } else if !path.is_dir() {
                    self.status_msg = Some(format!(
                        "{} '{}' {}",
                        t!("editor.cmd_rmd"),
                        target,
                        t!("editor.cmd_not_dir")
                    ));
                } else if let Err(e) = std::fs::remove_dir_all(path) {
                    self.status_msg = Some(format!("{} {}", t!("editor.cmd_rmd"), e));
                } else {
                    self.sidebar.rebuild_tree();
                    self.mark_all_dirty();
                    self.status_msg = Some(format!("{} {}", t!("editor.cmd_removed_dir"), target));
                }
            }
            return Ok(false);
        }

        if let Some(target) = cmd.strip_prefix("rmf ") {
            let target = target.trim();
            if target.is_empty() {
                self.status_msg = Some(t!("editor.cmd_rmf_missing").into());
            } else {
                let path = std::path::Path::new(target);
                if !path.exists() {
                    self.status_msg = Some(format!(
                        "{} '{}' {}",
                        t!("editor.cmd_rmf"),
                        target,
                        t!("editor.cmd_not_exist")
                    ));
                } else {
                    let result = if path.is_dir() {
                        std::fs::remove_dir_all(path)
                    } else {
                        std::fs::remove_file(path)
                    };
                    match result {
                        Ok(()) => {
                            self.sidebar.rebuild_tree();
                            self.mark_all_dirty();
                            self.status_msg =
                                Some(format!("{} {}", t!("editor.cmd_removed"), target));
                        }
                        Err(e) => {
                            self.status_msg =
                                Some(format!("{} {}", t!("editor.cmd_rmf_failed"), e));
                        }
                    }
                }
            }
            return Ok(false);
        }

        if let Some(file) = cmd.strip_prefix("ne").map(|s| s.trim()) {
            self.sidebar.rebuild_tree();
            self.mark_all_dirty();
            if file.is_empty() || file == "." {
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

        // `:run` always uses nerun.toml extension mapping and executes
        // in the embedded terminal. Any trailing args are ignored so
        // `:run`, `:run <anything>` both compile the current file ($CWF).
        if cmd == "run" || cmd.starts_with("run ") {
            self.start_configured_run();
            return Ok(false);
        }

        if let Some(name) = cmd.strip_prefix("tp ") {
            let name = name.trim();
            if name.is_empty() {
                let tp_list = TeleportManager::get_all();
                if tp_list.is_empty() {
                    self.status_msg = Some(t!("editor.cmd_no_tp").into());
                } else {
                    let mut msg = t!("editor.cmd_tp_list");
                    let mut sorted: Vec<_> = tp_list.into_iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(&b.0));
                    for (n, p) in &sorted {
                        msg.push_str(&format!(" {} -> {}", n, p.display()));
                    }
                    self.status_msg = Some(msg);
                }
            } else {
                match Navigator::new() {
                    Ok(mut nav) => match TeleportManager::get_path(name) {
                        Some(path) => match nav.go_to(&path) {
                            Ok(()) => {
                                self.sidebar.rebuild_tree();
                                self.mark_all_dirty();
                                self.status_msg = Some(format!(
                                    "{} '{}' -> {}",
                                    t!("editor.cmd_tp_jumped"),
                                    name,
                                    nav.display_path()
                                ));
                            }
                            Err(e) => {
                                self.status_msg =
                                    Some(format!("{} {}", t!("editor.cmd_tp_error"), e));
                            }
                        },
                        None => {
                            self.status_msg =
                                Some(format!("{} '{}'", t!("editor.cmd_tp_not_found"), name));
                        }
                    },
                    Err(_) => {
                        self.status_msg = Some(t!("editor.cmd_no_cwd").into());
                    }
                }
            }
            return Ok(false);
        }

        if let Some(name) = cmd.strip_prefix("theme ") {
            let name = name.trim();
            if name.is_empty() {
                let current = crate::utils::theme::ThemeManager::current();
                self.status_msg = Some(format!(
                    "{} {}",
                    t!("editor.cmd_current_theme"),
                    current.name
                ));
            } else if crate::utils::theme::ThemeManager::set_theme(name) {
                self.mark_all_dirty();
                self.status_msg = Some(format!("{} '{}'", t!("editor.cmd_theme_switched"), name));
            } else {
                self.status_msg = Some(format!(
                    "{} '{}'. {}",
                    t!("editor.cmd_theme_not_found"),
                    name,
                    t!("editor.cmd_theme_list_hint")
                ));
            }
            return Ok(false);
        }
        if cmd == "theme list" || cmd == "theme ls" {
            let themes = crate::utils::theme::ThemeManager::list_themes();
            let mut msg = t!("editor.cmd_themes");
            for t in &themes {
                msg.push_str(&format!(" {}", t));
            }
            self.status_msg = Some(msg);
            return Ok(false);
        }

        if let Some(name) = cmd.strip_prefix("tp-add ") {
            let name = name.trim();
            if name.is_empty() {
                self.status_msg = Some(t!("editor.cmd_tp_add_missing").into());
            } else {
                match Navigator::new() {
                    Ok(nav) => match TeleportManager::add_current(&nav, name) {
                        Ok(()) => {
                            self.status_msg =
                                Some(format!("{} '{}'", t!("editor.cmd_tp_added"), name));
                        }
                        Err(e) => {
                            self.status_msg =
                                Some(format!("{} {}", t!("editor.cmd_tp_add_error"), e));
                        }
                    },
                    Err(_) => {
                        self.status_msg = Some(t!("editor.cmd_no_cwd").into());
                    }
                }
            }
            return Ok(false);
        }

        if cmd == "I" || cmd == "i" {
            self.mode = Mode::Insert;
            self.status_msg = None;
            return Ok(false);
        }
        if cmd == "V" || cmd == "v" {
            self.mode = Mode::Visual;
            self.clear_selection();
            self.status_msg = Some(t!("editor.cmd_visual_hint").into());
            return Ok(false);
        }
        if cmd == "H" || cmd == "h" {
            self.mode = Mode::Help;
            self.status_msg = None;
            return Ok(false);
        }
        if cmd == "F" || cmd == "f" {
            self.mode = Mode::Search;
            self.search.query.clear();
            self.search.matches.clear();
            self.search.match_idx = 0;
            self.mark_all_dirty();
            return Ok(false);
        }
        if cmd == "x" {
            self.prev_mode = Mode::Normal;
            self.mode = Mode::Query;
            self.query_buf.clear();
            self.mark_all_dirty();
            return Ok(false);
        }

        if cmd == "B" || cmd == "b" {
            self.toggle_sidebar();
            return Ok(false);
        }

        if cmd == "e" {
            self.prev_mode = Mode::Normal;
            self.enter_nfe();
            return Ok(false);
        }

        self.status_msg = Some(format!("{} {}", t!("editor.cmd_unknown"), cmd));
        Ok(false)
    }
}
