use crate::nfe::fileops;
use crate::nfe::state::{NfeState, PanelFocus};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEvent, MouseEventKind};
use std::time::Duration;

pub enum Action {
    Continue,
    Quit,
    OpenEditor,
}

pub fn handle_event(state: &mut NfeState, ev: Event) -> Result<Action> {
    // Fullscreen image viewer captures everything first: Esc/q leave
    // the viewer (never the app), the rest drives zoom/pan.
    if state.viewer_active {
        match ev {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return Ok(handle_viewer_key(state, key));
            }
            Event::Resize(w, h) => {
                state.term_w = w as usize;
                state.term_h = h as usize;
                state.regrid_viewer();
            }
            _ => {}
        }
        return Ok(Action::Continue);
    }

    // If help is visible, any key dismisses it
    if state.help_visible {
        if let Event::Key(key) = &ev {
            if key.kind == KeyEventKind::Press {
                state.help_visible = false;
            }
        }
        return Ok(Action::Continue);
    }

    // If line prompt is active, route to line prompt handler
    if state.line_prompt_active {
        if let Event::Key(key) = &ev {
            if key.kind == KeyEventKind::Press {
                handle_line_prompt(state, *key);
            }
        }
        return Ok(Action::Continue);
    }

    // If find is active, route to find handler
    if state.find_active {
        if let Event::Key(key) = &ev {
            if key.kind == KeyEventKind::Press {
                handle_find_mode(state, *key)?;
            }
        }
        return Ok(Action::Continue);
    }

    match ev {
        Event::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return Ok(Action::Continue);
            }
            handle_key(state, key)
        }
        Event::Mouse(mouse) => {
            handle_mouse(state, mouse);
            Ok(Action::Continue)
        }
        Event::Resize(w, h) => {
            state.term_w = w as usize;
            state.term_h = h as usize;
            state.scroll_to_selected();
            Ok(Action::Continue)
        }
        _ => Ok(Action::Continue),
    }
}

fn handle_key(state: &mut NfeState, key: crossterm::event::KeyEvent) -> Result<Action> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            if !state.filter.is_empty() {
                state.filter.clear();
                state.reload();
            } else {
                state.running = false;
                return Ok(Action::Quit);
            }
            Ok(Action::Continue)
        }

        KeyCode::Up | KeyCode::Char('k') => {
            if state.focus == PanelFocus::List {
                state.scroll_up(1);
            } else {
                state.scroll_preview_up();
            }
            Ok(Action::Continue)
        }

        KeyCode::Down | KeyCode::Char('j') => {
            if state.focus == PanelFocus::List {
                state.scroll_down(1);
            } else {
                state.scroll_preview_down();
            }
            Ok(Action::Continue)
        }

        KeyCode::PageUp => {
            if state.focus == PanelFocus::List {
                let n = state.list_height().saturating_sub(1);
                state.scroll_up(n);
            } else {
                let n = state.list_height().saturating_sub(1);
                for _ in 0..n {
                    state.scroll_preview_up();
                }
            }
            Ok(Action::Continue)
        }

        KeyCode::PageDown => {
            if state.focus == PanelFocus::List {
                let n = state.list_height().saturating_sub(1);
                state.scroll_down(n);
            } else {
                let n = state.list_height().saturating_sub(1);
                for _ in 0..n {
                    state.scroll_preview_down();
                }
            }
            Ok(Action::Continue)
        }

        KeyCode::Home => {
            if state.focus == PanelFocus::List {
                state.selected = 0;
                state.scroll = 0;
            } else {
                state.preview_scroll = 0;
            }
            Ok(Action::Continue)
        }

        KeyCode::End => {
            if state.focus == PanelFocus::List {
                state.selected = state.entries.len().saturating_sub(1);
                state.scroll_to_selected();
            } else {
                state.preview_scroll = state
                    .preview_content
                    .len()
                    .saturating_sub(state.list_height());
            }
            Ok(Action::Continue)
        }

        KeyCode::Enter | KeyCode::Right => {
            if state.focus == PanelFocus::List {
                if state.selected_is_dir() {
                    state.enter_dir();
                } else {
                    state.load_preview_content();
                }
            }
            Ok(Action::Continue)
        }

        KeyCode::Left | KeyCode::Char('b') => {
            state.go_to_parent();
            Ok(Action::Continue)
        }

        KeyCode::Tab => {
            state.focus = match state.focus {
                PanelFocus::List => PanelFocus::Preview,
                PanelFocus::Preview => PanelFocus::List,
            };
            Ok(Action::Continue)
        }

        KeyCode::Char('~') => {
            if let Some(home) = dirs::home_dir() {
                state.go_to_dir(&home);
            }
            Ok(Action::Continue)
        }

        KeyCode::Char('g') => {
            state.focus = PanelFocus::List;
            enter_goto_mode(state)?;
            Ok(Action::Continue)
        }

        // ── V: fullscreen image viewer ──
        KeyCode::Char('V') => {
            state.open_viewer();
            Ok(Action::Continue)
        }

        // ── h: Help overlay ──
        KeyCode::Char('h') => {
            state.toggle_help();
            Ok(Action::Continue)
        }

        // ── l: Go to line ──
        KeyCode::Char('l') => {
            state.start_line_prompt();
            Ok(Action::Continue)
        }

        // ── f: Recursive find ──
        KeyCode::Char('f') => {
            state.start_find();
            Ok(Action::Continue)
        }

        KeyCode::Char('i') => {
            state.toggle_hidden();
            Ok(Action::Continue)
        }

        // ── v: cycle image preview density (sextant/half) ──
        KeyCode::Char('v') => {
            state.toggle_preview_mode();
            Ok(Action::Continue)
        }

        KeyCode::Char('s') => {
            state.cycle_sort();
            Ok(Action::Continue)
        }

        KeyCode::Char('S') => {
            state.toggle_sort_desc();
            Ok(Action::Continue)
        }

        KeyCode::Char('r') => {
            if let Some(entry) = state.current_entry().cloned() {
                fileops::rename_entry(state, &entry)?;
            }
            Ok(Action::Continue)
        }

        KeyCode::Char('d') => {
            if let Some(entry) = state.current_entry().cloned() {
                fileops::delete_entry(state, &entry)?;
            }
            Ok(Action::Continue)
        }

        KeyCode::Char('e') => {
            if state.focus == PanelFocus::List {
                if let Some(entry) = state.current_entry() {
                    if !entry.is_dir {
                        return Ok(Action::OpenEditor);
                    }
                }
            }
            Ok(Action::Continue)
        }

        // ── o: open image in OS viewer · W: relaunch in WT ──
        KeyCode::Char('o') => {
            state.open_external();
            Ok(Action::Continue)
        }
        KeyCode::Char('W') => {
            if state.relaunch_in_wt() {
                return Ok(Action::Quit);
            }
            Ok(Action::Continue)
        }

        KeyCode::Char('/') => {
            enter_filter_mode(state)?;
            Ok(Action::Continue)
        }

        KeyCode::Char('n') if state.focus == PanelFocus::List => {
            fileops::create_file(state)?;
            Ok(Action::Continue)
        }

        KeyCode::Char('m') if state.focus == PanelFocus::List => {
            fileops::create_dir(state)?;
            Ok(Action::Continue)
        }

        KeyCode::F(5) => {
            state.refresh();
            state.set_status("Refreshed".to_string());
            Ok(Action::Continue)
        }

        _ => Ok(Action::Continue),
    }
}

// ── Line prompt handler ──

fn handle_line_prompt(state: &mut NfeState, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.cancel_line_prompt();
        }
        KeyCode::Enter => {
            state.execute_line_jump();
        }
        KeyCode::Backspace => {
            state.line_prompt.pop();
        }
        KeyCode::Char(c) => {
            if c == '+' || c == '-' || c.is_ascii_digit() {
                state.line_prompt.push(c);
            }
        }
        _ => {}
    }
}

// ── Find mode handler ──

fn handle_find_mode(state: &mut NfeState, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            state.cancel_find();
        }
        KeyCode::Enter => {
            // Select the first find result and navigate to its directory
            if let Some(entry) = state.find_results.first().cloned() {
                if entry.is_dir {
                    state.go_to_dir(&entry.path);
                } else if let Some(parent) = entry.path.parent() {
                    state.go_to_dir(parent);
                    // Try to select the file
                    let name = entry.name.clone();
                    if let Some(idx) = state.entries.iter().position(|e| e.name == name) {
                        state.selected = idx;
                        state.scroll_to_selected();
                    }
                }
                state.set_status(format!("Found: {}", entry.path.display()));
            }
            state.cancel_find();
        }
        KeyCode::Backspace => {
            state.find_query.pop();
            state.update_find();
        }
        KeyCode::Char(c) => {
            state.find_query.push(c);
            state.update_find();
        }
        _ => {}
    }
    Ok(())
}

// ── Fullscreen image viewer keys ──

fn handle_viewer_key(state: &mut NfeState, key: crossterm::event::KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.close_viewer();
        }
        // V/+/- step zoom, 0 resets to fit (stops: 0.25-8x).
        KeyCode::Char('V') | KeyCode::Char('+') | KeyCode::Char('=') => {
            state.step_viewer_zoom(1);
        }
        KeyCode::Char('-') | KeyCode::Char('_') => {
            state.step_viewer_zoom(-1);
        }
        KeyCode::Char('0') => {
            state.reset_viewer_zoom();
        }
        // o: OS viewer (real pixels on every terminal).
        // W: relaunch in Windows Terminal (conhost escape hatch).
        KeyCode::Char('o') => {
            state.open_external();
        }
        KeyCode::Char('W') => {
            if state.relaunch_in_wt() {
                return Action::Quit;
            }
        }
        KeyCode::Char('v') => {
            state.toggle_preview_mode();
            state.regrid_viewer();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            state.pan_viewer(-4, 0);
            state.regrid_viewer();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.pan_viewer(4, 0);
            state.regrid_viewer();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.pan_viewer(0, -4);
            state.regrid_viewer();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.pan_viewer(0, 4);
            state.regrid_viewer();
        }
        KeyCode::PageUp => {
            state.pan_viewer(0, -(state.viewer_dims().1 as i32 / 2));
            state.regrid_viewer();
        }
        KeyCode::PageDown => {
            state.pan_viewer(0, state.viewer_dims().1 as i32 / 2);
            state.regrid_viewer();
        }
        KeyCode::Home => {
            state.viewer_ox = 0;
            state.viewer_oy = 0;
            state.regrid_viewer();
        }
        _ => {}
    }
    Action::Continue
}

// ── Mouse handler ──

fn handle_mouse(state: &mut NfeState, mouse: MouseEvent) {
    if state.help_visible
        || state.line_prompt_active
        || state.find_active
        || state.viewer_active
    {
        return;
    }

    match mouse.kind {
        MouseEventKind::Down(_) => {
            let row = mouse.row as usize;
            let col = mouse.column as usize;
            let content_top = 1;
            let list_h = state.list_height();

            if row >= content_top && row < content_top + list_h {
                let num_w = state.line_number_width();
                let list_w = state.list_width();
                if col >= num_w && col < num_w + list_w {
                    let idx = state.scroll + (row - content_top);
                    if idx < state.entries.len() {
                        state.selected = idx;
                        state.focus = PanelFocus::List;
                    }
                } else if col > num_w + list_w {
                    state.focus = PanelFocus::Preview;
                }
            }
        }
        MouseEventKind::ScrollUp => {
            if state.focus == PanelFocus::List {
                state.scroll_up(1);
            } else {
                state.scroll_preview_up();
            }
        }
        MouseEventKind::ScrollDown => {
            if state.focus == PanelFocus::List {
                state.scroll_down(1);
            } else {
                state.scroll_preview_down();
            }
        }
        _ => {}
    }
}

// ── Goto mode (g key) ──

fn enter_goto_mode(state: &mut NfeState) -> Result<()> {
    use crossterm::cursor::MoveTo;
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::io::{stdout, Write};

    let status_row = state.term_h.saturating_sub(1);
    let mut buf = String::new();

    loop {
        let prompt = format!(" Go to: {} ", buf);
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
                    break;
                }
                KeyCode::Enter => {
                    if !buf.is_empty() {
                        let target = state.resolve_goto(&buf);
                        if target.exists() && target.is_dir() {
                            state.go_to_dir(&target);
                        } else if go_to_teleport(state, buf.trim()).unwrap_or(false) {
                            // Teleport bookmark matched.
                        } else {
                            state.set_status(format!("No such directory: {}", buf.trim()));
                        }
                    }
                    break;
                }
                KeyCode::Char(c) => {
                    buf.push(c);
                }
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Tab => {
                    // Complete against the current directory (never the
                    // process CWD): split the buffer into a base dir +
                    // partial name, both resolved against current_dir.
                    let (base, partial) = match buf.rfind(['/', '\\']) {
                        Some(pos) => {
                            let base = state.resolve_goto(&buf[..=pos]);
                            (base, buf[pos + 1..].to_string())
                        }
                        None => (state.current_dir.clone(), buf.clone()),
                    };
                    if let Ok(entries) = std::fs::read_dir(&base) {
                        let lower = partial.to_lowercase();
                        let mut matches: Vec<String> = entries
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .filter(|n| n.to_lowercase().starts_with(&lower))
                            .collect();
                        matches.sort();
                        if matches.len() == 1 {
                            let completed = if base == state.current_dir {
                                matches[0].clone()
                            } else {
                                let mut s = buf[..buf.len() - partial.len()].to_string();
                                s.push_str(&matches[0]);
                                s
                            };
                            buf = completed;
                        } else if matches.len() > 1 {
                            state.set_status(format!(
                                "{} matches: {}",
                                matches.len(),
                                matches.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
                            ));
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    state.scroll_to_selected();
    Ok(())
}

// ── Filter mode (/ key) ──

fn enter_filter_mode(state: &mut NfeState) -> Result<()> {
    use crossterm::cursor::MoveTo;
    use crossterm::queue;
    use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
    use std::io::{stdout, Write};

    let status_row = state.term_h.saturating_sub(1);
    let mut filter = state.filter.clone();

    loop {
        let prompt = format!(" Filter: {} ", filter);
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
                    state.filter = String::new();
                    state.reload();
                    break;
                }
                KeyCode::Enter => {
                    state.reload();
                    break;
                }
                KeyCode::Char(c) => {
                    filter.push(c);
                    state.filter = filter.clone();
                    state.reload();
                }
                KeyCode::Backspace => {
                    filter.pop();
                    state.filter = filter.clone();
                    state.reload();
                }
                _ => {}
            },
            _ => {}
        }
    }

    if !state.filter.is_empty() && state.entries.is_empty() {
        state.filter.clear();
        state.reload();
    }

    state.scroll_to_selected();
    Ok(())
}

fn go_to_teleport(state: &mut NfeState, name: &str) -> Result<bool> {
    use crate::teleport::TeleportManager;
    if let Some(path) = TeleportManager::get_path(name) {
        state.go_to_dir(&path);
        Ok(true)
    } else {
        Ok(false)
    }
}
