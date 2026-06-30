use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;

use super::super::{byte_to_col, Editor, Focus, Mode};

impl Editor {
    pub(crate) fn render_tab_bar(&self, stdout: &mut std::io::Stdout, y: usize, x_offset: usize, width: usize) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let use_color = self.editor_cfg.color_enabled;
        let active = self.active_tab;
        let mut x = x_offset;

        queue!(
            stdout,
            MoveTo(x_offset as u16, y as u16),
            SetBackgroundColor(theme.editor.sidebar_bg.to_crossterm()),
            Clear(ClearType::UntilNewLine),
        )?;

        let cols = x_offset + width;
        for (i, tab) in self.tabs.iter().enumerate() {
            if x >= cols {
                break;
            }
            let name = if self.is_scratch_path(&tab.path) {
                "[scratch]".to_string()
            } else {
                tab.path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "untitled".to_string())
            };
            let mark = if tab.modified { " ●" } else { "" };
            let label = format!(" {} {} ", name, mark);
            let close_btn = "✕";
            let label_chars = label.chars().count();
            let full_w = label_chars + 1 + close_btn.chars().count();
            let is_active = i == active;

            let (bg, fg) = if is_active {
                let bg_c = if use_color { theme.editor.editor_bg.to_crossterm() } else { Color::Rgb { r: 18, g: 18, b: 28 } };
                let fg_c = if use_color { theme.syntax.normal.to_crossterm() } else { Color::White };
                (bg_c, fg_c)
            } else {
                let bg_c = if use_color { theme.editor.sidebar_bg.to_crossterm() } else { Color::Rgb { r: 18, g: 18, b: 28 } };
                let fg_c = if use_color { theme.editor.gutter_text.to_crossterm() } else { Color::DarkGrey };
                (bg_c, fg_c)
            };

            let close_fg = if use_color { theme.editor.gutter_text.to_crossterm() } else { Color::DarkGrey };

            let remaining = cols.saturating_sub(x);
            if full_w > remaining {
                let avail = remaining.saturating_sub(2);
                    if avail > 3 {
                        let truncated_name: String = name.chars().take(avail).collect();
                        let short = format!(" {}… {} ", truncated_name, close_btn);
                        queue!(
                            stdout,
                            MoveTo(x as u16, y as u16),
                            SetBackgroundColor(bg),
                            SetForegroundColor(fg),
                            Print(&short),
                            ResetColor,
                        )?;
                    }
                    break;
            }

            queue!(
                stdout,
                MoveTo(x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(fg),
                Print(&label),
            )?;
            x += label_chars;

            queue!(
                stdout,
                SetForegroundColor(close_fg),
                Print(close_btn),
                ResetColor,
            )?;
            x += close_btn.chars().count();
        }

        // ── close-all X button (fixed at far right) ──────────────
        if x_offset + width >= 3 {
            let far_right = x_offset + width;
            let close_all_x = far_right.saturating_sub(2);
            let separator_x = close_all_x.saturating_sub(1);
            queue!(
                stdout,
                MoveTo(separator_x as u16, y as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                Print("│"),
                MoveTo(close_all_x as u16, y as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                Print("✕"),
                ResetColor,
            )?;
        }

        queue!(stdout, ResetColor)?;
        Ok(())
    }

    pub(crate) fn render_cwd_bar(&self, stdout: &mut std::io::Stdout, x_offset: usize) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cy = self.term_h.saturating_sub(3);
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "?".to_string());
        let avail_w = self.term_w.saturating_sub(x_offset);
        let cwd_label = format!(" ── CWD: {} ", cwd);
        let fill = avail_w.saturating_sub(cwd_label.chars().count());
        let line = format!("{}{}", cwd_label, "─".repeat(fill));
        let use_color = self.editor_cfg.color_enabled;
        let fg = if use_color {
            theme.editor.hint_text.to_crossterm()
        } else {
            Color::White
        };
        queue!(
            stdout,
            MoveTo(x_offset as u16, cy as u16),
            SetBackgroundColor(theme.editor.hint_bg.to_crossterm()),
            SetForegroundColor(fg),
            Print(&line),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        Ok(())
    }

    pub(crate) fn render_status_bar(&self, stdout: &mut std::io::Stdout, x_offset: usize) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let sy = self.term_h.saturating_sub(2);
        let avail_w = self.term_w.saturating_sub(x_offset);
        let tag = if self.modified { " ●" } else { " ✓" };
        let mode_str = match self.mode {
            Mode::Normal => "",
            Mode::Insert => " [INSERT]",
            Mode::InsertMulti => " [INSERT*]",
            Mode::Jump => " [JUMP]",
            Mode::Search => " [SEARCH]",
            Mode::Visual => " [VISUAL]",
            Mode::Command => " [COMMAND]",
            Mode::Help => " [HELP]",
            Mode::FileFinder => " [FIND]",
            Mode::Gosc => " [GOSC]",
            Mode::Query => " [QUERY]",
        };
        let panel_str = match self.focus {
            Focus::Run => " [RUN]",
            Focus::Terminal => " [TERM]",
            Focus::Editor => "",
        };
        let sel_str = if let Some((ay, ab)) = self.cursor.selection_anchor {
            if (ay, ab) == (self.cursor.y, self.cursor.byte) {
                String::new()
            } else {
                let ((sy2, sb), (ey, eb)) = super::super::sel_range(self.cursor.y, self.cursor.byte, ay, ab);
                if sy2 == ey {
                    let chars = self.lines[sy2][sb..eb.min(self.lines[sy2].len())]
                        .chars()
                        .count();
                    format!(" [{} chars]", chars)
                } else {
                    format!(" [{} lines]", ey - sy2 + 1)
                }
            }
        } else {
            String::new()
        };
        let col = byte_to_col(self.current(), self.cursor.byte) + 1;
        let total = self.lines.len();
        let status = format!(
            " {}{}   Ln {}/{}  Col {}{}{}{}  ",
            self.path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
            tag,
            self.cursor.y + 1,
            total,
            col,
            sel_str,
            mode_str,
            panel_str,
        );

        let right = if self.modified { "modified" } else { "saved" };
        let pad = avail_w.saturating_sub(status.len() + right.len() + 2);
        let full = format!("{}{}{} ", status, " ".repeat(pad), right);

        let use_color = self.editor_cfg.color_enabled;
        let status_bg = if use_color {
            theme.editor.status_bg.to_crossterm()
        } else {
            Color::Rgb { r: 33, g: 33, b: 44 }
        };
        queue!(
            stdout,
            MoveTo(x_offset as u16, sy as u16),
            SetBackgroundColor(status_bg),
            SetForegroundColor(theme.editor.status_text.to_crossterm()),
            Print(&full),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        Ok(())
    }

    pub(crate) fn render_hint_bar(&self, stdout: &mut std::io::Stdout, x_offset: usize) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let hy = self.term_h.saturating_sub(1);
        let msg_y = self.term_h.saturating_sub(3);
        let avail_w = self.term_w.saturating_sub(x_offset);

        let (status_line, hint) = match self.focus {
            Focus::Run => {
                let hint = if !self.run_cmd_buf.is_empty() {
                    format!("{}  Enter=exec  Esc=cancel", self.run_cmd_buf)
                } else {
                    let visible = self.run_panel_height();
                    let total = self.run_lines.len();
                    let top = self.sv.run + 1;
                    let bot = (self.sv.run + visible).min(total);
                    if total > 0 {
                        format!(
                            " Run output — lines {}-{}/{}  :=cmd  ↑↓/j/k=scroll  Esc=close",
                            top, bot, total
                        )
                    } else {
                        " Run output — (empty)  :=cmd  Esc=close".to_string()
                    }
                };
                (String::new(), hint)
            }
            Focus::Terminal => {
                if let Some(ref term) = self.terminal {
                    if term.exited {
                        let code = term.exit_code.map(|c| c.to_string()).unwrap_or_default();
                        (String::new(), format!(" Terminal exited {}  Esc/q=close", code))
                    } else {
                        let total = term.total_lines();
                        (String::new(), format!(" Terminal — {} lines  PgUp/PgDn=scroll  Ctrl+Shift+C=copy  Ctrl+C=interrupt  Esc=close", total))
                    }
                } else {
                    (String::new(), " Terminal".to_string())
                }
            }
            Focus::Editor => match self.mode {
                Mode::Search => {
                    let count = self.search.matches.len();
                    let idx = if count > 0 {
                        self.search.match_idx + 1
                    } else {
                        0
                    };
                    let hint = format!(
                        " Search: {}  [{}/{}]  Enter=next  Shift+Enter=prev  Esc=close",
                        self.search.query, idx, count
                    );
                    (String::new(), hint)
                }
                Mode::Command => {
                    let hint = format!(
                        ":{}  cd|back|gos|gs|mkdir|touch|ne|tp|tp-add  Enter=exec  Esc=cancel",
                        self.cmd_buf
                    );
                    (String::new(), hint)
                }
                Mode::Help => (String::new(), " Help — Esc/q=close  ↑↓=scroll".to_string()),
                Mode::Visual => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), " ^O=insert  ^C=copy  ^X=cut  ^H=help  Esc=exit".to_string())
                    } else {
                        (String::new(), " Visual mode — arrows=select  ^O=insert  ^C=copy  ^X=cut  ^H=help  Esc=exit".to_string())
                    }
                }
                Mode::Insert => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), " Esc=normal  ^W=visual  ^P=paste  ^H=help".to_string())
                    } else {
                        (String::new(), " Insert mode — Esc=normal  ^W=visual  ^P=paste  ^C=copy  ^F=find  ^Z=undo  ^R=redo  ^Y=yank  ^A=selAll  ^L=selLine  ^D=nextOcc  ^H=help".to_string())
                    }
                }
                Mode::Normal => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), " ^O=insert  ^W=visual  :command  ^H=help  ^Q=closeTab/quit".to_string())
                    } else {
                        (String::new(), " ^O=insert  ^W=visual  ^B=sidebar  ^P=files  :command  ^H=help  ^Q=closeTab/quit  ^PgDn/PgUp=tabs  ^N=files".to_string())
                    }
                }
                Mode::InsertMulti => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), " Esc=exit multi-cursor".to_string())
                    } else {
                        (String::new(), " Insert* — Esc=exit multi-cursor  ^D=nextOcc".to_string())
                    }
                }
                Mode::Jump => {
                    let hint = format!(" Go to line — {}  Enter=jump  Esc=cancel", self.jump_buf);
                    (String::new(), hint)
                }
                Mode::FileFinder => {
                    let hint = if self.ff_query.starts_with("gs:") {
                        if self.gs_state.loading {
                            format!(
                                " gs — searching... {} results so far  Esc=close",
                                self.gs_state.results.len()
                            )
                        } else {
                            format!(
                                " gs — {} results  ↑↓=nav  Enter=open  Esc=close",
                                self.gs_state.results.len()
                            )
                        }
                    } else {
                        format!(
                            " Find — {} results  ↑↓=nav  Enter=open  Esc=close",
                            self.ff_results.len()
                        )
                    };
                    (String::new(), hint)
                }
                Mode::Gosc => {
                    let hint = format!(
                        " gosc — {} dirs  type number+Enter=nav  0=exit  Esc=close",
                        self.gosc_dirs.len()
                    );
                    (String::new(), hint)
                }
                Mode::Query => {
                    let hint = if self.query_buf.is_empty() {
                        " x:   /s /l /a  /s /l N to M  /p old to new  /add  /set  /qal  /undo  /redo  /upper  /lower  /help  ; chains  Enter=exec  Esc=exit".into()
                    } else {
                        format!(" x: {}  Enter=exec  Esc=exit", self.query_buf)
                    };
                    (String::new(), hint)
                }
            },
        };

        let use_color = self.editor_cfg.color_enabled;
        let hint_fg = if use_color {
            theme.editor.hint_text.to_crossterm()
        } else {
            Color::White
        };
        let hint_bg = theme.editor.hint_bg.to_crossterm();

        if !status_line.is_empty() {
            let trimmed = if status_line.len() > avail_w {
                let byte_end = status_line.floor_char_boundary(avail_w);
                &status_line[..byte_end]
            } else {
                &status_line
            };
            queue!(
                stdout,
                MoveTo(x_offset as u16, msg_y as u16),
                SetBackgroundColor(hint_bg),
                SetForegroundColor(hint_fg),
                Print(trimmed),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        let hint_trimmed = if hint.len() > avail_w {
            let byte_end = hint.floor_char_boundary(avail_w);
            &hint[..byte_end]
        } else {
            &hint
        };

        queue!(
            stdout,
            MoveTo(x_offset as u16, hy as u16),
            SetBackgroundColor(hint_bg),
            SetForegroundColor(hint_fg),
            Print(hint_trimmed),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        Ok(())
    }
}
