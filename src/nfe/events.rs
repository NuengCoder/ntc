use crate::nfe::fileops;
use crate::nfe::state::{NfeMode, NfeState, PanelFocus};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind};

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

    // Input modes own the keyboard; the main loop repaints after every
    // event, so filter/query edits show up as they are typed.
    match state.mode {
        NfeMode::Prompt => {
            if let Event::Key(key) = &ev {
                if key.kind == KeyEventKind::Press {
                    handle_prompt_key(state, *key);
                }
            }
            return Ok(Action::Continue);
        }
        NfeMode::Filter => {
            if let Event::Key(key) = &ev {
                if key.kind == KeyEventKind::Press {
                    handle_filter_key(state, *key);
                }
            }
            return Ok(Action::Continue);
        }
        NfeMode::Find => {
            if let Event::Key(key) = &ev {
                if key.kind == KeyEventKind::Press {
                    handle_find_key(state, *key);
                }
            }
            return Ok(Action::Continue);
        }
        NfeMode::Browse => {}
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
                state.rebuild_entries();
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

        // ── g · : : command prompt (file / dir / line) ──
        KeyCode::Char('g') | KeyCode::Char(':') => {
            state.focus = PanelFocus::List;
            state.start_prompt("");
            Ok(Action::Continue)
        }

        // ── l: line jump, pre-filled into the command prompt ──
        KeyCode::Char('l') => {
            state.start_prompt("line ");
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

        // ── f: recursive find mode ──
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

        // ── o: open the selected image in the OS viewer ──
        KeyCode::Char('o') => {
            state.open_external();
            Ok(Action::Continue)
        }

        KeyCode::Char('/') => {
            state.start_filter();
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

// ── Command prompt (g / :) ──

fn handle_prompt_key(state: &mut NfeState, key: crossterm::event::KeyEvent) {
    // Ctrl+E accepts the ghost hint, like End.
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('e') {
        state.prompt_accept_or_end();
        return;
    }
    match key.code {
        KeyCode::Esc => state.cancel_prompt(),
        KeyCode::Enter => state.run_prompt(),
        KeyCode::Tab => state.prompt_tab(),
        KeyCode::Backspace => state.prompt_backspace(),
        KeyCode::Delete => state.prompt_delete(),
        KeyCode::Left => state.prompt_move_left(),
        KeyCode::Right => state.prompt_accept_or_right(),
        KeyCode::Home => state.prompt_home(),
        KeyCode::End => state.prompt_accept_or_end(),
        KeyCode::Char(c) => state.prompt_push(c),
        _ => {}
    }
}

// ── Filter box (/) — live, no Enter needed ──

fn handle_filter_key(state: &mut NfeState, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => state.cancel_filter(),
        KeyCode::Enter => state.finish_filter(),
        KeyCode::Backspace => state.filter_backspace(),
        KeyCode::Char(c) => state.filter_push(c),
        _ => {}
    }
}

// ── Find mode (f) ──

fn handle_find_key(state: &mut NfeState, key: crossterm::event::KeyEvent) {
    if !state.find_nav {
        // Typing focus: every printable character narrows the search.
        match key.code {
            KeyCode::Esc => {
                state.cancel_find();
                return;
            }
            KeyCode::Enter => {
                state.accept_find();
                return;
            }
            KeyCode::Tab => {
                state.find_nav = true;
                return;
            }
            KeyCode::Up => {
                state.find_move(-1);
                return;
            }
            KeyCode::Down => {
                state.find_move(1);
                return;
            }
            KeyCode::PageUp => {
                let n = state.list_height().saturating_sub(1) as isize;
                state.find_move(-n);
                return;
            }
            KeyCode::PageDown => {
                let n = state.list_height().saturating_sub(1) as isize;
                state.find_move(n);
                return;
            }
            KeyCode::Left => {
                state.cancel_find();
                return;
            }
            KeyCode::Right => {
                state.accept_find();
                return;
            }
            KeyCode::Backspace => {
                state.find_query.pop();
                state.update_find();
                return;
            }
            KeyCode::Char(c) => {
                state.find_query.push(c);
                state.update_find();
                return;
            }
            _ => return,
        }
    }

    // Results focus: hjkl and the arrows walk the hit list.
    match key.code {
        // Esc always leaves find mode; `Tab`/`h` return to typing.
        KeyCode::Esc => state.cancel_find(),
        KeyCode::Tab => state.find_nav = false,
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => state.accept_find(),
        KeyCode::Char('h') | KeyCode::Left => {
            state.find_nav = false;
            state.set_status("Find: typing again (Esc leaves find mode)".to_string());
        }
        KeyCode::Char('k') | KeyCode::Up => state.find_move(-1),
        KeyCode::Char('j') | KeyCode::Down => state.find_move(1),
        KeyCode::PageUp => {
            let n = state.list_height().saturating_sub(1) as isize;
            state.find_move(-n);
        }
        KeyCode::PageDown => {
            let n = state.list_height().saturating_sub(1) as isize;
            state.find_move(n);
        }
        KeyCode::Home => {
            state.selected = 0;
            state.scroll = 0;
        }
        KeyCode::End => {
            state.selected = state.entries.len().saturating_sub(1);
            state.scroll_to_selected();
        }
        // `/` in the result list narrows the hits without losing the query
        // (typing focus keeps `/` literal, so `src/main.rs` stays typeable).
        KeyCode::Char('/') => state.start_filter(),
        KeyCode::Backspace => {
            state.find_nav = false;
            state.find_query.pop();
            state.update_find();
        }
        KeyCode::Char(c) => {
            state.find_nav = false;
            state.find_query.push(c);
            state.update_find();
        }
        _ => {}
    }
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
        KeyCode::Char('o') => {
            state.open_external();
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
    if state.help_visible || state.mode != NfeMode::Browse || state.viewer_active {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nfe::state::{NfeMode, NfeState};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn tree(tag: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("ntc_nfe_ev_{}_{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("main.rs"), "x").unwrap();
        std::fs::write(root.join("sub").join("nested.rs"), "x").unwrap();
        root
    }

    fn press(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    fn send(state: &mut NfeState, keys: &str) {
        for c in keys.chars() {
            handle_event(state, press(KeyCode::Char(c))).unwrap();
        }
    }

    #[test]
    fn find_mode_routes_typing_navigation_and_filter() {
        let root = tree("findkeys");
        let mut s = NfeState::new(Some(&root));

        handle_event(&mut s, press(KeyCode::Char('f'))).unwrap();
        assert_eq!(s.mode, NfeMode::Find);
        assert!(s.in_find);

        // Typing focus: every printable character narrows the search.
        send(&mut s, "nested");
        assert_eq!(s.find_query, "nested");
        assert_eq!(s.entries.len(), 1);
        assert_eq!(s.entries[0].display, "sub/nested.rs");

        // Tab moves focus to the result list: hjkl walks it, it does not
        // type into the query.
        handle_event(&mut s, press(KeyCode::Tab)).unwrap();
        assert!(s.find_nav);
        let before = s.find_query.clone();
        handle_event(&mut s, press(KeyCode::Char('k'))).unwrap();
        handle_event(&mut s, press(KeyCode::Char('j'))).unwrap();
        assert_eq!(s.find_query, before);

        // `/` narrows the hit list without touching the query.
        handle_event(&mut s, press(KeyCode::Char('/'))).unwrap();
        assert_eq!(s.mode, NfeMode::Filter);
        send(&mut s, "zz");
        assert!(s.entries.is_empty());
        handle_event(&mut s, press(KeyCode::Esc)).unwrap();
        assert_eq!(s.mode, NfeMode::Find);
        assert_eq!(s.find_query, "nested");

        // Esc leaves find mode (never the app).
        handle_event(&mut s, press(KeyCode::Esc)).unwrap();
        assert_eq!(s.mode, NfeMode::Browse);
        assert!(!s.in_find);
        assert!(s.running);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn browse_keys_open_command_and_live_filter() {
        let root = tree("browsekeys");
        let mut s = NfeState::new(Some(&root));

        // `:` and `g` both open the command prompt.
        handle_event(&mut s, press(KeyCode::Char(':'))).unwrap();
        assert_eq!(s.mode, NfeMode::Prompt);
        send(&mut s, "file main.rs");
        assert_eq!(s.prompt_buf, "file main.rs");
        handle_event(&mut s, press(KeyCode::Enter)).unwrap();
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(s.current_entry().map(|e| e.name.as_str()), Some("main.rs"));

        handle_event(&mut s, press(KeyCode::Char('g'))).unwrap();
        assert_eq!(s.mode, NfeMode::Prompt);
        handle_event(&mut s, press(KeyCode::Esc)).unwrap();
        assert_eq!(s.mode, NfeMode::Browse);

        // `/` filters as the text is typed; Enter keeps it.
        handle_event(&mut s, press(KeyCode::Char('/'))).unwrap();
        assert_eq!(s.mode, NfeMode::Filter);
        send(&mut s, "ne");
        assert_eq!(s.filter, "ne");
        assert!(s.entries.iter().all(|e| e.display.contains("ne")));
        assert!(s.entries.is_empty(), "no listing row contains 'ne'");
        handle_event(&mut s, press(KeyCode::Enter)).unwrap();
        assert_eq!(s.mode, NfeMode::Browse);
        assert_eq!(s.filter, "ne");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn w_is_not_bound_any_more() {
        let root = tree("nokeyw");
        let mut s = NfeState::new(Some(&root));
        let selected = s.selected;
        send(&mut s, "wW");
        // No jump, no relaunch, no exit: the keys are simply unused.
        assert_eq!(s.selected, selected);
        assert_eq!(s.mode, NfeMode::Browse);
        assert!(s.running);
        assert!(s.filter.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }
}
