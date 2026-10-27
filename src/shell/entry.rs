use crate::config::Config as NtcConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::session::SessionState;
use crate::shell::alias::{expand_command_line, expand_command_line_interactive};
use crate::shell::commands::execute_command;
use crate::shell::helpers::{show_nav_view, take_watch_refresh_suppressed};
use crate::t;
use crate::watcher;

use anyhow::Result;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::FileHistory;
use rustyline::validate::Validator;
use rustyline::{Cmd, ConditionalEventHandler, Event, EventContext, RepeatCount};
use rustyline::{Config as RlConfig, Context, Editor};
use rustyline::{EventHandler, KeyCode, KeyEvent, Modifiers};
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

    fn tab_handler(&self, delta: isize) -> super::hinter::TabCycleHandler {
        self.hinter.tab_handler(delta)
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

/// End / Ctrl+E: accept the ghost hint when one is showing at the caret.
/// Mirrors rustyline's own Right-arrow rule (`has_hint() && cursor at end`);
/// returning `None` keeps the default key behaviour (both move the caret to
/// the end of the line) when there is nothing to complete.
struct AcceptHintOrEnd;

impl ConditionalEventHandler for AcceptHintOrEnd {
    fn handle(
        &self,
        _evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        ctx: &EventContext,
    ) -> Option<Cmd> {
        if ctx.has_hint() && ctx.pos() == ctx.line().len() {
            Some(Cmd::CompleteHint)
        } else {
            None
        }
    }
}

/// Read the bodies of every `<<` heredoc declared on `line`, prompting with
/// a `heredoc>` continuation until each delimiter line is seen. Returns
/// `None` when input is aborted (EOF / Ctrl-C), in which case nothing runs.
fn read_heredocs_interactive(rl: &mut Editor<NtcHelper, FileHistory>, line: &str) -> Option<()> {
    let delims = crate::shell::pipeline::scan_heredoc_delims(line);
    if delims.is_empty() {
        crate::shell::pipeline::set_heredoc_bodies(Vec::new());
        return Some(());
    }
    let mut bodies: Vec<Vec<u8>> = Vec::new();
    for delim in delims {
        let mut body = String::new();
        loop {
            match rl.readline("heredoc> ") {
                Ok(l) => {
                    let l = l.trim_end_matches(['\r', '\n']);
                    if l == delim {
                        break;
                    }
                    body.push_str(l);
                    body.push('\n');
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    crate::shell::pipeline::set_heredoc_bodies(Vec::new());
                    return None;
                }
                Err(_) => {
                    crate::shell::pipeline::set_heredoc_bodies(Vec::new());
                    return None;
                }
            }
        }
        bodies.push(body.into_bytes());
    }
    crate::shell::pipeline::set_heredoc_bodies(bodies);
    Some(())
}

// ============================================================================
// Main shell entry points
// ============================================================================

/// True when this shell runs inside the editor's embedded terminal
/// (spawned with `NTC_IN_EDITOR=1`). The embedded shell keeps the PTY
/// working directory (the editor's CWD) and must not restore or save
/// the outer session's directory.
pub fn embedded_in_editor() -> bool {
    std::env::var("NTC_IN_EDITOR").is_ok()
}

pub fn run_shell() -> Result<()> {
    let mut nav = Navigator::new()?;
    // Embedded shell: keep the spawn cwd (editor's current cwd).
    // Standalone shell: restore the last session directory as before.
    if !embedded_in_editor() {
        let last_dir = SessionState::read_global().last_directory.clone();
        if let Some(ref last_dir) = last_dir {
            if last_dir.exists() {
                let _ = nav.go_to(last_dir);
            }
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
        if NtcConfig::global_get_intro_view_enabled() {
            print_info(&t!(
                "entry.ntpkg_active",
                count,
                if count == 1 { "" } else { "s" }
            ));
        }
    }

    let mut rl = Editor::<NtcHelper, FileHistory>::with_config(RlConfig::default())
        .map_err(|e| anyhow::anyhow!("Failed to create line editor: {}", e))?;
    let helper = NtcHelper::new();
    let tab_next = helper.tab_handler(1);
    let tab_prev = helper.tab_handler(-1);
    rl.set_helper(Some(helper));
    // Tab / Shift-Tab cycle ghost-text candidates in place (no newline);
    // Right-arrow (rustyline default) accepts the ghost.
    rl.bind_sequence(
        KeyEvent(KeyCode::Tab, Modifiers::NONE),
        EventHandler::Conditional(Box::new(tab_next)),
    );
    rl.bind_sequence(
        KeyEvent(KeyCode::BackTab, Modifiers::NONE),
        EventHandler::Conditional(Box::new(tab_prev)),
    );
    // End / Ctrl+E accept the ghost hint, like the right arrow does.
    let accept_hint = || EventHandler::Conditional(Box::new(AcceptHintOrEnd));
    rl.bind_sequence(KeyEvent(KeyCode::End, Modifiers::NONE), accept_hint());
    rl.bind_sequence(KeyEvent(KeyCode::Char('e'), Modifiers::CTRL), accept_hint());

    if let Some(history_path) = NtcConfig::read_global().resolve_history_path() {
        let _ = rl.load_history(&history_path);
    }

    let mut watcher_handle: Option<Arc<watcher::WatcherHandle>> = None;
    if NtcConfig::global_get_file_watcher_enabled() {
        match watcher::start_watcher(nav.current_path()) {
            Ok(w) => {
                let wh = Arc::new(w);
                watcher_handle = Some(wh);
                if NtcConfig::global_get_intro_view_enabled() {
                    print_info(&t!("entry.watcher_started"));
                }
            }
            Err(e) => print_warning(&t!("entry.watcher_failed", e)),
        }
    }

    let show_header =
        NtcConfig::global_get_dir_view_enabled() || NtcConfig::global_get_file_view_enabled();
    if show_header {
        println!();
        println!(
            "{}",
            crate::utils::terminal::asciify(
                "╔══════════════════════════════════════════════════════════════════╗"
            )
        );
        println!(
            "{}",
            crate::utils::theme::ThemeColor::Cyan.paint_ansi_bold(&format!(
                "{:^66}",
                t!("entry.welcome_fmt", env!("CARGO_PKG_VERSION"))
            ))
        );
        println!(
            "{}",
            crate::utils::terminal::asciify(
                "╚══════════════════════════════════════════════════════════════════╝"
            )
        );
        println!();
        println!(
            "{}",
            crate::utils::theme::ThemeColor::BrightBlack.paint_ansi(&t!("entry.startup_hint"))
        );
    }
    // Daily new-release notice (max 1x/24h, background thread, silent on
    // failure): needs internet + GitHub tag newer than this build.
    crate::shell::update_check::spawn_update_check();
    show_nav_view(&nav);

    loop {
        // Check if theme changed
        if crate::utils::theme::ThemeManager::take_theme_changed() {
            // Clear screen and redraw tree with new colors
            crate::navigator::clear_screen();
            show_nav_view(&nav);
        }
        if let Some(ref wh) = watcher_handle {
            if let Some(summary) = wh.poll() {
                if !summary.is_empty() {
                    summary.print_change_box();
                }
                show_nav_view(&nav);
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
                println!(
                    "{}",
                    crate::utils::theme::ThemeColor::Green.paint_ansi(&t!("entry.goodbye"))
                );
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

        // Expand all aliases in the entire line (interactive `--select-*` menus included)
        // Heredocs (`<<EOF`) are read from the terminal before expansion.
        if crate::shell::pipeline::has_heredoc(line) {
            if read_heredocs_interactive(&mut rl, line).is_none() {
                continue;
            }
        } else {
            crate::shell::pipeline::set_heredoc_bodies(Vec::new());
        }

        let expanded = match expand_command_line_interactive(line, &mut nav)? {
            None => continue,
            Some(s) => s,
        };
        if expanded.trim().is_empty() {
            continue;
        }

        // Embedded terminal: flag the command as running so the editor's
        // terminal keeps its panel on Esc until the command finishes
        // (see editor/terminal.rs). Harmless in a normal terminal.
        let in_editor = embedded_in_editor();
        if in_editor {
            use std::io::Write;
            print!("\u{1b}]1337;ntc-busy\u{7}");
            let _ = std::io::stdout().flush();
        }

        // Execute the (possibly piped / redirected) command line.
        // Drop any stale suppression flag left by an earlier command line.
        let _ = take_watch_refresh_suppressed();
        let mut should_exit = false;
        match execute_command(&expanded, &mut nav, &watcher_handle) {
            Ok(exit) => {
                // Keep watcher in sync after the command (cheap no-op if same path)
                if let Some(ref wh) = watcher_handle {
                    let _ = wh.update_path(nav.current_path());
                }
                // Refresh git status cache so prompt stays accurate
                crate::utils::git::invalidate();
                should_exit = exit;
            }
            Err(e) => {
                print_error(&format!("{}", e));
            }
        }
        if in_editor {
            use std::io::Write;
            print!("\u{1b}]1337;ntc-idle\u{7}");
            let _ = std::io::stdout().flush();
        }

        // `clear` already redrew the view: swallow any pending watcher events
        // so no change box, second tree, or trigger alias follows it.
        let suppress_watch = take_watch_refresh_suppressed();

        // After commands, check for any watcher-triggered changes that arrived during execution
        if !should_exit {
            if let Some(ref wh) = watcher_handle {
                let pending = wh.poll();
                if !suppress_watch {
                    if let Some(summary) = pending {
                        if !summary.is_empty() {
                            summary.print_change_box();
                        }
                        show_nav_view(&nav);
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
        }

        if should_exit {
            // Embedded shell: never overwrite the outer session's directory.
            if !embedded_in_editor() {
                SessionState::write_global().last_directory =
                    Some(nav.current_path().to_path_buf());
                SessionState::save_global();
            }
            println!(
                "{}",
                crate::utils::theme::ThemeColor::Green.paint_ansi(&t!("entry.goodbye"))
            );
            break;
        }
    }

    // Also save on EOF (skipped when embedded in the editor)
    if !embedded_in_editor() {
        SessionState::write_global().last_directory = Some(nav.current_path().to_path_buf());
        SessionState::save_global();
    }

    if NtcConfig::global_get_history_enabled() {
        if let Some(history_path) = NtcConfig::read_global().resolve_history_path() {
            let _ = rl.save_history(&history_path);
        }
    }

    Ok(())
}
