use crate::editor::{next_word_byte, prev_word_byte, sel_range, Editor};
use crate::t;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

impl Editor {
    /// Vim-style Visual mode: plain hjkl/awsd/arrows move the cursor,
    /// Shift+direction selects, Ctrl+Shift+direction selects by word,
    /// single-letter verbs act on the selection. Alt+jk moves the selected
    /// block, Alt+h/l indents it and Alt+Left/Right slides the selection
    /// one character. Space inserts, Backspace removes a space before the
    /// selection. `s` saves, Ctrl+L selects the current line, `d` hops to
    /// the next occurrence of the selection while keeping the previous one
    /// highlighted, `w` drops those highlights, `q` opens Query with the
    /// selection intact. `:`/Esc leave; quit is `:q` / `0` from Normal only.
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
                match (ctrl, alt, code) {
                    // ── leave ──
                    (false, false, KeyCode::Esc) => {
                        self.clear_selection();
                        self.clear_visual_marks();
                        self.mode = crate::editor::Mode::Normal;
                    }
                    // ── save the buffer without leaving Visual ──
                    (false, false, KeyCode::Char('s')) | (false, false, KeyCode::Char('S')) => {
                        self.save_with_status(stdout);
                    }
                    // ── Query keeps the selection (and the marks) ──
                    (false, false, KeyCode::Char('q')) | (false, false, KeyCode::Char('Q')) => {
                        self.prev_mode = crate::editor::Mode::Visual;
                        self.query_buf.clear();
                        self.prompt_cursor = 0;
                        self.mode = crate::editor::Mode::Query;
                        self.status_msg = None;
                        self.mark_all_dirty();
                    }
                    // ── `w`: drop the `d` highlights, stay in Visual ──
                    (false, false, KeyCode::Char('w')) | (false, false, KeyCode::Char('W')) => {
                        self.clear_visual_marks();
                    }
                    // ── Ctrl+L: select the whole line under the cursor ──
                    (true, false, KeyCode::Char('l')) => {
                        self.select_line();
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
                    // ── Space / Backspace: insert / remove a space and
                    // keep the selection following the edit. ──
                    (false, _, KeyCode::Char(' ')) => {
                        self.visual_insert_space();
                    }
                    (false, _, KeyCode::Backspace) => {
                        if !self.visual_backspace_space() {
                            self.status_msg = Some(t!("editor.visual_backspace_no_space"));
                        }
                    }
                    // ── Ctrl+Shift+arrows: select word by word (before the
                    // Ctrl-only word motion below: same key codes). ──
                    (true, false, KeyCode::Left) if shift => {
                        self.extend_word_left();
                    }
                    (true, false, KeyCode::Right) if shift => {
                        self.extend_word_right();
                    }
                    (true, false, KeyCode::Up) if shift => {
                        self.extend_up();
                    }
                    (true, false, KeyCode::Down) if shift => {
                        self.extend_down();
                    }
                    (true, false, KeyCode::Char('H')) | (true, false, KeyCode::Char('A'))
                        if shift =>
                    {
                        self.extend_word_left();
                    }
                    (true, false, KeyCode::Char('L')) | (true, false, KeyCode::Char('D'))
                        if shift =>
                    {
                        self.extend_word_right();
                    }
                    // Ctrl+Shift+Home/End select to the buffer edges.
                    (true, false, KeyCode::Home) if shift => {
                        self.start_or_extend_selection();
                        self.cursor.y = 0;
                        self.cursor.byte = 0;
                        self.scroll_visible();
                    }
                    (true, false, KeyCode::End) if shift => {
                        self.start_or_extend_selection();
                        self.cursor.y = self.lines.len().saturating_sub(1);
                        self.cursor.byte = self.current().len();
                        self.scroll_visible();
                    }
                    // ── Shift+Home/End: select from the cursor to the edge.
                    (false, _, KeyCode::Home) if shift => {
                        self.extend_line_start();
                    }
                    (false, _, KeyCode::End) if shift => {
                        self.extend_line_end();
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
                    // `l` / Right move one character (Ctrl+L selects the
                    // line). `d` keeps the selection highlighted and hops
                    // to the next occurrence of it.
                    (false, false, KeyCode::Char('l')) | (false, false, KeyCode::Right) => {
                        self.clear_selection();
                        self.move_right_charwise();
                    }
                    (false, false, KeyCode::Char('d')) => {
                        self.visual_next_occurrence();
                    }
                    (false, false, KeyCode::Char('k')) | (false, false, KeyCode::Up) => {
                        self.clear_selection();
                        self.move_up_line();
                    }
                    (false, false, KeyCode::Char('j')) | (false, false, KeyCode::Down) => {
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
                    (false, false, KeyCode::Char('K')) => {
                        self.extend_up();
                    }
                    (false, false, KeyCode::Char('J')) => {
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
                    (true, false, KeyCode::Char('d'))
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
                    // Alt+h/l indent and dedent the selected block; the
                    // arrow twins slide the selection instead.
                    (false, true, KeyCode::Char('h')) | (false, true, KeyCode::Char('H')) => {
                        if self.has_selection() {
                            self.dedent_selected_block_strict();
                        }
                    }
                    (false, true, KeyCode::Char('l')) | (false, true, KeyCode::Char('L')) => {
                        if self.has_selection() {
                            self.indent_lines();
                        }
                    }
                    (false, true, KeyCode::Left) => {
                        self.move_selection_char_left();
                    }
                    (false, true, KeyCode::Right) => {
                        self.move_selection_char_right();
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

    /// Drop the selections kept alive by Visual `d` (also happens whenever
    /// Visual mode is left).
    pub(crate) fn clear_visual_marks(&mut self) {
        if !self.visual_marks.is_empty() {
            self.visual_marks.clear();
            self.mark_all_dirty();
        }
    }

    /// Keep the current selection highlighted, then move the selection to
    /// the next occurrence of the same text (wrapping to the top). Unlike
    /// Insert `Ctrl+D` this never adds cursors: the old match simply stays
    /// lit until `w` or leaving Visual.
    pub(crate) fn visual_next_occurrence(&mut self) {
        if !self.has_selection() {
            return;
        }
        let text = self.selected_text();
        if text.is_empty() || text.contains('\n') {
            return;
        }
        if let Some((ay, ab)) = self.cursor.selection_anchor {
            let range = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            if !self.visual_marks.contains(&range) {
                self.visual_marks.push(range);
            }
        }
        self.cmd_select_next(&text);
    }
}
