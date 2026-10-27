use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use super::super::{byte_to_col, Editor, Focus, Mode};
use crate::t;

impl Editor {
    pub(crate) fn render_tab_bar(
        &self,
        stdout: &mut std::io::Stdout,
        y: usize,
        x_offset: usize,
        width: usize,
    ) -> std::io::Result<()> {
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
            let name = self.tab_display_name(tab);
            let label = self.tab_label(i);
            let close_btn = "✕";
            let label_chars = label.chars().count();
            let full_w = label_chars + 1 + close_btn.chars().count();
            let is_active = i == active;

            let (bg, fg) = if is_active {
                let bg_c = if use_color {
                    theme.editor.editor_bg.to_crossterm()
                } else {
                    Color::Rgb {
                        r: 18,
                        g: 18,
                        b: 28,
                    }
                };
                let fg_c = if use_color {
                    theme.syntax.normal.to_crossterm()
                } else {
                    Color::White
                };
                (bg_c, fg_c)
            } else {
                let bg_c = if use_color {
                    theme.editor.sidebar_bg.to_crossterm()
                } else {
                    Color::Rgb {
                        r: 18,
                        g: 18,
                        b: 28,
                    }
                };
                let fg_c = if use_color {
                    theme.editor.gutter_text.to_crossterm()
                } else {
                    Color::DarkGrey
                };
                (bg_c, fg_c)
            };

            let close_fg = if use_color {
                theme.editor.gutter_text.to_crossterm()
            } else {
                Color::DarkGrey
            };

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

    pub(crate) fn render_cwd_bar(
        &self,
        stdout: &mut std::io::Stdout,
        x_offset: usize,
    ) -> std::io::Result<()> {
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

    fn mode_label(&self) -> String {
        match self.focus {
            Focus::Run => t!("editor.mode_run"),
            Focus::Terminal => t!("editor.mode_terminal"),
            Focus::Editor => match self.mode {
                Mode::Normal => t!("editor.mode_normal"),
                Mode::Insert => t!("editor.mode_insert"),
                Mode::InsertMulti => t!("editor.mode_insert_multi"),
                Mode::Visual => t!("editor.mode_visual"),
                Mode::Search => t!("editor.mode_search"),
                Mode::Command => t!("editor.mode_command"),
                Mode::Help => t!("editor.mode_help"),
                Mode::FileFinder => t!("editor.mode_finder"),
                Mode::Gosc => t!("editor.mode_gosc"),
                Mode::Jump => t!("editor.mode_jump"),
                Mode::Query => t!("editor.mode_query"),
                Mode::Gmode => t!("editor.mode_gmode"),
                Mode::Nfe => t!("editor.mode_nfe"),
                Mode::Encoding => t!("editor.mode_encoding"),
                Mode::GitDiff => t!("editor.mode_git_diff"),
            },
        }
    }

    pub(crate) fn render_status_bar(
        &self,
        stdout: &mut std::io::Stdout,
        x_offset: usize,
    ) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let sy = self.term_h.saturating_sub(2);
        let avail_w = self.term_w.saturating_sub(x_offset);
        let label = self.mode_label();

        let (status, right) = {
            let tag = if self.modified {
                t!("editor.status_modified_tag")
            } else {
                t!("editor.status_saved_tag")
            };
            let sel_str = if let Some((ay, ab)) = self.cursor.selection_anchor {
                if (ay, ab) == (self.cursor.y, self.cursor.byte) {
                    String::new()
                } else {
                    let ((sy2, sb), (ey, eb)) =
                        super::super::sel_range(self.cursor.y, self.cursor.byte, ay, ab);
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
            let git_frag = if crate::config::Config::global_get_git_integration_enabled() {
                crate::utils::git::prompt_fragment(
                    &self
                        .path
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| std::path::PathBuf::from(".")),
                )
            } else {
                String::new()
            };
            let git_str = if git_frag.is_empty() {
                String::new()
            } else {
                format!(" [{}] ", git_frag)
            };
            let status = format!(
                " {} {}{}   Ln {}/{}  Col {}{}{} [{}] ",
                label,
                self.path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                tag,
                self.cursor.y + 1,
                total,
                col,
                sel_str,
                git_str,
                self.encoding.label(),
            );
            let right = if self.modified {
                t!("editor.status_modified")
            } else {
                t!("editor.status_saved")
            };
            (status, right)
        };

        let pad = avail_w.saturating_sub(status.len() + right.len() + 2);
        let mut full = format!("{}{}{} ", status, " ".repeat(pad), right);
        if full.chars().count() > avail_w {
            let end = full.floor_char_boundary(avail_w);
            full.truncate(end);
        }

        let use_color = self.editor_cfg.color_enabled;
        let status_bg = if use_color {
            theme.editor.status_bg.to_crossterm()
        } else {
            Color::Rgb {
                r: 33,
                g: 33,
                b: 44,
            }
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

    pub(crate) fn render_hint_bar(
        &self,
        stdout: &mut std::io::Stdout,
        x_offset: usize,
    ) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let hy = self.term_h.saturating_sub(1);
        let msg_y = self.term_h.saturating_sub(3);
        let avail_w = self.term_w.saturating_sub(x_offset);

        // Byte range of the dimmed ghost suggestion inside `hint`, when the
        // active prompt has one (see `Editor::prompt_ghost`).
        let mut ghost: Option<(usize, usize)> = None;
        // Screen column of the prompt caret, when the prompt has one.
        let mut caret: Option<usize> = None;
        let (status_line, hint) = match self.focus {
            Focus::Run => {
                let hint = if !self.run_cmd_buf.is_empty() {
                    format!("{}  {}", self.run_cmd_buf, t!("editor.hint_run_entry"))
                } else {
                    let visible = self.run_panel_height();
                    let total = self.run_lines.len();
                    let top = self.sv.run + 1;
                    let bot = (self.sv.run + visible).min(total);
                    if total > 0 {
                        format!(
                            " {} {}-{}/{}  {}",
                            t!("editor.hint_run_output"),
                            top,
                            bot,
                            total,
                            t!("editor.hint_run_scroll")
                        )
                    } else {
                        format!(
                            " {}  {}",
                            t!("editor.hint_run_empty"),
                            t!("editor.hint_run_scroll")
                        )
                    }
                };
                (String::new(), hint)
            }
            Focus::Terminal => {
                if let Some(ref term) = self.terminal {
                    if term.exited {
                        let code = term.exit_code.map(|c| c.to_string()).unwrap_or_default();
                        (
                            String::new(),
                            format!(
                                " {} {}  {}",
                                t!("editor.hint_term_exited"),
                                code,
                                t!("editor.hint_term_close")
                            ),
                        )
                    } else {
                        let total = term.total_lines();
                        (
                            String::new(),
                            format!(
                                " {} {} {}  {}",
                                t!("editor.hint_term_lines"),
                                total,
                                t!("editor.hint_term_actions"),
                                t!("editor.hint_term_close")
                            ),
                        )
                    }
                } else {
                    (String::new(), format!(" {}", t!("editor.hint_terminal")))
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
                        " {} {}  [{}/{}]  {}",
                        t!("editor.hint_search"),
                        self.search.query,
                        idx,
                        count,
                        t!("editor.hint_search_actions")
                    );
                    (String::new(), hint)
                }
                Mode::Command => {
                    let pos = self.prompt_pos();
                    let (prompt, g) =
                        with_prompt_ghost(":", &self.cmd_buf, pos, self.prompt_ghost());
                    ghost = g;
                    caret = Some(prompt_caret_col(":", &self.cmd_buf, pos));
                    let hint = format!(
                        "{}  {}  {}",
                        prompt,
                        t!("editor.hint_command_nav"),
                        t!("editor.hint_command_actions")
                    );
                    (String::new(), hint)
                }
                Mode::Help => (String::new(), format!(" {}", t!("editor.hint_help"))),
                Mode::Visual => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), t!("editor.hint_visual"))
                    } else {
                        (String::new(), t!("editor.hint_visual_no_msg"))
                    }
                }
                Mode::Insert => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), t!("editor.hint_insert_msg"))
                    } else {
                        (String::new(), t!("editor.hint_insert"))
                    }
                }
                Mode::Normal => {
                    if let Some(kind) = self.create_kind {
                        (
                            String::new(),
                            format!(
                                " {} {}  {}",
                                kind.prompt_label(),
                                self.create_buf,
                                t!("editor.hint_create_actions")
                            ),
                        )
                    } else if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), t!("editor.hint_normal_msg"))
                    } else {
                        (String::new(), t!("editor.hint_normal"))
                    }
                }
                Mode::InsertMulti => {
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), t!("editor.hint_insert_multi_msg"))
                    } else {
                        (String::new(), t!("editor.hint_insert_multi"))
                    }
                }
                Mode::Jump => {
                    let hint = format!(
                        " {} {}  {}",
                        t!("editor.hint_jump"),
                        self.jump_buf,
                        t!("editor.hint_jump_actions")
                    );
                    (String::new(), hint)
                }
                Mode::FileFinder => {
                    let hint = if self.ff_query.starts_with("gs:") {
                        if self.gs_state.loading {
                            format!(
                                " {} {} {}  {}",
                                t!("editor.hint_gs_loading"),
                                t!("editor.hint_results_so_far"),
                                self.gs_state.results.len(),
                                t!("editor.hint_gs_close")
                            )
                        } else {
                            format!(
                                " {} {} {}  {}",
                                t!("editor.hint_gs_done"),
                                self.gs_state.results.len(),
                                t!("editor.hint_results"),
                                t!("editor.hint_gs_close")
                            )
                        }
                    } else {
                        format!(
                            " {} {} {}  {}",
                            t!("editor.hint_find"),
                            self.ff_results.len(),
                            t!("editor.hint_results"),
                            t!("editor.hint_gs_close")
                        )
                    };
                    (String::new(), hint)
                }
                Mode::Gosc => {
                    let hint = format!(
                        " {} {} {}  {}",
                        t!("editor.hint_gosc"),
                        self.gosc_dirs.len(),
                        t!("editor.hint_gosc_nav"),
                        t!("editor.hint_gosc_close")
                    );
                    (String::new(), hint)
                }
                Mode::Encoding => {
                    let current = self.encoding.label();
                    let hint = format!(
                        " {} {}  {}",
                        t!("editor.hint_encoding"),
                        current,
                        t!("editor.enc_hint")
                    );
                    (String::new(), hint)
                }
                Mode::GitDiff => {
                    if let Some(state) = &self.git_diff {
                        let total = state.lines.len();
                        let lines_shown = self.term_h.saturating_sub(4);
                        let pct = if total > 0 {
                            (((state.scroll + lines_shown) as f64 / total as f64) * 100.0) as usize
                        } else {
                            100
                        };
                        (
                            format!(
                                " {} {}",
                                t!("editor.mode_git_diff"),
                                t!("editor.git_quit_hint")
                            ),
                            format!(" {}% ", pct.min(100)),
                        )
                    } else {
                        (String::new(), String::new())
                    }
                }
                Mode::Query => {
                    let hint = if self.query_buf.is_empty() {
                        t!("editor.hint_query")
                    } else {
                        let prefix = format!(" {} ", t!("editor.hint_query_prefix"));
                        let pos = self.prompt_pos();
                        let (prompt, g) =
                            with_prompt_ghost(&prefix, &self.query_buf, pos, self.prompt_ghost());
                        ghost = g;
                        caret = Some(prompt_caret_col(&prefix, &self.query_buf, pos));
                        format!("{}  {}", prompt, t!("editor.hint_query_actions"))
                    };
                    // Like Visual/Insert/Normal: surface the last command's
                    // result (`status_msg`) on the message row — otherwise a
                    // successful query shows no feedback while staying here.
                    if let Some(ref msg) = self.status_msg {
                        (format!(" ✦ {}  │", msg), hint)
                    } else {
                        (String::new(), hint)
                    }
                }
                Mode::Gmode => {
                    let prefix = format!(" {} ", t!("editor.hint_gmode"));
                    let pos = self.prompt_pos();
                    let (prompt, g) =
                        with_prompt_ghost(&prefix, &self.gmode_buf, pos, self.prompt_ghost());
                    ghost = g;
                    caret = Some(prompt_caret_col(&prefix, &self.gmode_buf, pos));
                    let hint = format!("{}  {}", prompt, t!("editor.hint_gmode_actions"));
                    (String::new(), hint)
                }
                Mode::Nfe => match &self.nfe_cmd {
                    Some(buf) => {
                        let pos = self.prompt_pos();
                        let (prompt, g) = with_prompt_ghost(" nfe:", buf, pos, self.prompt_ghost());
                        ghost = g;
                        caret = Some(prompt_caret_col(" nfe:", buf, pos));
                        let hint = format!("{}  {}", prompt, t!("editor.hint_nfe_cmd_actions"));
                        if let Some(ref msg) = self.status_msg {
                            (format!(" ✦ {}  │", msg), hint)
                        } else {
                            (String::new(), hint)
                        }
                    }
                    // Deleting a sidebar entry parks the `y`/`n` prompt in
                    // `status_msg`, so it must be surfaced here or the
                    // confirmation never appears.
                    None => match self.status_msg {
                        Some(ref msg) => (format!(" ✦ {}  │", msg), t!("editor.hint_nfe")),
                        None => (String::new(), t!("editor.hint_nfe")),
                    },
                },
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

        let ghost_fg = if use_color {
            theme.editor.gutter_text.to_crossterm()
        } else {
            Color::DarkGrey
        };
        queue!(
            stdout,
            MoveTo(x_offset as u16, hy as u16),
            SetBackgroundColor(hint_bg),
            SetForegroundColor(hint_fg),
        )?;
        // Paint character by character: the hint is dimmed and the prompt
        // caret (when the prompt has one) is drawn as a solid block.
        let ghost_range = ghost.map(|(s, len)| (s, s.saturating_add(len)));
        for (i, ch) in hint_trimmed.char_indices() {
            let dim = matches!(ghost_range, Some((s, e)) if i >= s && i < e);
            let at_caret = caret == Some(i);
            if at_caret {
                queue!(
                    stdout,
                    SetBackgroundColor(hint_fg),
                    SetForegroundColor(hint_bg)
                )?;
            } else if dim {
                queue!(stdout, SetForegroundColor(ghost_fg))?;
            }
            queue!(stdout, Print(ch))?;
            if at_caret {
                queue!(
                    stdout,
                    SetBackgroundColor(hint_bg),
                    SetForegroundColor(hint_fg)
                )?;
            } else if dim {
                queue!(stdout, SetForegroundColor(hint_fg))?;
            }
        }
        // Caret just past the last drawn character: block over the space.
        if caret == Some(hint_trimmed.len()) && hint_trimmed.len() < avail_w {
            queue!(
                stdout,
                SetBackgroundColor(hint_fg),
                SetForegroundColor(hint_bg),
                Print(" "),
                SetBackgroundColor(hint_bg),
                SetForegroundColor(hint_fg),
            )?;
        }
        queue!(stdout, ResetColor, Clear(ClearType::UntilNewLine))?;
        Ok(())
    }
}

/// Build a prompt line `prefix + buffer` with the ghost suggestion inserted
/// at the caret, and report the ghost's byte range so the renderer can dim
/// exactly that part.
fn with_prompt_ghost(
    prefix: &str,
    buf: &str,
    pos: usize,
    ghost: Option<String>,
) -> (String, Option<(usize, usize)>) {
    let caret = prompt_caret_col(prefix, buf, pos);
    let at = caret - prefix.len();
    let mut text = String::with_capacity(prefix.len() + buf.len());
    text.push_str(prefix);
    text.push_str(&buf[..at]);
    let start = text.len();
    let mut len = 0;
    if let Some(g) = ghost {
        if !g.is_empty() {
            text.push_str(&g);
            len = g.len();
        }
    }
    text.push_str(&buf[at..]);
    (text, if len > 0 { Some((start, len)) } else { None })
}

/// Byte index of the prompt caret inside the line built by
/// [`with_prompt_ghost`].
fn prompt_caret_col(prefix: &str, buf: &str, pos: usize) -> usize {
    let mut p = pos.min(buf.len());
    while p > 0 && !buf.is_char_boundary(p) {
        p -= 1;
    }
    prefix.len() + p
}

#[cfg(test)]
mod tests {
    use super::{prompt_caret_col, with_prompt_ghost};

    #[test]
    fn ghost_range_marks_the_suggestion() {
        let (text, range) = with_prompt_ghost(":", "wr", 2, Some("ite".to_string()));
        assert_eq!(text, ":write");
        assert_eq!(range, Some((3, 3)));
        assert_eq!(&text[3..6], "ite");
        assert_eq!(
            with_prompt_ghost(":", "w", 1, None),
            (":w".to_string(), None)
        );
        assert_eq!(
            with_prompt_ghost(":", "w", 1, Some(String::new())),
            (":w".to_string(), None)
        );
    }

    #[test]
    fn ghost_is_inserted_at_the_caret() {
        // Caret between `w` and `ite`: the suggestion lands at the caret and
        // the text after it shifts right.
        let (text, range) = with_prompt_ghost(":", "write", 1, Some("x".to_string()));
        assert_eq!(text, ":wxrite");
        assert_eq!(range, Some((2, 1)));
        assert_eq!(prompt_caret_col(":", "write", 1), 2);
        assert_eq!(prompt_caret_col(" nfe:", "abc", 3), 8);
        // Mid-character offsets fall back to the char boundary.
        assert_eq!(prompt_caret_col("", "αβ", 1), 0);
    }
}
