use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Write;

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::queue;

/// Number of visual columns for the gutter (auto-expands in render).
pub(crate) fn gutter_width(total_lines: usize) -> usize {
    let digits = total_lines.to_string().len();
    digits + 2 // padding on both sides
}

pub(crate) fn term_size() -> (usize, usize) {
    terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

pub(crate) fn set_status_bar(
    stdout: &mut std::io::Stdout,
    bg: Color,
    fg: Color,
    msg: &str,
) -> std::io::Result<()> {
    let h = term_size().1;
    queue!(
        stdout,
        MoveTo(0, h.saturating_sub(1) as u16),
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        Print(" "),
        Print(msg),
        ResetColor,
        Clear(ClearType::UntilNewLine),
    )?;
    stdout.flush()
}

pub(crate) fn prompt_yes_no(
    stdout: &mut std::io::Stdout,
    bg: Color,
    msg: &str,
) -> std::io::Result<Option<bool>> {
    let h = term_size().1;
    queue!(
        stdout,
        MoveTo(0, h.saturating_sub(1) as u16),
        SetBackgroundColor(bg),
        SetForegroundColor(Color::White),
        Print(&format!(" {} : ", msg)),
        ResetColor,
        Clear(ClearType::UntilNewLine),
    )?;
    stdout.flush()?;
    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('y'),
                kind: KeyEventKind::Press,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('Y'),
                kind: KeyEventKind::Press,
                ..
            }) => return Ok(Some(true)),
            Event::Key(KeyEvent {
                code: KeyCode::Char('n'),
                kind: KeyEventKind::Press,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('N'),
                kind: KeyEventKind::Press,
                ..
            }) => return Ok(Some(false)),
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => return Ok(None),
            _ => {}
        }
    }
}

/// Canonical (start, end) from cursor + anchor, where start <= end.
pub(crate) fn sel_range(
    cursor_y: usize,
    cursor_byte: usize,
    anchor_y: usize,
    anchor_byte: usize,
) -> ((usize, usize), (usize, usize)) {
    if (cursor_y, cursor_byte) <= (anchor_y, anchor_byte) {
        ((cursor_y, cursor_byte), (anchor_y, anchor_byte))
    } else {
        ((anchor_y, anchor_byte), (cursor_y, cursor_byte))
    }
}
