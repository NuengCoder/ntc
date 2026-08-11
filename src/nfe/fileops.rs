use crate::nfe::state::{FileEntry, NfeState};
use anyhow::Result;
use std::io::{stdout, Write};

pub fn rename_entry(state: &mut NfeState, entry: &FileEntry) -> Result<()> {
    use crossterm::cursor::MoveTo;
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::time::Duration;

    let status_row = state.term_h.saturating_sub(1);
    let mut new_name = entry.name.clone();

    loop {
        let prompt = format!(" Rename '{}' to: {} ", entry.name, new_name);
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
            stdout(),
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{:w$}", prompt, w = state.term_w)),
            ResetColor,
        )?;
        stdout().flush()?;

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
    use crossterm::cursor::MoveTo;
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::time::Duration;

    let status_row = state.term_h.saturating_sub(1);
    let mut confirmed = false;

    let prompt = format!(" Delete '{}'? (y/N): ", entry.name);
    let bg = crossterm::style::Color::Rgb {
        r: 80,
        g: 30,
        b: 30,
    };
    let fg = crossterm::style::Color::Rgb {
        r: 255,
        g: 150,
        b: 150,
    };

    queue!(
        stdout(),
        MoveTo(0, status_row as u16),
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        Print(format!("{:w$}", prompt, w = state.term_w)),
        ResetColor,
    )?;
    stdout().flush()?;

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
    use crossterm::cursor::MoveTo;
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::time::Duration;

    let status_row = state.term_h.saturating_sub(1);
    let mut name = String::new();

    loop {
        let prompt = format!(" New file: {} ", name);
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
            stdout(),
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{:w$}", prompt, w = state.term_w)),
            ResetColor,
        )?;
        stdout().flush()?;

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
    use crossterm::cursor::MoveTo;
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::time::Duration;

    let status_row = state.term_h.saturating_sub(1);
    let mut name = String::new();

    loop {
        let prompt = format!(" New directory: {} ", name);
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
            stdout(),
            MoveTo(0, status_row as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{:w$}", prompt, w = state.term_w)),
            ResetColor,
        )?;
        stdout().flush()?;

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
