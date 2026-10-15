use anyhow::Result;
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};

use crate::editor::types::GsResultList;
use crate::editor::{Editor, Mode};
use crate::filetype::FormatConfig;

impl Editor {
    fn is_gs_mode(&self) -> bool {
        self.ff_query.starts_with("gs:")
    }

    pub(crate) fn handle_file_finder_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
                    (true, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        self.search.query.clear();
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    _ => match code {
                        KeyCode::Esc => {
                            if self.is_gs_mode() {
                                self.gs_state.results.clear();
                                self.gs_state.query.clear();
                                self.gs_state.loading = false;
                                self.gs_state.loading_pattern.clear();
                            }
                            self.mode = Mode::Normal;
                            self.ff_query.clear();
                            self.ff_results.clear();
                            self.mark_all_dirty();
                        }
                        KeyCode::Enter => {
                            if self.is_gs_mode() {
                                if let Some(path) = self.gs_state.results.current_path() {
                                    let path = path.to_path_buf();
                                    let line = self.gs_state.results.current_line().unwrap_or(0);
                                    self.open_file(&path);
                                    if line > 0 {
                                        self.cursor.y = line.saturating_sub(1);
                                        self.cursor.byte = 0;
                                        self.scroll_visible();
                                    }
                                }
                                self.gs_state.results.clear();
                                self.gs_state.query.clear();
                            } else {
                                if !self.ff_results.is_empty() {
                                    let path = self.ff_results[self.ff_idx].1.clone();
                                    let line = self.ff_results[self.ff_idx].3;
                                    self.open_file(&path);
                                    if line > 0 {
                                        self.cursor.y = line.saturating_sub(1);
                                        self.cursor.byte = 0;
                                        self.scroll_visible();
                                    }
                                }
                                self.ff_results.clear();
                            }
                            self.ff_query.clear();
                            self.mode = Mode::Normal;
                            self.mark_all_dirty();
                        }
                        KeyCode::Up => {
                            if self.is_gs_mode() {
                                self.gs_state.results.move_prev();
                            } else if self.ff_idx > 0 {
                                self.ff_idx -= 1;
                            } else {
                                self.ff_idx = self.ff_results.len().saturating_sub(1);
                            }
                            self.mark_all_dirty();
                        }
                        KeyCode::Down => {
                            if self.is_gs_mode() {
                                self.gs_state.results.move_next();
                            } else if self.ff_idx + 1 < self.ff_results.len() {
                                self.ff_idx += 1;
                            } else {
                                self.ff_idx = 0;
                            }
                            self.mark_all_dirty();
                        }
                        KeyCode::Backspace => {
                            self.ff_query.pop();
                            if self.is_gs_mode() {
                                self.rebuild_gs_results();
                            } else {
                                self.rebuild_ff_results();
                            }
                            self.mark_all_dirty();
                        }
                        KeyCode::Char(c) => {
                            self.ff_query.push(c);
                            if self.is_gs_mode() {
                                self.rebuild_gs_results();
                            } else {
                                self.rebuild_ff_results();
                            }
                            self.mark_all_dirty();
                        }
                        _ => {}
                    },
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            Event::Mouse(m) => {
                let eo = self.editor_offset();
                let content_top = 1;
                let rows = self.term_h.saturating_sub(1);
                let max_results = rows.saturating_sub(2);

                if self.is_gs_mode() {
                    let total = self.gs_state.results.len();
                    let max_vis = max_results.min(total);
                    let cur = self.gs_state.results.current_index().unwrap_or(0);
                    let half = max_vis / 2;
                    let start_idx = if cur > half {
                        cur.saturating_sub(half).min(total.saturating_sub(max_vis))
                    } else {
                        0
                    };

                    match m.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let r = m.row as usize;
                            let c = m.column as usize;
                            let vis_start = content_top + 2;
                            let vis_end = vis_start + max_vis;
                            if r >= vis_start && r < vis_end && c >= eo {
                                let click_idx = start_idx + (r - vis_start);
                                if click_idx < total {
                                    let steps = click_idx.abs_diff(cur);
                                    if click_idx > cur {
                                        for _ in 0..steps {
                                            self.gs_state.results.move_next();
                                        }
                                    } else {
                                        for _ in 0..steps {
                                            self.gs_state.results.move_prev();
                                        }
                                    }
                                    self.mark_all_dirty();
                                }
                            }
                        }
                        MouseEventKind::ScrollUp => {
                            self.gs_state.results.move_prev();
                            self.mark_all_dirty();
                        }
                        MouseEventKind::ScrollDown => {
                            self.gs_state.results.move_next();
                            self.mark_all_dirty();
                        }
                        _ => {}
                    }
                } else {
                    let max_vis = max_results.min(self.ff_results.len());
                    let half = max_vis / 2;
                    let start_idx = if self.ff_idx > half {
                        self.ff_idx
                            .saturating_sub(half)
                            .min(self.ff_results.len().saturating_sub(max_vis))
                    } else {
                        0
                    };

                    match m.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let r = m.row as usize;
                            let c = m.column as usize;
                            let vis_start = content_top + 2;
                            let vis_end = vis_start + max_vis;
                            if r >= vis_start && r < vis_end && c >= eo {
                                let idx = start_idx + (r - vis_start);
                                if idx < self.ff_results.len() {
                                    self.ff_idx = idx;
                                    self.mark_all_dirty();
                                }
                            }
                        }
                        MouseEventKind::ScrollUp if self.ff_idx > 0 => {
                            self.ff_idx -= 1;
                            self.mark_all_dirty();
                        }
                        MouseEventKind::ScrollDown if self.ff_idx + 1 < self.ff_results.len() => {
                            self.ff_idx += 1;
                            self.mark_all_dirty();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn rebuild_gs_results(&mut self) {
        let query = self.ff_query.strip_prefix("gs:").unwrap_or("").to_string();
        self.gs_state.query = query.clone();
        if query.is_empty() {
            self.gs_state.results.clear();
            self.gs_state.loading = false;
            self.gs_state.loading_pattern.clear();
            return;
        }
        self.gs_state.loading = true;
        self.gs_state.loading_pattern = query.clone();
        self.gs_state.results.clear();
        self.mark_all_dirty();

        let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let matches = crate::utils::search::search_content(&root, &query, 10);
        let mut list = GsResultList::new();
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
}

/// Display suffix for a file-finder result: `../rel/dir` when the file
/// name is duplicated in the result set, otherwise empty. Relative to
/// `root` (the walk root / CWD) — never an absolute full path, unless the
/// file escapes the root (different drive → UNC-stripped absolute parent
/// fallback). A file directly under `root` shows no suffix (bare name).
pub(crate) fn finder_dup_suffix(
    path: &std::path::Path,
    root: &std::path::Path,
    duplicated: bool,
) -> String {
    if !duplicated {
        return String::new();
    }
    let parent = match path.parent() {
        Some(p) => p,
        None => return String::new(),
    };
    let pn = crate::filetype::strip_unc_prefix(parent).replace('\\', "/");
    let rn = crate::filetype::strip_unc_prefix(root).replace('\\', "/");
    #[cfg(windows)]
    let (pn, rn) = (pn.to_lowercase(), rn.to_lowercase());
    if pn == rn {
        return String::new();
    }
    let prefix = if rn.ends_with('/') {
        rn.clone()
    } else {
        format!("{rn}/")
    };
    match pn.strip_prefix(&prefix) {
        Some(rel) => format!("../{rel}"),
        None => crate::filetype::strip_unc_prefix(parent),
    }
}

impl Editor {
    fn rebuild_ff_results(&mut self) {
        if self.ff_query.is_empty() {
            self.ff_results.clear();
            return;
        }
        let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let cache_fresh = self
            .file_index
            .as_ref()
            .map(|(ts, _)| ts.elapsed().as_secs() < 10)
            .unwrap_or(false);
        if !cache_fresh {
            let ignored_dirs = crate::config::Config::get_ignored_dirs_combined();
            let fmt_cfg = FormatConfig::from_both();
            // Hide binaries: known-binary extensions plus extensionless
            // files that fail the text sniff (compiled outputs).
            let files: Vec<(String, std::path::PathBuf)> =
                crate::search::walk_files(&root, 10, &ignored_dirs, &fmt_cfg)
                    .filter(|(_, path)| !crate::filetype::is_finder_hidden(path))
                    .collect();
            self.file_index = Some((std::time::Instant::now(), files));
        }

        let q = self.ff_query.to_lowercase();
        let mut exact: Vec<(String, std::path::PathBuf, f64, usize)> = Vec::with_capacity(64);
        let mut partial: Vec<(String, std::path::PathBuf, f64, usize)> = Vec::with_capacity(128);
        let mut fuzzy: Vec<(String, std::path::PathBuf, f64, usize)> = Vec::new();

        if let Some((_, ref files)) = self.file_index {
            for (name, path) in files {
                let name_lower = name.to_lowercase();
                if name_lower == q {
                    exact.push((name.clone(), path.clone(), 1.0, 0));
                } else if name_lower.contains(&q) {
                    let score = q.len() as f64 / name_lower.len() as f64;
                    partial.push((name.clone(), path.clone(), score, 0));
                } else {
                    let score = crate::fuzzy::jaro_winkler(&name_lower, &q);
                    if score >= 0.75 {
                        fuzzy.push((name.clone(), path.clone(), score, 0));
                    }
                }
            }
        }

        // Path tiebreaks keep same-name duplicates in a stable order
        // across index rebuilds (walk order is filesystem-dependent).
        if !exact.is_empty() {
            exact.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
            self.ff_results = exact;
        } else if !partial.is_empty() {
            partial.sort_by(|a, b| {
                b.2.partial_cmp(&a.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1.cmp(&b.1))
            });
            self.ff_results = partial;
        } else {
            fuzzy.sort_by(|a, b| {
                b.2.partial_cmp(&a.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1.cmp(&b.1))
            });
            fuzzy.truncate(20);
            self.ff_results = fuzzy;
        }
        self.ff_idx = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn dup_suffix_unique_is_bare() {
        let root = PathBuf::from("/proj");
        assert_eq!(
            finder_dup_suffix(&PathBuf::from("/proj/src/mod.rs"), &root, false),
            ""
        );
    }

    #[test]
    fn dup_suffix_shows_rel_parent() {
        let root = PathBuf::from("/proj");
        assert_eq!(
            finder_dup_suffix(&PathBuf::from("/proj/src/editor/mod.rs"), &root, true),
            "../src/editor"
        );
    }

    #[test]
    fn dup_suffix_root_level_is_bare() {
        let root = PathBuf::from("/proj");
        assert_eq!(
            finder_dup_suffix(&PathBuf::from("/proj/mod.rs"), &root, true),
            ""
        );
    }
}
