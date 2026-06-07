use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use std::io::Write;

use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;

use super::{
    byte_to_col, char_col_width, gutter_width,
    Editor, Mode, SIDEBAR_WIDTH,
};

impl Editor {
    // ── render ───────────────────────────────────────────────────────────────

    pub(crate) fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let rows = self.term_h.saturating_sub(3); // -3: horizontal scroll bar + status + hint
        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);
        let gw = gutter_width(self.lines.len());
        let sb_col = cols.saturating_sub(1); // vertical scroll bar column (full width)
        let text_cols = editor_cols.saturating_sub(gw + 2); // +1 separator, +1 scroll bar

        // ── sidebar ──────────────────────────────────────────────────────────
        if self.sidebar.open {
            self.render_sidebar(stdout)?;
        }

        // ── editor content ───────────────────────────────────────────────────
        let vis_start = self.scroll;
        let ds = self.dirty_start.saturating_sub(vis_start).min(rows);
        let de = self.dirty_end.saturating_sub(vis_start).min(rows);
        let has_text_work = ds < rows && de > ds;

        if has_text_work {
            let mut all_sel: Vec<((usize, usize), (usize, usize))> = Vec::new();
            if self.has_selection() {
                let (ay, ab) = self.selection_anchor.unwrap();
                all_sel.push(super::sel_range(self.cursor_y, self.cursor_byte, ay, ab));
            }
            for c in &self.extra_cursors {
                if let Some((ay, ab)) = c.anchor {
                    if (ay, ab) != (c.y, c.byte) {
                        all_sel.push(super::sel_range(c.y, c.byte, ay, ab));
                    }
                }
            }

            for i in ds..de {
                let idx = self.scroll + i;
                if idx >= self.lines.len() {
                    queue!(stdout, MoveTo(eo as u16, i as u16), Clear(ClearType::UntilNewLine))?;
                    let total = self.lines.len();
                    if total > rows {
                        let thumb = self.scroll * (rows - 1) / (total - rows);
                        let ch = if i == thumb { '▌' } else { '░' };
                        queue!(
                            stdout,
                            MoveTo(sb_col as u16, i as u16),
                            SetForegroundColor(Color::Rgb {
                                r: 108,
                                g: 108,
                                b: 128
                            }),
                            Print(ch),
                            ResetColor
                        )?;
                    }
                    continue;
                }

                let is_cur = idx == self.cursor_y;
                let line = &self.lines[idx];
                let num = idx + 1;

                // Ensure syntax tokens are available for this line
                let syntax_enabled = self.editor_cfg.syntax_enabled;
                let use_color = self.editor_cfg.color_enabled;
                if syntax_enabled && self.syntax.language.is_some() {
                    self.syntax.tokenize_line(idx, line);
                }

                // ── gutter ──
                let gutter_num = format!("{:>w$}", num, w = gw - 1);
                queue!(stdout, MoveTo(eo as u16, i as u16))?;
                if is_cur {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::Rgb {
                            r: 44,
                            g: 44,
                            b: 58
                        }),
                        SetForegroundColor(Color::White),
                        Print(&gutter_num),
                        Print(" "),
                        ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Rgb {
                            r: 108,
                            g: 108,
                            b: 128
                        }),
                        Print(&gutter_num),
                        Print(" "),
                        ResetColor,
                    )?;
                }

                // ── text area with horizontal scroll ──
                let line_vis_width = byte_to_col(line, line.len());
                let visible_start_col = self.scroll_x;

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

                    let in_search = if self.mode == Mode::Search || !self.search_query.is_empty() {
                        self.search_matches
                            .iter()
                            .enumerate()
                            .any(|(mi, &(mly, msb, meb))| {
                                mly == idx
                                    && byte_start >= msb
                                    && byte_start < meb
                                    && mi == self.search_match_idx
                            })
                    } else {
                        false
                    };
                    let in_search_other = if !self.search_query.is_empty() {
                        self.search_matches.iter().any(|&(mly, msb, meb)| {
                            mly == idx && byte_start >= msb && byte_start < meb
                        })
                    } else {
                        false
                    };

                    queue!(stdout, MoveTo(abs_x as u16, i as u16))?;

                    if in_search {
                        queue!(
                            stdout,
                            SetBackgroundColor(Color::Rgb {
                                r: 162,
                                g: 119,
                                b: 255
                            }),
                            SetForegroundColor(Color::Black),
                            Print(ch),
                            ResetColor
                        )?;
                    } else if in_search_other {
                        queue!(
                            stdout,
                        SetBackgroundColor(Color::Rgb {
                            r: 44,
                            g: 44,
                            b: 58
                        }),
                        SetForegroundColor(Color::White),
                            Print(ch),
                            ResetColor
                        )?;
                    } else if in_sel {
                        queue!(
                            stdout,
                        SetBackgroundColor(Color::Rgb {
                            r: 54,
                            g: 51,
                            b: 84
                        }),
                        SetForegroundColor(Color::White),
                        Print(ch),
                            ResetColor
                        )?;
                    } else if is_cur {
                        let fg = if use_color && syntax_enabled {
                            self.syntax
                                .token_type_at(idx, byte_start)
                                .map(crate::syntax::color_for)
                                .unwrap_or(Color::Rgb {
                                    r: 216,
                                    g: 216,
                                    b: 224,
                                })
                        } else {
                            Color::Rgb {
                                r: 216,
                                g: 216,
                                b: 224,
                            }
                        };
                        queue!(
                            stdout,
                            SetBackgroundColor(Color::Rgb {
                                r: 28,
                                g: 28,
                                b: 36
                            }),
                            SetForegroundColor(fg),
                            Print(ch),
                            ResetColor
                        )?;
                    } else {
                        let fg = if use_color && syntax_enabled {
                            self.syntax
                                .token_type_at(idx, byte_start)
                                .map(crate::syntax::color_for)
                                .unwrap_or(Color::Rgb {
                                    r: 200,
                                    g: 200,
                                    b: 208,
                                })
                        } else {
                            Color::Rgb {
                                r: 200,
                                g: 200,
                                b: 208,
                            }
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
                        MoveTo(last_x as u16, i as u16),
                        SetBackgroundColor(Color::Rgb {
                            r: 54,
                            g: 51,
                            b: 84
                        }),
                        Print(" ".repeat(cols - last_x)),
                        ResetColor
                    )?;
                }

                // Clear trailing characters before rendering scroll indicators
                queue!(stdout, Clear(ClearType::UntilNewLine))?;

                if line_vis_width > self.scroll_x + text_cols {
                    let right_x = eo + gw + text_cols;
                    if right_x < cols {
                        queue!(
                            stdout,
                            MoveTo(right_x as u16, i as u16),
                            SetForegroundColor(Color::DarkYellow),
                            Print("›"),
                            ResetColor
                        )?;
                    }
                }
                if self.scroll_x > 0 {
                    queue!(
                        stdout,
                        MoveTo((eo + gw) as u16, i as u16),
                        SetForegroundColor(Color::DarkYellow),
                        Print("‹"),
                        ResetColor
                    )?;
                }

                // ── scroll bar per-row ──
                {
                    let total = self.lines.len();
                    if total > rows {
                        let thumb = self.scroll * (rows - 1) / (total - rows);
                        let ch = if i == thumb { '▌' } else { '░' };
                        queue!(
                            stdout,
                            MoveTo(sb_col as u16, i as u16),
                        SetForegroundColor(Color::Rgb {
                            r: 108,
                            g: 108,
                            b: 128
                        }),
                        Print(ch),
                        ResetColor
                    )?;
                }
            }

            // Primary cursor
                if is_cur {
                    let cursor_col = byte_to_col(self.current(), self.cursor_byte);
                    let screen_col = if cursor_col >= self.scroll_x {
                        eo + gw + (cursor_col - self.scroll_x)
                    } else {
                        eo + gw
                    };
                    if screen_col < cols {
                        let line_s = self.current();
                        let cursor_ch = if self.cursor_byte < line_s.len() {
                            line_s[self.cursor_byte..].chars().next().unwrap_or(' ')
                        } else {
                            ' '
                        };
                        queue!(
                            stdout,
                            MoveTo(screen_col as u16, i as u16),
                            SetBackgroundColor(Color::White),
                            SetForegroundColor(Color::Black),
                            Print(cursor_ch),
                            ResetColor,
                        )?;
                    }
                }
                // Extra cursors on this row
                for ec in &self.extra_cursors {
                    if ec.y == idx {
                        let cursor_col = byte_to_col(&self.lines[idx], ec.byte);
                        let screen_col = if cursor_col >= self.scroll_x {
                            eo + gw + (cursor_col - self.scroll_x)
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
                                MoveTo(screen_col as u16, i as u16),
                                SetBackgroundColor(Color::Rgb { r: 255, g: 140, b: 154 }),
                                SetForegroundColor(Color::Black),
                                Print(cursor_ch),
                                ResetColor,
                            )?;
                        }
                    }
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

        // ── horizontal scroll bar ──────────────────────────────────────────
        {
            let hrow = rows; // row index for the horizontal scroll bar (just above status)
            let max_vis = self
                .lines
                .iter()
                .map(|l| byte_to_col(l, l.len()))
                .max()
                .unwrap_or(0);
            if max_vis > text_cols {
                let avail = cols.saturating_sub(2); // leave 1 char margin each side
                let thumb_start = self.scroll_x * avail / max_vis;
                let thumb_end = (self.scroll_x + text_cols).min(max_vis) * avail / max_vis;
                let thumb_w = (thumb_end.saturating_sub(thumb_start)).max(1);

                queue!(
                    stdout,
                    MoveTo(0, hrow as u16),
                    SetBackgroundColor(Color::Rgb { r: 13, g: 13, b: 21 }),
                    SetForegroundColor(Color::Rgb { r: 61, g: 61, b: 77 }),
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
                            SetForegroundColor(Color::Rgb { r: 162, g: 119, b: 255 })
                        } else {
                            SetForegroundColor(Color::Rgb { r: 61, g: 61, b: 77 })
                        },
                        Print(ch),
                        ResetColor,
                    )?;
                }
                if avail + 1 < cols {
                    queue!(
                        stdout,
                        MoveTo((avail + 1) as u16, hrow as u16),
                        SetForegroundColor(Color::Rgb { r: 61, g: 61, b: 77 }),
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

        self.render_status_bar(stdout)?;
        self.render_hint_bar(stdout)?;
        stdout.flush()?;

        self.dirty_start = 0;
        self.dirty_end = 0;

        Ok(())
    }

    fn render_sidebar(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let use_color = self.editor_cfg.color_enabled;
        let rows = self.term_h.saturating_sub(3);
        let sw = SIDEBAR_WIDTH;
        let nodes = &self.sidebar.nodes;
        let scroll = self.sidebar.scroll;
        let num_visible = rows.min(nodes.len().saturating_sub(scroll));

        for i in 0..rows {
            // Background for entire sidebar row
            let has_node = i < num_visible;
            let is_selected = has_node && (scroll + i) == self.sidebar.selected;
            let bg = if is_selected {
                Color::Rgb { r: 44, g: 44, b: 58 }
            } else {
                Color::Rgb { r: 13, g: 13, b: 21 }
            };
            queue!(
                stdout,
                MoveTo(0, i as u16),
                SetBackgroundColor(bg),
                Print(" ".repeat(sw)),
            )?;

            // Node content
            if has_node {
                let idx = scroll + i;
                let node = &nodes[idx];
                let is_open = node.path == self.path;

                let mut x = 1 + node.depth * 2;
                if x < sw {
                    if node.is_dir {
                        let ind = if node.expanded { "▼" } else { "▶" };
                        let ind_fg = if use_color {
                            Color::Rgb { r: 130, g: 226, b: 255 }
                        } else {
                            Color::White
                        };
                        queue!(
                            stdout,
                            MoveTo(x as u16, i as u16),
                            SetForegroundColor(ind_fg),
                            Print(ind),
                        )?;
                        x += 2;
                    } else {
                        let dot_fg = if use_color {
                            Color::Rgb { r: 108, g: 108, b: 128 }
                        } else {
                            Color::White
                        };
                        queue!(
                            stdout,
                            MoveTo(x as u16, i as u16),
                            SetForegroundColor(dot_fg),
                            Print('·'),
                        )?;
                        x += 2;
                    }

                    if x < sw {
                        let max_name = sw.saturating_sub(x + 1);
                        let display: String = if node.name.chars().count() > max_name {
                            node.name
                                .chars()
                                .take(max_name.saturating_sub(1))
                                .collect::<String>()
                                + "…"
                        } else {
                            node.name.clone()
                        };
                        let fg = if !use_color {
                            Color::White
                        } else if is_open {
                            Color::Rgb { r: 255, g: 202, b: 133 }
                        } else if is_selected {
                            Color::White
                        } else if node.is_dir {
                            Color::Rgb { r: 120, g: 220, b: 232 }
                        } else {
                            Color::Rgb { r: 200, g: 200, b: 208 }
                        };
                        queue!(
                            stdout,
                            SetForegroundColor(fg),
                            MoveTo(x as u16, i as u16),
                            Print(&display),
                        )?;
                    }
                }
            }

            // Separator
            queue!(
                stdout,
                ResetColor,
                MoveTo(sw as u16, i as u16),
                SetForegroundColor(Color::Rgb { r: 61, g: 61, b: 77 }),
                Print('│'),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_status_bar(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let sy = self.term_h.saturating_sub(2);
        let tag = if self.modified { " ●" } else { " ✓" };
        let mode_str = match self.mode {
            Mode::Normal => "",
            Mode::Insert => " [INSERT]",
            Mode::Search => " [SEARCH]",
            Mode::Visual => " [VISUAL]",
            Mode::Command => " [COMMAND]",
            Mode::Help => " [HELP]",
            Mode::FileFinder => " [FIND]",
            Mode::Gosc => " [GOSC]",
        };
        let sel_str = if self.has_selection() {
            let (ay, ab) = self.selection_anchor.unwrap();
            let ((sy2, sb), (ey, eb)) = super::sel_range(self.cursor_y, self.cursor_byte, ay, ab);
            if sy2 == ey {
                let chars = self.lines[sy2][sb..eb.min(self.lines[sy2].len())]
                    .chars()
                    .count();
                format!(" [{} chars]", chars)
            } else {
                format!(" [{} lines]", ey - sy2 + 1)
            }
        } else {
            String::new()
        };
        let col = byte_to_col(self.current(), self.cursor_byte) + 1;
        let total = self.lines.len();
        let status = format!(
            " {}{}   Ln {}/{}  Col {}{}{}  ",
            self.path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
            tag,
            self.cursor_y + 1,
            total,
            col,
            sel_str,
            mode_str,
        );

        // Right-align file info
        let right = if self.modified { "modified" } else { "saved" };
        let pad = self.term_w.saturating_sub(status.len() + right.len() + 2);
        let full = format!("{}{}{} ", status, " ".repeat(pad), right);

        let use_color = self.editor_cfg.color_enabled;
        let status_bg = if use_color {
            Color::Rgb { r: 162, g: 119, b: 255 }
        } else {
            Color::Rgb { r: 33, g: 33, b: 44 }
        };
        queue!(
            stdout,
            MoveTo(0, sy as u16),
            SetBackgroundColor(status_bg),
            SetForegroundColor(Color::White),
            Print(&full),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        Ok(())
    }

    fn render_hint_bar(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let hy = self.term_h.saturating_sub(1);

        let hint = match self.mode {
            Mode::Search => {
                let count = self.search_matches.len();
                let idx = if count > 0 {
                    self.search_match_idx + 1
                } else {
                    0
                };
                format!(
                    " Search: {}  [{}/{}]  Enter=next  Shift+Enter=prev  Esc=close",
                    self.search_query, idx, count
                )
            }
            Mode::Command => {
                format!(
                    ":{}  cd|back|gos|mkdir|touch|ne|tp|tp-add  Enter=exec  Esc=cancel",
                    self.cmd_buf
                )
            }
            Mode::Help => " Help — Esc/q=close  ↑↓=scroll".to_string(),
            Mode::Visual => {
                if let Some(ref msg) = self.status_msg {
                    format!(
                        " ✦ {}  │  ^I=insert  ^C=copy  ^X=cut  ^H=help  Esc=exit",
                        msg
                    )
                } else {
                    " Visual mode — arrows=select  ^I=insert  ^C=copy  ^X=cut  ^H=help  Esc=exit"
                        .to_string()
                }
            }
            Mode::Insert => {
                if let Some(ref msg) = self.status_msg {
                    format!(" ✦ {}  │  Esc=normal  ^W=visual  ^P=paste  ^H=help", msg)
                } else {
                    " Insert mode — Esc=normal  ^W=visual  ^P=paste  ^C=copy  ^F=find  ^Z=undo  ^R=redo  ^Y=yank  ^A=selAll  ^L=selLine  ^D=nextOcc  ^H=help".to_string()
                }
            }
            Mode::Normal => {
                if let Some(ref msg) = self.status_msg {
                    format!(
                        " ✦ {}  │  ^I=insert  ^W=visual  :command  ^H=help  ^Q=quit",
                        msg
                    )
                } else {
                    " ^I=insert  ^W=visual  ^B=sidebar  ^P=files  :command  ^H=help  ^Q=quit".to_string()
                }
            }
            Mode::FileFinder => {
                format!(
                    " Find — {} results  ↑↓=nav  Enter=open  Esc=close",
                    self.ff_results.len()
                )
            }
            Mode::Gosc => {
                format!(
                    " gosc — {} dirs  type number+Enter=nav  0=exit  Esc=close",
                    self.gosc_dirs.len()
                )
            }
        };

        let hint_trimmed = if hint.len() > self.term_w {
            &hint[..self.term_w]
        } else {
            &hint
        };

        let use_color = self.editor_cfg.color_enabled;
        let hint_fg = if use_color {
            Color::Rgb { r: 130, g: 226, b: 255 }
        } else {
            Color::White
        };
        queue!(
            stdout,
            MoveTo(0, hy as u16),
            SetBackgroundColor(Color::Rgb {
                r: 21,
                g: 21,
                b: 29
            }),
            SetForegroundColor(hint_fg),
            Print(hint_trimmed),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        Ok(())
    }

    fn render_help_screen(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let rows = self.term_h.saturating_sub(3);
        let cols = self.term_w;

        let help_lines = vec![
            "─".repeat(cols.min(60)),
            " NORMAL MODE (read-only)".to_string(),
            "  ^I        enter Insert mode".to_string(),
            "  ^W        enter Visual mode".to_string(),
            "  :         open command line".to_string(),
            "  :auto on  enable auto-save on exit".to_string(),
            "  :auto off disable auto-save on exit".to_string(),
            "  ^H        open this help screen".to_string(),
            "  ^Q / Esc  exit editor (with save prompt)".to_string(),
            "  arrows    navigate cursor".to_string(),
            "".to_string(),
            " INSERT MODE (editing)".to_string(),
            "  ^C        copy selection".to_string(),
            "  ^F        find / search".to_string(),
            "  ^Z        undo".to_string(),
            "  ^R        redo".to_string(),
            "  ^Y        yank (copy) current line".to_string(),
            "  ^A        select all".to_string(),
            "  ^L        select current line".to_string(),
            "  ^D        find next occurrence".to_string(),
            "  ^X        cut selection".to_string(),
            "  ^S        save".to_string(),
            "  ^W        enter Visual mode".to_string(),
            "  Esc       return to Normal mode".to_string(),
            "  ^Q        quit (with save prompt)".to_string(),
            "  ^H        help".to_string(),
            "  Home      start of line".to_string(),
            "  End       end of line".to_string(),
            "".to_string(),
            " VISUAL MODE (selection)".to_string(),
            "  arrows    extend selection".to_string(),
            "  ^I        enter Insert mode".to_string(),
            "  ^C        copy + exit".to_string(),
            "  ^X        cut + exit".to_string(),
            "  ^Q        quit (with save prompt)".to_string(),
            "  ^H        help".to_string(),
            "  Esc       exit visual mode".to_string(),
            "  mouse     click & drag to select".to_string(),
            "".to_string(),
            " COMMAND MODE".to_string(),
            "  :<number>          go to line number".to_string(),
            "  :l N or :line N    go to line N".to_string(),
            "  :q or :quit        quit editor (with save prompt)".to_string(),
            "  :wq                save and quit".to_string(),
            "  :w or :write       save file".to_string(),
            "  :auto on           enable auto-save".to_string(),
            "  :auto off          disable auto-save".to_string(),
            "  :syntax on         enable syntax highlighting".to_string(),
            "  :syntax off        disable syntax highlighting".to_string(),
            "  :cd <path>         change directory (Navigator)".to_string(),
            "  :back [N]          go to parent directory (N levels)".to_string(),
            "  :gos <N>           navigate to Nth subdirectory".to_string(),
            "  :gosc              list numbered subdirectories".to_string(),
            "  :mkdir <dir>       create directory".to_string(),
            "  :touch <file>      create new file".to_string(),
            "  :ne <file>         navigate & edit (create if missing)".to_string(),
            "  :ne .              show CWD in sidebar".to_string(),
            "  :tp <name>         jump to teleport savepoint".to_string(),
            "  :tp-add <name>     save CWD as teleport".to_string(),
            "  Esc                 cancel".to_string(),
            "".to_string(),
            "─".repeat(cols.min(60)),
        ];

        for i in 0..rows {
            let idx = self.help_scroll + i;
            let line = help_lines.get(idx).map(|s| s.as_str()).unwrap_or("");
            queue!(
                stdout,
                MoveTo(0, i as u16),
                SetForegroundColor(Color::Rgb {
                    r: 200,
                    g: 200,
                    b: 208
                }),
                Print(line),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        Ok(())
    }

    fn render_file_finder(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let cols = self.term_w;
        let rows = self.term_h.saturating_sub(3);
        let max_results = rows.saturating_sub(2);

        // Row 0: prompt
        let prompt = format!(
            " Find: {}",
            if self.ff_query.is_empty() {
                "Type to search..."
            } else {
                &self.ff_query
            }
        );
        queue!(
            stdout,
            MoveTo(0, 0),
            SetBackgroundColor(Color::Rgb {
                r: 28,
                g: 28,
                b: 36
            }),
            SetForegroundColor(Color::White),
            Print(&prompt),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;

        // Row 1: separator
        queue!(
            stdout,
            MoveTo(0, 1),
            SetBackgroundColor(Color::Rgb {
                r: 13,
                g: 13,
                b: 21
            }),
            SetForegroundColor(Color::Rgb {
                r: 61,
                g: 61,
                b: 77
            }),
            Print("─".repeat(cols.min(80))),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;

        // Results rows
        if !self.ff_results.is_empty() {
            let start_idx = self
                .ff_idx
                .saturating_sub(max_results.saturating_sub(1));
            let end_idx = (start_idx + max_results).min(self.ff_results.len());
            for i in start_idx..end_idx {
                let ri = 2 + (i - start_idx);
                if ri >= rows {
                    break;
                }
                let (ref name, _, _) = self.ff_results[i];
                let is_selected = i == self.ff_idx;
                let display = if name.chars().count() > cols.saturating_sub(4) {
                    name.chars()
                        .take(cols.saturating_sub(5))
                        .collect::<String>()
                        + "…"
                } else {
                    name.clone()
                };
                if is_selected {
                    queue!(
                        stdout,
                        MoveTo(0, ri as u16),
                        SetBackgroundColor(Color::Rgb {
                            r: 44,
                            g: 44,
                            b: 58
                        }),
                        SetForegroundColor(Color::White),
                        Print(format!("  {}", display)),
                        ResetColor,
                        Clear(ClearType::UntilNewLine),
                    )?;
                } else {
                    queue!(
                        stdout,
                        MoveTo(0, ri as u16),
                        SetForegroundColor(Color::Rgb {
                            r: 200,
                            g: 200,
                            b: 208
                        }),
                        Print(format!("  {}", display)),
                        ResetColor,
                        Clear(ClearType::UntilNewLine),
                    )?;
                }
            }
        }

        Ok(())
    }

    fn render_gosc(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let cols = self.term_w;
        let rows = self.term_h.saturating_sub(3);
        let panel_w = 46usize.min(cols.saturating_sub(4));
        let panel_h = (self.gosc_dirs.len() + 5).min(rows.saturating_sub(4));
        let left = (cols.saturating_sub(panel_w)) / 2;
        let top = (rows.saturating_sub(panel_h)) / 2;
        let inner_w = panel_w.saturating_sub(4);

        // Clear panel area
        for i in 0..panel_h {
            queue!(
                stdout,
                MoveTo(left as u16, (top + i) as u16),
                SetBackgroundColor(Color::Rgb {
                    r: 28,
                    g: 28,
                    b: 36
                }),
                Print(" ".repeat(panel_w)),
                ResetColor,
            )?;
        }

        // Title
        let title = " gosc — Navigate Continuously ";
        let title_x = left + (panel_w.saturating_sub(title.len())) / 2;
        queue!(
            stdout,
            MoveTo(title_x as u16, top as u16),
            SetForegroundColor(Color::Cyan),
            SetBackgroundColor(Color::Rgb {
                r: 28,
                g: 28,
                b: 36
            }),
            Print(title),
            ResetColor,
        )?;

        // Separator
        let sep = "─".repeat(panel_w.saturating_sub(2));
        queue!(
            stdout,
            MoveTo((left + 1) as u16, (top + 1) as u16),
            SetForegroundColor(Color::Rgb {
                r: 61,
                g: 61,
                b: 77
            }),
            SetBackgroundColor(Color::Rgb {
                r: 28,
                g: 28,
                b: 36
            }),
            Print(&sep),
            ResetColor,
        )?;

        // Entries
        let max_vis = panel_h.saturating_sub(4).min(self.gosc_dirs.len());
        for i in 0..max_vis {
            let ri = top + 2 + i;
            let prefix = format!(" {:>2}. ", i + 1);
            let name = &self.gosc_dirs[i];
            let display = if name.len() > inner_w.saturating_sub(5) {
                format!("{}…", &name[..inner_w.saturating_sub(6)])
            } else {
                name.clone()
            };
            let line = format!("{}{}", prefix, display);
            queue!(
                stdout,
                MoveTo((left + 2) as u16, ri as u16),
                SetForegroundColor(Color::Rgb {
                    r: 200,
                    g: 200,
                    b: 208
                }),
                SetBackgroundColor(Color::Rgb {
                    r: 28,
                    g: 28,
                    b: 36
                }),
                Print(&line),
                ResetColor,
            )?;
        }

        // Input line
        if panel_h >= 5 {
            let input_ri = top + panel_h - 2;
            let hint_prefix = if self.gosc_buf.is_empty() { "0=exit " } else { "" };
            let input_label = format!("{}{}>", hint_prefix, self.gosc_buf);
            queue!(
                stdout,
                MoveTo((left + 2) as u16, input_ri as u16),
                SetForegroundColor(Color::Green),
                SetBackgroundColor(Color::Rgb {
                    r: 28,
                    g: 28,
                    b: 36
                }),
                Print(&input_label),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        // Bottom hint
        if panel_h >= 6 {
            let hint_ri = top + panel_h - 1;
            let hint = " Enter=go  -N=back N  Esc=close ";
            queue!(
                stdout,
                MoveTo((left + 2) as u16, hint_ri as u16),
                SetForegroundColor(Color::Rgb {
                    r: 108,
                    g: 108,
                    b: 128
                }),
                SetBackgroundColor(Color::Rgb {
                    r: 28,
                    g: 28,
                    b: 36
                }),
                Print(hint),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        Ok(())
    }
}
