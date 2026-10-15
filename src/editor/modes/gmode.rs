use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::editor::{Editor, Mode};
use crate::t;

/// Outcome of one gmode line: where the editor goes next.
/// `Stay` keeps the prompt open so the user can fix a bad command.
#[derive(PartialEq)]
enum GmodeDone {
    ToNormal,
    Stay,
    Exit,
}

impl Editor {
    pub(crate) fn handle_gmode_event(
        &mut self,
        ev: Event,
        stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = ev
        {
            match code {
                KeyCode::Esc => {
                    self.mode = std::mem::replace(&mut self.prev_mode, Mode::Normal);
                    self.gmode_buf.clear();
                    self.status_msg = None;
                }
                KeyCode::Enter => {
                    let cmd = self.gmode_buf.clone();
                    self.gmode_buf.clear();
                    if cmd.is_empty() {
                        let prev = std::mem::replace(&mut self.prev_mode, Mode::Normal);
                        self.mode = prev;
                        return Ok(());
                    }
                    match self.execute_gmode(&cmd, stdout)? {
                        GmodeDone::Exit => return Err(anyhow::anyhow!("__exit__")),
                        GmodeDone::ToNormal => {
                            if self.mode == Mode::Gmode {
                                self.mode = Mode::Normal;
                            }
                        }
                        GmodeDone::Stay => {}
                    }
                }
                KeyCode::Backspace => {
                    self.gmode_buf.pop();
                }
                KeyCode::Char(c) => {
                    self.gmode_buf.push(c);
                }
                _ => {}
            }
        } else if let Event::Resize(w, h) = ev {
            self.term_w = w as usize;
            self.term_h = h as usize;
        }
        Ok(())
    }

    /// Create a file (`is_file`) or directory relative to CWD for
    /// gmode `n <name>` / `m <name>`. Files are opened in the field.
    fn gmode_create(&mut self, name: &str, is_file: bool) {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        match super::nfe::resolve_new_path(&cwd, name) {
            Err(msg) => {
                self.status_msg = Some(msg);
            }
            Ok(path) => {
                if path.exists() {
                    self.status_msg = Some(t!("editor.nfe_create_exists", name));
                    return;
                }
                let ok = if is_file {
                    let parents_ok = path
                        .parent()
                        .map(|p| std::fs::create_dir_all(p).is_ok())
                        .unwrap_or(true);
                    parents_ok && std::fs::write(&path, "").is_ok()
                } else {
                    std::fs::create_dir_all(&path).is_ok()
                };
                if !ok {
                    self.status_msg = Some(t!("editor.nfe_create_failed", name));
                    return;
                }
                if is_file {
                    self.open_file(&path);
                }
                self.status_msg = Some(if is_file {
                    t!("editor.nfe_created_file", name)
                } else {
                    t!("editor.nfe_created_dir", name)
                });
                self.mark_all_dirty();
            }
        }
    }

    /// Run one gmode line.
    fn execute_gmode(&mut self, cmd: &str, stdout: &mut std::io::Stdout) -> Result<GmodeDone> {
        match cmd {
            "t" => {
                self.next_tab();
                self.mark_all_dirty();
            }
            "T" => {
                self.prev_tab();
                self.mark_all_dirty();
            }
            "qt" => {
                let should_quit = self.close_current_tab();
                if should_quit {
                    return Ok(if self.exit_flow(stdout)? {
                        GmodeDone::Exit
                    } else {
                        GmodeDone::ToNormal
                    });
                }
            }
            "aqt" => {
                self.close_all_tabs();
                self.mark_all_dirty();
            }
            "q" => {
                self.mode = Mode::Normal;
                return Ok(if self.exit_flow(stdout)? {
                    GmodeDone::Exit
                } else {
                    GmodeDone::ToNormal
                });
            }
            "u" => {
                self.undo();
            }
            "r" => {
                self.redo();
            }
            "i" => {
                self.mode = Mode::Insert;
                self.completion = None;
                self.sighelp = None;
                self.status_msg = None;
            }
            "v" => {
                self.mode = Mode::Visual;
                self.clear_selection();
                self.status_msg = Some(t!("editor.mode_visual_start"));
            }
            "e" => {
                self.enter_nfe();
            }
            _ if cmd == "n" || cmd.starts_with("n ") => {
                let name = cmd.strip_prefix('n').unwrap_or("").trim();
                if name.is_empty() {
                    self.status_msg = Some(t!("editor.nfe_new_usage"));
                    return Ok(GmodeDone::Stay);
                }
                self.gmode_create(name, true);
            }
            _ if cmd == "m" || cmd.starts_with("m ") => {
                let name = cmd.strip_prefix('m').unwrap_or("").trim();
                if name.is_empty() {
                    self.status_msg = Some(t!("editor.nfe_new_usage"));
                    return Ok(GmodeDone::Stay);
                }
                self.gmode_create(name, false);
            }
            _ if cmd.chars().all(|c| c.is_ascii_digit()) => {
                if let Ok(n) = cmd.parse::<usize>() {
                    if n > 0 && n <= self.lines.len() {
                        self.cursor.y = n.saturating_sub(1);
                        self.cursor.byte = 0;
                        self.scroll_visible();
                        self.mark_all_dirty();
                    } else {
                        self.status_msg = Some(t!("editor.mode_jump_range", n, self.lines.len()));
                        return Ok(GmodeDone::Stay);
                    }
                }
            }
            _ => {
                self.status_msg = Some(t!("editor.gmode_unknown", cmd));
                return Ok(GmodeDone::Stay);
            }
        }
        Ok(GmodeDone::ToNormal)
    }
}
