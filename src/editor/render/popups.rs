use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;

use super::super::{byte_to_col, gutter_width, Editor};

impl Editor {
    pub(crate) fn render_help_screen(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let content_top = 1;
        let rows = self.term_h.saturating_sub(5);
        let cols = self.term_w;
        let eo = self.editor_offset();
        let editor_cols = cols.saturating_sub(eo);
        let sb_col = cols.saturating_sub(1);

        let help_lines = vec![
            "─".repeat(editor_cols.min(60)),
" NORMAL MODE (read-only)".to_string(),
            "  ^O        enter Insert mode".to_string(),
            "  ^W        enter Visual mode".to_string(),
            "  ^G        jump to line number (Jump mode)".to_string(),
            "  :         open command line".to_string(),
            "  ^E        open command line (alt)".to_string(),
            "  :auto on  enable auto-save on exit".to_string(),
            "  :auto off disable auto-save on exit".to_string(),
            "  ^H        open this help screen".to_string(),
            "  ^Q        close current tab / quit (if last tab)".to_string(),
            "  ^Q+Shift  close all tabs".to_string(),
            "  ^PgDn     next tab".to_string(),
            "  ^PgUp     previous tab".to_string(),
            "  ^N        open file in new tab".to_string(),
            "  ^F        find / search in file".to_string(),
            "  ^D        add cursor at next occurrence".to_string(),
            "  ^Alt+D    split primary cursor".to_string(),
            "  mouse     click tab bar, scroll wheel to navigate".to_string(),
            "  arrows    move primary cursor".to_string(),
            "  Sh+arrows word-level movement".to_string(),
            "".to_string(),
" INSERT MODE (editing)".to_string(),
            "  ^C        copy selection".to_string(),
            "  ^F        find / search".to_string(),
            "  ^N        open file in new tab".to_string(),
            "  ^G        jump to line (returns to Insert)".to_string(),
            "  ^Z        undo".to_string(),
            "  ^R        redo".to_string(),
            "  ^Y        yank (copy) current line".to_string(),
            "  ^A        select all".to_string(),
            "  ^L        select current line".to_string(),
            "  ^D        find next occurrence".to_string(),
            "  ^Alt+D    split cursor at selection".to_string(),
            "  ^X        cut selection".to_string(),
            "  ^S        save".to_string(),
            "  ^W        enter Visual mode".to_string(),
            "  ^E        open command line".to_string(),
            "  Esc       return to Normal mode".to_string(),
            "  ^Q        quit (with save prompt)".to_string(),
            "  ^H        help".to_string(),
            "  Home      start of line".to_string(),
            "  End       end of line".to_string(),
            "  ^D auto   enters Insert* (multi-cursor)".to_string(),
            "            Esc returns to Insert".to_string(),
            "".to_string(),
" VISUAL MODE (selection)".to_string(),
            "  arrows    extend selection".to_string(),
            "  ^F        find / search".to_string(),
            "  ^N        open file in new tab".to_string(),
            "  ^G        jump to line (returns to Visual)".to_string(),
            "  ^O        enter Insert mode".to_string(),
            "  ^E        open command line".to_string(),
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
            "  :mkd <dir>         create directory (alias for :mkdir)".to_string(),
            "  :touch <file>      create new file".to_string(),
            "  :mkf <file>        create new file (alias for :touch)".to_string(),
            "  :rmd <target>      recursively delete directory".to_string(),
            "  :rmf <target>      force delete file or directory".to_string(),
            "  :rnmf <src> <dst>  rename file".to_string(),
            "  :rnmd <src> <dst>  rename directory".to_string(),
            "  :ne <file>         navigate & edit (create if missing)".to_string(),
            "  :ne .              show CWD in sidebar".to_string(),
            "  :theme <name>      switch to theme by name".to_string(),
            "  :theme list        list available themes".to_string(),
            "  :theme             show current theme".to_string(),
            "  :tp <name>         jump to teleport savepoint".to_string(),
            "  :tp-add <name>     save CWD as teleport".to_string(),
            "  :gs <pattern>       search file contents across project".to_string(),
            "  :run <command>     execute command in persistent run panel".to_string(),
            "  Esc                 cancel".to_string(),
            "".to_string(),
            " QUERY MODE (Ctrl+Alt+Q in Insert, :x in Command)".to_string(),
            "  /s /l                 clear selection".to_string(),
            "  /s /l /a              select all lines".to_string(),
            "  /s /l N to M          select lines N..M (1-indexed)".to_string(),
            "  /s /l N to M /e str   filter selected lines containing str".to_string(),
            "  /p old to new         replace all 'old' with 'new' in selection".to_string(),
            "  /upper                uppercase selection (selection required)".to_string(),
            "  /lower                lowercase selection (selection required)".to_string(),
            "  /add <text> to #end   append text to each selected line".to_string(),
            "  /add <text> to #start prepend text to each selected line".to_string(),
            "  /set <pat> to #end    right-align first occurrence of pat".to_string(),
            "  /set <pat> to #start  left-align first occurrence of pat".to_string(),
            "  /qal add <n> <q>      add query alias (e.g. /qal sw /p ${1} to ${2})".to_string(),
            "  /qal rm <n>           remove alias".to_string(),
            "  /qal info <n>         show alias".to_string(),
            "  /qal edit <n> <q>     update alias".to_string(),
            "  /qal rnm <a> to <b>   rename alias".to_string(),
            "  /qal list             list aliases".to_string(),
            "  /q <name> [args]      run alias".to_string(),
            "  <name> [args]         run alias directly (no /)".to_string(),
            "  /undo                 undo".to_string(),
            "  /redo                 redo".to_string(),
            "  /exit /quit /Esc      exit Query Mode".to_string(),
            "  ;                     chain multiple commands".to_string(),
            "  ${1} ${2}             alias positional params".to_string(),
            "  ${1:-default}         alias param with default".to_string(),
            "  ${1[]}                alias array param (all remaining args)".to_string(),
            "".to_string(),
            " TERMINAL / RUN PANEL".to_string(),
            "  Ctrl+T          toggle terminal panel".to_string(),
            "  :run <command>  execute command, opens persistent run panel".to_string(),
            "  mouse           click panel to focus, click editor to return".to_string(),
            "  Esc (in panel)  run: close | terminal: return to editor".to_string(),
            "  drag ─── row    resize panel height".to_string(),
            "".to_string(),
            "─".repeat(editor_cols.min(60)),
        ];

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
            format!(" gs: {}", self.ff_query)
        } else {
            format!(
                " Find: {}",
                if self.ff_query.is_empty() {
                    "Type to search..."
                } else {
                    &self.ff_query
                }
            )
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
                let loading_msg = format!("  finding... {} pls wait", self.gs_state.loading_pattern);
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
                        display_str.chars()
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
                let end_idx = (start_idx + max_vis).min(self.ff_results.len());
                for i in start_idx..end_idx {
                    let ri = content_top + 2 + (i - start_idx);
                    if ri >= content_top + rows {
                        break;
                    }
                    let (ref name, _, _, _) = self.ff_results[i];
                    let is_selected = i == self.ff_idx;
                    let display = if name.chars().count() > editor_cols.saturating_sub(4) {
                        name.chars()
                            .take(editor_cols.saturating_sub(5))
                            .collect::<String>()
                            + "…"
                    } else {
                        name.clone()
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

        let title = " gosc — Navigate Continuously ";
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
                format!("{}…", &name[..inner_w.saturating_sub(6)])
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
            let hint_prefix = if self.gosc_buf.is_empty() { "0=exit " } else { "" };
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
            let hint = " Enter=go  -N=back N  Esc=close ";
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

    pub(crate) fn render_completion_popup(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        if !self.completion_visible || self.completion_items.is_empty() {
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

        let max_visible = 12usize.min(rows.saturating_sub(screen_cursor_y + 2).max(3));
        let count = self.completion_items.len();
        let visible = count.min(max_visible);

        let mut max_label = 0;
        let mut max_detail = 0;
        for item in &self.completion_items {
            let l = item.label.len();
            let d = item.detail.len();
            if l > max_label { max_label = l; }
            if d > max_detail { max_detail = d; }
        }
        let popup_w = (max_label + 2 + max_detail + 4).min(cols.saturating_sub(screen_cursor_x + 1));
        let popup_x = screen_cursor_x.min(cols.saturating_sub(popup_w + 1));

        let popup_y = if screen_cursor_y + 2 + visible < rows {
            screen_cursor_y + 1
        } else {
            rows.saturating_sub(visible + 1)
        };

        let bg = theme.editor.sidebar_bg.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();
        for i in 0..visible + 2 {
            let y = popup_y + i;
            if y >= rows { break; }
            queue!(
                stdout,
                MoveTo(popup_x as u16, y as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(border_fg),
                Print(" ".repeat(popup_w)),
                ResetColor,
            )?;
        }

        let title = format!(" {} ", "completions");
        let remaining = popup_w.saturating_sub(1 + title.len());
        queue!(
            stdout,
            MoveTo(popup_x as u16, popup_y as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(border_fg),
            Print("┌"),
            SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
            Print(&title),
            SetForegroundColor(border_fg),
            Print("─".repeat(remaining)),
            ResetColor,
        )?;

        for i in 0..visible {
            let y = popup_y + 1 + i;
            let item = &self.completion_items[i];
            let is_sel = i == self.completion_idx;

            let label_w = max_label.min(popup_w.saturating_sub(max_detail + 5));
            let label = if item.label.len() > label_w {
                format!("{}…", &item.label[..label_w.saturating_sub(1)])
            } else {
                item.label.clone()
            };

            let detail_w = max_detail.min(popup_w.saturating_sub(label_w + 4));
            let detail = if item.detail.len() > detail_w {
                format!("{}…", &item.detail[..detail_w.saturating_sub(1)])
            } else {
                item.detail.clone()
            };

            if is_sel {
                queue!(
                    stdout,
                    MoveTo(popup_x as u16, y as u16),
                    SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                    SetForegroundColor(theme.editor.status_text.to_crossterm()),
                    Print(" "),
                    SetForegroundColor(theme.syntax.keyword.to_crossterm()),
                    Print(&label),
                    SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                    Print("  "),
                    SetForegroundColor(theme.syntax.normal.to_crossterm()),
                    Print(&detail),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            } else {
                queue!(
                    stdout,
                    MoveTo(popup_x as u16, y as u16),
                    SetBackgroundColor(bg),
                    SetForegroundColor(theme.editor.border.to_crossterm()),
                    Print(" "),
                    SetForegroundColor(theme.syntax.normal.to_crossterm()),
                    Print(&label),
                    SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                    Print("  "),
                    SetForegroundColor(theme.syntax.comment.to_crossterm()),
                    Print(&detail),
                    ResetColor,
                    Clear(ClearType::UntilNewLine),
                )?;
            }
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
        let visible = lines.len().min(20).min(rows.saturating_sub(screen_cursor_y + 2).max(3));
        let max_w = lines.iter().take(visible).map(|l| l.len()).max().unwrap_or(40);
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
            if y >= rows { break; }
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
            Print(" info "),
            SetForegroundColor(border_fg),
            Print("─".repeat(popup_w.saturating_sub(7))),
            ResetColor,
        )?;

        for (i, line_text) in lines.iter().enumerate().take(visible) {
            let y = popup_y + 1 + i;
            let display = if line_text.len() > popup_w.saturating_sub(4) {
                format!(" {}…", &line_text[..popup_w.saturating_sub(5)])
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
}
