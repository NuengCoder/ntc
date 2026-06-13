use crate::navigator::{Navigator};
use crate::output::{print_error, print_info, print_warning};
use crate::session::SessionState;
use crate::watcher;
use crate::shell::alias::expand_command_line;
use crate::shell::commands::execute_command;
use crate::shell::helpers::show_tree;
use crate::config::Config as NtcConfig;

use anyhow::Result;
use colored::*;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::history::FileHistory;
use rustyline::{Context, Editor, Config as RlConfig};
use std::sync::Arc;

/// Combined rustyline helper wrapping our custom hinter.
struct NtcHelper {
    hinter: super::hinter::NtcHinter,
}

impl NtcHelper {
    fn new() -> Self {
        NtcHelper {
            hinter: super::hinter::NtcHinter::new(),
        }
    }
}

impl Hinter for NtcHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for NtcHelper {}
impl Completer for NtcHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        Ok((0, vec![]))
    }
}
impl Validator for NtcHelper {}
impl rustyline::Helper for NtcHelper {}

// ============================================================================
// Main shell entry points
// ============================================================================

pub fn run_shell() -> Result<()> {
    let last_dir = SessionState::read_global().last_directory.clone();
    let mut nav = Navigator::new()?;
    if let Some(ref last_dir) = last_dir {
        if last_dir.exists() {
            let _ = nav.go_to(last_dir);
        }
    }
    run_shell_with_nav(nav)
}

pub fn run_shell_with_nav(nav: Navigator) -> Result<()> {
    run_classic_shell(nav)
}

pub fn run_classic_shell(mut nav: Navigator) -> Result<()> {
    if std::env::var("NTC_SHELL").is_ok() {
        print_warning("Already inside an ntc shell — nested shells are not supported.");
        return Ok(());
    }
    std::env::set_var("NTC_SHELL", "1");

    let mut rl = Editor::<NtcHelper, FileHistory>::with_config(RlConfig::default())
        .map_err(|e| anyhow::anyhow!("Failed to create line editor: {}", e))?;
    rl.set_helper(Some(NtcHelper::new()));

    if let Some(history_path) = NtcConfig::read_global().resolve_history_path() {
        let _ = rl.load_history(&history_path);
    }

    let mut watcher_handle: Option<Arc<watcher::WatcherHandle>> = None;
    if NtcConfig::global_get_file_watcher_enabled() {
        match watcher::start_watcher(nav.current_path()) {
            Ok(w) => {
                let wh = Arc::new(w);
                watcher_handle = Some(wh);
                print_info("File watcher started (recursive)");
            }
            Err(e) => print_warning(&format!("Watcher failed: {}", e)),
        }
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║{}║", format!("        Welcome to ntc {} - Navigate, Toolkit, Center          ", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
    show_tree(&nav, Some(1), false, false, false, false, false);
    
    loop {
        // Check if theme changed
        if crate::utils::theme::ThemeManager::take_theme_changed() {
            // Clear screen and redraw tree with new colors
            crate::navigator::clear_screen();
            show_tree(&nav, Some(1), false, false, false, false, false);
        }
        if let Some(ref wh) = watcher_handle {
            if let Some(summary) = wh.poll() {
                if !summary.is_empty() {
                    summary.print_change_box();
                }
                show_tree(&nav, Some(1), false, false, false, false, false);
                if !summary.is_empty() {
                    summary.print_refresh_box();
                }
                // Auto-run trigger alias if configured
                if let Some(ref alias) = summary.trigger_alias {
                    let expanded = expand_command_line(alias);
                    for cmd in expanded {
                        match execute_command(&cmd, &mut nav, &watcher_handle) {
                            Ok(_)  => {}
                            Err(e) => print_error(&format!("watch trigger: {}", e)),
                        }
                    }
                }
            }
        }
        
        let display_path = nav.display_path();
        let watcher_suffix = if watcher_handle.is_some() { " ~" } else { "" };
        
        // NOTE: Prompt must stay plain text — colored prompts with ANSI codes
        // break rustyline's cursor positioning and line editing.
        let prompt = format!("ntc [{}{}]> ", display_path, watcher_suffix);
        
        let line = match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(&line).ok();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".green());
                break;
            }
            Err(_) => break,
        };
        
        let line = line.trim().trim_start_matches('\u{feff}').trim_end_matches('\u{feff}');
        if line.is_empty() {
            continue;
        }
        
        // Expand all aliases in the entire line
        let expanded_commands = expand_command_line(line);
        
        // Execute each expanded command in sequence
        let mut should_exit = false;
        for cmd in expanded_commands {
            match execute_command(&cmd, &mut nav, &watcher_handle) {
                Ok(exit) => {
                    // Keep watcher in sync after every command (cheap no-op if same path)
                    if let Some(ref wh) = watcher_handle {
                        let _ = wh.update_path(nav.current_path());
                    }
                    if exit {
                        should_exit = true;
                        break;
                    }
                }
                Err(e) => {
                    print_error(&format!("{}", e));
                    break;
                }
            }
        }
        
        // After commands, check for any watcher-triggered changes that arrived during execution
        if !should_exit {
            if let Some(ref wh) = watcher_handle {
                if let Some(summary) = wh.poll() {
                    if !summary.is_empty() {
                        summary.print_change_box();
                    }
                    show_tree(&nav, Some(1), false, false, false, false, false);
                    if !summary.is_empty() {
                        summary.print_refresh_box();
                    }
                    // Auto-run trigger alias if configured
                    if let Some(ref alias) = summary.trigger_alias {
                        let expanded = expand_command_line(alias);
                        for cmd in expanded {
                            match execute_command(&cmd, &mut nav, &watcher_handle) {
                                Ok(_)  => {}
                                Err(e) => print_error(&format!("watch trigger: {}", e)),
                            }
                        }
                    }
                }
            }

        }
        
        if should_exit {
            SessionState::write_global().last_directory = Some(nav.current_path().to_path_buf());
            SessionState::save_global();
            println!("{}", "Goodbye!".green());
            break;
        }
    }
    
    // Also save on EOF
    SessionState::write_global().last_directory = Some(nav.current_path().to_path_buf());
    SessionState::save_global();
    
    if NtcConfig::global_get_history_enabled() {
        if let Some(history_path) = NtcConfig::read_global().resolve_history_path() {
            let _ = rl.save_history(&history_path);
        }
    }
    
    Ok(())
}

