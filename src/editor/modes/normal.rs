use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{Editor, Mode};
use crate::t;

impl Editor {
    /// Vim-style Normal mode: single-letter verbs + hjkl/awsd/arrows motion.
    /// Typing letters never inserts; quit is `:q` / `0` / gmode-`q` only.
    /// Uppercase twins mirror lowercase verbs because most terminals
    /// report Shift+letter as a bare uppercase char (no shift flag).
    pub(crate) fn handle_normal_event(
        &mut self,
        ev: Event,
        stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                match (ctrl, alt, code) {
                    // ── Flagged uppercase selects (kitty-style terminals
                    // report Shift as a flag). First: bare uppercase below
                    // (Help/save/motion) must not shadow these.
                    (false, _, KeyCode::Char('H')) if shift => {
                        self.extend_left();
                    }
                    (false, _, KeyCode::Char('A')) if shift => {
                        self.extend_left();
                    }
                    (false, _, KeyCode::Char('L')) if shift => {
                        self.extend_right();
                    }
                    (false, _, KeyCode::Char('D')) if shift => {
                        self.extend_right();
                    }
                    (false, _, KeyCode::Char('K')) if shift => {
                        self.extend_up();
                    }
                    (false, _, KeyCode::Char('W')) if shift => {
                        self.extend_up();
                    }
                    (false, _, KeyCode::Char('J')) if shift => {
                        self.extend_down();
                    }
                    (false, _, KeyCode::Char('S')) if shift => {
                        self.extend_down();
                    }
                    // ── verbs ──
                    (false, false, KeyCode::Char('i'))
                    | (false, false, KeyCode::Char('I')) => {
                        self.mode = Mode::Insert;
                        self.completion = None;
                        self.sighelp = None;
                        self.status_msg = None;
                    }
                    (false, false, KeyCode::Char('v'))
                    | (false, false, KeyCode::Char('V')) => {
                        self.mode = Mode::Visual;
                        self.clear_selection();
                        self.status_msg = Some(t!("editor.mode_visual_start"));
                    }
                    (false, false, KeyCode::Char('q'))
                    | (false, false, KeyCode::Char('Q')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Query;
                        self.query_buf.clear();
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    (false, false, KeyCode::Char('n'))
                    | (false, false, KeyCode::Char('N')) => {
                        self.save_with_status(stdout);
                    }
                    (false, false, KeyCode::Char('e'))
                    | (false, false, KeyCode::Char('E')) => {
                        self.prev_mode = Mode::Normal;
                        self.enter_nfe();
                    }
                    (false, false, KeyCode::Char('f'))
                    | (false, false, KeyCode::Char('F')) => {
                        self.mode = Mode::Search;
                        if self.has_selection() {
                            self.search.query = self.selected_text();
                        } else {
                            self.search.query.clear();
                        }
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            self.jump_to_match(0);
                        }
                        self.mark_all_dirty();
                    }
                    (false, false, KeyCode::Char('b'))
                    | (false, false, KeyCode::Char('B')) => {
                        self.mode = Mode::FileFinder;
                        self.ff_query.clear();
                        self.ff_results.clear();
                        self.ff_idx = 0;
                        self.mark_all_dirty();
                    }
                    (false, false, KeyCode::Char('t'))
                    | (false, false, KeyCode::Char('T')) => {
                        self.toggle_terminal();
                    }
                    (false, false, KeyCode::Char('u'))
                    | (false, false, KeyCode::Char('U')) => {
                        self.open_encoding_picker();
                    }
                    (false, false, KeyCode::Char('g'))
                    | (false, false, KeyCode::Char('G')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Gmode;
                        self.gmode_buf.clear();
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    (false, _, KeyCode::Char(':')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    // Unfolded Shift+; arrives as ';' + SHIFT on some
                    // terminals instead of folded ':'.
                    (false, _, KeyCode::Char(';')) => {
                        self.mode = Mode::Command;
                        self.cmd_buf.clear();
                    }
                    (false, _, KeyCode::Char('?')) => {
                        self.mode = Mode::Help;
                        self.status_msg = None;
                    }
                    (false, false, KeyCode::Char('0')) => {
                        self.mode = Mode::Normal;
                        if self.exit_flow(stdout)? {
                            return Err(anyhow::anyhow!("__exit__"));
                        }
                    }
                    (true, false, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (false, false, KeyCode::Esc) => {
                        self.clear_extra_cursors();
                        self.status_msg = None;
                    }
                    // ── fast copy without Visual ──
                    (false, false, KeyCode::Char('c'))
                    | (false, false, KeyCode::Char('C')) => {
                        self.copy_selection();
                    }
                    // ── Shift+direction selects (kitty-style flagged
                    // keys; classic terminals use Shift+arrows) ──
                    (false, _, KeyCode::Left) if shift => {
                        self.extend_left();
                    }
                    (false, _, KeyCode::Right) if shift => {
                        self.extend_right();
                    }
                    (false, _, KeyCode::Up) if shift => {
                        self.extend_up();
                    }
                    (false, _, KeyCode::Down) if shift => {
                        self.extend_down();
                    }
                    (false, _, KeyCode::Char('h')) if shift => {
                        self.extend_left();
                    }
                    (false, _, KeyCode::Char('l')) if shift => {
                        self.extend_right();
                    }
                    (false, _, KeyCode::Char('k')) if shift => {
                        self.extend_up();
                    }
                    (false, _, KeyCode::Char('j')) if shift => {
                        self.extend_down();
                    }
                    (false, _, KeyCode::Char('a')) if shift => {
                        self.extend_left();
                    }
                    (false, _, KeyCode::Char('d')) if shift => {
                        self.extend_right();
                    }
                    (false, _, KeyCode::Char('w')) if shift => {
                        self.extend_up();
                    }
                    (false, _, KeyCode::Char('s')) if shift => {
                        self.extend_down();
                    }
                    (false, false, KeyCode::Char('h'))
                    | (false, false, KeyCode::Char('H'))
                    | (false, false, KeyCode::Char('a'))
                    | (false, false, KeyCode::Char('A'))
                    | (false, false, KeyCode::Left) => {
                        self.move_left_charwise();
                    }
                    (false, false, KeyCode::Char('l'))
                    | (false, false, KeyCode::Char('L'))
                    | (false, false, KeyCode::Char('d'))
                    | (false, false, KeyCode::Char('D'))
                    | (false, false, KeyCode::Right) => {
                        self.move_right_charwise();
                    }
                    (false, false, KeyCode::Char('k'))
                    | (false, false, KeyCode::Char('K'))
                    | (false, false, KeyCode::Char('w'))
                    | (false, false, KeyCode::Char('W'))
                    | (false, false, KeyCode::Up) => {
                        self.move_up_line();
                    }
                    (false, false, KeyCode::Char('j'))
                    | (false, false, KeyCode::Char('J'))
                    | (false, false, KeyCode::Char('s'))
                    | (false, false, KeyCode::Char('S'))
                    | (false, false, KeyCode::Down) => {
                        self.move_down_line();
                    }
                    (false, true, KeyCode::Left) => {
                        self.sv.x = self.sv.x.saturating_sub(3);
                    }
                    (false, true, KeyCode::Right) => {
                        self.sv.x = (self.sv.x + 3).min(self.max_scroll_x());
                    }
                    (false, _, KeyCode::Home) => {
                        self.move_line_start_toggle();
                    }
                    (false, _, KeyCode::End) => {
                        self.move_line_end();
                    }
                    (false, _, KeyCode::PageUp) => {
                        self.page_up();
                    }
                    (false, _, KeyCode::PageDown) => {
                        self.page_down();
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
        self.update_hover();
        Ok(())
    }
}
