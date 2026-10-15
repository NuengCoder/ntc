use crate::editor::{next_word_byte, prev_word_byte, Editor};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

impl Editor {
    /// Vim-style Visual mode: plain hjkl/awsd/arrows move the cursor,
    /// Shift+direction selects, single-letter verbs act on the selection.
    /// Alt+hjkl/arrows moves the selected block (j/k) or indents it (h/l).
    /// `:`/Esc leave; quit is `:q` / `0` from Normal only.
    pub(crate) fn handle_visual_event(
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
                let _ = stdout;
                match (ctrl, alt, code) {
                    // ── leave ──
                    (false, false, KeyCode::Esc) => {
                        self.clear_selection();
                        self.mode = crate::editor::Mode::Normal;
                    }
                    // ── verbs ──
                    (false, false, KeyCode::Char('a')) => {
                        if self.has_selection()
                            && self.cursor.selection_anchor == Some((0, 0))
                            && self.cursor.y == self.lines.len().saturating_sub(1)
                            && self.cursor.byte == self.current().len()
                        {
                            self.clear_selection();
                        } else {
                            self.select_all();
                        }
                    }
                    (false, false, KeyCode::Char('c')) | (false, false, KeyCode::Char('C')) => {
                        self.copy_selection();
                    }
                    (false, false, KeyCode::Char('x')) | (false, false, KeyCode::Char('X')) => {
                        if self.has_selection() {
                            self.snapshot();
                            self.cut_selection();
                        }
                    }
                    (false, false, KeyCode::Char('y')) | (false, false, KeyCode::Char('Y')) => {
                        self.yank_line();
                    }
                    (false, false, KeyCode::Char('p')) | (false, false, KeyCode::Char('P')) => {
                        self.snapshot();
                        if self.has_selection() {
                            self.delete_selection();
                        }
                        self.paste();
                    }
                    (false, false, KeyCode::Char('i')) | (false, false, KeyCode::Char('I')) => {
                        self.mode = crate::editor::Mode::Insert;
                        self.completion = None;
                        self.sighelp = None;
                        self.status_msg = None;
                    }
                    (false, false, KeyCode::Char('u')) | (false, false, KeyCode::Char('U')) => {
                        self.undo();
                        self.clamp();
                    }
                    (false, false, KeyCode::Char('r')) | (false, false, KeyCode::Char('R')) => {
                        self.redo();
                        self.clamp();
                    }
                    (false, _, KeyCode::Char(':')) => {
                        self.mode = crate::editor::Mode::Command;
                        self.cmd_buf.clear();
                    }
                    (false, false, KeyCode::Char('e')) | (false, false, KeyCode::Char('E')) => {
                        self.prev_mode = crate::editor::Mode::Visual;
                        self.enter_nfe();
                    }
                    (false, false, KeyCode::Char('f')) | (false, false, KeyCode::Char('F')) => {
                        let sel = self.selected_text();
                        self.clear_selection();
                        self.mode = crate::editor::Mode::Search;
                        if !sel.is_empty() {
                            self.search.query = sel;
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
                    (false, false, KeyCode::Char('g')) | (false, false, KeyCode::Char('G')) => {
                        self.clear_selection();
                        self.prev_mode = crate::editor::Mode::Visual;
                        self.mode = crate::editor::Mode::Gmode;
                        self.gmode_buf.clear();
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    (true, false, KeyCode::Char('j')) => {
                        self.clear_selection();
                        self.prev_mode = crate::editor::Mode::Visual;
                        self.mode = crate::editor::Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    // ── Shift+arrows select (before plain: same codes) ──
                    (false, false, KeyCode::Left) if shift => {
                        self.extend_left();
                    }
                    (false, false, KeyCode::Right) if shift => {
                        self.extend_right();
                    }
                    (false, false, KeyCode::Up) if shift => {
                        self.extend_up();
                    }
                    (false, false, KeyCode::Down) if shift => {
                        self.extend_down();
                    }
                    // ── plain motion: move only, never selects.
                    // (`a` is the select-all toggle, so left = h/Left.) ──
                    (false, false, KeyCode::Char('h')) | (false, false, KeyCode::Left) => {
                        self.clear_selection();
                        self.move_left_charwise();
                    }
                    (false, false, KeyCode::Char('l'))
                    | (false, false, KeyCode::Char('d'))
                    | (false, false, KeyCode::Right) => {
                        self.clear_selection();
                        self.move_right_charwise();
                    }
                    (false, false, KeyCode::Char('k'))
                    | (false, false, KeyCode::Char('w'))
                    | (false, false, KeyCode::Up) => {
                        self.clear_selection();
                        self.move_up_line();
                    }
                    (false, false, KeyCode::Char('j'))
                    | (false, false, KeyCode::Char('s'))
                    | (false, false, KeyCode::Down) => {
                        self.clear_selection();
                        self.move_down_line();
                    }
                    // ── Shift+letter arrives as uppercase: extend. ──
                    (false, false, KeyCode::Char('H')) | (false, false, KeyCode::Char('A')) => {
                        self.extend_left();
                    }
                    (false, false, KeyCode::Char('L')) | (false, false, KeyCode::Char('D')) => {
                        self.extend_right();
                    }
                    (false, false, KeyCode::Char('K')) | (false, false, KeyCode::Char('W')) => {
                        self.extend_up();
                    }
                    (false, false, KeyCode::Char('J')) | (false, false, KeyCode::Char('S')) => {
                        self.extend_down();
                    }
                    // ── Ctrl+word motion (h/l, a/d, arrows): move, never selects.
                    (true, false, KeyCode::Char('h'))
                    | (true, false, KeyCode::Char('H'))
                    | (true, false, KeyCode::Char('a'))
                    | (true, false, KeyCode::Char('A'))
                    | (true, false, KeyCode::Left) => {
                        self.clear_selection();
                        if self.cursor.byte > 0 {
                            self.cursor.byte = prev_word_byte(self.current(), self.cursor.byte);
                        } else if self.cursor.y > 0 {
                            self.cursor.y -= 1;
                            self.cursor.byte = self.current().len();
                        }
                    }
                    (true, false, KeyCode::Char('l'))
                    | (true, false, KeyCode::Char('L'))
                    | (true, false, KeyCode::Char('d'))
                    | (true, false, KeyCode::Char('D'))
                    | (true, false, KeyCode::Right) => {
                        self.clear_selection();
                        let line = self.current();
                        if self.cursor.byte < line.len() {
                            self.cursor.byte = next_word_byte(line, self.cursor.byte);
                        } else if self.cursor.y + 1 < self.lines.len() {
                            self.cursor.y += 1;
                            self.cursor.byte = 0;
                        }
                    }
                    // ── Alt+hjkl/arrows: move selected block (j/k) or
                    // indent/dedent it (l/h). Selection only; no selection → no-op.
                    // (Alt+Left/Right used to scroll; now they move instead.)
                    (false, true, KeyCode::Char('k'))
                    | (false, true, KeyCode::Char('K'))
                    | (false, true, KeyCode::Up) => {
                        if self.has_selection() {
                            self.move_selected_block_up();
                        }
                    }
                    (false, true, KeyCode::Char('j'))
                    | (false, true, KeyCode::Char('J'))
                    | (false, true, KeyCode::Down) => {
                        if self.has_selection() {
                            self.move_selected_block_down();
                        }
                    }
                    (false, true, KeyCode::Char('h'))
                    | (false, true, KeyCode::Char('H'))
                    | (false, true, KeyCode::Left) => {
                        if self.has_selection() {
                            self.dedent_selected_block_strict();
                        }
                    }
                    (false, true, KeyCode::Char('l'))
                    | (false, true, KeyCode::Char('L'))
                    | (false, true, KeyCode::Right) => {
                        if self.has_selection() {
                            self.indent_lines();
                        }
                    }
                    (false, _, KeyCode::Home) => {
                        self.clear_selection();
                        self.move_line_start_toggle();
                    }
                    (false, _, KeyCode::End) => {
                        self.clear_selection();
                        self.move_line_end();
                    }
                    (false, _, KeyCode::PageUp) => {
                        self.start_or_extend_selection();
                        self.page_up();
                    }
                    (false, _, KeyCode::PageDown) => {
                        self.start_or_extend_selection();
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
        Ok(())
    }
}
