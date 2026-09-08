use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::editor::{Editor, Mode};
use crate::t;

impl Editor {
    pub(crate) fn handle_jump_event(&mut self, ev: Event) -> Result<()> {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = ev
        {
            match code {
                KeyCode::Esc => {
                    self.mode = std::mem::replace(&mut self.prev_mode, Mode::Normal);
                    self.jump_buf.clear();
                    self.status_msg = None;
                }
                KeyCode::Enter => {
                    if let Ok(n) = self.jump_buf.parse::<usize>() {
                        if n > 0 && n <= self.lines.len() {
                            self.cursor.y = n.saturating_sub(1);
                            self.cursor.byte = 0;
                            self.scroll_visible();
                            self.mark_all_dirty();
                        } else {
                            self.status_msg =
                                Some(t!("editor.mode_jump_range", n, self.lines.len()));
                        }
                    }
                    let prev = std::mem::replace(&mut self.prev_mode, Mode::Normal);
                    self.mode = prev;
                    self.jump_buf.clear();
                }
                KeyCode::Backspace => {
                    self.jump_buf.pop();
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.jump_buf.push(c);
                }
                _ => {}
            }
        } else if let Event::Resize(w, h) = ev {
            self.term_w = w as usize;
            self.term_h = h as usize;
        }
        Ok(())
    }
}
