pub(crate) mod events;
pub(crate) mod fileops;
pub(crate) mod render;
pub(crate) mod state;

use crate::nfe::events::{handle_event, Action};
use crate::nfe::state::NfeState;
use anyhow::Result;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;
use std::path::Path;
use std::time::Duration;

/// Run the NTC File Explorer TUI starting at the given optional path.
pub fn run_explorer(path: Option<&Path>) -> Result<()> {
    let mut stdout = stdout();

    enter_tui(&mut stdout)?;

    let (w, h) = terminal::size()?;
    let mut state = NfeState::new(path);
    state.term_w = w as usize;
    state.term_h = h as usize;

    state.reload();
    state.render(&mut stdout)?;

    while state.running {
        if !crossterm::event::poll(Duration::from_millis(50))? {
            continue;
        }

        let ev = crossterm::event::read()?;
        match handle_event(&mut state, ev)? {
            Action::Quit => break,
            Action::OpenEditor => {
                let entry = state.current_entry().cloned();
                if let Some(entry) = entry {
                    if !entry.is_dir {
                        leave_tui(&mut stdout)?;
                        crate::editor::edit_file(&entry.path)?;
                        enter_tui(&mut stdout)?;
                        let (w2, h2) = terminal::size()?;
                        state.term_w = w2 as usize;
                        state.term_h = h2 as usize;
                        state.refresh();
                        state.render(&mut stdout)?;
                        continue;
                    }
                }
            }
            Action::Continue => {}
        }

        state.render(&mut stdout)?;
    }

    leave_tui(&mut stdout)?;
    Ok(())
}

fn enter_tui(stdout: &mut impl std::io::Write) -> std::io::Result<()> {
    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

fn leave_tui(stdout: &mut impl std::io::Write) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture, Show, LeaveAlternateScreen)?;
    Ok(())
}
