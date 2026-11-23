use crate::shell::commands::builtin::nfe::nfe::state::{FileEntry, NfeState};
use crate::utils::ansi_text::visible_width;
use anyhow::Result;
use std::io::{stdout, Write};

/// Paint one prompt line on the status row without ever wrapping it.
///
/// Padding by *display* width (not char count) keeps CJK filenames — where one
/// character is two columns wide — from exceeding the terminal width, which
/// made the terminal wrap and repaint the prompt indefinitely. Clearing the
/// line also removes leftovers when the text shrinks (backspace).
fn draw_prompt(
    state: &NfeState,
    prompt: &str,
    bg: crossterm::style::Color,
    fg: crossterm::style::Color,
) -> Result<()> {
    use crossterm::cursor::MoveTo;
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use crossterm::terminal::{Clear, ClearType};

    let width = state.term_w.max(1);
    // When the prompt is too long for the row, keep the tail: that is where
    // the caret and the freshly typed name live.
    let fitted = if visible_width(prompt) > width {
        tail_visible(prompt, width)
    } else {
        prompt
    };
    let pad = width.saturating_sub(visible_width(fitted));
    let line = format!("{}{}", fitted, " ".repeat(pad));
    let status_row = state.term_h.saturating_sub(1);
    queue!(
        stdout(),
        MoveTo(0, status_row as u16),
        Clear(ClearType::CurrentLine),
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        Print(line),
        ResetColor,
    )?;
    stdout().flush()?;
    Ok(())
}

/// Longest suffix of `text` that fits in `width` display columns.
fn tail_visible(text: &str, width: usize) -> &str {
    let mut start = text.len();
    let mut used = 0usize;
    for (i, ch) in text.char_indices().rev() {
        let cw = crate::shell::commands::builtin::ne::editor::char_col_width(ch);
        if used + cw > width {
            break;
        }
        used += cw;
        start = i;
    }
    &text[start..]
}

pub fn rename_entry(state: &mut NfeState, entry: &FileEntry) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use std::time::Duration;

    let mut new_name = entry.name.clone();

    loop {
        let prompt = format!(" Rename '{}' to: {} ", entry.name, new_name);
        draw_prompt(
            state,
            &prompt,
            crossterm::style::Color::Rgb {
                r: 40,
                g: 40,
                b: 60,
            },
            crossterm::style::Color::Rgb {
                r: 220,
                g: 220,
                b: 240,
            },
        )?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    state.set_status("Rename cancelled".to_string());
                    break;
                }
                KeyCode::Enter => {
                    let new_path = entry.path.with_file_name(&new_name);
                    if new_path.exists() {
                        state.set_status("Target already exists".to_string());
                        break;
                    }
                    match std::fs::rename(&entry.path, &new_path) {
                        Ok(_) => {
                            state.set_status(format!("Renamed to '{}'", new_name));
                            state.reload();
                        }
                        Err(e) => {
                            state.set_status(format!("Rename failed: {}", e));
                        }
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    new_name.push(c);
                }
                KeyCode::Backspace => {
                    new_name.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

pub fn delete_entry(state: &mut NfeState, entry: &FileEntry) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use std::time::Duration;

    let mut confirmed = false;

    let prompt = format!(" Delete '{}'? (y/N): ", entry.name);
    draw_prompt(
        state,
        &prompt,
        crossterm::style::Color::Rgb {
            r: 80,
            g: 30,
            b: 30,
        },
        crossterm::style::Color::Rgb {
            r: 255,
            g: 150,
            b: 150,
        },
    )?;

    loop {
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    state.set_status("Delete cancelled".to_string());
                    break;
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    confirmed = true;
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    if confirmed {
        let result = if entry.is_dir {
            std::fs::remove_dir_all(&entry.path)
        } else {
            std::fs::remove_file(&entry.path)
        };

        match result {
            Ok(_) => {
                state.set_status(format!("Deleted '{}'", entry.name));
                state.reload();
            }
            Err(e) => {
                state.set_status(format!("Delete failed: {}", e));
            }
        }
    }

    Ok(())
}

pub fn create_file(state: &mut NfeState) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use std::time::Duration;

    let mut name = String::new();

    loop {
        let prompt = format!(" New file: {} ", name);
        draw_prompt(
            state,
            &prompt,
            crossterm::style::Color::Rgb {
                r: 40,
                g: 60,
                b: 40,
            },
            crossterm::style::Color::Rgb {
                r: 150,
                g: 255,
                b: 150,
            },
        )?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    state.set_status("Cancelled".to_string());
                    break;
                }
                KeyCode::Enter if !name.is_empty() => {
                    let new_path = state.current_dir.join(&name);
                    if new_path.exists() {
                        state.set_status("File already exists".to_string());
                        break;
                    }
                    match std::fs::write(&new_path, "") {
                        Ok(_) => {
                            state.set_status(format!("Created '{}'", name));
                            state.reload();
                        }
                        Err(e) => {
                            state.set_status(format!("Create failed: {}", e));
                        }
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    name.push(c);
                }
                KeyCode::Backspace => {
                    name.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

pub fn create_dir(state: &mut NfeState) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use std::time::Duration;

    let mut name = String::new();

    loop {
        let prompt = format!(" New directory: {} ", name);
        draw_prompt(
            state,
            &prompt,
            crossterm::style::Color::Rgb {
                r: 40,
                g: 60,
                b: 40,
            },
            crossterm::style::Color::Rgb {
                r: 150,
                g: 255,
                b: 150,
            },
        )?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    state.set_status("Cancelled".to_string());
                    break;
                }
                KeyCode::Enter if !name.is_empty() => {
                    let new_path = state.current_dir.join(&name);
                    if new_path.exists() {
                        state.set_status("Directory already exists".to_string());
                        break;
                    }
                    match std::fs::create_dir(&new_path) {
                        Ok(_) => {
                            state.set_status(format!("Created directory '{}'", name));
                            state.reload();
                        }
                        Err(e) => {
                            state.set_status(format!("Create failed: {}", e));
                        }
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    name.push(c);
                }
                KeyCode::Backspace => {
                    name.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_keeps_wide_characters_within_width() {
        assert_eq!(tail_visible("abcdef", 3), "def");
        // Two columns per CJK character: only one fits in three columns.
        assert_eq!(tail_visible("日本語", 3), "語");
        assert_eq!(tail_visible("abc", 10), "abc");
        assert_eq!(visible_width(tail_visible("ab日本", 4)), 4);
    }
}
