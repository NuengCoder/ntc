use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;

use super::super::{Editor, Mode};

impl Editor {
    pub(crate) fn render_run_panel(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let split = self.run_split_row();
        let panel_top = split + 1;
        let panel_h = self.run_panel_height();
        let cols = self.term_w;

        for row in split..panel_top + panel_h {
            queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::CurrentLine))?;
        }

        let sep_title = if let Some(ref cmd) = self.run_executing {
            format!(" EXECUTING: {} ", cmd)
        } else {
            format!(" RUN OUTPUT  ({} lines, j/k scroll, Esc/q close) ", self.run_lines.len())
        };
        let sep_fill = cols.saturating_sub(sep_title.chars().count());
        let sep = format!("{}{}", sep_title, "\u{2500}".repeat(sep_fill));
        queue!(
            stdout,
            MoveTo(0, split as u16),
            SetForegroundColor(theme.editor.run_header_fg.to_crossterm()),
            Print(&sep),
            ResetColor,
        )?;

        let use_color = self.editor_cfg.color_enabled;
        for i in 0..panel_h {
            let idx = self.sv.run + i;
            let row = panel_top + i;
            let line = self.run_lines.get(idx).map(|s| s.as_str()).unwrap_or("~");
            let truncated = if line.len() > cols {
                if line.is_char_boundary(cols) { &line[..cols] } else { &line[..line.floor_char_boundary(cols)] }
            } else { line };
            if use_color {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
                    SetBackgroundColor(theme.editor.run_panel_bg.to_crossterm()),
                    SetForegroundColor(theme.editor.run_output_fg.to_crossterm()),
                    Print(truncated),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            } else {
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
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
        let content_top = 1;
        let rows = if self.mode == Mode::Run {
            self.run_split_row().saturating_sub(1)
        } else {
            self.term_h.saturating_sub(4)
        };
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
}
