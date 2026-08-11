use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use super::super::{Editor, Focus};
use crate::t;

impl Editor {
    pub(crate) fn render_run_panel(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let split = self.run_split_row();
        let panel_top = split + 1;
        let panel_h = self.run_panel_height();
        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);

        for row in split..panel_top + panel_h {
            queue!(
                stdout,
                MoveTo(eo as u16, row as u16),
                Clear(ClearType::UntilNewLine)
            )?;
        }

        let sep_title = if let Some(ref cmd) = self.run_executing {
            format!(" {} {} ", t!("editor.panel_executing"), cmd)
        } else if let Some(ref cmd) = self.last_run_cmd {
            format!(
                " {} {}  ({} {}) ",
                t!("editor.panel_run"),
                cmd,
                self.run_lines.len(),
                t!("editor.panel_lines")
            )
        } else {
            format!(
                " {}  ({} {}, {}) ",
                t!("editor.panel_run_output"),
                self.run_lines.len(),
                t!("editor.panel_lines"),
                t!("editor.panel_run_hint")
            )
        };
        let sep_fill = editor_cols.saturating_sub(sep_title.chars().count());
        let sep = format!("{}{}", sep_title, "\u{2500}".repeat(sep_fill));
        let sep_fg = if self.focus == Focus::Run {
            theme.editor.run_header_fg
        } else {
            theme.editor.run_header_unfocused_fg
        };
        queue!(
            stdout,
            MoveTo(eo as u16, split as u16),
            SetForegroundColor(sep_fg.to_crossterm()),
            Print(&sep),
            ResetColor,
        )?;

        let use_color = self.editor_cfg.color_enabled;
        for i in 0..panel_h {
            let idx = self.sv.run + i;
            let row = panel_top + i;
            let line = self.run_lines.get(idx).map(|s| s.as_str()).unwrap_or("~");
            let truncated = if line.len() > editor_cols {
                if line.is_char_boundary(editor_cols) {
                    &line[..editor_cols]
                } else {
                    &line[..line.floor_char_boundary(editor_cols)]
                }
            } else {
                line
            };
            if use_color {
                queue!(
                    stdout,
                    MoveTo(eo as u16, row as u16),
                    SetBackgroundColor(theme.editor.run_panel_bg.to_crossterm()),
                    SetForegroundColor(theme.editor.run_output_fg.to_crossterm()),
                    Print(truncated),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            } else {
                queue!(
                    stdout,
                    MoveTo(eo as u16, row as u16),
                    Print(truncated),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn render_sidebar(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let use_color = self.editor_cfg.color_enabled;
        let content_top = 0;
        let rows = self.term_h;
        let sw = self.sidebar.width;
        let nodes = &self.sidebar.nodes;
        let scroll = self.sidebar.scroll;
        let num_visible = rows.min(nodes.len().saturating_sub(scroll));
        let total_nodes = nodes.len();
        let sb_col = sw.saturating_sub(1);

        let vscroll_thumb_start: Option<usize>;
        let vscroll_thumb_height: usize;
        if total_nodes > rows {
            let avail = rows;
            vscroll_thumb_height = (avail * rows / total_nodes).max(1);
            let max_scroll = total_nodes.saturating_sub(1);
            vscroll_thumb_start = Some(scroll * (avail - vscroll_thumb_height) / max_scroll);
        } else {
            vscroll_thumb_start = None;
            vscroll_thumb_height = 0;
        }

        for i in 0..rows {
            let sy = content_top + i;
            let has_node = i < num_visible;
            let is_selected = has_node && (scroll + i) == self.sidebar.selected;
            let bg = if is_selected {
                theme.editor.sidebar_selected_bg.to_crossterm()
            } else {
                theme.editor.sidebar_bg.to_crossterm()
            };
            queue!(
                stdout,
                MoveTo(0, sy as u16),
                SetBackgroundColor(bg),
                Print(" ".repeat(sw)),
            )?;

            if has_node {
                let idx = scroll + i;
                let node = &nodes[idx];
                let is_open = node.path == self.path;

                let mut x = 1 + node.depth * 2;
                if x < sw {
                    if node.is_dir {
                        let ind = if node.expanded { "▼" } else { "▶" };
                        let ind_fg = if use_color {
                            theme.editor.sidebar_dir.to_crossterm()
                        } else {
                            theme.editor.sidebar_file.to_crossterm()
                        };
                        queue!(
                            stdout,
                            MoveTo(x as u16, sy as u16),
                            SetForegroundColor(ind_fg),
                            Print(ind),
                        )?;
                        x += 2;
                    } else {
                        let dot_fg = if use_color {
                            theme.editor.gutter_text.to_crossterm()
                        } else {
                            theme.editor.sidebar_file.to_crossterm()
                        };
                        queue!(
                            stdout,
                            MoveTo(x as u16, sy as u16),
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
                                .take(max_name.saturating_sub(3))
                                .collect::<String>()
                                + "..."
                        } else {
                            node.name.clone()
                        };
                        let fg = if !use_color {
                            theme.editor.sidebar_selected.to_crossterm()
                        } else if is_open {
                            theme.editor.sidebar_current.to_crossterm()
                        } else if is_selected {
                            theme.editor.sidebar_selected.to_crossterm()
                        } else if node.is_dir {
                            theme.editor.sidebar_dir.to_crossterm()
                        } else {
                            theme.editor.sidebar_file.to_crossterm()
                        };
                        queue!(
                            stdout,
                            SetForegroundColor(fg),
                            MoveTo(x as u16, sy as u16),
                            Print(&display),
                        )?;
                    }
                }
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
                )?;
            }

            queue!(
                stdout,
                ResetColor,
                MoveTo(sw as u16, sy as u16),
                SetForegroundColor(theme.editor.border.to_crossterm()),
                Print('│'),
                ResetColor,
            )?;
        }

        Ok(())
    }

    pub(crate) fn render_terminal_panel(
        &self,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let split = self.run_split_row();
        let panel_top = split + 1;
        let panel_h = self.run_panel_height();
        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);
        let sb_col = cols.saturating_sub(1);

        // Separator header
        let sep_title = if let Some(ref term) = self.terminal {
            format!(" {} ", term.title)
        } else {
            format!(" {} ", t!("editor.panel_terminal"))
        };
        let sep_fill = editor_cols.saturating_sub(sep_title.chars().count());
        let sep = format!("{}{}", sep_title, "\u{2500}".repeat(sep_fill));
        let sep_fg = if self.focus == Focus::Terminal {
            theme.editor.run_header_fg
        } else {
            theme.editor.run_header_unfocused_fg
        };
        queue!(
            stdout,
            MoveTo(eo as u16, split as u16),
            SetForegroundColor(sep_fg.to_crossterm()),
            Print(&sep),
            ResetColor,
        )?;

        if let Some(ref term) = self.terminal {
            let default_bg = (30, 30, 30);
            let default_fg = (204, 204, 204);
            let total = term.total_lines();
            let scroll = self.sv.run.min(total.saturating_sub(1));
            let scroll_x = self.sv.terminal_scroll_x;
            let sb_len = term.scrollback.len();

            // Cursor screen position (in absolute line indexes)
            let cursor_abs_y = sb_len + term.cursor_y;

            // Scrollbar
            let vscroll_thumb_height;
            let vscroll_thumb_start;
            if total > panel_h {
                vscroll_thumb_height = (panel_h * panel_h / total).max(1);
                let max_scroll = total - panel_h;
                vscroll_thumb_start = max_scroll.checked_div(1).map_or(0, |_| {
                    scroll * (panel_h - vscroll_thumb_height) / max_scroll
                });
            } else {
                vscroll_thumb_height = 0;
                vscroll_thumb_start = 0;
            }

            let selection_range = term.selection_range();

            for i in 0..panel_h {
                let row = panel_top + i;
                let line_idx = scroll + i;

                // Set background for the entire row
                queue!(
                    stdout,
                    MoveTo(eo as u16, row as u16),
                    SetBackgroundColor(crossterm::style::Color::Rgb {
                        r: default_bg.0,
                        g: default_bg.1,
                        b: default_bg.2,
                    }),
                    SetForegroundColor(crossterm::style::Color::Rgb {
                        r: default_fg.0,
                        g: default_fg.1,
                        b: default_fg.2,
                    }),
                    Clear(ClearType::UntilNewLine),
                )?;

                // Check if this line has any selection cells
                let sel_on_line = selection_range
                    .map(|((sy, _), (ey, _))| line_idx >= sy && line_idx <= ey)
                    .unwrap_or(false);

                if line_idx < total {
                    let line = term.get_line(line_idx);
                    let mut last_fg = default_fg;
                    let mut last_bg = default_bg;
                    let mut need_move = true;
                    let is_cursor_line = term.cursor_visible && line_idx == cursor_abs_y;

                    let sel_start = selection_range.map(|r| r.0);
                    let sel_end = selection_range.map(|r| r.1);

                    let max_cells = editor_cols.saturating_sub(1);
                    for (x, (ci, cell)) in
                        (eo..).zip(line.iter().enumerate().skip(scroll_x).take(max_cells))
                    {
                        let in_sel = sel_on_line
                            && sel_start
                                .zip(sel_end)
                                .map(|((sy, sx), (ey, ex))| {
                                    if line_idx == sy && line_idx == ey {
                                        ci >= sx && ci < ex
                                    } else if line_idx == sy {
                                        ci >= sx
                                    } else if line_idx == ey {
                                        ci < ex
                                    } else {
                                        true
                                    }
                                })
                                .unwrap_or(false);

                        let is_cursor = is_cursor_line && ci == term.cursor_x;
                        let (fg, bg) = if is_cursor {
                            (default_bg, default_fg)
                        } else if in_sel {
                            (cell.bg, cell.fg)
                        } else {
                            (cell.fg, cell.bg)
                        };

                        let fg_changed = fg != last_fg;
                        let bg_changed = bg != last_bg;

                        if need_move || fg_changed || bg_changed {
                            queue!(stdout, MoveTo(x as u16, row as u16))?;
                            if fg_changed {
                                queue!(
                                    stdout,
                                    SetForegroundColor(crossterm::style::Color::Rgb {
                                        r: fg.0,
                                        g: fg.1,
                                        b: fg.2,
                                    })
                                )?;
                                last_fg = fg;
                            }
                            if bg_changed {
                                queue!(
                                    stdout,
                                    SetBackgroundColor(crossterm::style::Color::Rgb {
                                        r: bg.0,
                                        g: bg.1,
                                        b: bg.2,
                                    })
                                )?;
                                last_bg = bg;
                            }
                            need_move = false;
                        }
                        queue!(stdout, Print(if is_cursor { ' ' } else { cell.ch }))?;
                    }

                    // Left overflow indicator
                    if scroll_x > 0 && line_idx < total {
                        queue!(
                            stdout,
                            MoveTo(eo as u16, row as u16),
                            SetForegroundColor(crossterm::style::Color::Rgb {
                                r: default_fg.0,
                                g: default_fg.1,
                                b: default_fg.2,
                            }),
                            SetBackgroundColor(crossterm::style::Color::Rgb {
                                r: default_bg.0,
                                g: default_bg.1,
                                b: default_bg.2,
                            }),
                            Print('\u{2039}'),
                        )?;
                    }

                    // Right overflow indicator
                    if line_idx < total && line.len() > scroll_x + max_cells {
                        queue!(
                            stdout,
                            MoveTo((eo + max_cells - 1) as u16, row as u16),
                            SetForegroundColor(crossterm::style::Color::Rgb {
                                r: default_fg.0,
                                g: default_fg.1,
                                b: default_fg.2,
                            }),
                            SetBackgroundColor(crossterm::style::Color::Rgb {
                                r: default_bg.0,
                                g: default_bg.1,
                                b: default_bg.2,
                            }),
                            Print('\u{203A}'),
                        )?;
                    }
                }

                // Render scrollbar on right edge
                if vscroll_thumb_height > 0 {
                    let in_thumb =
                        i >= vscroll_thumb_start && i < vscroll_thumb_start + vscroll_thumb_height;
                    let ch = if in_thumb { '\u{2588}' } else { '\u{2591}' };
                    queue!(
                        stdout,
                        MoveTo(sb_col as u16, row as u16),
                        SetBackgroundColor(crossterm::style::Color::Rgb {
                            r: default_bg.0,
                            g: default_bg.1,
                            b: default_bg.2,
                        }),
                        SetForegroundColor(if in_thumb {
                            theme.editor.scrollbar_thumb.to_crossterm()
                        } else {
                            theme.editor.scrollbar.to_crossterm()
                        }),
                        Print(ch),
                        ResetColor,
                    )?;
                }
            }
            queue!(stdout, ResetColor)?;
        }

        Ok(())
    }
}
