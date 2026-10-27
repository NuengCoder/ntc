use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{
    Attribute, Color as CxColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
    SetForegroundColor,
};
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
        let git_map = if crate::config::Config::global_get_git_integration_enabled() {
            crate::utils::git::status_map(&self.sidebar.root)
        } else {
            std::collections::HashMap::new()
        };
        let content_top = 0;
        let rows = self.term_h;
        let sw = self.sidebar_cols();
        let nodes = &self.sidebar.nodes;
        let scroll = self.sidebar.scroll;
        let num_visible = rows.min(nodes.len().saturating_sub(scroll));
        let total_nodes = nodes.len();
        let sb_col = sw.saturating_sub(1);
        // Node numbers are 1-based positions in the flattened tree, so
        // expanding/collapsing a directory renumbers everything below it.
        let num_w = total_nodes.to_string().len().max(2);
        let num_fg = if use_color {
            theme.editor.gutter_text.to_crossterm()
        } else {
            theme.editor.sidebar_selected.to_crossterm()
        };

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

                if sw > num_w {
                    let label = format!("{:>width$} ", idx + 1, width = num_w);
                    queue!(
                        stdout,
                        MoveTo(1, sy as u16),
                        SetForegroundColor(if is_selected && use_color {
                            theme.editor.sidebar_selected.to_crossterm()
                        } else {
                            num_fg
                        }),
                        Print(label),
                    )?;
                }

                // 1 margin + number field + its trailing space.
                let mut x = 2 + num_w + node.depth * 2;
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

                        // Git status symbol after the name
                        if !node.is_dir {
                            if let Some(&code) = git_map.get(&node.path) {
                                let gfg = match code {
                                    'A' | 'C' => theme.shell.git_added.to_crossterm(),
                                    'D' => theme.shell.git_deleted.to_crossterm(),
                                    '?' => theme.shell.git_untracked.to_crossterm(),
                                    'U' => theme.shell.git_conflict.to_crossterm(),
                                    _ => theme.shell.git_modified.to_crossterm(),
                                };
                                let sx = x + display.chars().count() + 1;
                                if sx < sw {
                                    queue!(
                                        stdout,
                                        SetForegroundColor(gfg),
                                        MoveTo(sx as u16, sy as u16),
                                        Print(code),
                                    )?;
                                }
                            }
                        }
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
        let (btop, bleft, bright, bbottom) = self.terminal_box();
        let panel_top = btop + 1;
        let (content_cols, content_h) = self.terminal_content_size();
        let cols = self.term_w;
        let sb_col = cols.saturating_sub(1);

        // Rect-box frame: teal when focused, dim when not, white when
        // color is off. Title keeps the legacy header fg behavior.
        let use_box_color = self.editor_cfg.color_enabled;
        let border_fg = if !use_box_color {
            CxColor::White
        } else if self.focus == Focus::Terminal {
            theme.shell.separator.to_crossterm()
        } else {
            theme.editor.run_header_unfocused_fg.to_crossterm()
        };
        let title_fg = if !use_box_color {
            CxColor::White
        } else if self.focus == Focus::Terminal {
            theme.editor.run_header_fg.to_crossterm()
        } else {
            theme.editor.run_header_unfocused_fg.to_crossterm()
        };
        let box_title = if let Some(ref term) = self.terminal {
            format!(" {} ", term.title)
        } else {
            format!(" {} ", t!("editor.panel_terminal"))
        };
        let inner_w = bright.saturating_sub(bleft + 1);
        let mut title_chars: String = box_title.chars().take(inner_w).collect();
        while title_chars.chars().count() < inner_w {
            title_chars.push('\u{2500}');
        }
        queue!(
            stdout,
            MoveTo(bleft as u16, btop as u16),
            SetForegroundColor(border_fg),
            Print('\u{250C}'),
            SetForegroundColor(title_fg),
            Print(&title_chars),
            SetForegroundColor(border_fg),
            Print('\u{2510}'),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;
        // Bottom border (drawn now so content rows can't overrun it).
        queue!(
            stdout,
            MoveTo(bleft as u16, bbottom as u16),
            SetForegroundColor(border_fg),
            Print('\u{2514}'),
            Print("\u{2500}".repeat(inner_w)),
            Print('\u{2518}'),
            ResetColor,
            Clear(ClearType::UntilNewLine),
        )?;

        if let Some(ref term) = self.terminal {
            let total = term.total_lines();
            let scroll = self.sv.run.min(total.saturating_sub(1));
            let scroll_x = self.sv.terminal_scroll_x;

            // Cursor screen position (alt-aware: grid-relative on the
            // alternate screen, scrollback-relative otherwise).
            let cursor_abs_y = term.cursor_abs_y();

            // Scrollbar (outside the box, shares the content rows)
            let vscroll_thumb_height;
            let vscroll_thumb_start;
            if total > content_h {
                vscroll_thumb_height = (content_h * content_h / total).max(1);
                let max_scroll = total - content_h;
                vscroll_thumb_start = max_scroll.checked_div(1).map_or(0, |_| {
                    scroll * (content_h - vscroll_thumb_height) / max_scroll
                });
            } else {
                vscroll_thumb_height = 0;
                vscroll_thumb_start = 0;
            }

            let selection_range = term.selection_range();

            for i in 0..content_h {
                let row = panel_top + i;
                let line_idx = scroll + i;

                // Transparent passthrough: clear the row WITHOUT painting any
                // background, so the embedded ntc shell inherits the user's
                // own terminal colors exactly like the standalone shell.
                // Side walls of the rect box frame the content.
                queue!(
                    stdout,
                    MoveTo((bleft + 1) as u16, row as u16),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
                queue!(
                    stdout,
                    MoveTo(bleft as u16, row as u16),
                    SetForegroundColor(border_fg),
                    Print('\u{2502}'),
                    MoveTo(bright as u16, row as u16),
                    Print('\u{2502}'),
                    ResetColor,
                )?;

                // Check if this line has any selection cells
                let sel_on_line = selection_range
                    .map(|((sy, _), (ey, _))| line_idx >= sy && line_idx <= ey)
                    .unwrap_or(false);

                if line_idx < total {
                    let line = term.get_line(line_idx);
                    let mut last_fg: Option<(u8, u8, u8)> = None;
                    let mut last_bg: Option<(u8, u8, u8)> = None;
                    let mut last_rev = false;
                    let mut last_bold = false;
                    let mut need_move = true;
                    let is_cursor_line =
                        term.cursor_visible && !term.app_cursor_hidden && line_idx == cursor_abs_y;

                    let sel_start = selection_range.map(|r| r.0);
                    let sel_end = selection_range.map(|r| r.1);

                    let max_cells = content_cols;
                    let content_x0 = bleft + 1;
                    for (x, (ci, cell)) in
                        (content_x0..).zip(line.iter().enumerate().skip(scroll_x).take(max_cells))
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
                        // Cursor and passthrough-selection use reverse video
                        // so they stay visible on any background. Explicitly
                        // colored selections keep the classic fg/bg swap.
                        let (fg, bg, rev) = if is_cursor {
                            (cell.fg, cell.bg, true)
                        } else if in_sel {
                            match (cell.fg, cell.bg) {
                                (None, None) => (None, None, true),
                                (fg, bg) => (bg, fg, false),
                            }
                        } else {
                            (cell.fg, cell.bg, false)
                        };

                        let bold = cell.bold && !is_cursor;
                        if need_move
                            || fg != last_fg
                            || bg != last_bg
                            || rev != last_rev
                            || bold != last_bold
                        {
                            queue!(stdout, MoveTo(x as u16, row as u16))?;
                            if rev != last_rev {
                                if rev {
                                    queue!(stdout, SetAttribute(Attribute::Reverse))?;
                                } else {
                                    queue!(stdout, SetAttribute(Attribute::NoReverse))?;
                                }
                                last_rev = rev;
                            }
                            if bold != last_bold {
                                if bold {
                                    queue!(stdout, SetAttribute(Attribute::Bold))?;
                                } else {
                                    queue!(stdout, SetAttribute(Attribute::NoBold))?;
                                }
                                last_bold = bold;
                            }
                            if fg != last_fg {
                                match fg {
                                    Some((r, g, b)) => queue!(
                                        stdout,
                                        SetForegroundColor(CxColor::Rgb { r, g, b })
                                    )?,
                                    None => queue!(stdout, SetForegroundColor(CxColor::Reset))?,
                                }
                                last_fg = fg;
                            }
                            if bg != last_bg {
                                match bg {
                                    Some((r, g, b)) => queue!(
                                        stdout,
                                        SetBackgroundColor(CxColor::Rgb { r, g, b })
                                    )?,
                                    None => queue!(stdout, SetBackgroundColor(CxColor::Reset))?,
                                }
                                last_bg = bg;
                            }
                            need_move = false;
                        }
                        let ch = if cell.ch == '\0' { ' ' } else { cell.ch };
                        queue!(stdout, Print(if is_cursor { ' ' } else { ch }))?;
                    }
                    if last_rev {
                        queue!(stdout, SetAttribute(Attribute::NoReverse))?;
                    }
                    if last_bold {
                        queue!(stdout, SetAttribute(Attribute::NoBold))?;
                    }

                    // Left overflow indicator (passthrough colors)
                    if scroll_x > 0 && line_idx < total {
                        queue!(
                            stdout,
                            MoveTo(content_x0 as u16, row as u16),
                            ResetColor,
                            Print('\u{2039}'),
                        )?;
                    }

                    // Right overflow indicator (passthrough colors)
                    if line_idx < total && line.len() > scroll_x + max_cells {
                        queue!(
                            stdout,
                            MoveTo((content_x0 + max_cells - 1) as u16, row as u16),
                            ResetColor,
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
                        ResetColor,
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
