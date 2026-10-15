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

        if self.viewer_active {
            self.render_viewer_overlay(stdout)?;
            stdout.flush()?;
            return Ok(());
        }

        // A closed viewer can leave a Kitty placement behind (Kitty
        // images persist as terminal objects — text repaint cannot
        // erase them). Consume the stale cache exactly once.
        if self.viewer_kitty.take().is_some() {
            queue!(stdout, Print(crate::nfe::kitty::delete_cmd()))?;
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

                // Git status symbol after the name
                if !entry.is_dir {
                    if let Some(&code) = self.git_status.get(&entry.name) {
                        let theme = crate::utils::theme::ThemeManager::current();
                        let color = match code {
                            'A' | 'C' => theme.shell.git_added.to_crossterm(),
                            'D' => theme.shell.git_deleted.to_crossterm(),
                            '?' => theme.shell.git_untracked.to_crossterm(),
                            'U' => theme.shell.git_conflict.to_crossterm(),
                            _ => theme.shell.git_modified.to_crossterm(),
                        };
                        let x = name_x + 2 + display_name.chars().count();
                        queue!(
                            stdout,
                            MoveTo(x as u16, row as u16),
                            SetForegroundColor(color),
                            Print(code),
                            ResetColor,
                        )?;
                    }
                }
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

    fn render_preview(&mut self, stdout: &mut impl Write) -> std::io::Result<()> {
        let num_w = self.line_number_width();
        let list_w = self.list_width();
        let preview_w = self.preview_width();
        let list_h = self.list_height();
        let content_top = 1;
        let preview_x = num_w + list_w + 1;

        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.run_panel_bg.to_crossterm();
        let fg = theme.editor.run_output_fg.to_crossterm();

        // Pixel preview path (Enter on image with Kitty/Sixel backend).
        // Shown-key match: leave every pixel on screen (text repaints
        // could never restore them). Anything else erases the old rect
        // first (bg repaint + Kitty delete), then blits.
        #[cfg(feature = "image2doc")]
        {
            use crate::nfe::sixel::PixelBackend;
            let backend = self.viewer_backend();
            if backend != PixelBackend::Sextant {
                if let Some(path) = self
                    .current_entry()
                    .filter(|e| !e.is_dir && crate::nfe::image::is_image_file(&e.path))
                    .map(|e| e.path.clone())
                {
                    let (cw, ch) = crate::nfe::sixel::cell_pixel_size();
                    let key = (path.clone(), preview_w, list_h, cw, ch);
                    let shown = self.panel_shown.clone();
                    if shown.as_ref() != Some(&key) {
                        if shown.is_some() {
                            self.erase_panel_rect(
                                stdout,
                                preview_x,
                                content_top,
                                preview_w,
                                list_h,
                                bg,
                            )?;
                            if backend == PixelBackend::Kitty {
                                queue!(stdout, Print(crate::nfe::kitty::delete_cmd()))?;
                            }
                        }
                        let bg_rgb = match theme.editor.run_panel_bg {
                            crate::utils::theme::ThemeColor::Rgb { r, g, b } => (r, g, b),
                            _ => (0, 0, 0),
                        };
                        self.build_panel_seq(&path, preview_w, list_h, cw, ch, bg_rgb);
                        if let Some(blit) = self
                            .panel_seq
                            .as_ref()
                            .and_then(|(k, b)| (*k == key).then(|| b.clone()))
                        {
                            match blit {
                                crate::nfe::state::PanelBlit::Sixel(seq) => {
                                    queue!(stdout, MoveTo(preview_x as u16, content_top as u16))?;
                                    stdout.flush()?;
                                    stdout.write_all(seq.as_bytes())?;
                                    stdout.flush()?;
                                }
                                crate::nfe::state::PanelBlit::Kitty(chunks) => {
                                    queue!(
                                        stdout,
                                        Print(crate::nfe::kitty::delete_cmd()),
                                        MoveTo(preview_x as u16, content_top as u16),
                                    )?;
                                    stdout.flush()?;
                                    for chunk in &chunks {
                                        stdout.write_all(chunk.as_bytes())?;
                                    }
                                    stdout.flush()?;
                                }
                            }
                            self.panel_shown = Some(key);
                            return Ok(());
                        }
                        // Build failed: fall through to text painting with
                        // an explanatory line.
                        self.preview_content = vec!["(cannot preview image)".to_string()];
                        self.panel_shown = None;
                    } else {
                        return Ok(());
                    }
                }
            }
        }
        // Leaving pixel mode: erase the stale rect once (bg repaint +
        // Kitty delete — text alone cannot clear pixel graphics).
        if self.panel_shown.take().is_some() {
            self.erase_panel_rect(stdout, preview_x, content_top, preview_w, list_h, bg)?;
            #[cfg(feature = "image2doc")]
            {
                use crate::nfe::sixel::PixelBackend;
                if self.viewer_backend() == PixelBackend::Kitty {
                    queue!(stdout, Print(crate::nfe::kitty::delete_cmd()))?;
                }
            }
            self.panel_seq = None;
        }

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
            if self.preview_image_active() {
                if let Some(img) = self.preview_image.clone() {
                    self.render_image_preview(stdout, entry, &img)?;
                    return Ok(());
                }
            }
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

    /// Repaint a panel rect with the background (erases Sixel cells;
    /// pair with the Kitty delete for Kitty placements).
    fn erase_panel_rect(
        &self,
        stdout: &mut impl Write,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        bg: crossterm::style::Color,
    ) -> std::io::Result<()> {
        for r in y..y.saturating_add(h) {
            queue!(
                stdout,
                MoveTo(x as u16, r as u16),
                SetBackgroundColor(bg),
                Print(" ".repeat(w)),
                ResetColor,
            )?;
        }
        Ok(())
    }

    /// True when a half-block image preview is loaded for the entry
    /// under the cursor (scroll bounds and painting use the image grid).
    fn preview_image_active(&self) -> bool {
        match (&self.preview_image, self.current_entry()) {
            (Some(_), Some(entry)) => self.preview_file.as_ref() == Some(&entry.path),
            _ => false,
        }
    }

    /// Paint the image header (`─name (WxH)`) plus half-block color rows.
    /// Row 0 is pinned; `preview_scroll` offsets into the image grid.
    fn render_image_preview(
        &self,
        stdout: &mut impl Write,
        entry: &crate::nfe::state::FileEntry,
        img: &crate::nfe::image::ImagePreview,
    ) -> std::io::Result<()> {
        use crossterm::style::Color;
        let num_w = self.line_number_width();
        let list_w = self.list_width();
        let preview_w = self.preview_width();
        let list_h = self.list_height();
        let content_top = 1;
        let preview_x = num_w + list_w + 1;

        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.run_panel_bg.to_crossterm();
        let fg = theme.editor.run_output_fg.to_crossterm();

        let header = format!("─{} ({}x{})", entry.name, img.orig_w, img.orig_h);
        let max_w = preview_w.saturating_sub(1);
        let header: String = if header.chars().count() > max_w {
            header.chars().take(max_w).collect()
        } else {
            header
        };
        queue!(
            stdout,
            MoveTo(preview_x as u16, content_top as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(header),
            ResetColor,
        )?;

        let max_cells = max_w.min(img.cells_wide());
        for i in 1..list_h {
            let img_row = self.preview_scroll + (i - 1);
            if img_row >= img.rows.len() {
                break;
            }
            let row = content_top + i;
            for (dx, cell) in img.rows[img_row].iter().take(max_cells).enumerate() {
                let x = preview_x + dx;
                let cell_fg = Color::Rgb {
                    r: cell.fg.0,
                    g: cell.fg.1,
                    b: cell.fg.2,
                };
                let cell_bg = Color::Rgb {
                    r: cell.bg.0,
                    g: cell.bg.1,
                    b: cell.bg.2,
                };
                queue!(
                    stdout,
                    MoveTo(x as u16, row as u16),
                    SetBackgroundColor(cell_bg),
                    SetForegroundColor(cell_fg),
                    Print(cell.ch),
                    ResetColor,
                )?;
            }
        }
        Ok(())
    }

    /// Fullscreen image viewer: title row, centered image, hint row.
    /// Sixel-capable terminals get true pixels (blitted once per state
    /// change and left on screen); otherwise the sextant/half-block
    /// grid paints per frame. Cells outside the sample (small image on
    /// a big screen) are skipped so the cleared background shows
    /// instead of edge-smeared pixels.
    /// Centered bordered box ("square div") around a shown image rect.
    /// Clipped to the content area (below the title, above the hint);
    /// sides without room are skipped gracefully.
    fn paint_box(
        &self,
        stdout: &mut impl Write,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        cols: usize,
        rows: usize,
        border: crossterm::style::Color,
        bg: crossterm::style::Color,
    ) -> std::io::Result<()> {
        if w == 0 || h == 0 {
            return Ok(());
        }
        // (x,y) is the image origin; the frame sits one cell outside.
        let x1 = x.saturating_add(w).saturating_add(1);
        let y1 = y.saturating_add(h).saturating_add(1);
        let bx = x.saturating_sub(1);
        let by = y.saturating_sub(1);
        // Collect frame cells first (clipped), then paint in one pass
        // so border/interior ordering never matters.
        let mut cells: Vec<(usize, usize, char)> = Vec::new();
        cells.push((bx, by, '┌'));
        cells.push((x1, by, '┐'));
        cells.push((bx, y1, '└'));
        cells.push((x1, y1, '┘'));
        for cx in x..x.saturating_add(w) {
            cells.push((cx, by, '─'));
            cells.push((cx, y1, '─'));
        }
        for cy in y..y.saturating_add(h) {
            cells.push((bx, cy, '│'));
            cells.push((x1, cy, '│'));
        }
        for (cx, cy, ch) in cells {
            if cx < cols && cy > 0 && cy + 1 < rows {
                queue!(
                    stdout,
                    MoveTo(cx as u16, cy as u16),
                    SetBackgroundColor(bg),
                    SetForegroundColor(border),
                    Print(ch),
                    ResetColor,
                )?;
            }
        }
        Ok(())
    }

    fn render_viewer_overlay(&mut self, stdout: &mut impl Write) -> std::io::Result<()> {
        use crossterm::style::Color;
        let cols = self.term_w;
        let rows = self.term_h;
        let theme = crate::utils::theme::ThemeManager::current();
        let bg = theme.editor.sidebar_bg.to_crossterm();
        let fg = theme.editor.status_text.to_crossterm();
        let border_fg = theme.editor.border.to_crossterm();
        let bg_rgb = match theme.editor.sidebar_bg {
            crate::utils::theme::ThemeColor::Rgb { r, g, b } => (r, g, b),
            _ => (0, 0, 0),
        };

        // Decide pixel blits BEFORE touching the screen: a cache hit
        // must skip BOTH the blit and the clear, or the retained image
        // is erased and never redrawn.
        #[cfg(feature = "image2doc")]
        let sixel_blit: Option<(String, crate::nfe::image::BlitGeom)> = {
            use crate::nfe::sixel::PixelBackend;
            if self.viewer_backend() == PixelBackend::Sixel && self.viewer_src.is_some() {
                let (gw, gh) = self.viewer_dims();
                let (cw, ch) = crate::nfe::sixel::cell_pixel_size();
                self.sixel_seq(gw, gh, cw, ch, bg_rgb)
            } else {
                None
            }
        };
        #[cfg(feature = "image2doc")]
        let kitty_blit: Option<(Vec<String>, crate::nfe::image::BlitGeom)> = {
            use crate::nfe::sixel::PixelBackend;
            if self.viewer_backend() == PixelBackend::Kitty && self.viewer_src.is_some() {
                let (gw, gh) = self.viewer_dims();
                let (cw, ch) = crate::nfe::sixel::cell_pixel_size();
                self.kitty_seq(gw, gh, cw, ch)
            } else {
                None
            }
        };
        let paints_grid = self.viewer_grid.is_some();
        #[cfg(feature = "image2doc")]
        let paints_pixels = sixel_blit.is_some() || kitty_blit.is_some();
        #[cfg(not(feature = "image2doc"))]
        let paints_pixels = false;
        if paints_pixels || paints_grid {
            for y in 0..rows {
                queue!(
                    stdout,
                    MoveTo(0, y as u16),
                    SetBackgroundColor(bg),
                    Print(" ".repeat(cols)),
                    ResetColor,
                )?;
            }
        }

        let name = self
            .viewer_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "(image)".to_string());
        #[cfg(feature = "image2doc")]
        let (ow, oh) = self
            .viewer_src
            .as_ref()
            .map(|s| (s.orig_w, s.orig_h))
            .unwrap_or((0, 0));
        #[cfg(not(feature = "image2doc"))]
        let (ow, oh) = self
            .viewer_grid
            .as_ref()
            .map(|g| (g.orig_w, g.orig_h))
            .unwrap_or((0, 0));
        let backend_label = {
            #[cfg(feature = "image2doc")]
            {
                match self.viewer_backend() {
                    crate::nfe::sixel::PixelBackend::Kitty => "kitty".to_string(),
                    crate::nfe::sixel::PixelBackend::Sixel => "sixel".to_string(),
                    crate::nfe::sixel::PixelBackend::Sextant => match self.preview_mode {
                        crate::nfe::image::ImageMode::Sextant => "sextant".to_string(),
                        crate::nfe::image::ImageMode::Half => "half".to_string(),
                    },
                }
            }
            #[cfg(not(feature = "image2doc"))]
            {
                "cells".to_string()
            }
        };
        let title = format!(
            " {} ({}x{}) · {} · {} ",
            name,
            ow,
            oh,
            backend_label,
            self.zoom_label()
        );
        queue!(
            stdout,
            MoveTo(0, 0),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(title.chars().take(cols).collect::<String>()),
            ResetColor,
        )?;

        #[cfg(feature = "image2doc")]
        if kitty_blit.is_some() || sixel_blit.is_some() {
            // Kitty first: a fresh payload deletes the previous
            // placement, then transmits at the target cell. Cursor must
            // be placed (and flushed) before the final chunk lands.
            if let Some((chunks, g)) = kitty_blit {
                queue!(stdout, Print(crate::nfe::kitty::delete_cmd()))?;
                queue!(stdout, MoveTo(g.pad_x as u16, (1 + g.pad_y) as u16))?;
                stdout.flush()?;
                for chunk in &chunks {
                    stdout.write_all(chunk.as_bytes())?;
                }
                stdout.flush()?;
                self.paint_box(
                    stdout,
                    g.pad_x,
                    1 + g.pad_y,
                    g.w,
                    g.h,
                    cols,
                    rows,
                    border_fg,
                    bg,
                )?;
            } else if let Some((seq, g)) = sixel_blit {
                // Flush the cursor move first: the DCS payload must
                // start exactly at the target cell.
                queue!(stdout, MoveTo(g.pad_x as u16, (1 + g.pad_y) as u16))?;
                stdout.flush()?;
                stdout.write_all(seq.as_bytes())?;
                stdout.flush()?;
                self.paint_box(
                    stdout,
                    g.pad_x,
                    1 + g.pad_y,
                    g.w,
                    g.h,
                    cols,
                    rows,
                    border_fg,
                    bg,
                )?;
            }
            let hint = " +/- zoom · 0 fit · arrows/hjkl pan · o open · W wt · Esc back ";
            queue!(
                stdout,
                MoveTo(0, rows.saturating_sub(1) as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(fg),
                Print(hint),
                ResetColor,
            )?;
            return Ok(());
        }

        if let Some(grid) = &self.viewer_grid {
            let (gw, gh) = self.viewer_dims();
            let (sx, sy) = match self.preview_mode {
                crate::nfe::image::ImageMode::Sextant => (2u32, 3u32),
                crate::nfe::image::ImageMode::Half => (1u32, 2u32),
            };
            let (samp_w, samp_h) = self.viewer_samp;
            let shown_w = grid.rows.first().map(|r| r.len()).unwrap_or(0).min(gw);
            let shown_h = grid.rows.len().min(gh);
            let pad_x = gw.saturating_sub(shown_w) / 2;
            let pad_y = gh.saturating_sub(shown_h) / 2;
            self.paint_box(
                stdout,
                pad_x,
                1 + pad_y,
                shown_w,
                shown_h,
                cols,
                rows,
                border_fg,
                bg,
            )?;
            for (ri, row) in grid.rows.iter().take(gh).enumerate() {
                let y = 1 + pad_y + ri;
                if y + 1 >= rows {
                    break;
                }
                for (ci, cell) in row.iter().take(gw).enumerate() {
                    // Sample rect of this cell; skip fully-outside cells.
                    let x0 = self.viewer_ox + ci as u32 * sx;
                    let y0 = self.viewer_oy + ri as u32 * sy;
                    if x0 >= samp_w || y0 >= samp_h {
                        continue;
                    }
                    let cell_fg = Color::Rgb {
                        r: cell.fg.0,
                        g: cell.fg.1,
                        b: cell.fg.2,
                    };
                    let cell_bg = Color::Rgb {
                        r: cell.bg.0,
                        g: cell.bg.1,
                        b: cell.bg.2,
                    };
                    queue!(
                        stdout,
                        MoveTo((pad_x + ci) as u16, y as u16),
                        SetBackgroundColor(cell_bg),
                        SetForegroundColor(cell_fg),
                        Print(cell.ch),
                        ResetColor,
                    )?;
                }
            }
        }

        let hint = " +/- zoom · 0 fit · arrows/hjkl pan · o open · W wt · Esc back ";
        queue!(
            stdout,
            MoveTo(0, rows.saturating_sub(1) as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(hint),
            ResetColor,
        )?;
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
        } else if crate::nfe::image::is_image_file(&entry.path) {
            lines.push(" (image) Press Enter to view".to_string());
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
        self.preview_content.clear();
        self.preview_image = None;
        let entry = self.current_entry().cloned();
        if let Some(entry) = entry {
            if entry.is_dir {
                self.preview_content = self.render_dir_preview_lines(&entry);
            } else if crate::nfe::image::is_image_file(&entry.path) {
                // Pixel backends skip the text grid (render builds the
                // payload lazily); sextants grid up front as before.
                #[cfg(feature = "image2doc")]
                let pixel = self.viewer_backend() != crate::nfe::sixel::PixelBackend::Sextant;
                #[cfg(not(feature = "image2doc"))]
                let pixel = false;
                if pixel {
                    self.preview_content.clear();
                    self.preview_image = None;
                    self.panel_seq = None;
                } else {
                    #[cfg(feature = "image2doc")]
                    {
                        let w = self.preview_width().saturating_sub(1).max(8);
                        let h = self.list_height().saturating_sub(1).max(4);
                        match self.preview_mode {
                            crate::nfe::image::ImageMode::Sextant => {
                                match crate::nfe::image::load_sextants(&entry.path, w, h) {
                                    Ok(prev) => self.preview_image = Some(prev),
                                    Err(e) => {
                                        self.preview_content =
                                            vec![format!("(cannot preview image: {})", e)];
                                    }
                                }
                            }
                            crate::nfe::image::ImageMode::Half => {
                                match crate::nfe::image::load_halfblock(&entry.path, w, h) {
                                    Ok(prev) => self.preview_image = Some(prev),
                                    Err(e) => {
                                        self.preview_content =
                                            vec![format!("(cannot preview image: {})", e)];
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(not(feature = "image2doc"))]
                    {
                        self.preview_image = None;
                        self.preview_content =
                            vec!["(image preview unavailable in this build)".to_string()];
                    }
                }
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
            bl("  v                 Image: sextant/half blocks", bw),
            bl("  V                 Fullscreen image view", bw),
            bl("  +/-/0             Zoom in/out/fit (viewer)", bw),
            bl("  o                 Open image in OS viewer", bw),
            bl("  W                 Reopen in Windows Terminal", bw),
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
        // Image previews scroll over the grid (one row is taken by the
        // pinned header); text keeps its existing bounds.
        let max_lines = if self.preview_image_active() {
            let grid = self
                .preview_image
                .as_ref()
                .map(|img| img.cells_high())
                .unwrap_or(0);
            grid.saturating_sub(self.list_height().saturating_sub(1).max(1))
        } else {
            self.preview_content
                .len()
                .saturating_sub(self.list_height())
        };
        if self.preview_scroll < max_lines {
            self.preview_scroll += 1;
        }
    }
}
