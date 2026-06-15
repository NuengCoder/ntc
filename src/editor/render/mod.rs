mod bars;
mod panels;
mod popups;

use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;
use std::io::Write;

use super::{byte_to_col, char_col_width, gutter_width, Editor, Mode};

impl Editor {
    pub(crate) fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let is_run = self.mode == Mode::Run;
        let total_editor_rows = if is_run {
            self.run_split_row()
        } else {
            self.term_h.saturating_sub(4)
        };
        let rows = total_editor_rows.saturating_sub(1);
        let tab_bar_y = 0;
        let content_y = 1;

        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);
        let gw = gutter_width(self.lines.len());
        let sb_col = cols.saturating_sub(1);
        let text_cols = editor_cols.saturating_sub(gw + 2);

        self.render_tab_bar(stdout, tab_bar_y, cols)?;

        let vscroll_thumb_start: Option<usize>;
        let vscroll_thumb_height: usize;
        let total_lines = self.lines.len();
        if total_lines > rows {
            let avail = rows;
            vscroll_thumb_height = (avail * rows / total_lines).max(1);
            let max_scroll = total_lines.saturating_sub(1);
            vscroll_thumb_start = Some(self.sv.offset * (avail - vscroll_thumb_height) / max_scroll);
        } else {
            vscroll_thumb_start = None;
            vscroll_thumb_height = 0;
        }

        if self.sidebar.open {
            self.render_sidebar(stdout)?;
        }

        let vis_start = self.sv.offset;
        let ds = self.dirty_start.saturating_sub(vis_start).min(rows);
        let de = self.dirty_end.saturating_sub(vis_start).min(rows);
        let has_text_work = ds < rows && de > ds;

        if has_text_work {
            let mut all_sel: Vec<((usize, usize), (usize, usize))> = Vec::new();
            if let Some((ay, ab)) = self.cursor.selection_anchor {
                if (ay, ab) != (self.cursor.y, self.cursor.byte) {
                    all_sel.push(super::sel_range(self.cursor.y, self.cursor.byte, ay, ab));
                }
            }
            for c in &self.cursor.extra_cursors {
                if let Some((ay, ab)) = c.anchor {
                    if (ay, ab) != (c.y, c.byte) {
                        all_sel.push(super::sel_range(c.y, c.byte, ay, ab));
                    }
                }
            }

            for i in ds..de {
                let sy = content_y + i;
                let idx = self.sv.offset + i;
                if idx >= self.lines.len() {
                    queue!(stdout, MoveTo(eo as u16, sy as u16), Clear(ClearType::UntilNewLine))?;
                    if let Some(ts) = vscroll_thumb_start {
                        let in_thumb = i >= ts && i < ts + vscroll_thumb_height;
                        let ch = if in_thumb { '█' } else { '░' };
                        queue!(
                            stdout,
                            MoveTo(sb_col as u16, sy as u16),
                            if in_thumb {
                                SetForegroundColor(theme.editor.scrollbar_thumb.to_crossterm())
                            } else {
                                SetForegroundColor(theme.editor.scrollbar.to_crossterm())
                            },
                            Print(ch),
                            ResetColor
                        )?;
                    }
                    continue;
                }

                let is_cur = idx == self.cursor.y;
                let line = &self.lines[idx];
                let num = idx + 1;

                let syntax_enabled = self.editor_cfg.syntax_enabled;
                let use_color = self.editor_cfg.color_enabled;
                if syntax_enabled && self.syntax.language.is_some() {
                    self.syntax.tokenize_line(idx, line);
                }

                let gutter_num = format!("{:>w$}", num, w = gw - 1);
                queue!(stdout, MoveTo(eo as u16, sy as u16))?;
                if is_cur {
                    queue!(
                        stdout,
                        SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                        SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                        Print(&gutter_num),
                        Print(" "),
                        ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                        Print(&gutter_num),
                        Print(" "),
                        ResetColor,
                    )?;
                }

                let line_vis_width = byte_to_col(line, line.len());
                let visible_start_col = self.sv.x;

                let mut col_acc = 0usize;
                let mut byte_acc = 0usize;
                let mut skip_done = false;
                let mut display_chars: Vec<(char, usize, usize)> = Vec::new();

                for ch in line.chars() {
                    let w = char_col_width(ch);
                    if !skip_done {
                        if col_acc + w > visible_start_col {
                            skip_done = true;
                        } else {
                            col_acc += w;
                            byte_acc += ch.len_utf8();
                            continue;
                        }
                    }
                    let vis_col = col_acc.saturating_sub(visible_start_col);
                    if vis_col >= text_cols {
                        break;
                    }
                    display_chars.push((ch, byte_acc, vis_col));
                    col_acc += w;
                    byte_acc += ch.len_utf8();
                }

                let mut last_x = eo + gw;
                for (ch, byte_start, vis_col) in &display_chars {
                    let byte_start = *byte_start;
                    let vis_col = *vis_col;
                    let abs_x = eo + gw + vis_col;

                    let in_sel = all_sel.iter().any(|&((sy, sb), (ey, eb))| {
                        if idx < sy || idx > ey {
                            false
                        } else if idx == sy && idx == ey {
                            byte_start >= sb && byte_start < eb
                        } else if idx == sy {
                            byte_start >= sb
                        } else if idx == ey {
                            byte_start < eb
                        } else {
                            true
                        }
                    });

                    let in_search = if self.mode == Mode::Search || !self.search.query.is_empty() {
                        self.search.matches
                            .iter()
                            .enumerate()
                            .any(|(mi, &(mly, msb, meb))| {
                                mly == idx
                                    && byte_start >= msb
                                    && byte_start < meb
                                    && mi == self.search.match_idx
                            })
                    } else {
                        false
                    };
                    let in_search_other = if !self.search.query.is_empty() {
                        self.search.matches.iter().any(|&(mly, msb, meb)| {
                            mly == idx && byte_start >= msb && byte_start < meb
                        })
                    } else {
                        false
                    };

                    queue!(stdout, MoveTo(abs_x as u16, sy as u16))?;

                    if in_search {
                        queue!(
                            stdout,
                            SetBackgroundColor(theme.editor.search_current_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                            Print(ch),
                            ResetColor
                        )?;
                    } else if in_search_other {
                        queue!(
                            stdout,
                            SetBackgroundColor(theme.editor.search_match_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.status_text.to_crossterm()),
                            Print(ch),
                            ResetColor
                        )?;
                    } else if in_sel {
                        queue!(
                            stdout,
                            SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.status_text.to_crossterm()),
                            Print(ch),
                            ResetColor
                        )?;
                    } else if is_cur {
                        let fg = if use_color && syntax_enabled {
                            self.syntax
                                .token_type_at(idx, byte_start)
                                .map(crate::syntax::color_for)
                                .unwrap_or(theme.syntax.normal.to_crossterm())
                        } else {
                            theme.syntax.normal.to_crossterm()
                        };
                        queue!(
                            stdout,
                            SetBackgroundColor(theme.editor.editor_bg.to_crossterm()),
                            SetForegroundColor(fg),
                            Print(ch),
                            ResetColor
                        )?;
                    } else {
                        let fg = if use_color && syntax_enabled {
                            self.syntax
                                .token_type_at(idx, byte_start)
                                .map(crate::syntax::color_for)
                                .unwrap_or(theme.syntax.normal.to_crossterm())
                        } else {
                            theme.syntax.normal.to_crossterm()
                        };
                        queue!(
                            stdout,
                            SetForegroundColor(fg),
                            Print(ch),
                            ResetColor
                        )?;
                    }
                    last_x = abs_x + char_col_width(*ch);
                }

                let in_sel_mid = all_sel.iter().any(|&((sy, _), (ey, _))| idx > sy && idx < ey);
                if in_sel_mid && last_x < cols {
                    queue!(
                        stdout,
                        MoveTo(last_x as u16, sy as u16),
                        SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                        Print(" ".repeat(cols - last_x)),
                        ResetColor
                    )?;
                }

                queue!(stdout, Clear(ClearType::UntilNewLine))?;

                if line_vis_width > self.sv.x + text_cols {
                    let right_x = eo + gw + text_cols;
                    if right_x < cols {
                        queue!(
                            stdout,
                            MoveTo(right_x as u16, sy as u16),
                            SetForegroundColor(theme.editor.border.to_crossterm()),
                            Print("›"),
                            ResetColor
                        )?;
                    }
                }
                if self.sv.x > 0 {
                    queue!(
                        stdout,
                        MoveTo((eo + gw) as u16, sy as u16),
                        SetForegroundColor(theme.editor.border.to_crossterm()),
                        Print("‹"),
                        ResetColor
                    )?;
                }

                if let Some(ts) = vscroll_thumb_start {
                    let in_thumb = i >= ts && i < ts + vscroll_thumb_height;
                    let ch = if in_thumb { '█' } else { '░' };
                    queue!(
                        stdout,
                        MoveTo(sb_col as u16, sy as u16),
                        if in_thumb {
                            SetForegroundColor(theme.editor.scrollbar_thumb.to_crossterm())
                        } else {
                            SetForegroundColor(theme.editor.scrollbar.to_crossterm())
                        },
                        Print(ch),
                        ResetColor
                    )?;
                }

                if is_cur {
                    let cursor_col = byte_to_col(self.current(), self.cursor.byte);
                    let screen_col = if cursor_col >= self.sv.x {
                        eo + gw + (cursor_col - self.sv.x)
                    } else {
                        eo + gw
                    };
                    if screen_col < cols {
                        let line_s = self.current();
                        let cursor_ch = if self.cursor.byte < line_s.len() {
                            line_s[self.cursor.byte..].chars().next().unwrap_or(' ')
                        } else {
                            ' '
                        };
                        queue!(
                            stdout,
                            MoveTo(screen_col as u16, sy as u16),
                            SetBackgroundColor(theme.editor.cursor_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                            Print(cursor_ch),
                            ResetColor,
                        )?;
                    }
                }
                for ec in &self.cursor.extra_cursors {
                    if ec.y == idx {
                        let cursor_col = byte_to_col(&self.lines[idx], ec.byte);
                        let screen_col = if cursor_col >= self.sv.x {
                            eo + gw + (cursor_col - self.sv.x)
                        } else {
                            eo + gw
                        };
                        if screen_col < cols {
                            let line_s = &self.lines[idx];
                            let cursor_ch = if ec.byte < line_s.len() {
                                line_s[ec.byte..].chars().next().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            queue!(
                                stdout,
                                MoveTo(screen_col as u16, sy as u16),
                                SetBackgroundColor(theme.editor.extra_cursor_bg.to_crossterm()),
                                SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                                Print(cursor_ch),
                                ResetColor,
                            )?;
                        }
                    }
                }
            }
        }

        // Pre-tokenize next 10 lines beyond visible area for smooth scrolling
        {
            let syntax_enabled = self.editor_cfg.syntax_enabled;
            if syntax_enabled && self.syntax.language.is_some() {
                let vis_end = self.sv.offset + rows;
                let pre_end = (vis_end + 10).min(self.lines.len());
                for idx in vis_end..pre_end {
                    let line = &self.lines[idx];
                    self.syntax.tokenize_line(idx, line);
                }
            }
        }

        if matches!(self.mode, Mode::Help) {
            self.render_help_screen(stdout)?;
        }

        if matches!(self.mode, Mode::FileFinder) {
            self.render_file_finder(stdout)?;
        }

        if matches!(self.mode, Mode::Gosc) {
            self.render_gosc(stdout)?;
        }

        if self.completion_visible {
            self.render_completion_popup(stdout)?;
        }

        {
            let hrow = content_y + rows;
            let max_vis = self
                .lines
                .iter()
                .map(|l| byte_to_col(l, l.len()))
                .max()
                .unwrap_or(0);
            if max_vis > text_cols {
                let avail = cols.saturating_sub(2);
                let thumb_start = self.sv.x * avail / max_vis;
                let thumb_end = (self.sv.x + text_cols).min(max_vis) * avail / max_vis;
                let thumb_w = (thumb_end.saturating_sub(thumb_start)).max(1);

                queue!(
                    stdout,
                    MoveTo(0, hrow as u16),
                    SetBackgroundColor(theme.editor.editor_bg.to_crossterm()),
                    SetForegroundColor(theme.editor.scrollbar.to_crossterm()),
                    Print(" "),
                )?;
                for h in 1..avail + 1 {
                    let ch = if h >= thumb_start && h < thumb_start + thumb_w {
                        '━'
                    } else {
                        '─'
                    };
                    queue!(
                        stdout,
                        MoveTo(h as u16, hrow as u16),
                        if h >= thumb_start && h < thumb_start + thumb_w {
                            SetForegroundColor(theme.editor.scrollbar_thumb.to_crossterm())
                        } else {
                            SetForegroundColor(theme.editor.scrollbar.to_crossterm())
                        },
                        Print(ch),
                        ResetColor,
                    )?;
                }
                if avail + 1 < cols {
                    queue!(
                        stdout,
                        MoveTo((avail + 1) as u16, hrow as u16),
                        SetForegroundColor(theme.editor.scrollbar.to_crossterm()),
                        Print(" "),
                        ResetColor,
                    )?;
                }
                queue!(stdout, ResetColor, Clear(ClearType::UntilNewLine))?;
            } else {
                queue!(
                    stdout,
                    MoveTo(0, hrow as u16),
                    Clear(ClearType::UntilNewLine),
                )?;
            }
        }

        if is_run {
            self.render_run_panel(stdout)?;
        }

        self.render_cwd_bar(stdout)?;
        self.render_status_bar(stdout)?;
        self.render_hint_bar(stdout)?;
        stdout.flush()?;

        self.dirty_start = 0;
        self.dirty_end = 0;

        Ok(())
    }
}
