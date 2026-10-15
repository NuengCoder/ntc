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
pub(crate) mod clipboard;
mod config_commands;
mod downloader;
mod encode_decode;
mod fmt;
mod git;
mod gs_interactive;
mod http_client;
mod ignore;
mod ipcfg;
mod misc;
mod mode;
mod nav;
mod ndb;
mod nsheet;
mod nssh;
mod pathman;
mod qr;
mod qsc;
mod qur;
mod ran_commands;
mod report;
mod scan;
mod scr_commands;
mod serve;
mod serve_http;
pub(crate) mod chess_engine;
mod serve_chess;
mod smtp_client;
mod spec;
mod sysmon;
mod theme;
mod tl;
mod tmprt;
mod todo;
pub(crate) mod upgrade;
mod wi;

#[cfg(feature = "image2doc")]
mod image;

// ============================================================================
// Command execution
// ============================================================================

/// Execute a single command (already fully expanded)
pub(crate) fn execute_command(
    input: &str,
    nav: &mut Navigator,
    watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").trim();

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

    match cmd.as_str() {
        "cd" => nav::cmd_cd(args, nav),
        "godrive" | "god" => nav::cmd_godrive(args, nav),
        "back" | "b" => nav::cmd_back(args, nav),
        "view" => report::cmd_view(args, nav),
        "report" => report::cmd_report(args, nav),
        "txtc" => report::cmd_txtc(args, nav),
        "txtf" => report::cmd_txtf(args, nav),
        "config" => config_commands::cmd_config(args, nav),
        "clear" => misc::cmd_clear(args, nav),
        "encode" => encode_decode::cmd_encode(args, nav),
        "decode" => encode_decode::cmd_decode(args, nav),
        "fmt" => fmt::cmd_fmt(args, nav),
        "cvcode" => encode_decode::cmd_cvcode(args, nav),
        "math" => misc::cmd_math(args, nav),
        "mode" => mode::cmd_mode(args, nav),
        "version" => misc::cmd_version(args, nav),
        "where" => nav::cmd_where(nav),
        "gos" => nav::cmd_gos(nav),
        "gosc" => nav::cmd_gosc(nav),
        "ral" => alias_commands::cmd_ral(args, nav),
        "ran" => ran_commands::cmd_ran(args, nav),
        "run" | "r" => alias_commands::cmd_run(args, nav),
        "scr" | "script" => scr_commands::cmd_scr(args, nav),
        "showcg" => config_commands::cmd_showcg(args, nav),
        "opencg" | "editcfg" => config_commands::cmd_opencg(args, nav),
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
        "git" => git::cmd_git(args, nav),
        "ndb" => ndb::cmd_ndb(args, nav),
        "nsheet" => nsheet::cmd_nsheet(args, nav),
        "nssh" => nssh::cmd_nssh(args, nav),
        _ => misc::cmd_fallback(input, nav, watcher_handle),
    }
}

/// Execute a system command
fn execute_system_command(cmd: &str, cwd: &Path) -> Result<bool> {
    print_info(&format!("Executing: {}", cmd));
    println!();

    let status = run_system_command(cmd, cwd);
    println!();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                print_success(&t!("cmd.completed"));
            } else {
                match exit_status.code() {
                    Some(code) => print_error(&format!("Command exited with code: {}", code)),
                    None => print_warning(&t!("cmd.terminated")),
                }
            }
            Ok(false)
        }
        Err(e) => {
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

fn run_system_command(args: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    init_ctrlc_handler();

    #[cfg(windows)]
    {
        let mut child = std::process::Command::new("cmd")
            .args(["/C", args])
            .current_dir(cwd)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        CHILD_PID.store(child.id(), Ordering::SeqCst);
        let status = child.wait()?;
        CHILD_PID.store(0, Ordering::SeqCst);
        Ok(status)
    }

    #[cfg(not(windows))]
    {
        let mut child = std::process::Command::new("sh")
            .args(["-c", args])
            .current_dir(cwd)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        CHILD_PID.store(child.id(), Ordering::SeqCst);
        let status = child.wait()?;
        CHILD_PID.store(0, Ordering::SeqCst);
        Ok(status)
    }
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
        "config" | "wi" => "config",
        "ne" | "ntceditor" => "editor",
        "size" => "size",
        "view" | "txtc" | "txtf" => "view",
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
        "nsheet" => "nsheet",
        "nssh" => "nssh",
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
/// `gs <pattern> [-d <n>] [-l <n> | --limit <n> | --limit=<n>] [-i]`
pub(crate) fn parse_gs_args(args: &str) -> GsArgs {
    let default_depth = Config::global_get_max_depth();
    let parts = tokenize_gs_args(args);
    let mut pattern_parts: Vec<String> = Vec::new();
    let mut max_depth = default_depth;
    let mut limit: Option<usize> = None;
    let mut interactive = false;

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
            other => pattern_parts.push(other.to_string()),
        }
        i += 1;
    }

    GsArgs {
        pattern: pattern_parts.join(" "),
        max_depth,
        limit,
        interactive,
    }
}

fn list_supported_files(nav: &Navigator) -> Result<Vec<(String, PathBuf)>> {
    let fmt_cfg = FormatConfig::from_both();
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(nav.current_path()) {
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
    show_file_selection_menu_range(_nav, files, copy_mode, None)
}

fn show_file_selection_menu_range(
    _nav: &Navigator,
    files: Vec<(String, PathBuf)>,
    copy_mode: bool,
    range: Option<crate::output::FileLineRange>,
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
            let show_lines = Config::global_get_show_line_numbers();
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
    fn help_registry_covers_safe_commands() {
        assert_eq!(help_topic_for_command("mv"), Some("fileops"));
        assert_eq!(help_topic_for_command("move"), Some("fileops"));
        assert_eq!(help_topic_for_command("cp"), Some("fileops"));
        assert_eq!(help_topic_for_command("gs"), Some("search"));
        assert_eq!(help_topic_for_command("wi"), Some("config"));
        assert_eq!(help_topic_for_command("ne"), Some("editor"));
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
}
