use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;

use super::super::Editor;

const BYTES_PER_ROW: usize = 16;

impl Editor {
    pub(crate) fn render_hex_view(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let hex = match self.hex.as_ref() {
            Some(h) => h,
            None => return Ok(()),
        };
        let rows = self.term_h.saturating_sub(4).max(1);
        let cols = self.term_w;
        let content_y = 1;

        // Clear the content area
        for r in 0..rows {
            let row = content_y + r;
            if row >= self.term_h { break; }
            queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::UntilNewLine))?;
        }

        for r in 0..rows {
            let row = content_y + r;
            if row >= self.term_h { break; }

            let base = hex.offset + r * BYTES_PER_ROW;
            if base >= hex.data.len() { break; }

            let mut x: usize = 0;

            // ── Offset column ──
            let offset_str = format!("{:08X}", base);
            queue!(
                stdout,
                MoveTo(x as u16, row as u16),
                SetForegroundColor(theme.editor.gutter_text.to_crossterm()),
                Print(&offset_str),
            )?;
            x += 8;

            // separator
            queue!(stdout, Print(" │ "))?;
            x += 3;

            // ── Hex bytes column ──
            let bytes_end = (base + BYTES_PER_ROW).min(hex.data.len());
            for i in base..bytes_end {
                let b = hex.data[i];

                if i == hex.cursor {
                    let byte_str = if hex.nibble {
                        format!("{:1X} ", hex.pending_byte >> 4)
                    } else {
                        format!("{:02X}", b)
                    };
                    let rest = if hex.nibble { "" } else { " " };

                    queue!(
                        stdout,
                        SetBackgroundColor(theme.editor.cursor_bg.to_crossterm()),
                        SetForegroundColor(theme.editor.cursor_text.to_crossterm()),
                        Print(&byte_str),
                        ResetColor,
                        Print(rest),
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

            // Pad remaining hex columns if row is incomplete
            if bytes_end - base < 16 {
                let remaining = 16 - (bytes_end - base);
                for i in 0..remaining {
                    let sep = if i == 7 { "  " } else { " " };
                    queue!(stdout, Print("  "), Print(sep))?;
                    x += if i == 7 { 4 } else { 3 };
                }
            }

            // separator
            queue!(stdout, Print(" │ "))?;
            x += 3;

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
