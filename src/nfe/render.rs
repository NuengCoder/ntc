use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;

use crate::nfe::state::{NfeState, PanelFocus};

impl NfeState {
    pub fn render(&mut self, stdout: &mut impl Write) -> std::io::Result<()> {
        queue!(stdout, MoveTo(0, 0))?;

        if self.help_visible {
            self.render_help_overlay(stdout)?;
            stdout.flush()?;
            return Ok(());
        }

        self.render_path_bar(stdout)?;
        self.render_file_list(stdout)?;
        self.render_vertical_separator(stdout)?;
        self.render_preview(stdout)?;
        self.render_status_bar(stdout)?;

        stdout.flush()?;
        Ok(())
    }

    fn render_path_bar(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let cols = self.term_w;
        let path_str = self.current_dir.to_string_lossy();

        let bg = crossterm::style::Color::Rgb {
            r: 30,
            g: 30,
            b: 50,
        };
        let fg = crossterm::style::Color::Rgb {
            r: 200,
            g: 200,
            b: 220,
        };

        let icon = "\u{1F4C1} ";
        let display = if path_str.len() + icon.len() > cols.saturating_sub(2) {
            let max = cols.saturating_sub(icon.len() + 4);
            let truncated: String = path_str
                .chars()
                .rev()
                .take(max)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            format!("{}...{}", icon, truncated)
        } else {
            format!("{}{}", icon, path_str)
        };

        queue!(
            stdout,
            MoveTo(0, 0),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(" "),
            Print(&display),
        )?;

        let remaining = cols.saturating_sub(display.chars().count() + 1);
        if remaining > 0 {
            queue!(stdout, Print(" ".repeat(remaining)))?;
        }
        queue!(stdout, ResetColor)?;
        Ok(())
    }

    fn render_file_list(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let list_w = self.list_width();
        let list_h = self.list_height();
        let content_top = 1;
        let num_w = self.line_number_width();

        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.sidebar_bg.to_crossterm();
        let selected_bg = theme.editor.sidebar_selected_bg.to_crossterm();
        let dir_fg = theme.editor.sidebar_dir.to_crossterm();
        let file_fg = theme.editor.sidebar_file.to_crossterm();
        let selected_fg = theme.editor.sidebar_selected.to_crossterm();
        let gutter_fg = theme.editor.gutter_text.to_crossterm();

        for i in 0..list_h {
            let row = content_top + i;
            let idx = self.scroll + i;
            let is_selected = idx == self.selected && self.focus == PanelFocus::List;
            let cell_bg = if is_selected {
                selected_bg.clone()
            } else {
                bg.clone()
            };

            let full_w = num_w + list_w;
            queue!(
                stdout,
                MoveTo(0, row as u16),
                SetBackgroundColor(cell_bg),
                Print(" ".repeat(full_w)),
            )?;

            // Relative line number
            if idx < self.entries.len() {
                let dist = if idx >= self.selected {
                    idx - self.selected
                } else {
                    self.selected - idx
                };
                let num_str = if dist == 0 {
                    " 0 ".to_string()
                } else {
                    format!("{:>2} ", dist)
                };
                let num_fg = if dist == 0 {
                    theme.editor.sidebar_current.to_crossterm()
                } else {
                    gutter_fg.clone()
                };
                queue!(
                    stdout,
                    MoveTo(0, row as u16),
                    SetForegroundColor(num_fg),
                    Print(&num_str),
                )?;
            }

            if idx < self.entries.len() {
                let entry = &self.entries[idx];
                let name_x = num_w;

                let icon = if entry.is_dir { "\u{1F4C1} " } else { "  " };
                let name_width = list_w.saturating_sub(1);
                let display_name: String = if entry.name.chars().count() > name_width {
                    entry
                        .name
                        .chars()
                        .take(name_width.saturating_sub(3))
                        .collect::<String>()
                        + "..."
                } else {
                    entry.name.clone()
                };

                let fg = if is_selected {
                    selected_fg.clone()
                } else if entry.is_dir {
                    dir_fg.clone()
                } else {
                    file_fg.clone()
                };

                queue!(
                    stdout,
                    MoveTo(name_x as u16, row as u16),
                    SetForegroundColor(fg),
                    Print(icon),
                    Print(&display_name),
                )?;
            }
        }

        Ok(())
    }

    fn render_vertical_separator(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let num_w = self.line_number_width();
        let list_w = self.list_width();
        let x = num_w + list_w;
        let content_top = 1;
        let list_h = self.list_height();
        let border_fg = crate::utils::theme::ThemeManager::current()
            .editor
            .border
            .to_crossterm();

        for i in 0..list_h {
            let row = content_top + i;
            queue!(
                stdout,
                MoveTo(x as u16, row as u16),
                SetForegroundColor(border_fg.clone()),
                Print('\u{2502}'),
                ResetColor,
            )?;
        }
        Ok(())
    }

    fn render_preview(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let num_w = self.line_number_width();
        let list_w = self.list_width();
        let preview_w = self.preview_width();
        let list_h = self.list_height();
        let content_top = 1;
        let preview_x = num_w + list_w + 1;

        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.run_panel_bg.to_crossterm();
        let fg = theme.editor.run_output_fg.to_crossterm();

        for i in 0..list_h {
            let row = content_top + i;
            queue!(
                stdout,
                MoveTo(preview_x as u16, row as u16),
                SetBackgroundColor(bg.clone()),
                SetForegroundColor(fg.clone()),
                Clear(ClearType::UntilNewLine),
            )?;
        }

        if let Some(entry) = self.current_entry() {
            let lines = if entry.is_dir && self.preview_content.is_empty() {
                self.render_dir_preview_lines(entry)
            } else if !entry.is_dir && self.preview_content.is_empty() {
                self.render_file_info_lines(entry)
            } else {
                self.preview_content.clone()
            };

            for i in 0..list_h.min(lines.len()) {
                let row = content_top + i;
                let line_idx = self.preview_scroll + i;
                if line_idx < lines.len() {
                    let line = &lines[line_idx];
                    let max_w = preview_w.saturating_sub(1);
                    let truncated: String = if line.chars().count() > max_w {
                        line.chars().take(max_w).collect()
                    } else {
                        line.clone()
                    };
                    queue!(
                        stdout,
                        MoveTo(preview_x as u16, row as u16),
                        SetBackgroundColor(bg.clone()),
                        SetForegroundColor(fg.clone()),
                        Print(truncated),
                        ResetColor,
                    )?;
                }
            }
        } else if self.entries.is_empty() {
            let empty_msg = if self.filter.is_empty() {
                "(empty directory)"
            } else {
                "(no matches)"
            };
            let row = content_top + list_h / 2;
            let x = preview_x + preview_w.saturating_sub(empty_msg.len()) / 2;
            queue!(
                stdout,
                MoveTo(x as u16, row as u16),
                SetForegroundColor(crossterm::style::Color::DarkGrey),
                Print(empty_msg),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_file_info_lines(&self, entry: &crate::nfe::state::FileEntry) -> Vec<String> {
        let preview_w = self.preview_width();
        let mut lines = Vec::new();

        lines.push(format!(
            "\u{2500}{:\u{2500}<pw$}",
            entry.name,
            pw = preview_w.saturating_sub(2)
        ));
        lines.push(String::new());

        let size_str = crate::explorer::human_readable_size(entry.size);
        lines.push(format!(" Size: {}", size_str));
        lines.push(format!(" Type: {}", entry.extension().to_uppercase()));
        if let Ok(duration) = entry.modified.duration_since(std::time::UNIX_EPOCH) {
            let secs = duration.as_secs();
            let days = secs / 86400;
            if let Some(dt) = chrono::DateTime::from_timestamp(secs as i64, 0) {
                lines.push(format!(" Modified: {}", dt.format("%Y-%m-%d %H:%M:%S")));
            } else {
                lines.push(format!(" Modified: {} days ago", days));
            }
        }
        lines.push(String::new());

        if crate::filetype::is_supported_format(&entry.path) {
            lines.push(" (preview) Press Enter to load file contents".to_string());
        } else {
            lines.push(" (binary / unsupported format)".to_string());
        }

        lines
    }

    fn render_dir_preview_lines(&self, entry: &crate::nfe::state::FileEntry) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "\u{2500}{:\u{2500}<pw$}",
            entry.name,
            pw = self.preview_width().saturating_sub(2)
        ));
        lines.push(String::new());

        match std::fs::read_dir(&entry.path) {
            Ok(entries) => {
                let mut count = 0;
                for e in entries.flatten() {
                    if !self.show_hidden && e.file_name().to_string_lossy().starts_with('.') {
                        continue;
                    }
                    let name = e.file_name().to_string_lossy().to_string();
                    let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let icon = if is_dir { "\u{1F4C1}" } else { " " };
                    lines.push(format!(" {} {}", icon, name));
                    count += 1;
                    if count >= 50 {
                        lines.push(" ... (more)".to_string());
                        break;
                    }
                }
                if count == 0 {
                    lines.push(" (empty)".to_string());
                }
            }
            Err(_) => {
                lines.push(" (cannot read)".to_string());
            }
        }

        lines
    }

    pub fn load_preview_content(&mut self) {
        self.preview_scroll = 0;
        let entry = self.current_entry().cloned();
        if let Some(entry) = entry {
            if entry.is_dir {
                self.preview_content = self.render_dir_preview_lines(&entry);
            } else if crate::filetype::is_supported_format(&entry.path) {
                let max_lines = self.list_height() * 2;
                match std::fs::read_to_string(&entry.path) {
                    Ok(content) => {
                        let lines: Vec<String> = content
                            .lines()
                            .take(max_lines)
                            .map(|l| l.to_string())
                            .collect();
                        self.preview_content = if content.lines().count() > max_lines {
                            let mut l = lines;
                            l.push("... (file truncated)".to_string());
                            l
                        } else {
                            lines
                        };
                    }
                    Err(_) => {
                        self.preview_content = vec!["(cannot read file)".to_string()];
                    }
                }
            } else {
                self.preview_content = vec!["(binary file)".to_string()];
            }
            self.preview_file = Some(entry.path);
        }
    }

    fn render_status_bar(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let cols = self.term_w;
        let status_row = self.term_h.saturating_sub(1);

        // If an input mode is active, render the prompt instead
        if self.line_prompt_active {
            return self.render_line_prompt(stdout);
        }
        if self.find_active {
            return self.render_find_prompt(stdout);
        }

        let dir_count = self.dirs.len();
        let file_count = self.files.len();
        let total = dir_count + file_count;
        let sort_label = self.sort_mode.label();
        let sort_arrow = if self.sort_desc {
            "\u{25BC}"
        } else {
            "\u{25B2}"
        };

        let mut left = format!(
            " {} items | {} dirs | {} files | Sort: {}{} ",
            total, dir_count, file_count, sort_label, sort_arrow
        );

        let mut right = String::new();
        if let Some(ref msg) = self.status_msg {
            if msg.len() + 2 < cols / 2 {
                let elapsed = self.status_msg_timer.elapsed().as_secs();
                if elapsed < 3 {
                    right = format!(" {} ", msg);
                }
            }
        }
        if right.is_empty() {
            if self.filter.is_empty() {
                right = " h help | b back | f find | / filter | q quit ".to_string();
            } else {
                right = format!(" filter: {} ", self.filter);
            }
        }

        let left_len = left.chars().count();
        let right_len = right.chars().count();
        if left_len + right_len + 2 > cols {
            let max_left = cols.saturating_sub(right_len + 3);
            if left_len > max_left {
                left = left.chars().take(max_left).collect::<String>();
            }
        }

        let padding = cols.saturating_sub(left.chars().count() + right.chars().count());
        let full_line = format!("{}{}{}", left, " ".repeat(padding), right);

        let bg = crossterm::style::Color::Rgb {
            r: 30,
            g: 30,
            b: 50,
        };
        let fg = crossterm::style::Color::Rgb {
            r: 180,
            g: 180,
            b: 200,
        };

        queue!(
            stdout,
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(&full_line),
            ResetColor,
        )?;

        Ok(())
    }

    fn render_line_prompt(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let cols = self.term_w;
        let status_row = self.term_h.saturating_sub(1);
        let prompt = format!(" Go to line (+N / -N / N): {} ", self.line_prompt);
        let bg = crossterm::style::Color::Rgb {
            r: 40,
            g: 40,
            b: 60,
        };
        let fg = crossterm::style::Color::Rgb {
            r: 220,
            g: 220,
            b: 240,
        };
        queue!(
            stdout,
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{:w$}", prompt, w = cols)),
            ResetColor,
        )?;
        Ok(())
    }

    fn render_find_prompt(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let cols = self.term_w;
        let status_row = self.term_h.saturating_sub(1);
        let count = self.find_results.len();
        let prompt = format!(
            " Find (recursive): {}  [{} matches] ",
            self.find_query, count
        );
        let bg = crossterm::style::Color::Rgb {
            r: 40,
            g: 60,
            b: 40,
        };
        let fg = crossterm::style::Color::Rgb {
            r: 150,
            g: 255,
            b: 150,
        };
        queue!(
            stdout,
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{:w$}", prompt, w = cols)),
            ResetColor,
        )?;
        Ok(())
    }

    fn render_help_overlay(&self, stdout: &mut impl Write) -> std::io::Result<()> {
        let cols = self.term_w;
        let rows = self.term_h;

        let bw = 52usize;

        fn bl(s: &str, bw: usize) -> String {
            let inner = s.chars().count();
            let pad = bw.saturating_sub(inner + 2);
            format!("║{}{:pad$}║", s, "", pad = pad)
        }
        fn bd(bw: usize) -> String {
            format!("╔{}╗", "═".repeat(bw - 2))
        }
        fn be(bw: usize) -> String {
            format!("╚{}╝", "═".repeat(bw - 2))
        }
        fn bs(bw: usize) -> String {
            format!("╠{}╣", "═".repeat(bw - 2))
        }

        let lines = vec![
            bd(bw),
            bl("              NFE Help", bw),
            bs(bw),
            bl("  NAVIGATION", bw),
            bl("  ↑/↓   j/k        Move selection up/down", bw),
            bl("  →     Enter       Enter dir / load preview", bw),
            bl("  ←     b           Go to parent directory", bw),
            bl("  Tab               Toggle panel focus", bw),
            bl("  ~                 Go to home directory", bw),
            bl("  g                 Go to directory prompt", bw),
            bl("  l                 Go to line (relative/abs)", bw),
            bs(bw),
            bl("  SEARCH", bw),
            bl("  f                 Recursive find (dirs+files)", bw),
            bl("  /                 Filter current listing", bw),
            bs(bw),
            bl("  FILE OPS", bw),
            bl("  e                 Open in editor", bw),
            bl("  r                 Rename", bw),
            bl("  d                 Delete (with confirm)", bw),
            bl("  n                 New file", bw),
            bl("  m                 New directory", bw),
            bs(bw),
            bl("  VIEW", bw),
            bl("  i                 Toggle hidden files", bw),
            bl("  s                 Cycle sort mode", bw),
            bl("  S                 Toggle sort direction", bw),
            bl("  F5                Refresh", bw),
            bs(bw),
            bl("  q     Esc         Quit", bw),
            be(bw),
            format!("{:^w$}", " Press any key to close ", w = bw),
        ];

        for y in 0..rows {
            let row_bg = crossterm::style::Color::Rgb {
                r: 20,
                g: 20,
                b: 35,
            };
            let row_fg = crossterm::style::Color::Rgb {
                r: 180,
                g: 180,
                b: 200,
            };
            queue!(
                stdout,
                MoveTo(0, y as u16),
                SetBackgroundColor(row_bg),
                SetForegroundColor(row_fg),
                Print(" ".repeat(cols)),
            )?;
        }

        let start_y = (rows.saturating_sub(lines.len())) / 2;
        for (i, line) in lines.iter().enumerate() {
            let y = start_y + i;
            if y < rows {
                let total_len = line.chars().count();
                let left_pad = cols.saturating_sub(total_len) / 2;
                let padded = format!("{}{}", " ".repeat(left_pad), line);

                let fg = if line.contains("║") && !line.contains("═") && !line.contains("Press")
                {
                    crossterm::style::Color::Rgb {
                        r: 220,
                        g: 220,
                        b: 240,
                    }
                } else if line.contains("═")
                    || line.contains("╔")
                    || line.contains("╚")
                    || line.contains("╠")
                {
                    crossterm::style::Color::Rgb {
                        r: 120,
                        g: 120,
                        b: 180,
                    }
                } else {
                    crossterm::style::Color::Rgb {
                        r: 160,
                        g: 160,
                        b: 180,
                    }
                };
                queue!(
                    stdout,
                    MoveTo(0, y as u16),
                    SetForegroundColor(fg),
                    Print(padded),
                    ResetColor,
                )?;
            }
        }

        Ok(())
    }

    pub fn scroll_preview_up(&mut self) {
        if self.preview_scroll > 0 {
            self.preview_scroll = self.preview_scroll.saturating_sub(1);
        }
    }

    pub fn scroll_preview_down(&mut self) {
        let max_lines = self
            .preview_content
            .len()
            .saturating_sub(self.list_height());
        if self.preview_scroll < max_lines {
            self.preview_scroll += 1;
        }
    }
}
