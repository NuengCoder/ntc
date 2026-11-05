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
        // Idle-mode hints on an empty line are teaching text: End / Ctrl+E
        // must move the caret, never insert the suggestion.
        if ctx.line().is_empty() {
            return None;
        }
        if ctx.has_hint() && ctx.pos() == ctx.line().len() {
            Some(Cmd::CompleteHint)
        } else {
            None
        }
    }
}

/// Right-arrow on an **empty** line must not accept the idle-mode hint.
/// Everywhere else it keeps rustyline's default (complete the hint when the
/// caret is at the end, otherwise move right).
struct RightIgnoresIdleHint;

impl ConditionalEventHandler for RightIgnoresIdleHint {
    fn handle(
        &self,
        _evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        ctx: &EventContext,
    ) -> Option<Cmd> {
        if ctx.line().is_empty() {
            Some(Cmd::Noop)
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
            let readline = rl.readline("heredoc> ");
            crate::shell::idle::prompt_ended();
            match readline {
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

/// Which foreign shell (if any) currently owns the prompt.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ForeignMode {
    None,
    Ps,
    Bash,
    Zsh,
}

/// Prompt fragment for a host mode: the last path component of the host cwd,
/// falling back to ntc's own directory name when the host has not answered.
fn host_display(host_dir: &str, fallback: &str) -> String {
    if host_dir.is_empty() {
        return fallback.to_string();
    }
    // The directory comes from a foreign shell, so its separator need not be
    // this platform's (`C:\Users\me\proj` for `ntc ps`, POSIX paths for bash).
    let last = host_dir
        .rsplit(|c| c == '/' || c == '\\')
        .find(|part| !part.is_empty());
    match last {
        // `C:` alone is a drive root, not a directory name.
        Some(name) if !(name.len() == 2 && name.ends_with(':')) => name.to_string(),
        _ => fallback.to_string(),
    }
}

/// Keep ntc's directory (and therefore the tree view) in step with a `cd`
/// performed inside the foreign shell. Failures are silent: a UNC or removed
/// path must not break the prompt.
fn sync_host_dir(nav: &mut Navigator, host_dir: &mut String, cwd: Option<String>) {
    let Some(cwd) = cwd else {
        return;
    };
    if cwd.is_empty() {
        return;
    }
    *host_dir = cwd.clone();
    let path = std::path::PathBuf::from(&cwd);
    if path.is_dir() && nav.current_path() != path.as_path() {
        if nav.go_to(&path).is_ok() {
            // Mirror the built-in `cd`: redraw so the tree matches the prompt.
            show_nav_view(nav);
        }
    }
}

/// Prompt shown while a foreign host owns the shell: `ntc ps [src]>` on
/// Windows, `ntc arch [src]>` (the distro label) on Linux.
fn host_prompt(label: &str, host_dir: &str, fallback: &str) -> String {
    format!("ntc {} [{}]> ", label, host_display(host_dir, fallback))
}

/// True for the bare launcher spellings that should enter `ntc ps`.
/// Arguments (`powershell -File x.ps1`) keep the ordinary one-shot path.
fn integration_ps_entry(line: &str) -> bool {
    if !crate::shell::ext_complete::integration_enabled() {
        return false;
    }
    if !crate::shell::ext_complete::platform::is_windows() {
        return false;
    }
    let mut parts = line.split_whitespace();
    let Some(first) = parts.next() else {
        return false;
    };
    if parts.next().is_some() {
        return false;
    }
    crate::shell::ext_complete::is_powershell(first)
}

/// True for a line that ends the hosted shell (`exit`, `exit 3`, `exec ls`).
fn exits_shell(line: &str) -> bool {
    matches!(
        line.split_whitespace().next().unwrap_or(""),
        "exit" | "exec"
    )
}

/// True for a bare `zsh` on macOS: it enters the integrated mode
/// (`ntc zsh [dir]>`) instead of spawning a nested shell. `zsh -c …` and
/// `zsh script.zsh` keep the ordinary one-shot path.
fn integration_zsh_entry(line: &str) -> bool {
    if !crate::shell::ext_complete::integration_enabled() {
        return false;
    }
    if !crate::shell::ext_complete::platform::is_macos() {
        return false;
    }
    let mut parts = line.split_whitespace();
    let Some(first) = parts.next() else {
        return false;
    };
    if parts.next().is_some() {
        return false;
    }
    crate::shell::ext_complete::is_zsh(first)
}

/// True for a bare `bash` on desktop Linux or Termux: it enters the
/// integrated mode (`ntc <distro> [dir]>`, `ntc termux [dir]>`) instead of
/// spawning a nested shell. `bash -c …` and `bash script.sh` keep the
/// ordinary one-shot path.
fn integration_bash_entry(line: &str) -> bool {
    if !crate::shell::ext_complete::integration_enabled() {
        return false;
    }
    if !crate::shell::ext_complete::platform::is_bash_platform() {
        return false;
    }
    let mut parts = line.split_whitespace();
    let Some(first) = parts.next() else {
        return false;
    };
    if parts.next().is_some() {
        return false;
    }
    crate::shell::ext_complete::is_bash(first)
}

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
    rl.bind_sequence(
        KeyEvent(KeyCode::Right, Modifiers::NONE),
        EventHandler::Conditional(Box::new(RightIgnoresIdleHint)),
    );

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

    let mut mode = ForeignMode::None;
    let mut host_dir = String::new();

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

        // Build the prompt. Host modes keep ntc's line editor in charge while
        // a foreign shell executes the lines.
        let tutorial_prefix = crate::tutorial::prompt_prefix();
        let prompt = match mode {
            ForeignMode::Ps => host_prompt("ps", &host_dir, &display_path),
            ForeignMode::Bash => host_prompt(
                crate::shell::ext_complete::platform::prompt_label(),
                &host_dir,
                &display_path,
            ),
            ForeignMode::Zsh => host_prompt("zsh", &host_dir, &display_path),
            ForeignMode::None if venv_prefix.is_empty() => {
                format!(
                    "{tutorial_prefix}ntc [{}{}{}]> ",
                    display_path, watcher_suffix, git_suffix
                )
            }
            ForeignMode::None => {
                format!(
                    "{}{tutorial_prefix}ntc [{}{}{}]> ",
                    venv_prefix, display_path, watcher_suffix, git_suffix
                )
            }
        };

        // A new prompt shows the next idle-mode suggestion.
        if let Some(helper) = rl.helper_mut() {
            helper.hinter.next_idle_hint();
        }
        let readline = rl.readline(&prompt);
        // The idle ghost is drawn outside rustyline; stop the animator (and
        // forget the ghost) as soon as the prompt is done, before running the
        // command or printing "^C".
        crate::shell::idle::prompt_ended();
        let line = match readline {
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

        // Raw line before alias expansion: `tutorial` lessons check it so
        // running a freshly created alias still counts.
        crate::tutorial::note_input(line);

        // ── ntc ps mode: PowerShell runs the line, ntc keeps the editor ──
        if mode == ForeignMode::Ps {
            if line.eq_ignore_ascii_case("exit") {
                mode = ForeignMode::None;
                crate::shell::ext_complete::leave_ps_mode();
                crate::shell::ext_complete::clear_line_cache();
                continue;
            }
            // `Clear-Host` writes to the console buffer the host does not
            // own; ntc clears its own screen instead.
            if line.eq_ignore_ascii_case("cls") || line.eq_ignore_ascii_case("clear") {
                crate::navigator::clear_screen();
                continue;
            }
            match crate::shell::ext_complete::run_line(line) {
                Ok(code) if code != 0 => {
                    crate::shell::pipeline::mark_failed();
                }
                Ok(_) => {}
                Err(e) => print_error(&format!("{}", e)),
            }
            sync_host_dir(
                &mut nav,
                &mut host_dir,
                crate::shell::ext_complete::ps_cwd(),
            );
            // Variables and cwd may have moved: never reuse the old answers.
            crate::shell::ext_complete::clear_line_cache();
            crate::utils::git::invalidate();
            continue;
        }

        // ── integrated bash mode: `ntc <distro> [dir]>` ──
        if mode == ForeignMode::Bash {
            if line.eq_ignore_ascii_case("exit") {
                mode = ForeignMode::None;
                crate::shell::ext_complete::leave_bash_mode();
                crate::shell::ext_complete::clear_line_cache();
                continue;
            }
            if line.eq_ignore_ascii_case("clear") {
                crate::navigator::clear_screen();
                continue;
            }
            match crate::shell::ext_complete::bash_run_line(line) {
                Ok(code) if code != 0 => {
                    crate::shell::pipeline::mark_failed();
                }
                Ok(_) => {}
                Err(e) => {
                    // A dead host means `exit N` / `exec …` ran, or Ctrl-C
                    // dropped it. `exit` returns to ntc; a killed host keeps
                    // the mode and the next line starts a fresh bash.
                    if !crate::shell::ext_complete::bash_host_alive() {
                        if exits_shell(line) {
                            crate::shell::ext_complete::leave_bash_mode();
                            mode = ForeignMode::None;
                        } else {
                            print_warning(&format!("{}", e));
                        }
                        crate::shell::ext_complete::clear_line_cache();
                        continue;
                    }
                    print_error(&format!("{}", e));
                }
            }
            sync_host_dir(
                &mut nav,
                &mut host_dir,
                crate::shell::ext_complete::bash_cwd(),
            );
            crate::shell::ext_complete::clear_line_cache();
            crate::utils::git::invalidate();
            continue;
        }

        // ── integrated zsh mode: `ntc zsh [dir]>` on macOS ──
        if mode == ForeignMode::Zsh {
            if line.eq_ignore_ascii_case("exit") {
                mode = ForeignMode::None;
                crate::shell::ext_complete::leave_zsh_mode();
                crate::shell::ext_complete::clear_line_cache();
                continue;
            }
            if line.eq_ignore_ascii_case("clear") {
                crate::navigator::clear_screen();
                continue;
            }
            match crate::shell::ext_complete::zsh_run_line(line) {
                Ok(code) if code != 0 => {
                    crate::shell::pipeline::mark_failed();
                }
                Ok(_) => {}
                Err(e) => {
                    // Same contract as bash: a killed host keeps the mode and
                    // the next line starts a fresh zsh; `exit` returns to ntc.
                    if !crate::shell::ext_complete::zsh_host_alive() {
                        if exits_shell(line) {
                            crate::shell::ext_complete::leave_zsh_mode();
                            mode = ForeignMode::None;
                        } else {
                            print_warning(&format!("{}", e));
                        }
                        crate::shell::ext_complete::clear_line_cache();
                        continue;
                    }
                    print_error(&format!("{}", e));
                }
            }
            sync_host_dir(
                &mut nav,
                &mut host_dir,
                crate::shell::ext_complete::zsh_cwd(),
            );
            crate::shell::ext_complete::clear_line_cache();
            crate::utils::git::invalidate();
            continue;
        }

        // `powershell` / `pwsh` / `powershell.exe` enters integrated mode
        // instead of spawning a nested shell (only with integration_shell on).
        if integration_ps_entry(line) {
            // Wire Ctrl-C before the host warms up so a stuck start can be
            // interrupted too.
            crate::shell::commands::init_ctrlc_handler();
            match crate::shell::ext_complete::enter_ps_mode() {
                Ok(()) => {
                    crate::shell::ext_complete::clear_line_cache();
                    mode = ForeignMode::Ps;
                    host_dir.clear();
                    sync_host_dir(
                        &mut nav,
                        &mut host_dir,
                        crate::shell::ext_complete::ps_cwd(),
                    );
                }
                Err(e) => print_warning(&format!("{}", e)),
            }
            continue;
        }

        // `bash` on desktop Linux enters `ntc <distro> [dir]>` the same way.
        if integration_bash_entry(line) {
            crate::shell::commands::init_ctrlc_handler();
            match crate::shell::ext_complete::enter_bash_mode() {
                Ok(()) => {
                    crate::shell::ext_complete::clear_line_cache();
                    mode = ForeignMode::Bash;
                    host_dir.clear();
                    sync_host_dir(
                        &mut nav,
                        &mut host_dir,
                        crate::shell::ext_complete::bash_cwd(),
                    );
                }
                Err(e) => print_warning(&format!("{}", e)),
            }
            continue;
        }

        // `zsh` on macOS enters `ntc zsh [dir]>`.
        if integration_zsh_entry(line) {
            crate::shell::commands::init_ctrlc_handler();
            match crate::shell::ext_complete::enter_zsh_mode() {
                Ok(()) => {
                    crate::shell::ext_complete::clear_line_cache();
                    mode = ForeignMode::Zsh;
                    host_dir.clear();
                    sync_host_dir(
                        &mut nav,
                        &mut host_dir,
                        crate::shell::ext_complete::zsh_cwd(),
                    );
                }
                Err(e) => print_warning(&format!("{}", e)),
            }
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

        // Tutorial progress: check the goal after the command has run (so a
        // `cd` has already moved us). Never reveals the answer.
        if !should_exit {
            crate::tutorial::after_command(&nav);
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
    crate::shell::ext_complete::shutdown_all();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_prompt_shows_the_label_and_last_directory() {
        // `bash` + Enter on Linux: the distro label replaces `ntc`.
        assert_eq!(
            host_prompt("arch", "/home/me/src", "ntc"),
            "ntc arch [src]> "
        );
        assert_eq!(host_prompt("ubuntu", "/tmp", "ntc"), "ntc ubuntu [tmp]> ");
        // PowerShell keeps its fixed label.
        assert_eq!(
            host_prompt("ps", "C:\\Users\\me\\proj", "ntc"),
            "ntc ps [proj]> "
        );
    }

    #[test]
    fn exit_lines_leave_the_hosted_shell() {
        assert!(exits_shell("exit"));
        assert!(exits_shell("exit 3"));
        assert!(exits_shell("exec ls"));
        assert!(!exits_shell("exitcode"));
        assert!(!exits_shell("echo exit"));
    }

    #[test]
    fn host_display_falls_back_to_ntc_directory() {
        assert_eq!(host_display("", "src"), "src");
        assert_eq!(host_display("/home/me/src", "ntc"), "src");
    }
}
