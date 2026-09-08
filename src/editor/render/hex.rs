use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use super::super::Editor;

const BYTES_PER_ROW: usize = 16;
const OFFSET_W: usize = 8;
const SEP_W: usize = 3;

impl Editor {
    pub(crate) fn render_hex_view(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let hex = match self.hex.as_ref() {
            Some(h) => h,
            None => return Ok(()),
        };
        let rows = self.visible_text_rows().max(1);
        let cols = self.term_w;
        let content_y = 1;
        let x_offset = self.editor_offset();
        let avail_w = cols.saturating_sub(x_offset);

        // Clear the content area (from the editor offset so the sidebar stays).
        for r in 0..rows {
            let row = content_y + r;
            if row >= self.term_h {
                break;
            }
            queue!(
                stdout,
                MoveTo(x_offset as u16, row as u16),
                Clear(ClearType::UntilNewLine)
            )?;
        }

        // How many hex bytes fit next to the offset and ASCII columns:
        // width = OFFSET_W + SEP_W + (3n + 1) + SEP_W + n = 15 + 4n.
        let fit_bytes = (avail_w.saturating_sub(15) / 4).min(BYTES_PER_ROW);

        for r in 0..rows {
            let row = content_y + r;
            if row >= self.term_h {
                break;
            }

            let base = hex.offset + r * BYTES_PER_ROW;
            if base >= hex.data.len() {
                break;
            }

            let mut x: usize = x_offset;

            // ── Offset column ──
            let offset_str = format!("{:08X}", base);
            let offset_draw = if offset_str.chars().count() > avail_w {
                let end = offset_str.floor_char_boundary(avail_w);
                &offset_str[..end]
            } else {
                &offset_str
            };
            queue!(
                stdout,
                MoveTo(x as u16, row as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                Print(offset_draw),
            )?;
            x += OFFSET_W;

            if fit_bytes == 0 {
                queue!(stdout, ResetColor, Clear(ClearType::UntilNewLine))?;
                continue;
            }

            // separator
            queue!(stdout, Print(" │ "))?;
            x += SEP_W;

            // ── Hex bytes column ──
            let bytes_end = (base + fit_bytes).min(hex.data.len());
            for i in base..bytes_end {
                let b = hex.data[i];

                if i == hex.cursor {
                    let byte_str = if hex.nibble {
                        format!("{:1X} ", hex.pending_byte >> 4)
                    } else {
                        format!("{:02X} ", b)
                    };
                    queue!(
                        stdout,
                        SetBackgroundColor(theme.editor.cursor_bg.to_crossterm()),
                        SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                        Print(&byte_str),
                        ResetColor,
                    )?;
                    x += if hex.nibble { 2 } else { 3 };
                } else {
                    let byte_str = format!("{:02X}", b);
                    let sep = if (i - base) == 7 { "  " } else { " " };
                    queue!(
                        stdout,
                        SetForegroundColor(theme.syntax.normal.to_crossterm()),
                        Print(&byte_str),
                        Print(sep),
                    )?;
                    x += if (i - base) == 7 { 4 } else { 3 };
                }
            }

            // separator
            queue!(stdout, Print(" │ "))?;
            x += SEP_W;

            // ── ASCII column ──
            for i in base..bytes_end {
                let b = hex.data[i];
                let ch = if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                };

                if i == hex.cursor {
                    queue!(
                        stdout,
                        SetBackgroundColor(theme.editor.cursor_bg.to_crossterm()),
                        SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                        Print(ch),
                        ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        SetForegroundColor(theme.syntax.string.to_crossterm()),
                        Print(ch),
                        ResetColor,
                    )?;
                }
            }

            // Clear remainder of line
            if cols > x {
                queue!(stdout, ResetColor, Clear(ClearType::UntilNewLine))?;
            }
        }

        Ok(())
    }
}