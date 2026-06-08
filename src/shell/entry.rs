use crate::config::Config;
use crate::navigator::{Navigator};
use crate::output::{print_error, print_info, print_warning};
use crate::session::SessionState;
use crate::watcher;
use crate::shell::alias::expand_command_line;
use crate::shell::commands::execute_command;
use crate::shell::helpers::show_tree;

use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;

// ============================================================================
// Main shell entry points
// ============================================================================

pub fn run_shell() -> Result<()> {
    let last_dir = SessionState::global().read().unwrap().last_directory.clone();
    let mut nav = Navigator::new()?;
    if let Some(ref last_dir) = last_dir {
        if last_dir.exists() {
            let _ = nav.go_to(last_dir);
        }
    }
    run_shell_with_nav(nav)
}

pub fn run_shell_with_nav(mut nav: Navigator) -> Result<()> {
    if std::env::var("NTC_SHELL").is_ok() {
        print_warning("Already inside an ntc shell — nested shells are not supported.");
        return Ok(());
    }
    std::env::set_var("NTC_SHELL", "1");
    
    let mut rl = DefaultEditor::new().expect("Failed to create line editor");
    
    if let Some(history_path) = Config::global().read().unwrap().resolve_history_path() {
        let _ = rl.load_history(&history_path);
    }
    
    let mut watcher_handle: Option<Arc<watcher::WatcherHandle>> = None;
    if Config::global_get_file_watcher_enabled() {
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
    println!("║{}║", format!("              Welcome to ntc {} - Navigate, Tree, Cat          ", env!("CARGO_PKG_VERSION")).cyan().bold());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", "Type 'help' for available commands, 'exit' to quit.".dimmed());
    show_tree(&nav, Some(1), false, false, false, false, false);
    
    loop {
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
        let watcher_indicator = if watcher_handle.is_some() { " ~" } else { "" };
        let prompt = format!("ntc [{}{}]> ", display_path, watcher_indicator);
        
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
            SessionState::global().write().unwrap().last_directory = Some(nav.current_path().to_path_buf());
            SessionState::save_global();
            println!("{}", "Goodbye!".green());
            break;
        }
    }
    
    // Also save on EOF
    SessionState::global().write().unwrap().last_directory = Some(nav.current_path().to_path_buf());
    SessionState::save_global();
    
    if Config::global_get_history_enabled() {
        if let Some(history_path) = Config::global().read().unwrap().resolve_history_path() {
            let _ = rl.save_history(&history_path);
        }
    }
    
    Ok(())
}