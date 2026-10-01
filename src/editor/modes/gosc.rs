use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};
use crate::t;

impl Editor {
    pub(crate) fn handle_gosc_event(&mut self, ev: Event) -> Result<()> {
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
                        if self.has_selection() {
                            self.search.query = self.selected_text();
                        } else {
                            self.search.query.clear();
                        }
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.gosc_dirs.clear();
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            self.jump_to_match(0);
                        }
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('t')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.gosc_dirs.clear();
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.gosc_dirs.clear();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    _ => match code {
                        KeyCode::Esc | KeyCode::Char('q') => {
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
                                let mut target = std::env::current_dir()
                                    .unwrap_or_else(|_| std::path::PathBuf::from("."));
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
                                    self.status_msg = Some(t!("editor.cmd_already_at_root"));
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
}
