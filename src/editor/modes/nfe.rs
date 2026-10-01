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

    /// Run one `nfe:` prompt line: `size full|def|N%`. Clears the prompt
    /// on success so navigation resumes; leaves it open on error.
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
            _ => {
                self.status_msg = Some(t!("editor.nfe_unknown", cmd));
            }
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
    /// activates, `:` opens the size prompt, Esc returns to the invoker.
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
                        self.sidebar.selected =
                            step(self.sidebar.selected, -(rows as isize));
                        self.nfe_ensure_visible();
                        self.mark_all_dirty();
                    }
                    (false, KeyCode::PageDown) => {
                        let rows = self.term_h.saturating_sub(4).max(1);
                        self.sidebar.selected =
                            step(self.sidebar.selected, rows as isize);
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
    use super::parse_nfe_size;

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
}
