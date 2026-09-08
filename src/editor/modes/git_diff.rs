use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};

impl Editor {
    pub(crate) fn handle_git_diff_event(
        &mut self,
        ev: Event,
        _stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = ev
        {
            let ctrl = modifiers.contains(KeyModifiers::CONTROL);
            match (ctrl, code) {
                (true, KeyCode::Char('g')) => {
                    self.close_git_diff();
                }
                _ => match code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.close_git_diff();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if let Some(state) = &mut self.git_diff {
                            state.scroll = state.scroll.saturating_sub(1);
                        }
                        self.mark_all_dirty();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if let Some(state) = &mut self.git_diff {
                            let max =
                                state.lines.len().saturating_sub(self.term_h.saturating_sub(4));
                            state.scroll = (state.scroll + 1).min(max);
                        }
                        self.mark_all_dirty();
                    }
                    KeyCode::PageUp => {
                        if let Some(state) = &mut self.git_diff {
                            state.scroll = state.scroll.saturating_sub(10);
                        }
                        self.mark_all_dirty();
                    }
                    KeyCode::PageDown => {
                        if let Some(state) = &mut self.git_diff {
                            let max =
                                state.lines.len().saturating_sub(self.term_h.saturating_sub(4));
                            state.scroll = (state.scroll + 10).min(max);
                        }
                        self.mark_all_dirty();
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn close_git_diff(&mut self) {
        self.git_diff = None;
        self.mode = Mode::Normal;
        self.status_msg = None;
        self.mark_all_dirty();
    }
}
