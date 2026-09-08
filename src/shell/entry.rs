use crate::config::Config as NtcConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::session::SessionState;
use crate::shell::alias::expand_command_line;
use crate::shell::commands::execute_command;
use crate::shell::helpers::show_tree;
use crate::t;
use crate::watcher;

use anyhow::Result;
use colored::*;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::FileHistory;
use rustyline::validate::Validator;
use rustyline::{Config as RlConfig, Context, Editor};
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
    // Inject active ntpkg runtime bin directories into PATH
    let pkg_dirs = crate::ntpkg::build_package_path_entries();
    if !pkg_dirs.is_empty() {
        let path_sep = if cfg!(windows) { ";" } else { ":" };
        let current_path = std::env::var("PATH").unwrap_or_default();
        let mut new_path = String::new();
        for dir in &pkg_dirs {
            if !new_path.is_empty() {
                new_path.push_str(path_sep);
            }
            new_path.push_str(&dir.to_string_lossy());
        }
        if !current_path.is_empty() {
            new_path.push_str(path_sep);
            new_path.push_str(&current_path);
        }
        std::env::set_var("PATH", &new_path);
        let count = pkg_dirs.len();
        print_info(&t!(
            "entry.ntpkg_active",
            count,
            if count == 1 { "" } else { "s" }
        ));
    }

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
                print_info(&t!("entry.watcher_started"));
            }
            Err(e) => print_warning(&t!("entry.watcher_failed", e)),
        }
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!(
        "{:^66}",
        t!("entry.welcome_fmt", env!("CARGO_PKG_VERSION"))
            .cyan()
            .bold()
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", t!("entry.startup_hint").dimmed());
    if NtcConfig::global_get_dir_view_enabled() {
        show_tree(&nav, Some(1), false, false, false, false, false, false);
    }

    loop {
        // Check if theme changed
        if crate::utils::theme::ThemeManager::take_theme_changed() {
            // Clear screen and redraw tree with new colors
            crate::navigator::clear_screen();
            if NtcConfig::global_get_dir_view_enabled() {
                show_tree(&nav, Some(1), false, false, false, false, false, false);
            }
        }
        if let Some(ref wh) = watcher_handle {
            if let Some(summary) = wh.poll() {
                if !summary.is_empty() {
                    summary.print_change_box();
                }
                if NtcConfig::global_get_dir_view_enabled() {
                    show_tree(&nav, Some(1), false, false, false, false, false, false);
                }
                if !summary.is_empty() {
                    summary.print_refresh_box();
                }
                // Auto-run trigger alias if configured
                if let Some(ref alias) = summary.trigger_alias {
                    let expanded = expand_command_line(alias);
                    for cmd in expanded {
                        match execute_command(&cmd, &mut nav, &watcher_handle) {
                            Ok(_) => {}
                            Err(e) => print_error(&format!("watch trigger: {}", e)),
                        }
                    }
                }
            }
        }

        let display_path = nav.current_dir_name();
        let watcher_suffix = if watcher_handle.is_some() { " ~" } else { "" };
        let git_suffix = if NtcConfig::global_get_git_integration_enabled() {
            crate::utils::git::prompt_fragment(nav.current_path())
        } else {
            String::new()
        };
        let git_suffix = if git_suffix.is_empty() {
            String::new()
        } else {
            format!(" {}", git_suffix)
        };

        // NOTE: Prompt must stay plain text — colored prompts with ANSI codes
        // break rustyline's cursor positioning and line editing.
        let venv_prefix = nav.get_colored_venv_prefix(); // Use colored version

        // Build the prompt
        let prompt = if venv_prefix.is_empty() {
            format!("ntc [{}{}{}]> ", display_path, watcher_suffix, git_suffix)
        } else {
            format!(
                "{}ntc [{}{}{}]> ",
                venv_prefix, display_path, watcher_suffix, git_suffix
            )
        };

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
                println!("{}", t!("entry.goodbye").green());
                break;
            }
            Err(_) => break,
        };

        let line = line
            .trim()
            .trim_start_matches('\u{feff}')
            .trim_end_matches('\u{feff}');
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
                    // Refresh git status cache so prompt stays accurate
                    crate::utils::git::invalidate();
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
                    if NtcConfig::global_get_dir_view_enabled() {
                        show_tree(&nav, Some(1), false, false, false, false, false, false);
                    }
                    if !summary.is_empty() {
                        summary.print_refresh_box();
                    }
                    // Auto-run trigger alias if configured
                    if let Some(ref alias) = summary.trigger_alias {
                        let expanded = expand_command_line(alias);
                        for cmd in expanded {
                            match execute_command(&cmd, &mut nav, &watcher_handle) {
                                Ok(_) => {}
                                Err(e) => print_error(&t!("entry.watch_trigger", e)),
                            }
                        }
                    }
                }
            }
        }

        if should_exit {
            SessionState::write_global().last_directory = Some(nav.current_path().to_path_buf());
            SessionState::save_global();
            println!("{}", t!("entry.goodbye").green());
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
