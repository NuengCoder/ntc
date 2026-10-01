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
            _ if cmd.chars().all(|c| c.is_ascii_digit()) => {
                if let Ok(n) = cmd.parse::<usize>() {
                    if n > 0 && n <= self.lines.len() {
                        self.cursor.y = n.saturating_sub(1);
                        self.cursor.byte = 0;
                        self.scroll_visible();
                        self.mark_all_dirty();
                    } else {
                        self.status_msg =
                            Some(t!("editor.mode_jump_range", n, self.lines.len()));
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
