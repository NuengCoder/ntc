use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use crate::watcher;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

// ── submodules ──────────────────────────────────────────────────────────
mod alias_commands;
mod arc;
mod autoup;
mod backup;
pub(crate) mod chess_engine;
pub(crate) mod clipboard;
mod config_commands;
mod count;
mod deadlock;
mod downloader;
mod encode_decode;
mod fmt;
mod fmt_doc;
mod fmt_sheet;
mod git;
mod gs_interactive;
mod http_client;
mod ignore;
mod ipcfg;
mod misc;
mod mode;
mod nav;
mod ndb;
mod nms;
mod nsheet;
mod nssh;
mod pathman;
mod qr;
mod qsc;
pub(crate) mod qur;
mod ran_commands;
mod report;
mod rev;
mod scan;
mod scope;
mod scr_commands;
mod serve;
mod serve_chess;
mod serve_http;
mod smtp_client;
mod sort;
mod spec;
mod sysmon;
mod theme;
mod time;
mod tl;
mod tmprt;
mod todo;
mod trim;
mod trim_text;
pub(crate) mod upgrade;
mod wi;

#[cfg(feature = "image2doc")]
mod image;

// ============================================================================
// Command execution
// ============================================================================

/// Execute a full command line, handling pipelines (`|`), logical
/// operators (`&&` / `||`) and redirections (`>` `>>` `<` `<<`). Falls
/// back to a single-command dispatch when no operator is present.
pub(crate) fn execute_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    execute_command_with_status(input, nav, watcher_handle).map(|(exit_shell, _)| exit_shell)
}

/// Execute a full command line and report both whether the shell should exit
/// and whether the final command/pipeline segment succeeded.
pub(crate) fn execute_command_with_status(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<(bool, bool)> {
    if crate::shell::pipeline::has_operators(input) {
        return crate::shell::pipeline::run(input, nav, watcher_handle);
    }
    crate::shell::pipeline::reset_failed();
    let exit_shell = execute_single_command(input, nav, watcher_handle)?;
    let success = !crate::shell::pipeline::take_failed();
    Ok((exit_shell, success))
}

/// Execute a single command (already fully expanded, no operators).
pub(crate) fn execute_single_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

    // `time` owns a full help screen; handle its help forms before generic
    // contextual help prints a shell completion hint for the same suffix.
    if cmd == "time" && matches!(args, "help" | "--help" | "-h") {
        return time::cmd_time(args, nav);
    }

    // Cisco-style contextual `<prefix> --help` at any depth (concise
    // next-step hints, `<cr>` when complete). Runs before dispatch so
    // `ignore dir --help` never creates a dir named `--help`.
    // `-h` is only intercepted here when it is NOT a data flag for the
    // command; flag-heavy commands keep their own `-h` handling.
    {
        let trimmed = input.trim();
        if let Some(last) = trimmed.split_whitespace().last() {
            let last_lower = last.to_lowercase();
            if last_lower == "--help" || last_lower == "-h" {
                let is_long = last_lower == "--help";
                if let Some(prefix) = crate::shell::help::strip_help_suffix(input) {
                    let prefix_trimmed = prefix.trim();
                    let root_of_prefix = prefix_trimmed
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_lowercase();
                    let prefix_len = if prefix_trimmed.is_empty() {
                        0
                    } else {
                        prefix_trimmed.split_whitespace().count()
                    };
                    // Bare `<flag-heavy-cmd> --help/-h` keeps its detailed
                    // built-in help; deeper `<cmd> <sub> --help` uses concise.
                    let flag_heavy = matches!(
                        root_of_prefix.as_str(),
                        "http" | "dl" | "up" | "ping" | "dns" | "smtp" | "serve"
                    );
                    let keep_native = flag_heavy && prefix_len <= 1;
                    // Short `-h` for flag-heavy commands is data/flag: never intercept.
                    let short_blocked = !is_long && flag_heavy;
                    if !short_blocked && !keep_native {
                        if crate::shell::help::print_contextual_help(&prefix) {
                            return Ok(false);
                        }
                    }
                }
            }
        }
    }

    // Legacy sole-arg `<cmd> -h | --help` full-topic fallback (only when
    // contextual help did not handle it).
    if args == "-h" || args == "--help" {
        if let Some(topic) = help_topic_for_command(&cmd) {
            crate::shell::help::print_topic_help(topic);
            return Ok(false);
        }
    }

    // The editor's embedded terminal already runs an ntc shell; starting a
    // second one would stack shells on the same PTY.
    if crate::shell::entry::embedded_in_editor() && is_ntc_shell_invocation(&cmd) {
        print_error(&t!("misc.exit.no_nested_ntc"));
        return Ok(false);
    }

    match cmd.as_str() {
        "cd" => nav::cmd_cd(args, nav),
        "godrive" | "god" => nav::cmd_godrive(args, nav),
        "back" | "b" => nav::cmd_back(args, nav),
        "view" => report::cmd_view(args, nav),
        "viewf" => report::cmd_viewf(args, nav),
        "viewd" => report::cmd_viewd(args, nav),
        "views" => report::cmd_views(args, nav),
        "report" => report::cmd_report(args, nav),
        "txtc" => report::cmd_txtc(args, nav),
        "txtf" => report::cmd_txtf(args, nav),
        "head" => trim::cmd_head(args, nav),
        "count" => count::cmd_count(args, nav),
        "tail" => trim::cmd_tail(args, nav),
        "betw" => trim::cmd_betw(args, nav),
        "trim" => trim_text::cmd_trim(args, nav),
        "config" | "cfg" => config_commands::cmd_config(args, nav),
        "clear" => misc::cmd_clear(args, nav),
        "encode" => encode_decode::cmd_encode(args, nav),
        "decode" => encode_decode::cmd_decode(args, nav),
        "fmt" => fmt::cmd_fmt(args, nav),
        "cvcode" => encode_decode::cmd_cvcode(args, nav),
        "math" => misc::cmd_math(args, nav),
        "mode" => mode::cmd_mode(args, nav),
        "version" => misc::cmd_version(args, nav),
        "where" => nav::cmd_where(args, nav),
        "gos" => nav::cmd_gos(nav),
        "gosc" => nav::cmd_gosc(nav),
        "ral" => alias_commands::cmd_ral(args, nav),
        "ran" => ran_commands::cmd_ran(args, nav),
        "run" | "r" => alias_commands::cmd_run(args, nav),
        "scr" | "script" => scr_commands::cmd_scr(args, nav),
        "showcg" => config_commands::cmd_showcg(args, nav),
        "opencg" | "editcfg" | "editcg" => config_commands::cmd_opencg(args, nav),
        "ne" | "ntceditor" => misc::cmd_ne(args, nav),
        "resetcg" | "reset-config" => config_commands::cmd_resetcg(args, nav),
        "restorecg" | "restore-config" => config_commands::cmd_restorecg(args, nav),
        "local" => config_commands::cmd_local(args, nav),
        "init" => config_commands::cmd_init(args, nav),
        "deinit" => config_commands::cmd_deinit(args, nav),
        "help" => misc::cmd_help(args, nav),
        "http" => http_client::cmd_http(args, nav),
        "tutorial" | "guide" => misc::cmd_tutorial(args, nav),
        "exit" | "quit" | "esc" => misc::cmd_exit(args, nav),
        "ignore" => ignore::cmd_ignore(args, nav),
        "care" => ignore::cmd_care(args, nav),
        "igcared" => ignore::cmd_igcared(args, nav),
        "igcare" => ignore::cmd_igcare(args, nav),
        "size" => misc::cmd_size(args, nav),
        "tp" => misc::cmd_tp(args, nav),
        "tpb" => misc::cmd_tpb(args, nav),
        "ui" => misc::cmd_ui(args, nav),
        "backup" => backup::cmd_backup(args, nav),
        "fs" => misc::cmd_fs(args, nav),
        "dl" => downloader::cmd_dl(args, nav),
        "dns" => downloader::cmd_dns(args, nav),
        "ds" => misc::cmd_ds(args, nav),
        "gs" => misc::cmd_gs(args, nav),
        "qur" => qur::cmd_qur(args, nav),
        "wi" => wi::cmd_wi(args, nav),
        "fgo" => nav::cmd_fgo(args, nav),
        "fsc" => misc::cmd_fsc(args, nav),
        "locate" => misc::cmd_locate(args, nav),
        "mkf" => misc::cmd_mkf(args, nav),
        "mkd" => misc::cmd_mkd(args, nav),
        "rmd" => misc::cmd_rmd(args, nav),
        "rmf" => misc::cmd_rmf(args, nav),
        "rnmf" => misc::cmd_rnmf(args, nav),
        "rnmd" => misc::cmd_rnmd(args, nav),
        "mv" | "move" => misc::cmd_mv(args, nav),
        "cp" => misc::cmd_cp(args, nav),
        "sys" | "top" => sysmon::cmd_sys(args, nav),
        "spec" => spec::cmd_spec(args, nav),
        "sort" => sort::cmd_sort(args, nav),
        "rev" => rev::cmd_rev(args, nav),
        "theme" => theme::cmd_theme(args, nav),
        "up" => downloader::cmd_up(args, nav),
        "ping" => downloader::cmd_ping(args, nav),
        "smtp" => smtp_client::cmd_smtp(args, nav),
        "serve" => serve::cmd_serve(args, nav),
        "py_activate" => misc::cmd_venva(args, nav),
        "py_deactivate" => misc::cmd_venvd(args, nav),
        "showenv" => misc::cmd_show_env(args, nav),
        #[cfg(feature = "image2doc")]
        "image" | "img" => image::cmd_image(args, nav),
        #[cfg(not(feature = "image2doc"))]
        "image" | "img" => {
            print_error(&t!("cmd.image_disabled"));
            Ok(false)
        }
        "ntpkg" => crate::ntpkg::cmd_ntpkg(args, nav),
        "ntpx" => crate::ntpkg::cmd_ntpx(args, nav),
        "clip" => clipboard::cmd_clip(args, nav),
        "qr" => qr::cmd_qr(args, nav),
        "qsc" => qsc::cmd_qsc(args, nav),
        "tl" => tl::cmd_tl(args, nav),
        "usrpath" => pathman::cmd_usrpath(args, nav),
        "syspath" => pathman::cmd_syspath(args, nav),
        "autoup" => autoup::cmd_autoup(args, nav),
        "explorer" | "nfe" => {
            let path = if args.is_empty() {
                None
            } else {
                Some(std::path::Path::new(args))
            };
            crate::nfe::run_explorer(path)?;
            Ok(false)
        }
        "todo" | "td" => todo::cmd_todo(args, nav),
        "arc" => arc::cmd_arc(args, nav),
        "scan" => scan::cmd_scan(args, nav),
        "ipcfg" => ipcfg::cmd_ipcfg(args, nav),
        "say" | "print" => misc::cmd_say(args, nav),
        "pathcp" => misc::cmd_pathcp(args, nav),
        "tmprt" => tmprt::cmd_tmprt(args, nav),
        "time" => time::cmd_time(args, nav),
        "git" => git::cmd_git(args, nav),
        "ndb" => ndb::cmd_ndb(args, nav),
        "nms" => nms::cmd_nms(args, nav),
        "nsheet" => nsheet::cmd_nsheet(args, nav),
        "nssh" => nssh::cmd_nssh(args, nav),
        "deadlock" => deadlock::cmd_deadlock(args, nav),
        "fake" => crate::fake::cmd_fake(args, nav),
        _ => misc::cmd_fallback(input, nav, watcher_handle),
    }
}

/// True when `cmd` names the ntc binary itself (`ntc`, `ntc.exe`, or a
/// path ending in either), as opposed to a command that merely contains
/// the letters (`ntceditor`).
fn is_ntc_shell_invocation(cmd: &str) -> bool {
    let trimmed = cmd.trim().trim_matches('"').replace('\\', "/");
    let last = trimmed
        .rsplit('/')
        .next()
        .unwrap_or(&trimmed)
        .to_lowercase();
    last == "ntc" || last == "ntc.exe"
}

/// Execute a system command
fn execute_system_command(cmd: &str, cwd: &Path) -> Result<bool> {
    let quiet = crate::shell::pipeline::is_quiet();
    if !quiet {
        print_info(&format!("Executing: {}", cmd));
        println!();
    }

    let status = run_system_command(cmd, cwd);
    if !quiet {
        println!();
    }

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                if !quiet {
                    print_success(&t!("cmd.completed"));
                }
            } else {
                // Non-zero exit counts as a failure for `&&` / `||`.
                crate::shell::pipeline::mark_failed();
                match exit_status.code() {
                    Some(code) => print_error(&format!("Command exited with code: {}", code)),
                    None => print_warning(&t!("cmd.terminated")),
                }
            }
            Ok(false)
        }
        Err(e) => {
            crate::shell::pipeline::mark_failed();
            print_error(&format!("Failed to execute command: {}", e));
            Ok(false)
        }
    }
}

static CHILD_PID: AtomicU32 = AtomicU32::new(0);

static SHUTDOWN_FLAGS: Mutex<Vec<Arc<AtomicBool>>> = Mutex::new(Vec::new());

pub fn register_shutdown_flag(flag: Arc<AtomicBool>) {
    if let Ok(mut v) = SHUTDOWN_FLAGS.lock() {
        v.push(flag);
    }
}

pub fn unregister_shutdown_flag(flag: &Arc<AtomicBool>) {
    if let Ok(mut v) = SHUTDOWN_FLAGS.lock() {
        v.retain(|f| !Arc::ptr_eq(f, flag));
    }
}

pub fn init_ctrlc_handler() {
    use std::sync::atomic::AtomicBool;
    static INIT: AtomicBool = AtomicBool::new(false);
    if INIT.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = ctrlc::set_handler(move || {
        let pid = CHILD_PID.load(Ordering::SeqCst);
        if pid > 0 {
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/F", "/T", "/PID", &pid.to_string()])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
            }
            #[cfg(not(windows))]
            {
                let _ = std::process::Command::new("kill")
                    .args(["-TERM", &pid.to_string()])
                    .status();
            }
        }
        if let Ok(flags) = SHUTDOWN_FLAGS.lock() {
            for f in flags.iter() {
                f.store(true, Ordering::SeqCst);
            }
        }
    });
}

pub(super) fn run_system_command(args: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    init_ctrlc_handler();

    // Input installed by a pipeline/`<` redirect (or piped process stdin).
    let input = crate::shell::stdin::take_input();
    let stdin_cfg = if input.is_some() {
        std::process::Stdio::piped()
    } else {
        std::process::Stdio::inherit()
    };

    #[cfg(windows)]
    let mut child = std::process::Command::new("cmd")
        .args(["/C", args])
        .current_dir(cwd)
        .stdin(stdin_cfg)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    #[cfg(not(windows))]
    let mut child = std::process::Command::new("sh")
        .args(["-c", args])
        .current_dir(cwd)
        .stdin(stdin_cfg)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    if let Some(bytes) = input {
        if let Some(mut si) = child.stdin.take() {
            use std::io::Write;
            let _ = si.write_all(&bytes);
            // dropping `si` closes the pipe, signalling EOF to the child
        }
    }

    CHILD_PID.store(child.id(), Ordering::SeqCst);
    let status = child.wait()?;
    CHILD_PID.store(0, Ordering::SeqCst);
    Ok(status)
}

pub(super) fn capture_system_command(args: &str, cwd: &Path) -> Result<std::process::Output> {
    #[cfg(windows)]
    let output = std::process::Command::new("cmd")
        .args(["/C", args])
        .current_dir(cwd)
        .output()?;

    #[cfg(not(windows))]
    let output = std::process::Command::new("sh")
        .args(["-c", args])
        .current_dir(cwd)
        .output()?;

    Ok(output)
}

/// Opt-in registry for universal `<cmd> -h | --help` (sole-arg only).
/// Returns the help topic for commands that treat bare `-h/--help` as
/// "show help". Commands where `-h` is data (gs search text, http header
/// flag, …) or that already handle it themselves (dl/up/serve/…) are
/// deliberately absent so their own handling is preserved.
fn help_topic_for_command(cmd: &str) -> Option<&'static str> {
    Some(match cmd {
        "mv" | "move" | "mkf" | "mkd" | "rmf" | "rmd" | "rnmf" | "rnmd" | "cp" => "fileops",
        "gs" | "fs" | "ds" | "locate" | "fsc" | "qur" => "search",
        "config" | "cfg" | "wi" => "config",
        "ne" | "ntceditor" => "editor",
        "size" => "size",
        "view" | "viewf" | "viewd" | "views" | "txtc" | "txtf" => "view",
        "head" | "tail" | "betw" | "sort" | "rev" | "count" | "trim" => "pipe",
        "report" => "report",
        "tp" | "tpb" => "teleport",
        "theme" => "theme",
        "backup" => "backup",
        "todo" | "td" => "todo",
        "arc" => "arc",
        "scan" => "scan",
        "tl" => "tl",
        "qr" => "qr",
        "math" => "math",
        "fmt" => "fmt",
        "ndb" => "ndb",
        "nms" => "nms",
        "nsheet" => "nsheet",
        "nssh" => "nssh",
        "deadlock" => "deadlock",
        "time" => "time",
        "fake" => "fake",
        "cd" | "back" | "b" | "where" | "gos" | "gosc" | "fgo" => "nav",
        _ => return None,
    })
}

/// ── shared arg parser for fs / ds ─────────────────────────────────────────
/// Accepts:  <pattern> [-d <depth>]   (depth flag must be at the end)
/// Returns:  (pattern, max_depth)
/// Handles double-quoted patterns: fs "my file.rs" -d 3
fn parse_search_args(args: &str) -> (String, usize) {
    let default_depth = Config::global_get_max_depth();

    // Tokenize respecting double quotes
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    // Look for -d <n> at the end: ["pattern", ..., "-d", "3"]
    let n = parts.len();
    if n >= 3 && parts[n - 2] == "-d" {
        if let Ok(depth) = parts[n - 1].parse::<usize>() {
            let pattern = parts[..n - 2].join(" ");
            return (pattern, depth);
        }
    }

    (args.to_string(), default_depth)
}

/// Parsed arguments for `gs`.
pub(crate) struct GsArgs {
    pub pattern: String,
    pub max_depth: usize,
    pub limit: Option<usize>,
    pub interactive: bool,
    /// File-scope globs (`-p` / positional `*.*` / `dir/`).
    pub paths: Vec<String>,
    /// `None` = default order (path, then line); `Some` = explicit `-s`.
    pub sort: Option<GsSort>,
}

/// Sort key for `gs -s|--sort`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GsSort {
    /// Match-line text, case-insensitive (default for bare `-s`).
    Line,
    /// File path, then line number.
    Path,
    /// File size ascending (small → large), tie-break path + line.
    Size,
}

fn tokenize_gs_args(args: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Parse `gs` args in any flag order:
/// `gs <pattern> [-d <n>] [-l <n> | --limit <n> | --limit=<n>] [-i]
///   [-p <glob> | --path <glob> | --path=<glob>]... [-s [line|path|size] | --sort ...]`
/// Positional path-like tokens (`*.*`, `dir/`, existing file/dir) after the
/// first pattern token act as `-p` filters; everything else joins the pattern.
pub(crate) fn parse_gs_args(args: &str) -> GsArgs {
    let default_depth = Config::global_get_max_depth();
    let parts = tokenize_gs_args(args);
    let mut pattern_parts: Vec<String> = Vec::new();
    let mut max_depth = default_depth;
    let mut limit: Option<usize> = None;
    let mut interactive = false;
    let mut paths: Vec<String> = Vec::new();
    let mut sort: Option<GsSort> = None;

    let mut i = 0;
    while i < parts.len() {
        match parts[i].as_str() {
            "-i" => {
                interactive = true;
            }
            "-d" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    max_depth = v;
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            "-l" | "--limit" => {
                if let Some(v) = parts.get(i + 1).and_then(|s| s.parse::<usize>().ok()) {
                    if v > 0 {
                        limit = Some(v);
                    }
                    i += 1;
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--limit=") => {
                if let Some(v) = s["--limit=".len()..].parse::<usize>().ok() {
                    if v > 0 {
                        limit = Some(v);
                    }
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("-p=") || s.starts_with("--path=") => {
                paths.push(
                    s.split_once('=')
                        .map(|(_, v)| v)
                        .unwrap_or_default()
                        .to_string(),
                );
            }
            "-p" | "--path" => {
                if let Some(next) = parts.get(i + 1) {
                    if !next.starts_with('-') {
                        paths.push(next.clone());
                        i += 1;
                    } else {
                        pattern_parts.push(parts[i].clone());
                    }
                } else {
                    pattern_parts.push(parts[i].clone());
                }
            }
            s if s.starts_with("--sort=") => match parse_gs_sort_value(&s["--sort=".len()..]) {
                Some(k) => sort = Some(k),
                None => pattern_parts.push(parts[i].clone()),
            },
            "-s" | "--sort" => {
                if let Some(next) = parts.get(i + 1) {
                    match parse_gs_sort_value(next) {
                        Some(k) => {
                            sort = Some(k);
                            i += 1;
                        }
                        // A value other than line|path|size remains part of
                        // the search pattern; bare -s always means line sort.
                        None => sort = Some(GsSort::Line),
                    }
                } else {
                    sort = Some(GsSort::Line);
                }
            }
            other => {
                // Positional `gs <pattern> <*.* + dir/>`: tokens after the
                // pattern head that look like paths become `-p` filters.
                if !pattern_parts.is_empty() && gs_looks_like_path(other) {
                    paths.push(other.to_string());
                } else {
                    pattern_parts.push(other.to_string());
                }
            }
        }
        i += 1;
    }

    GsArgs {
        pattern: pattern_parts.join(" "),
        max_depth,
        limit,
        interactive,
        paths,
        sort,
    }
}

/// `line|path|size` (case-insensitive).
fn parse_gs_sort_value(v: &str) -> Option<GsSort> {
    match v.to_lowercase().as_str() {
        "line" => Some(GsSort::Line),
        "path" => Some(GsSort::Path),
        "size" => Some(GsSort::Size),
        _ => None,
    }
}

/// Heuristic: `*.rs`, `src/`, `app.log`, existing file/dir.
fn gs_looks_like_path(tok: &str) -> bool {
    // An OR alternative (`a\|b`, `a|b`) belongs to the pattern even though the
    // escaped form carries a backslash — never treat it as an `-p` filter.
    if tok.contains('|') && !tok.contains('/') && !tok.contains('*') && !tok.contains('?') {
        return false;
    }
    if tok.contains('*') || tok.contains('?') || tok.contains('/') || tok.contains('\\') {
        return true;
    }
    if tok.starts_with('.') && tok.len() > 1 && !tok.contains(' ') {
        return true;
    }
    if !tok.contains(' ') {
        if let Some(dot) = tok.rfind('.') {
            let ext = &tok[dot + 1..];
            if !ext.is_empty() && ext.len() <= 5 && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
                let stem = &tok[..dot];
                if !stem.is_empty()
                    && stem
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
                {
                    return true;
                }
            }
        }
    }
    let p = std::path::Path::new(tok);
    p.is_file() || p.is_dir()
}

fn list_supported_files(nav: &Navigator) -> Result<Vec<(String, PathBuf)>> {
    list_supported_files_in(nav.current_path())
}

/// Supported files directly inside `dir` (not recursive), sorted by name.
fn list_supported_files_in(dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let fmt_cfg = FormatConfig::from_both();
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if crate::filetype::is_supported_format_with_config(&path, &fmt_cfg) {
                        files.push((name.to_string(), path));
                    }
                }
            }
        }
    }

    files.sort_by_cached_key(|a| a.0.to_lowercase());
    Ok(files)
}

fn show_file_selection_menu(
    _nav: &Navigator,
    files: Vec<(String, PathBuf)>,
    copy_mode: bool,
) -> Result<()> {
    show_file_selection_menu_range(_nav, files, copy_mode, None, None)
}

fn show_file_selection_menu_range(
    _nav: &Navigator,
    files: Vec<(String, PathBuf)>,
    copy_mode: bool,
    range: Option<crate::output::FileLineRange>,
    lines: Option<bool>,
) -> Result<()> {
    let title = if copy_mode {
        "Select a file to COPY to clipboard"
    } else {
        "Select a file to DISPLAY"
    };

    let items: Vec<MenuItem> = files
        .iter()
        .map(|(name, path)| MenuItem::with_detail(name, get_file_size(path)))
        .collect();

    let menu = InteractiveMenu::new(title)
        .items(items)
        .prompt(format!("Enter number (1-{}) or 0: ", files.len()));

    if let Some(idx) = menu.run()? {
        let (name, path) = &files[idx];
        if copy_mode {
            print_info(&format!("Copying '{}' to clipboard...", name));
            let show_lines = Config::global_get_show_line_numbers();
            let content = crate::output::cat_file_with_line_numbers(path, show_lines)?;
            crate::output::copy_to_clipboard(&content, "TXT")?;
            print_success(&format!("File '{}' copied to clipboard!", name));
        } else {
            let show_lines = lines.unwrap_or_else(Config::global_get_show_line_numbers);
            crate::output::cat_file_with_syntax_range(path, show_lines, range)?;
        }
    }

    Ok(())
}

fn get_file_size(path: &Path) -> String {
    if let Ok(metadata) = std::fs::metadata(path) {
        let size = metadata.len();
        if size < 1024 {
            format!("({} B)", size)
        } else if size < 1024 * 1024 {
            format!("({:.1} KB)", size as f64 / 1024.0)
        } else {
            format!("({:.1} MB)", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gs_parses_path_and_sort_flags() {
        let a = parse_gs_args("main -p \"*.rs\" -s size");
        assert_eq!(a.pattern, "main");
        assert_eq!(a.paths, vec!["*.rs".to_string()]);
        assert_eq!(a.sort, Some(GsSort::Size));

        let a = parse_gs_args("main -p=*.rs --path=src/*.rs");
        assert_eq!(a.paths, vec!["*.rs".to_string(), "src/*.rs".to_string()]);

        let b = parse_gs_args("main -s");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.sort, Some(GsSort::Line));

        let b = parse_gs_args("-s main");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.sort, Some(GsSort::Line));

        let c = parse_gs_args("main -s path");
        assert_eq!(c.sort, Some(GsSort::Path));

        let c = parse_gs_args("--sort=size main");
        assert_eq!(c.pattern, "main");
        assert_eq!(c.sort, Some(GsSort::Size));

        let c = parse_gs_args("main -s yesterday");
        assert_eq!(c.pattern, "main yesterday");
        assert_eq!(c.sort, Some(GsSort::Line));

        let d = parse_gs_args("main -p *.rs -p src/");
        assert_eq!(d.paths, vec!["*.rs".to_string(), "src/".to_string()]);

        // Positional `gs <pattern> <*.* + dir/>` == `gs <pattern> -p ...`.
        let e = parse_gs_args("main *.rs");
        assert_eq!(e.pattern, "main");
        assert_eq!(e.paths, vec!["*.rs".to_string()]);

        let e = parse_gs_args("\"foo bar\"");
        assert_eq!(e.pattern, "foo bar");

        // Multi-word patterns still join; flags keep working in any order.
        let f = parse_gs_args("-s size -d 3 foo bar");
        assert_eq!(f.pattern, "foo bar");
        assert_eq!(f.sort, Some(GsSort::Size));
        assert_eq!(f.max_depth, 3);
        assert!(f.paths.is_empty());
    }

    #[test]
    fn help_registry_covers_safe_commands() {
        assert_eq!(help_topic_for_command("mv"), Some("fileops"));
        assert_eq!(help_topic_for_command("move"), Some("fileops"));
        assert_eq!(help_topic_for_command("cp"), Some("fileops"));
        assert_eq!(help_topic_for_command("gs"), Some("search"));
        assert_eq!(help_topic_for_command("wi"), Some("config"));
        assert_eq!(help_topic_for_command("ne"), Some("editor"));
        assert_eq!(help_topic_for_command("time"), Some("time"));
    }

    #[test]
    fn help_registry_skips_data_flag_commands() {
        // `-h` is data or self-handled here: must NOT be intercepted.
        assert_eq!(help_topic_for_command("http"), None);
        assert_eq!(help_topic_for_command("serve"), None);
        assert_eq!(help_topic_for_command("dl"), None);
        assert_eq!(help_topic_for_command("tmprt"), None);
        assert_eq!(help_topic_for_command("nope"), None);
    }

    #[test]
    fn parse_gs_args_accepts_limit_flags() {
        let a = parse_gs_args("main -d 5 -l 10");
        assert_eq!(a.pattern, "main");
        assert_eq!(a.max_depth, 5);
        assert_eq!(a.limit, Some(10));
        let b = parse_gs_args("--limit=7 main -i");
        assert_eq!(b.pattern, "main");
        assert_eq!(b.limit, Some(7));
        assert!(b.interactive);
    }

    #[test]
    fn parse_gs_args_keeps_or_alternatives_in_the_pattern() {
        let a = parse_gs_args("error\\|warning -d 2");
        assert_eq!(a.pattern, "error\\|warning");
        assert_eq!(a.max_depth, 2);

        // A later OR alternative stays pattern text even though it carries a
        // backslash, which otherwise marks a path filter.
        let b = parse_gs_args("error warning\\|fatal");
        assert_eq!(b.pattern, "error warning\\|fatal");
        assert!(b.paths.is_empty());

        // Real path filters are still recognised after an OR pattern.
        let c = parse_gs_args("error\\|warning -p \"*.rs\"");
        assert_eq!(c.pattern, "error\\|warning");
        assert_eq!(c.paths, vec!["*.rs".to_string()]);
    }

    #[test]
    fn list_supported_files_in_filters_one_level_and_sorts() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("docs/b.txt", "b"),
                ("docs/a.rs", "a"),
                ("docs/image.png", "\0fake"),
                ("docs/nested/deep.txt", "deep"),
            ],
            &["docs", "docs/nested"],
        );
        let files = list_supported_files_in(Path::new("docs")).unwrap();
        let names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, vec!["a.rs", "b.txt"]);
    }
}
