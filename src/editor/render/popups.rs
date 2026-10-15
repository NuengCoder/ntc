use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use super::super::{byte_to_col, gutter_width, Editor, ALL_ENCODINGS};
use crate::t;

impl Editor {
    pub(crate) fn render_help_screen(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let content_top = 1;
        let rows = self.term_h.saturating_sub(5);
        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);
        let sb_col = cols.saturating_sub(1);

        let sep = "─".repeat(editor_cols.min(60));
        let mut help_lines: Vec<String> = vec![sep.clone()];
        for line in t!("editor.help_screen").lines() {
            help_lines.push(line.to_string());
        }
        help_lines.push(sep);

        let total_lines = help_lines.len();
        let max_scroll = total_lines.saturating_sub(rows);
        let help_scroll = self.sv.help.min(max_scroll);

        // Vertical scrollbar
        if total_lines > rows {
            let avail = rows;
            let thumb_height = (avail * rows / total_lines).max(1);
            let thumb_start = help_scroll * (avail - thumb_height) / max_scroll;

            for i in 0..rows {
                let sy = content_top + i;
                let idx = help_scroll + i;
                let line = help_lines.get(idx).map(|s| s.as_str()).unwrap_or("");
                queue!(
                    stdout,
                    MoveTo(eo as u16, sy as u16),
                    SetForegroundColor(theme.syntax.normal.to_crossterm()),
                    Print(line),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;

                // Draw scrollbar
                let in_thumb = i >= thumb_start && i < thumb_start + thumb_height;
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
                    ResetColor,
                )?;
            }
        } else {
            for i in 0..rows {
                let sy = content_top + i;
                let idx = help_scroll + i;
                let line = help_lines.get(idx).map(|s| s.as_str()).unwrap_or("");
                queue!(
                    stdout,
                    MoveTo(eo as u16, sy as u16),
                    SetForegroundColor(theme.syntax.normal.to_crossterm()),
                    Print(line),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn render_file_finder(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let eo = self.editor_offset();
        let content_top = 1;
        let editor_cols = self.term_w.saturating_sub(eo);
        let rows = self.term_h.saturating_sub(1);
        let max_results = rows.saturating_sub(2);

        let is_gs = self.ff_query.starts_with("gs:");

        let prompt = if is_gs {
            format!("{} {}", t!("editor.ff_prompt_gs_prefix"), self.ff_query)
        } else if self.ff_query.is_empty() {
            t!("editor.ff_prompt_find_empty")
        } else {
            format!("{} {}", t!("editor.ff_prompt_find_prefix"), self.ff_query)
        };
        queue!(
            stdout,
            MoveTo(eo as u16, content_top as u16),
            SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
            SetForegroundColor(theme.editor.status_text.to_crossterm()),
            Print(&prompt),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;

        queue!(
            stdout,
            MoveTo(eo as u16, (content_top + 1) as u16),
            SetBackgroundColor(theme.editor.sidebar_bg.to_crossterm()),
            SetForegroundColor(theme.editor.border.to_crossterm()),
            Print("─".repeat(editor_cols.min(80))),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;

        if is_gs {
            if self.gs_state.loading {
                let loading_msg =
                    t!("editor.gs_loading").replace("{}", &self.gs_state.loading_pattern);
                let ri = content_top + 2;
                if ri < content_top + rows {
                    queue!(
                        stdout,
                        MoveTo(eo as u16, ri as u16),
                        SetForegroundColor(theme.editor.hint_text.to_crossterm()),
                        Print(&loading_msg),
                        ResetColor,
                        Clear(ClearType::UntilNewLine),
                    )?;
                }
            } else if !self.gs_state.results.is_empty() {
                let total = self.gs_state.results.len();
                let max_vis = max_results.min(total);
                let cur = self.gs_state.results.current_index().unwrap_or(0);
                let half = max_vis / 2;
                let start_idx = if cur > half {
                    cur.saturating_sub(half).min(total.saturating_sub(max_vis))
                } else {
                    0
                };

                for offset in 0..max_vis {
                    let idx = (start_idx + offset) % total;
                    let ri = content_top + 2 + offset;
                    if ri >= content_top + rows {
                        break;
                    }
                    let display_str = self.gs_state.results.nth_display(idx).unwrap_or("");
                    let is_selected = idx == cur;
                    let display = if display_str.chars().count() > editor_cols.saturating_sub(4) {
                        display_str
                            .chars()
                            .take(editor_cols.saturating_sub(5))
                            .collect::<String>()
                            + "…"
                    } else {
                        display_str.to_string()
                    };
                    if is_selected {
                        queue!(
                            stdout,
                            MoveTo(eo as u16, ri as u16),
                            SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.status_text.to_crossterm()),
                            Print(format!("  {}", display)),
                            ResetColor,
                            Clear(ClearType::UntilNewLine),
                        )?;
                    } else {
                        queue!(
                            stdout,
                            MoveTo(eo as u16, ri as u16),
                            SetForegroundColor(theme.syntax.normal.to_crossterm()),
                            Print(format!("  {}", display)),
                            ResetColor,
                            Clear(ClearType::UntilNewLine),
                        )?;
                    }
                }
            }
        } else {
            if !self.ff_results.is_empty() {
                let max_vis = max_results.min(self.ff_results.len());
                let half = max_vis / 2;
                let start_idx = if self.ff_idx > half {
                    self.ff_idx
                        .saturating_sub(half)
                        .min(self.ff_results.len().saturating_sub(max_vis))
                } else {
                    0
                };
                // Duplicate file names get a `../rel/dir` suffix so same
                // names from different directories are distinguishable.
                // Unique names render bare as before.
                let mut name_counts: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();
                for (name, _, _, _) in &self.ff_results {
                    *name_counts.entry(name.to_lowercase()).or_insert(0) += 1;
                }
                let root =
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let end_idx = (start_idx + max_vis).min(self.ff_results.len());
                for i in start_idx..end_idx {
                    let ri = content_top + 2 + (i - start_idx);
                    if ri >= content_top + rows {
                        break;
                    }
                    let (ref name, ref path, _, _) = self.ff_results[i];
                    let is_selected = i == self.ff_idx;
                    let duplicated =
                        name_counts.get(&name.to_lowercase()).copied().unwrap_or(0) > 1;
                    let suffix =
                        crate::editor::modes::finder::finder_dup_suffix(path, &root, duplicated);
                    let full = if suffix.is_empty() {
                        name.clone()
                    } else {
                        format!("{}  {}", name, suffix)
                    };
                    let display = if full.chars().count() > editor_cols.saturating_sub(4) {
                        full.chars()
                            .take(editor_cols.saturating_sub(5))
                            .collect::<String>()
                            + "…"
                    } else {
                        full
                    };
                    if is_selected {
                        queue!(
                            stdout,
                            MoveTo(eo as u16, ri as u16),
                            SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                            SetForegroundColor(theme.editor.status_text.to_crossterm()),
                            Print(format!("  {}", display)),
                            ResetColor,
                            Clear(ClearType::UntilNewLine),
                        )?;
                    } else {
                        queue!(
                            stdout,
                            MoveTo(eo as u16, ri as u16),
                            SetForegroundColor(theme.syntax.normal.to_crossterm()),
                            Print(format!("  {}", display)),
                            ResetColor,
                            Clear(ClearType::UntilNewLine),
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn render_gosc(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let content_top = 1;
        let cols = self.term_w;
        let rows = self.term_h.saturating_sub(4);
        let panel_w = 46usize.min(cols.saturating_sub(4));
        let panel_h = (self.gosc_dirs.len() + 5).min(rows.saturating_sub(4));
        let left = (cols.saturating_sub(panel_w)) / 2;
        let top = content_top + (rows.saturating_sub(panel_h)) / 2;
        let inner_w = panel_w.saturating_sub(4);

        for i in 0..panel_h {
            queue!(
                stdout,
                MoveTo(left as u16, (top + i) as u16),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(" ".repeat(panel_w)),
                ResetColor,
            )?;
        }

        let title = t!("editor.gosc_title");
        let title_x = left + (panel_w.saturating_sub(title.len())) / 2;
        queue!(
            stdout,
            MoveTo(title_x as u16, top as u16),
            SetForegroundColor(theme.editor.sidebar_dir.to_crossterm()),
            SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
            Print(title),
            ResetColor,
        )?;

        let sep = "─".repeat(panel_w.saturating_sub(2));
        queue!(
            stdout,
            MoveTo((left + 1) as u16, (top + 1) as u16),
            SetForegroundColor(theme.editor.border.to_crossterm()),
            SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
            Print(&sep),
            ResetColor,
        )?;

        let max_vis = panel_h.saturating_sub(4).min(self.gosc_dirs.len());
        for i in 0..max_vis {
            let ri = top + 2 + i;
            let prefix = format!(" {:>2}. ", i + 1);
            let name = &self.gosc_dirs[i];
            let display = if name.len() > inner_w.saturating_sub(5) {
                let cut = name.floor_char_boundary(inner_w.saturating_sub(6));
                format!("{}…", &name[..cut])
            } else {
                name.clone()
            };
            let line = format!("{}{}", prefix, display);
            queue!(
                stdout,
                MoveTo((left + 2) as u16, ri as u16),
                SetForegroundColor(theme.syntax.normal.to_crossterm()),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(&line),
                ResetColor,
            )?;
        }

        if panel_h >= 5 {
            let input_ri = top + panel_h - 2;
            let hint_prefix = if self.gosc_buf.is_empty() {
                t!("editor.gosc_exit_hint")
            } else {
                "".to_string()
            };
            let input_label = format!("{}{}>", hint_prefix, self.gosc_buf);
            queue!(
                stdout,
                MoveTo((left + 2) as u16, input_ri as u16),
                SetForegroundColor(theme.shell.success.to_crossterm()),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(&input_label),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        if panel_h >= 6 {
            let hint_ri = top + panel_h - 1;
            let hint = t!("editor.gosc_hint");
            queue!(
                stdout,
                MoveTo((left + 2) as u16, hint_ri as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(hint),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        Ok(())
    }

    pub(crate) fn render_encoding_picker(
        &self,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let content_top = 1;
        let cols = self.term_w;
        let rows = self.term_h.saturating_sub(4);
        let panel_w = 40usize.min(cols.saturating_sub(4));
        let panel_h = (ALL_ENCODINGS.len() + 5).min(rows.saturating_sub(4));
        let left = (cols.saturating_sub(panel_w)) / 2;
        let top = content_top + (rows.saturating_sub(panel_h)) / 2;

        for i in 0..panel_h {
            queue!(
                stdout,
                MoveTo(left as u16, (top + i) as u16),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(" ".repeat(panel_w)),
                ResetColor,
            )?;
        }

        let title = t!("editor.enc_title");
        let title_x = left + (panel_w.saturating_sub(title.chars().count())) / 2;
        queue!(
            stdout,
            MoveTo(title_x as u16, top as u16),
            SetForegroundColor(theme.editor.sidebar_dir.to_crossterm()),
            SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
            Print(title),
            ResetColor,
        )?;

        let sep = "─".repeat(panel_w.saturating_sub(2));
        queue!(
            stdout,
            MoveTo((left + 1) as u16, (top + 1) as u16),
            SetForegroundColor(theme.editor.border.to_crossterm()),
            SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
            Print(&sep),
            ResetColor,
        )?;

        let current = self.encoding;
        for (i, enc) in ALL_ENCODINGS.iter().enumerate() {
            let ri = top + 2 + i;
            let is_sel = i == self.encoding_sel;
            let is_current = *enc == current;
            let mark = if is_current { "●" } else { " " };
            let line = format!(
                " {}  {}  {}",
                if is_sel { "▶" } else { " " },
                mark,
                enc.label()
            );
            let fg = if is_sel {
                theme.editor.sidebar_selected.to_crossterm()
            } else if is_current {
                theme.editor.sidebar_current.to_crossterm()
            } else {
                theme.syntax.normal.to_crossterm()
            };
            queue!(
                stdout,
                MoveTo((left + 2) as u16, ri as u16),
                SetForegroundColor(fg),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(&line),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        if panel_h >= 5 {
            let hint_ri = top + panel_h - 2;
            let hint = t!("editor.enc_hint");
            queue!(
                stdout,
                MoveTo((left + 2) as u16, hint_ri as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                SetBackgroundColor(theme.editor.sidebar_selected_bg.to_crossterm()),
                Print(hint),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        Ok(())
    }

    pub(crate) fn render_git_diff(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        if let Some(state) = &self.git_diff {
            let rows = self.term_h.saturating_sub(4);
            let cols = self.term_w;
            let use_color = self.editor_cfg.color_enabled;
            let content_y = 1;
            for i in 0..rows {
                let idx = state.scroll + i;
                queue!(
                    stdout,
                    MoveTo(0, (content_y + i) as u16),
                    Clear(ClearType::UntilNewLine)
                )?;
                if let Some(line) = state.lines.get(idx) {
                    let fg = if use_color && !line.is_empty() {
                        match line.as_bytes()[0] {
                            b'+' => theme.shell.git_added.to_crossterm(),
                            b'-' => theme.shell.git_deleted.to_crossterm(),
                            b'@' => theme.shell.git_modified.to_crossterm(),
                            _ => theme.syntax.normal.to_crossterm(),
                        }
                    } else {
                        Color::White
                    };
                    let print_len = line.chars().count().min(cols);
                    let clipped: String = line.chars().take(print_len).collect();
                    queue!(stdout, SetForegroundColor(fg), Print(&clipped), ResetColor,)?;
                }
            }
            let hint = format!(
                "{} ({} lines) — {}",
                t!("editor.git_quit_hint"),
                state.lines.len(),
                self.path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            );
            queue!(
                stdout,
                MoveTo(0, (self.term_h.saturating_sub(2)) as u16),
                SetForegroundColor(theme.editor.hint_text.to_crossterm()),
                Print(hint),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }
        Ok(())
    }

    pub(crate) fn render_hover_popup(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let text = match &self.hover_text {
            Some(t) => t,
            None => return Ok(()),
        };
        if text.is_empty() {
            return Ok(());
        }

        let rows = self.term_h.saturating_sub(4);
        let content_y = 1;
        let cols = self.term_w;
        let eo = self.editor_offset();
        let gw = gutter_width(self.lines.len());

        let cursor_col = byte_to_col(self.current(), self.cursor.byte);
        let screen_cursor_x = if cursor_col >= self.sv.x {
            eo + gw + (cursor_col - self.sv.x)
        } else {
            eo + gw
        };
        let screen_cursor_y = content_y + self.cursor.y.saturating_sub(self.sv.offset);

        let lines: Vec<&str> = text.lines().collect();
        let visible = lines
            .len()
            .min(20)
            .min(rows.saturating_sub(screen_cursor_y + 2).max(3));
        let max_w = lines
            .iter()
            .take(visible)
            .map(|l| l.len())
            .max()
            .unwrap_or(40);
        let popup_w = (max_w + 4).min(cols.saturating_sub(screen_cursor_x + 1).min(60));
        let popup_x = screen_cursor_x.min(cols.saturating_sub(popup_w + 1));

        let popup_y = if screen_cursor_y.saturating_add(visible) + 2 < rows {
            screen_cursor_y + 1
        } else {
            let needed = visible + 2;
            if screen_cursor_y >= needed {
                screen_cursor_y - needed + 1
            } else {
                1
            }
        };

        let bg = theme.editor.sidebar_bg.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();

        for i in 0..visible + 2 {
            let y = popup_y + i;
            if y >= rows {
                break;
            }
            queue!(
                stdout,
                MoveTo(popup_x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print(" ".repeat(popup_w)),
                ResetColor,
            )?;
        }

        queue!(
            stdout,
            MoveTo(popup_x as u16, popup_y as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(border_fg),
            Print("┌"),
            SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
            Print(format!(" {} ", t!("editor.hover_title"))),
            SetForegroundColor(border_fg),
            Print("─".repeat(popup_w.saturating_sub(7))),
            ResetColor,
        )?;

        for (i, line_text) in lines.iter().enumerate().take(visible) {
            let y = popup_y + 1 + i;
            let display = if line_text.len() > popup_w.saturating_sub(4) {
                let cut = line_text.floor_char_boundary(popup_w.saturating_sub(5));
                format!(" {}…", &line_text[..cut])
            } else {
                format!(" {}", line_text)
            };
            queue!(
                stdout,
                MoveTo(popup_x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(theme.editor.status_text.to_crossterm()),
                Print(&display),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }

        let bot_y = popup_y + 1 + visible;
        if bot_y < rows {
            queue!(
                stdout,
                MoveTo(popup_x as u16, bot_y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print("└"),
                ResetColor,
                Clear(ClearType::UntilNewLine),
            )?;
        }
        Ok(())
    }

    /// VSCode-style completion: fixed Box1 (candidates) + Box2 (detail),
    /// one line below the cursor, flipped above when space runs out.
    pub(crate) fn render_completion(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        use crate::editor::complete::{BOX1_WIDTH, BOX2_WIDTH, COMPLETE_ROWS};

        let st = match &self.completion {
            Some(s) if !s.items.is_empty() => s,
            _ => return Ok(()),
        };
        let theme = crate::utils::theme::ThemeManager::current();
        let rows = self.term_h.saturating_sub(4);
        let cols = self.term_w;
        let eo = self.editor_offset();
        let gw = gutter_width(self.lines.len());

        let cursor_col = byte_to_col(self.current(), self.cursor.byte);
        let sx = eo + gw + cursor_col.saturating_sub(self.sv.x);
        let sy = 1 + self.cursor.y.saturating_sub(self.sv.offset);

        let h = COMPLETE_ROWS + 2;
        let gap = 1;
        let total = BOX1_WIDTH + gap + BOX2_WIDTH;
        // Below cursor + 1; flip above when it would overflow.
        let mut y0 = sy + 1;
        if y0 + h > rows {
            y0 = sy.saturating_sub(h + 1);
        }
        let mut x0 = sx;
        if x0 + total > cols {
            x0 = cols.saturating_sub(total);
        }

        let bg = theme.editor.sidebar_bg.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();
        let text_fg = theme.editor.status_text.to_crossterm();
        let tag_fg = theme.syntax.keyword.to_crossterm();
        let sel_bg = theme.editor.selection_bg.to_crossterm();

        // Page the window so the selection is visible.
        let sel = st.selected.min(st.items.len() - 1);
        let start = (sel / COMPLETE_ROWS) * COMPLETE_ROWS;

        // Border titles: ` prefix (idx/total) `, data only, no new words.
        let title = format!(" {} ({}/{}) ", st.prefix, sel + 1, st.items.len());
        let title_w = BOX1_WIDTH.saturating_sub(2);
        let title = if title.chars().count() > title_w {
            let cut = title.floor_char_boundary(title_w.saturating_sub(1));
            format!("{}…", &title[..cut])
        } else {
            title
        };
        let fill = title_w.saturating_sub(title.chars().count());
        // Top borders.
        queue!(
            stdout,
            MoveTo(x0 as u16, y0 as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(border_fg),
            Print("┌"),
            SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
            Print(&title),
            SetForegroundColor(border_fg),
            Print(format!("{}┐", "─".repeat(fill))),
            ResetColor,
        )?;
        let x1 = x0 + BOX1_WIDTH + gap;
        queue!(
            stdout,
            MoveTo(x1 as u16, y0 as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(border_fg),
            Print(format!("┌{}┐", "─".repeat(BOX2_WIDTH - 2))),
            ResetColor,
        )?;

        // Body rows.
        let inner1 = BOX1_WIDTH - 2;
        let label_w = inner1.saturating_sub(5); // ` ` + label + ` ` + `[F]`
        for r in 0..COMPLETE_ROWS {
            let y = y0 + 1 + r;
            if y >= rows {
                break;
            }
            match st.items.get(start + r) {
                Some(item) => {
                    let is_sel = start + r == sel;
                    let label = if item.label.chars().count() > label_w {
                        let cut = item.label.floor_char_boundary(label_w.saturating_sub(1));
                        format!("{}…", &item.label[..cut])
                    } else {
                        item.label.clone()
                    };
                    let pad = label_w.saturating_sub(label.chars().count());
                    let (row_bg, row_fg, kind_fg) = if is_sel {
                        (sel_bg, text_fg, text_fg)
                    } else {
                        (bg, text_fg, tag_fg)
                    };
                    queue!(
                        stdout,
                        MoveTo(x0 as u16, y as u16),
                        SetBackgroundColor(row_bg),
                        SetForegroundColor(border_fg),
                        Print("│"),
                        SetForegroundColor(row_fg),
                        Print(format!(" {}{} ", label, " ".repeat(pad))),
                        SetForegroundColor(kind_fg),
                        Print(item.kind.tag()),
                        SetForegroundColor(border_fg),
                        Print("│"),
                        ResetColor,
                    )?;
                }
                None => {
                    queue!(
                        stdout,
                        MoveTo(x0 as u16, y as u16),
                        SetBackgroundColor(bg),
                        SetForegroundColor(border_fg),
                        Print(format!("│{}│", " ".repeat(inner1))),
                        ResetColor,
                    )?;
                }
            }
            // Box2 detail for the CURRENT selection on every row slot:
            // functions show the full wrapped signature first, then doc;
            // other kinds show the detail line, then doc lines.
            use crate::editor::complete::{wrap_sig, Kind as CKind};
            let inner2 = BOX2_WIDTH - 2;
            let sel_item = &st.items[sel];
            let sig_lines: Vec<String> =
                if sel_item.kind == CKind::Function && !sel_item.sig_params.is_empty() {
                    wrap_sig(
                        &sel_item.name,
                        &sel_item.return_type,
                        &sel_item.sig_params,
                        None,
                        inner2,
                        5,
                    )
                    .into_iter()
                    .map(|row| row.segments.into_iter().map(|(s, _)| s).collect::<String>())
                    .collect()
                } else {
                    Vec::new()
                };
            let b2 = if !sig_lines.is_empty() {
                sig_lines.get(r).cloned().or_else(|| {
                    sel_item
                        .doc
                        .lines()
                        .nth(r.saturating_sub(sig_lines.len()))
                        .map(|s| s.to_string())
                })
            } else if r == 0 {
                Some(sel_item.detail.clone())
            } else {
                sel_item.doc.lines().nth(r - 1).map(|s| s.to_string())
            };
            let text = match b2 {
                Some(t) if t.chars().count() > inner2 => {
                    let cut = t.floor_char_boundary(inner2.saturating_sub(1));
                    format!("{}…", &t[..cut])
                }
                Some(t) => t,
                None => String::new(),
            };
            let tpad = inner2.saturating_sub(text.chars().count());
            queue!(
                stdout,
                MoveTo(x1 as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print("│"),
                SetForegroundColor(text_fg),
                Print(format!("{}{}", text, " ".repeat(tpad))),
                SetForegroundColor(border_fg),
                Print("│"),
                ResetColor,
            )?;
        }
        // Bottom borders.
        let yb = y0 + h - 1;
        if yb < rows {
            queue!(
                stdout,
                MoveTo(x0 as u16, yb as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print(format!("└{}┘", "─".repeat(BOX1_WIDTH - 2))),
                ResetColor,
            )?;
            queue!(
                stdout,
                MoveTo(x1 as u16, yb as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print(format!("└{}┘", "─".repeat(BOX2_WIDTH - 2))),
                ResetColor,
            )?;
        }
        Ok(())
    }

    /// Paint pre-wrapped signature rows into a box interior.
    /// Returns the number of body rows consumed. The highlighted
    /// parameter uses hardcoded Yellow (per spec) on any theme.
    fn paint_sig_rows(
        &self,
        stdout: &mut std::io::Stdout,
        x0: usize,
        y0: usize,
        inner: usize,
        rows_sig: &[crate::editor::complete::SigRow],
        max_rows: usize,
    ) -> std::io::Result<usize> {
        use crate::editor::complete::SigRow;
        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.sidebar_bg.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();
        let text_fg = theme.editor.status_text.to_crossterm();
        let _ = inner;
        let mut used = 0usize;
        for row in rows_sig.iter().take(max_rows) {
            let y = y0 + 1 + used;
            if y >= self.term_h.saturating_sub(4) {
                break;
            }
            queue!(
                stdout,
                MoveTo(x0 as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print("│"),
                ResetColor,
            )?;
            let mut x = x0 + 1;
            queue!(
                stdout,
                MoveTo(x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(text_fg),
                Print(" "),
                ResetColor,
            )?;
            x += 1;
            for (text, hi) in &row.segments {
                if *hi {
                    queue!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        SetBackgroundColor(Color::Yellow),
                        SetForegroundColor(Color::Black),
                        Print(text),
                        ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        SetBackgroundColor(bg),
                        SetForegroundColor(text_fg),
                        Print(text),
                        ResetColor,
                    )?;
                }
                x += text.chars().count();
            }
            // Pad + right border.
            let end_x = x0 + inner + 1;
            if x < end_x {
                queue!(
                    stdout,
                    MoveTo(x as u16, y as u16),
                    SetBackgroundColor(bg),
                    Print(" ".repeat(end_x - x)),
                    ResetColor,
                )?;
            }
            queue!(
                stdout,
                MoveTo(end_x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print("│"),
                ResetColor,
            )?;
            used += 1;
            let _ = SigRow { segments: vec![] };
        }
        Ok(used)
    }

    /// Signature help: Box2-sized box above the cursor, full wrapped
    /// signature with the active parameter in Yellow.
    pub(crate) fn render_sighelp(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        use crate::editor::complete::{wrap_sig, BOX2_WIDTH};

        /// Fixed box height: borders + up to 4 signature rows + desc.
        const SIG_H: usize = 7;
        let sh = match &self.sighelp {
            Some(s) if !s.label.is_empty() => s,
            _ => return Ok(()),
        };
        let theme = crate::utils::theme::ThemeManager::current();
        let rows = self.term_h.saturating_sub(4);
        let cols = self.term_w;
        let eo = self.editor_offset();
        let gw = gutter_width(self.lines.len());

        let cursor_col = byte_to_col(self.current(), self.cursor.byte);
        let sx = eo + gw + cursor_col.saturating_sub(self.sv.x);
        let sy = 1 + self.cursor.y.saturating_sub(self.sv.offset);

        let y0 = sy.saturating_sub(SIG_H + 1);
        let mut x0 = sx;
        if x0 + BOX2_WIDTH > cols {
            x0 = cols.saturating_sub(BOX2_WIDTH);
        }
        let inner = BOX2_WIDTH - 2;
        let bg = theme.editor.sidebar_bg.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();
        let text_fg = theme.editor.status_text.to_crossterm();

        queue!(
            stdout,
            MoveTo(x0 as u16, y0 as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(border_fg),
            Print(format!("┌{}┐", "─".repeat(inner))),
            ResetColor,
        )?;
        let sig_rows = wrap_sig(&sh.label, &sh.ret, &sh.params, Some(sh.active), inner, 4);
        let used = self.paint_sig_rows(stdout, x0, y0, inner, &sig_rows, 4)?;
        // Description fills the rest (at least the last body row).
        let desc: Vec<&str> = sh.desc.lines().collect();
        for (di, r) in (used..(SIG_H - 2)).enumerate() {
            let y = y0 + 1 + r;
            if y >= rows {
                break;
            }
            let text = desc.get(di).copied().unwrap_or("");
            let text = if text.chars().count() > inner.saturating_sub(1) {
                let cut = text.floor_char_boundary(inner.saturating_sub(2));
                format!("{}…", &text[..cut])
            } else {
                text.to_string()
            };
            let pad = inner.saturating_sub(1 + text.chars().count());
            queue!(
                stdout,
                MoveTo(x0 as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print("│"),
                SetForegroundColor(text_fg),
                Print(format!(" {}{}", text, " ".repeat(pad))),
                SetForegroundColor(border_fg),
                Print("│"),
                ResetColor,
            )?;
        }
        let yb = y0 + SIG_H - 1;
        if yb < rows {
            queue!(
                stdout,
                MoveTo(x0 as u16, yb as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print(format!("└{}┘", "─".repeat(inner))),
                ResetColor,
            )?;
        }
        Ok(())
    }
}
