use super::{execute_system_command, parse_search_args};
use crate::config::Config;
use crate::explorer::{calculate_target_size, format_duration, human_readable_size};
use crate::filetype::{is_supported_format, strip_unc_prefix};
use crate::navigator::{clear_screen, Navigator};
use crate::output::{cat_file, print_error, print_info, print_success, print_warning};
use crate::search;
use crate::session::SessionState;
use crate::shell::help::{print_interactive_help, print_tp_help};
use crate::shell::helpers::{
    parse_target_arg_allow_file, resolve_file_operand, run_with_spinner, show_nav_view,
    suppress_watch_refresh,
};
use crate::shell::venv::clear_venv_environment;
use crate::teleport::TeleportManager;
use crate::watcher;

use crate::t;
use anyhow::Result;
use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// True when running as root (Unix) or Administrator (Windows).
fn is_root_or_admin() -> bool {
    #[cfg(unix)]
    {
        // SAFETY: `geteuid` takes no arguments, cannot fail and has no
        // preconditions, so calling it is always sound.
        return unsafe { libc::geteuid() } == 0;
    }
    #[cfg(windows)]
    {
        // `net session` succeeds only when elevated.
        return std::process::Command::new("net")
            .args(["session"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

/// `deadlock.vault` removal is restricted to root/administrator.
fn is_deadlock_vault(path: &Path) -> bool {
    let candidates = [
        crate::deadlock::vault::vault_path(),
        crate::deadlock::vault::legacy_vault_path(),
    ];
    // Direct match (same type / path).
    if candidates.iter().any(|vault| path == vault) {
        return true;
    }
    // Canonicalize both sides so different CWDs, relative paths and symlinks
    // all resolve to the real file location.
    let canon = match path.canonicalize() {
        Ok(c) => c,
        Err(_) => return false,
    };
    candidates
        .iter()
        .filter_map(|vault| vault.canonicalize().ok())
        .any(|vault| canon == vault)
}

/// True when `path` is the dedicated directory that holds the deadlock vault.
fn is_deadlock_vault_dir(path: &Path) -> bool {
    let vault = crate::deadlock::vault::vault_path();
    let Some(vault_dir) = vault.parent() else {
        return false;
    };
    match (path.canonicalize(), vault_dir.canonicalize()) {
        (Ok(dir), Ok(vault_dir)) => dir == vault_dir,
        _ => false,
    }
}

pub fn cmd_clear(_args: &str, nav: &mut Navigator) -> Result<bool> {
    // `clear` redraws the whole view itself, so the shell must not print a
    // file-watcher change box / refresh the tree after this command.
    suppress_watch_refresh();
    clear_screen();
    println!();
    let show_header =
        Config::global_get_dir_view_enabled() || Config::global_get_file_view_enabled();
    if show_header {
        #[cfg(not(target_os = "android"))]
        {
            use colored::Colorize;
            let theme = crate::utils::theme::ThemeManager::current();
            let a = |s: &str| crate::utils::terminal::asciify(s);
            let version = env!("CARGO_PKG_VERSION");
            println!(
                "{}",
                a("╔══════════════════════════════════════════════════════════════════╗")
            );
            println!(
                "{}",
                a(&format!("{:^66}", t!("misc.clear.welcome", version)))
                    .color(theme.shell.help_header.to_colored())
                    .bold()
            );
            println!(
                "{}",
                a("╚══════════════════════════════════════════════════════════════════╝")
            );
        }
        #[cfg(target_os = "android")]
        {
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╔══════════════════════════════════════════════════════════════════╗"
                )
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(&format!(
                    "{:^66}",
                    t!("misc.clear.welcome", env!("CARGO_PKG_VERSION"))
                ))
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╚══════════════════════════════════════════════════════════════════╝"
                )
            );
        }
    }
    if show_header {
        println!("{}", t!("misc.clear.type_help").dimmed());
    }
    show_nav_view(nav);
    Ok(false)
}

pub fn cmd_math(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let mut expr = args.trim().to_string();
    // `math -` (or bare `math`) evaluates piped / redirected input.
    if expr.is_empty() || expr == "-" {
        if let Some(s) = crate::shell::stdin::read_to_string() {
            expr = s.trim().to_string();
        }
    }
    crate::math::run(&expr)?;
    Ok(false)
}

pub fn cmd_version(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(target_os = "android"))]
    {
        use colored::Colorize;
        let theme = crate::utils::theme::ThemeManager::current();
        println!(
            "ntc {}",
            env!("CARGO_PKG_VERSION")
                .color(theme.shell.success.to_colored())
                .bold()
        );
    }
    #[cfg(target_os = "android")]
    {
        println!("ntc {}", env!("CARGO_PKG_VERSION"));
    }
    Ok(false)
}

pub fn cmd_help(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let topic = args.trim();
    if topic.is_empty() {
        print_interactive_help();
    } else if !crate::shell::help::print_topic_help(topic) {
        print_error(&t!("help.unknown_topic", topic));
    }
    Ok(false)
}

pub fn cmd_tutorial(_args: &str, nav: &mut Navigator) -> Result<bool> {
    crate::tutorial::run_tutorial(nav)?;
    Ok(false)
}

pub fn cmd_exit(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    // The embedded shell (editor terminal) must never close itself: the
    // editor owns that PTY, so exiting here would strand the panel.
    if std::env::var("NTC_IN_EDITOR").is_ok() {
        print_error(&t!("misc.exit.in_editor"));
        return Ok(false);
    }
    Ok(true)
}

#[derive(Debug, PartialEq, Eq)]
enum NeCreateKind {
    Init,
    Super,
}

#[derive(Debug, PartialEq, Eq)]
enum NeError {
    MissingCreateTarget,
    UsageOpen,
    OpenNotCreated(String),
    HyperUsage,
}

#[derive(Debug, PartialEq, Eq)]
enum NeAction {
    Welcome,
    Open {
        path: PathBuf,
        line: Option<usize>,
    },
    Create {
        kind: NeCreateKind,
        targets: Vec<PathBuf>,
        open: Option<usize>,
    },
    Hyper {
        name: String,
        lang: String,
        open: bool,
    },
    Error(NeError),
}

fn ne_flag_rest<'a>(args: &'a str, flag: &str) -> Option<&'a str> {
    let rest = args.strip_prefix(flag)?;
    if rest.is_empty() {
        Some("")
    } else if rest.starts_with(' ') || rest.starts_with('\t') {
        Some(rest.trim_start())
    } else {
        None
    }
}

fn parse_open_spec(raw: &str) -> (PathBuf, Option<usize>) {
    if let Some((file_part, line_part)) = raw.rsplit_once(':') {
        if let Ok(line) = line_part.parse::<usize>() {
            if line > 0 {
                return (PathBuf::from(file_part), Some(line));
            }
        }
    }
    (PathBuf::from(raw), None)
}

fn parse_ne_create(kind: NeCreateKind, rest: &str) -> NeAction {
    let tokens = tokenize_quoted(rest);
    if tokens.is_empty() {
        return NeAction::Error(NeError::MissingCreateTarget);
    }
    let mut targets = Vec::new();
    let mut open_name: Option<String> = None;
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "-I" {
            if i + 1 >= tokens.len() || i + 2 != tokens.len() {
                return NeAction::Error(NeError::UsageOpen);
            }
            open_name = Some(tokens[i + 1].clone());
            break;
        }
        for part in tokens[i].split(',') {
            let part = part.trim();
            if !part.is_empty() {
                targets.push(PathBuf::from(part));
            }
        }
        i += 1;
    }
    if targets.is_empty() {
        return NeAction::Error(NeError::MissingCreateTarget);
    }
    let open = match open_name {
        Some(name) => {
            let target = PathBuf::from(&name);
            match targets.iter().position(|p| p == &target) {
                Some(idx) => Some(idx),
                None => return NeAction::Error(NeError::OpenNotCreated(name)),
            }
        }
        None => None,
    };
    NeAction::Create {
        kind,
        targets,
        open,
    }
}

fn parse_ne_hyper(rest: &str) -> NeAction {
    let tokens = tokenize_quoted(rest);
    if tokens.is_empty() {
        return NeAction::Error(NeError::HyperUsage);
    }
    let name = tokens[0].clone();
    if name.is_empty() || name.contains('.') {
        return NeAction::Error(NeError::HyperUsage);
    }
    let mut lang = "c".to_string();
    let mut open = false;
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--lang" => {
                if i + 1 >= tokens.len() {
                    return NeAction::Error(NeError::HyperUsage);
                }
                lang = tokens[i + 1].clone();
                i += 2;
            }
            "-I" => {
                if open || i + 1 != tokens.len() {
                    return NeAction::Error(NeError::UsageOpen);
                }
                open = true;
                i += 1;
            }
            _ => return NeAction::Error(NeError::HyperUsage),
        }
    }
    NeAction::Hyper { name, lang, open }
}

fn parse_ne_action(args: &str) -> NeAction {
    let args = args.trim();
    if args.is_empty() || args == "." {
        return NeAction::Welcome;
    }
    if let Some(rest) = ne_flag_rest(args, "--hyper") {
        return parse_ne_hyper(rest);
    }
    if let Some(rest) = ne_flag_rest(args, "--init") {
        return parse_ne_create(NeCreateKind::Init, rest);
    }
    if let Some(rest) = ne_flag_rest(args, "--super") {
        return parse_ne_create(NeCreateKind::Super, rest);
    }
    let (path, line) = parse_open_spec(args);
    NeAction::Open { path, line }
}

pub(crate) fn launch_ne_editor(raw_path: PathBuf, line: Option<usize>) {
    if let Some(line) = line {
        crate::editor::GOTO_LINE.store(line, std::sync::atomic::Ordering::SeqCst);
    }
    let path = if raw_path.is_dir() {
        let welcome = raw_path.join("welcome.ntcEditor");
        let _ = crate::editor::refresh_welcome_file(&welcome);
        welcome
    } else {
        raw_path
    };
    let restored = SessionState::read_global().editor_session.clone();
    match crate::editor::edit_file_with_session(&path, restored) {
        Ok((_, captured)) => {
            SessionState::write_global().editor_session = captured;
            SessionState::save_global();
        }
        Err(e) => print_error(&t!("misc.ne.editor_error", e)),
    }
}

pub fn cmd_ne(args: &str, nav: &mut Navigator) -> Result<bool> {
    // Refuse nested editors inside the editor's embedded ntc shell
    // (spawned with `NTC_IN_EDITOR=1`).
    if std::env::var("NTC_IN_EDITOR").is_ok() {
        print_error(&t!("misc.ne.already_in_editor"));
        return Ok(false);
    }
    match parse_ne_action(args) {
        NeAction::Welcome => {
            let cwd = nav.current_path();
            let welcome = cwd.join("welcome.ntcEditor");
            let _ = crate::editor::refresh_welcome_file(&welcome);
            print_info(&t!("misc.ne.opened_welcome", nav.current_path().display()));
            launch_ne_editor(welcome, None);
        }
        NeAction::Open { path, line } => {
            if !path.exists() {
                print_error(&t!("misc.ne.file_missing", path.display()));
                return Ok(false);
            }
            launch_ne_editor(path, line);
        }
        NeAction::Create {
            kind,
            targets,
            open,
        } => {
            for target in &targets {
                if target.exists() {
                    print_warning(&t!("misc.mkf.exists", target.display()));
                    continue;
                }
                let created = match kind {
                    NeCreateKind::Init => crate::editor::init_file(target),
                    NeCreateKind::Super => crate::editor::init_super_file(target),
                };
                match created {
                    Ok(true) => {
                        let msg = match kind {
                            NeCreateKind::Init => {
                                t!("misc.ne.created_template", target.display())
                            }
                            NeCreateKind::Super => {
                                t!("misc.ne.created_super", target.display())
                            }
                        };
                        eprintln!("{}", msg);
                    }
                    Ok(false) => {
                        print_error(&t!("misc.ne.create_failed", target.display()));
                    }
                    Err(e) => {
                        print_error(&t!(
                            "misc.ne.create_failed",
                            format!("{}: {}", target.display(), e)
                        ));
                    }
                }
            }
            if let Some(idx) = open {
                if let Some(target) = targets.get(idx) {
                    if target.exists() {
                        launch_ne_editor(target.clone(), None);
                    }
                }
            }
        }
        NeAction::Hyper { name, lang, open } => {
            eprintln!("{}", t!("editor.hyper_creating", name));
            match crate::editor::init_hyper_project(&name, &lang) {
                Ok(_) => {
                    eprintln!("{}", t!("editor.hyper_created", name));
                    if open {
                        launch_ne_editor(PathBuf::from(&name), None);
                    }
                }
                Err(e) => print_error(&t!("misc.ne.hyper_error", e)),
            }
        }
        NeAction::Error(err) => match err {
            NeError::MissingCreateTarget => print_error(&t!("misc.ne.missing_file")),
            NeError::UsageOpen => print_error(&t!("misc.ne.usage_open")),
            NeError::OpenNotCreated(name) => print_error(&t!("misc.ne.open_not_created", name)),
            NeError::HyperUsage => print_error(&t!("misc.ne.usage_hyper")),
        },
    }
    Ok(false)
}

pub fn cmd_size(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parsed = match parse_target_arg_allow_file(args, nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let show_path = parsed.path.is_some();
    let target = parsed
        .path
        .unwrap_or_else(|| nav.current_path().to_path_buf());

    let mut care = false;
    let mut show_time = false;
    for arg in &parsed.rest {
        match arg.as_str() {
            "--care" => care = true,
            "--time" | "-t" => show_time = true,
            _ => {}
        }
    }
    let start = std::time::Instant::now();
    // Files skip the directory walk (and spinner): a single metadata call.
    if target.is_file() {
        let total = calculate_target_size(&target, care);
        let elapsed = start.elapsed();
        let human_str = human_readable_size(total);
        let display = strip_unc_prefix(&target);
        if show_time {
            println!("{} {} ({})", display, human_str, format_duration(elapsed));
        } else {
            println!("{} {}", display, human_str);
        }
        return Ok(false);
    }
    let total = if care {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_target_size(&target, true)
        })
    } else {
        run_with_spinner(&t!("misc.size.calculating"), || {
            calculate_target_size(&target, false)
        })
    };
    let elapsed = start.elapsed();
    let bytes_str = format!("{}", total);
    let human_str = human_readable_size(total);
    let time_str = format_duration(elapsed);
    let path_str = if show_path {
        strip_unc_prefix(&target)
    } else {
        String::new()
    };
    let max_label = 8;
    let mut max_value = bytes_str.len().max(human_str.len()).max(time_str.len());
    if show_path {
        max_value = max_value.max(path_str.len());
    }
    let w = max_label + max_value + 10;

    let bytes_label = t!("misc.size.bytes_label");
    let human_label = t!("misc.size.human_label");
    let time_label = t!("misc.size.time_label");
    let title_label = t!("misc.size.title");
    let a = |s: &str| crate::utils::terminal::asciify(s);
    let v = a("│");
    let h = a("─");
    println!();
    println!("{}{}{}", a("┌"), h.repeat(w), a("┐"));
    println!("{}{:^w$}", v, title_label.cyan().bold(), w = w);
    if show_path {
        let path_label = t!("misc.size.path_label");
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            path_label.yellow(),
            path_str.cyan(),
            w = w - path_label.len() - 4
        );
    }
    println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
    println!(
        "{} {} {:<w$} ",
        v,
        bytes_label.yellow(),
        bytes_str.yellow(),
        w = w - bytes_label.len() - 4
    );
    println!(
        "{} {} {:<w$} ",
        v,
        human_label.green().bold(),
        human_str.green().bold(),
        w = w - human_label.len() - 4
    );
    if show_time {
        println!("{}{}{}", a("├"), h.repeat(w), a("┤"));
        println!(
            "{} {} {:<w$} ",
            v,
            time_label.cyan(),
            time_str.cyan(),
            w = w - time_label.len() - 4
        );
    }
    println!("{}{}{}", a("└"), h.repeat(w), a("┘"));
    Ok(false)
}

pub fn cmd_tp(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::interactive_menu(nav)?;
    } else {
        let tp_parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = tp_parts[0].to_lowercase();
        let subargs = tp_parts.get(1).unwrap_or(&"").trim();

        match subcmd.as_str() {
            "add" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_add"));
                } else {
                    let add_parts: Vec<&str> = subargs.splitn(2, ' ').collect();
                    let name = add_parts[0];
                    if add_parts.len() > 1 {
                        let path = std::path::Path::new(add_parts[1]);
                        TeleportManager::add(name, path.to_path_buf())?;
                    } else {
                        TeleportManager::add_current(nav, name)?;
                    }
                }
            }
            "jump" | "to" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_jump"));
                } else if let Ok(num) = subargs.parse::<usize>() {
                    TeleportManager::jump_by_index(nav, num)?;
                } else {
                    TeleportManager::jump_by_name(nav, subargs)?;
                }
            }
            "info" => {
                if subargs.is_empty() {
                    print_error(&t!("misc.tp.usage_info"));
                    println!("{}", t!("misc.tp.example_info"));
                } else {
                    let teleports = TeleportManager::get_all();
                    let name_lower = subargs.to_lowercase();
                    if let Some(path) = teleports.get(&name_lower) {
                        println!();
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!("{}", t!("misc.tp.info_title").cyan().bold());
                        println!(
                            "{}",
                            "==================================================".cyan()
                        );
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_name").yellow(),
                            name_lower.blue().bold()
                        );
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_path").yellow(),
                            path.display().to_string().dimmed()
                        );
                        let exists = if path.exists() {
                            "yes".green()
                        } else {
                            "no".red()
                        };
                        println!("  {}: {}", t!("misc.tp.info_exists").yellow(), exists);
                        if let Ok(canonical) = std::fs::canonicalize(path) {
                            println!(
                                "  {}: {}",
                                t!("misc.tp.info_canonical").yellow(),
                                canonical.display().to_string().dimmed()
                            );
                        }
                        let total = teleports.len();
                        let sorted: Vec<&String> = teleports.keys().collect();
                        if let Some(pos) = sorted.iter().position(|k| *k == &name_lower) {
                            println!(
                                "  {}: {} / {}",
                                t!("misc.tp.info_index").yellow(),
                                (pos + 1).to_string().yellow(),
                                total.to_string().yellow()
                            );
                        }
                        println!(
                            "  {}: {}",
                            t!("misc.tp.info_source").yellow(),
                            t!("misc.tp.info_global").green()
                        );
                        println!();
                    } else {
                        print_error(&t!("misc.tp.not_found", subargs));
                    }
                }
            }
            "list" | "ls" => {
                TeleportManager::list()?;
            }
            "rm" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_rm"));
                } else {
                    for arg in subargs
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                    {
                        if let Ok(num) = arg.parse::<usize>() {
                            TeleportManager::remove_by_index(num)?;
                        } else {
                            TeleportManager::remove_by_name(arg)?;
                        }
                    }
                }
            }
            "rnm" | "rename" => {
                if subargs.is_empty() {
                    println!("{}", t!("misc.tp.usage_rnm"));
                } else {
                    let parts: Vec<&str> = subargs.splitn(4, ' ').collect();
                    if parts.len() >= 3 && parts[1].to_lowercase() == "to" {
                        TeleportManager::rename(parts[0], parts[2])?;
                    } else {
                        print_error(&t!("misc.tp.invalid_format"));
                    }
                }
            }
            "cls" => {
                TeleportManager::clear_all()?;
            }
            "help" => {
                print_tp_help();
            }
            _ => {
                print_error(&t!("misc.tp.unknown_subcmd", subcmd));
                println!("{}", t!("misc.tp.type_help"));
            }
        }
    }
    Ok(false)
}

pub fn cmd_gs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.trim().is_empty() {
        print_error(&t!("misc.gs.usage"));
        // When piped, keep stdout clean (examples would land in the pipe/file).
        if !crate::shell::stdin::has_input() {
            println!("{}", t!("misc.gs.example1"));
            println!("{}", t!("misc.gs.example2"));
            println!("{}", t!("misc.gs.example3"));
            println!("{}", t!("misc.gs.example4"));
            println!("{}", t!("misc.gs.example5"));
            println!("{}", t!("misc.gs.example6"));
            println!("{}", t!("misc.gs.example7"));
            println!("{}", t!("misc.gs.example8"));
        }
        return Ok(false);
    }
    let parsed = super::parse_gs_args(args);
    if parsed.pattern.trim().is_empty() {
        print_error(&t!("misc.gs.usage"));
        return Ok(false);
    }
    let pattern = parsed.pattern;
    let max_depth = parsed.max_depth;

    // `-p` scopes the filesystem walk — it makes no sense on piped text.
    if crate::shell::stdin::has_input() && !parsed.paths.is_empty() {
        print_error(&t!("misc.gs.no_pipe_path"));
        return Ok(false);
    }

    // Piped / redirected input: search the provided text (same
    // case-insensitive substring semantics as the file search) and print the
    // matching lines grep-style, instead of walking the filesystem.
    if crate::shell::stdin::has_input() {
        let text = crate::shell::stdin::read_to_string().unwrap_or_default();
        let mut matches = crate::search::search_text_lines(&text, &pattern);
        if let Some(sort) = parsed.sort {
            match sort {
                // `-s path|size` need file metadata that piped text lacks.
                super::GsSort::Path | super::GsSort::Size => {
                    print_error(&t!("misc.gs.pipe_sort_only_line"));
                    return Ok(false);
                }
                super::GsSort::Line => {
                    matches.sort_by(|a, b| {
                        a.line
                            .to_lowercase()
                            .cmp(&b.line.to_lowercase())
                            .then_with(|| a.line_number.cmp(&b.line_number))
                    });
                }
            }
        }
        if let Some(lim) = parsed.limit {
            if matches.len() > lim {
                matches.truncate(lim);
            }
        }
        if matches.len() > 20_000 {
            matches.truncate(20_000);
        }
        print!("{}", crate::search::format_text_matches(&matches, &pattern));
        return Ok(false);
    }

    let root = nav.current_path().to_path_buf();
    // Spinner in background thread so broad `-d` searches show progress
    // instead of looking crashed. Heavy-file guards live in search_content.
    let results = run_with_spinner(&t!("misc.gs.searching"), || {
        if parsed.paths.is_empty() {
            crate::search::search_content(&root, &pattern, max_depth)
        } else {
            let files = super::qur::resolve_qur_files(&root, &parsed.paths, max_depth);
            crate::search::search_content_in_files(&files, &pattern)
        }
    });
    let mut results = results;
    if let Some(sort) = parsed.sort {
        let content_sort = match sort {
            super::GsSort::Line => crate::search::ContentSort::Line,
            super::GsSort::Path => crate::search::ContentSort::Path,
            super::GsSort::Size => crate::search::ContentSort::Size,
        };
        crate::search::sort_content_matches(&mut results, content_sort);
    }
    let total_found = results.len();
    if results.len() > 20_000 {
        print_warning(&t!("misc.gs.many_results", results.len()));
        results.truncate(20_000);
    }
    // -l/--limit is non-interactive only (paged UI has its own 10/page).
    if !parsed.interactive {
        if let Some(lim) = parsed.limit {
            if results.len() > lim {
                results.truncate(lim);
            }
        }
    }
    if parsed.interactive {
        return super::gs_interactive::run_gs_interactive(results, &root, &pattern);
    }
    let output =
        crate::search::format_content_results(&results, &pattern, max_depth, &root, total_found);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fs.usage"));
        println!("{}", t!("misc.fs.example1"));
        println!("{}", t!("misc.fs.example2"));
        println!("{}", t!("misc.fs.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_ds(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.ds.usage"));
        println!("{}", t!("misc.ds.example1"));
        println!("{}", t!("misc.ds.example2"));
        println!("{}", t!("misc.ds.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_directories(nav.current_path(), &pattern, max_depth);
    let output = crate::search::format_search_results(&results, &pattern, max_depth, false);
    print!("{}", output);
    Ok(false)
}

pub fn cmd_fsc(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fsc.usage"));
        println!("{}", t!("misc.fsc.example1"));
        println!("{}", t!("misc.fsc.example2"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = crate::search::search_files(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    loop {
        println!();
        println!(
            "{}",
            t!("misc.fsc.found", results.len(), pattern).cyan().bold()
        );
        for (i, r) in results.iter().enumerate() {
            let path_str = r.full_path.to_string_lossy();
            println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
        }
        println!("  {}", t!("misc.fsc.done_opt").red());
        println!();
        print!("{} ", t!("misc.fsc.select_prompt").green());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "0" || input.is_empty() {
            println!("{}", t!("misc.fsc.done_str").dimmed());
            return Ok(false);
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= results.len() {
                let target = &results[n - 1].full_path;
                if target.is_file() {
                    if is_supported_format(target) {
                        let show_lines = Config::global_get_show_line_numbers();
                        cat_file(target, show_lines)?;
                        println!();
                        print!("{} ", t!("misc.fsc.copy_prompt").yellow());
                        io::stdout().flush()?;
                        let mut copy_choice = String::new();
                        io::stdin().read_line(&mut copy_choice)?;
                        let copy_choice = copy_choice.trim().to_lowercase();
                        if copy_choice == "y" || copy_choice == "yes" {
                            let content =
                                crate::output::cat_file_with_line_numbers(target, show_lines)?;
                            crate::output::copy_to_clipboard(&content, "TXT")?;
                            print_success(&t!("misc.fsc.copied", target.display()));
                        }
                    } else {
                        print_warning(&t!("misc.fsc.skipped", target.display()));
                    }
                } else {
                    print_warning(&t!("misc.fsc.not_file"));
                }
            } else {
                print_error(&t!("misc.fsc.invalid_selection"));
            }
        } else {
            print_error(&t!("misc.fsc.invalid_input"));
        }
        println!();
    }
}

pub fn cmd_locate(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.locate.usage"));
        println!("{}", t!("misc.locate.example1"));
        println!("{}", t!("misc.locate.example2"));
        println!("{}", t!("misc.locate.example3"));
        return Ok(false);
    }
    let (pattern, max_depth) = parse_search_args(args);
    let results = search::search_all(nav.current_path(), &pattern, max_depth);
    if results.is_empty() {
        println!("{}", t!("misc.locate.no_results", pattern).dimmed());
    } else {
        let has_exact = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Exact);
        let has_partial = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Partial);
        let label = if has_exact {
            t!("misc.locate.label_exact")
        } else if has_partial {
            t!("misc.locate.label_partial")
        } else {
            t!("misc.locate.label_didyoumean")
        };
        println!();
        println!(
            "{}",
            t!("misc.locate.header", label, pattern, results.len())
                .cyan()
                .bold()
        );
        for (i, r) in results.iter().enumerate() {
            let icon = if r.full_path.is_dir() { "📁" } else { "📄" };
            if r.match_kind == search::MatchKind::Fuzzy {
                println!(
                    "  {}. {} {} ({:.0}% match)",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed(),
                    r.score * 100.0
                );
            } else {
                println!(
                    "  {}. {} {}",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed()
                );
            }
        }
        println!();
    }
    Ok(false)
}

pub fn cmd_tpb(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::teleport_back(nav)?;
    } else {
        let args_lower = args.to_lowercase();
        match args_lower.as_str() {
            "history" | "hist" | "h" => {
                TeleportManager::show_history()?;
            }
            "clear" | "cls" => {
                TeleportManager::clear_stack();
            }
            _ => {
                print_error(&t!("misc.tpb.unknown", args));
                println!("{}", t!("misc.tpb.usage"));
                println!("{}", t!("misc.tpb.usage_basic"));
                println!("{}", t!("misc.tpb.usage_history"));
                println!("{}", t!("misc.tpb.usage_clear"));
            }
        }
    }
    Ok(false)
}

fn split_args(args: &str) -> Vec<String> {
    args.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// `mkf <file>[,…]` — create empty files (parent directories included).
/// Every operand accepts teleport savepoints (`mkf @tp1/notes.md`).
pub fn cmd_mkf(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkf.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if path.exists() {
            print_warning(&t!("misc.mkf.exists", item));
            continue;
        }
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        match std::fs::write(&path, "") {
            Ok(()) => print_success(&t!("misc.mkf.created", item)),
            Err(e) => print_error(&t!("misc.mkf.error", item, e)),
        }
    }
    Ok(false)
}

/// `mkd <dir>[,…]` — create directories (nested paths included).
/// Every operand accepts teleport savepoints (`mkd @tp1/build`).
pub fn cmd_mkd(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.mkd.missing"));
        return Ok(false);
    }
    for item in &items {
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if let Err(e) = std::fs::create_dir_all(&path) {
            print_error(&t!("misc.mkd.error", item, e));
        } else {
            print_success(&t!("misc.mkd.created", item));
        }
    }
    Ok(false)
}

/// `rmd <dir>[,…]` — remove directories (`*` patterns supported).
/// Every operand accepts teleport savepoints (`rmd @tp1/old`, `rmd @tp1/old*`).
pub fn cmd_rmd(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmd.missing"));
        return Ok(false);
    }
    for item in &items {
        if item.contains('*') {
            remove_rmd_pattern(item, nav)?;
            continue;
        }
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if !path.exists() {
            print_error(&t!("misc.rmd.not_exist", item));
        } else if !path.is_dir() {
            print_error(&t!("misc.rmd.not_dir", item));
        } else if is_deadlock_vault_dir(&path) && !is_root_or_admin() {
            print_error(&t!("misc.rmd.deadlock_vault_admin_only", item));
        } else {
            if is_deadlock_vault_dir(&path) {
                // Root/administrator: lift the OS-level deletion barrier.
                let _ = crate::deadlock::protect::unprotect(&crate::deadlock::vault::vault_path());
            }
            if let Err(e) = std::fs::remove_dir_all(&path) {
                print_error(&t!("misc.rmd.error", item, e));
            } else {
                print_success(&t!("misc.rmd.removed", item));
            }
        }
    }
    Ok(false)
}

/// Match the `*`-wildcard name pattern `pat` against the entries directly
/// inside `base`, keeping files (`want_dirs == false`) or directories.
/// The pattern is end-anchored so `*.log` means "ends with .log", not
/// "contains". Sorted. Pure for testability.
fn expand_glob_in_dir_filtered(base: &Path, pat: &str, want_dirs: bool) -> Vec<PathBuf> {
    let anchored = format!("{}$", pat);
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            let keep = match entry.file_type() {
                Ok(t) if want_dirs => t.is_dir(),
                Ok(t) => t.is_file(),
                Err(_) => false,
            };
            if !keep {
                continue;
            }
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if crate::search::matches_wildcard(&anchored, &name) {
                out.push(base.join(name));
            }
        }
    }
    out.sort();
    out
}

/// Match the `*`-wildcard file-name pattern `pat` against the files
/// directly inside `base`. Directories never match — patterns only ever
/// target files. Sorted. Pure for testability.
fn expand_glob_in_dir(base: &Path, pat: &str) -> Vec<PathBuf> {
    expand_glob_in_dir_filtered(base, pat, false)
}

/// Expand one `rmd` operand containing `*` wildcards (`*`, `old*`,
/// `parent/*`, `@tp1/old*`) into the matching directories. Files never
/// match — `rmd` only removes directories. The parent part goes through
/// `resolve_file_operand`, so plain paths and teleport savepoints both
/// expand from the right base. Pure for testability.
fn expand_rmd_pattern(item: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(item);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(item);
            expand_glob_in_dir_filtered(&base, pat, true)
        }
        None => expand_glob_in_dir_filtered(nav.current_path(), item, true),
    }
}

/// Delete the directories matched by one `rmd` wildcard operand. A bulk
/// delete always lists the matches and asks for confirmation first, so
/// `rmd *` can never wipe the working directory in one keystroke.
fn remove_rmd_pattern(item: &str, nav: &Navigator) -> Result<()> {
    let matches = expand_rmd_pattern(item, nav);
    if matches.is_empty() {
        print_error(&t!("misc.rmd.no_match", item));
        return Ok(());
    }
    println!();
    println!(
        "{}",
        t!("misc.rmd.pattern_matches", matches.len(), item)
            .yellow()
            .bold()
    );
    for (i, p) in matches.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().yellow(), p.display());
    }
    println!();
    let ok = crate::utils::interactive::prompt_yes_no(
        &t!("misc.rmd.confirm_delete", matches.len()),
        false,
    )
    .unwrap_or(false);
    if !ok {
        println!("{}", t!("misc.rmd.cancelled").dimmed());
        return Ok(());
    }
    for path in &matches {
        if is_deadlock_vault_dir(path) {
            if !is_root_or_admin() {
                print_error(&t!("misc.rmd.deadlock_vault_admin_only", path.display()));
                continue;
            }
            // Root/administrator: lift the OS-level deletion barrier.
            let _ = crate::deadlock::protect::unprotect(&crate::deadlock::vault::vault_path());
        }
        match std::fs::remove_dir_all(path) {
            Ok(()) => print_success(&t!("misc.rmd.removed", path.display())),
            Err(e) => print_error(&t!("misc.rmd.error", path.display(), e)),
        }
    }
    Ok(())
}

/// Expand one `rmf` operand containing `*` wildcards (`*.log`,
/// `test*`, `dir/*.tmp`, `@tp1/*.log`) into the matching files.
/// Directories never match — `rmf` only removes files. The parent part goes
/// through `resolve_file_operand`, so savepoints expand from the right base.
/// Pure for testability.
fn expand_rmf_pattern(item: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(item);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(item);
            expand_glob_in_dir(&base, pat)
        }
        None => expand_glob_in_dir(nav.current_path(), item),
    }
}

/// Expand a `cp`/`mv` src operand containing a `*` wildcard. The parent
/// directory is resolved through `resolve_file_operand` so plain paths
/// (`dir/*.py`), CWD-relative patterns (`*.py`) and teleport subpaths
/// (`@tp/*.py`) all expand from the right base.
fn expand_operand_pattern(spec: &str, nav: &Navigator) -> Vec<PathBuf> {
    let raw = Path::new(spec);
    match raw.parent().filter(|p| !p.as_os_str().is_empty()) {
        Some(dir) => {
            let base = match resolve_file_operand(&dir.to_string_lossy(), nav) {
                Ok(p) => p,
                Err(_) => return Vec::new(),
            };
            let pat = raw.file_name().and_then(|n| n.to_str()).unwrap_or(spec);
            expand_glob_in_dir(&base, pat)
        }
        None => expand_glob_in_dir(nav.current_path(), spec),
    }
}

/// Delete the files matched by one `rmf` wildcard operand: single match
/// goes directly, multiple matches list + confirm first.
fn remove_rmf_pattern(item: &str, nav: &Navigator) -> Result<()> {
    let matches = expand_rmf_pattern(item, nav);
    if matches.is_empty() {
        print_error(&t!("misc.rmf.no_match", item));
        return Ok(());
    }
    if matches.len() > 1 {
        println!();
        println!(
            "{}",
            t!("misc.rmf.pattern_matches", matches.len(), item)
                .yellow()
                .bold()
        );
        for (i, p) in matches.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().yellow(), p.display());
        }
        println!();
        let ok = crate::utils::interactive::prompt_yes_no(
            &t!("misc.rmf.confirm_delete", matches.len()),
            false,
        )
        .unwrap_or(false);
        if !ok {
            println!("{}", t!("misc.rmf.cancelled").dimmed());
            return Ok(());
        }
    }
    for path in &matches {
        if is_deadlock_vault(path) {
            if !is_root_or_admin() {
                print_error(&t!("misc.rmf.deadlock_vault_admin_only", path.display()));
                continue;
            }
            // Root/administrator may remove it, but the OS-level barrier
            // (deny-DELETE ACE / immutable flag) must be lifted first.
            let _ = crate::deadlock::protect::unprotect(path);
        }
        match std::fs::remove_file(path) {
            Ok(()) => print_success(&t!("misc.rmf.removed", path.display())),
            Err(e) => print_error(&t!("misc.rmf.error", path.display(), e)),
        }
    }
    Ok(())
}

/// `rmf <file>[,…]` — remove files (`*` patterns supported).
/// Every operand accepts teleport savepoints (`rmf @tp1/old.log`,
/// `rmf @tp1/*.log`).
pub fn cmd_rmf(args: &str, nav: &mut Navigator) -> Result<bool> {
    let items = split_args(args);
    if items.is_empty() {
        print_error(&t!("misc.rmf.missing"));
        return Ok(false);
    }
    for item in &items {
        if item.contains('*') {
            remove_rmf_pattern(item, nav)?;
            continue;
        }
        let path = match resolve_file_operand(item, nav) {
            Ok(p) => p,
            Err(_) => continue,
        };
        if !path.exists() {
            print_error(&t!("misc.rmf.not_exist", item));
        } else if path.is_dir() {
            print_error(&t!("misc.rmf.is_dir", item));
        } else if is_deadlock_vault(&path) && !is_root_or_admin() {
            print_error(&t!("misc.rmf.deadlock_vault_admin_only", item));
        } else {
            if is_deadlock_vault(&path) {
                // Root/administrator: lift the OS-level deletion barrier.
                let _ = crate::deadlock::protect::unprotect(&path);
            }
            match std::fs::remove_file(&path) {
                Ok(()) => print_success(&t!("misc.rmf.removed", item)),
                Err(e) => print_error(&t!("misc.rmf.error", item, e)),
            }
        }
    }
    Ok(false)
}

pub fn cmd_rnmf(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmf.missing"));
        println!("{}", t!("misc.rnmf.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmf.not_exist", old));
    } else if old_path.is_dir() {
        print_error(&t!("misc.rnmf.is_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmf.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmf.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmf.error", e)),
        }
    }
    Ok(false)
}

pub fn cmd_rnmd(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        print_error(&t!("misc.rnmd.missing"));
        println!("{}", t!("misc.rnmd.usage"));
        return Ok(false);
    }
    let old = parts[0];
    let new = parts[1];
    let old_path = std::path::Path::new(old);
    let new_path = std::path::Path::new(new);
    if !old_path.exists() {
        print_error(&t!("misc.rnmd.not_exist", old));
    } else if !old_path.is_dir() {
        print_error(&t!("misc.rnmd.not_dir", old));
    } else if new_path.exists() {
        print_error(&t!("misc.rnmd.exists", new));
    } else {
        match std::fs::rename(old_path, new_path) {
            Ok(()) => print_success(&t!("misc.rnmd.renamed", old, new)),
            Err(e) => print_error(&t!("misc.rnmd.error", e)),
        }
    }
    Ok(false)
}

pub fn cmd_ui(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    println!("{}", t!("misc.ui.removed"));
    Ok(false)
}

/// `mv <src> <dst> [-f]` — move/rename files or directories.
/// Both operands accept teleport savepoints (`mv @tp1/report.pdf @docs`,
/// `mv @tp1 ./here`); `dst` inside an existing directory keeps the source
/// file name (`mv report.pdf @docs`).
/// A `*` wildcard source expands to the matching files (files only,
/// non-recursive) and requires `dst` to be a directory (created when
/// missing): `mv *.py out/`.
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
/// Falls back to copy+delete when `rename` fails (e.g. cross-device).
pub fn cmd_mv(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.mv.missing"));
        println!("{}", t!("misc.mv.usage"));
        return Ok(false);
    }
    // `mv <pattern> <dir>`: expand the wildcard; dst must be a directory.
    if operands[0].contains('*') {
        let dst = match resolve_file_operand(&operands[1], nav) {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        move_wildcard_batch(&operands[0], &operands[1], &dst, force, nav);
        return Ok(false);
    }
    let src = match resolve_file_operand(&operands[0], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let mut dst = match resolve_file_operand(&operands[1], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    if !src.exists() {
        print_error(&t!("misc.mv.not_exist", operands[0]));
        return Ok(false);
    }
    // `mv file existing_dir/` moves inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.mv.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if dst.exists() && !force {
        print_error(&t!("misc.mv.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.mv.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.mv.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match std::fs::rename(&src, &dst) {
        Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
        Err(_) => match move_by_copy(&src, &dst) {
            Ok(()) => print_success(&t!("misc.mv.moved", operands[0], operands[1])),
            Err(e) => print_error(&t!("misc.mv.error", e)),
        },
    }
    Ok(false)
}

/// `cp <src> <dst> [-f]` — copy files or directories.
/// Both operands accept teleport savepoints (`cp @tp1/report.pdf @docs`,
/// `cp @tp1 ./here`); `dst` inside an existing directory keeps the source
/// file name (`cp report.pdf @docs`).
/// A `*` wildcard source expands to the matching files (files only,
/// non-recursive) and requires `dst` to be a directory (created when
/// missing): `cp *.py out/`.
/// Quoted paths with spaces supported; `-f/--force` overwrites the target.
pub fn cmd_cp(args: &str, nav: &mut Navigator) -> Result<bool> {
    let (operands, force) = parse_mv_args(args);
    if operands.len() < 2 {
        print_error(&t!("misc.cp.missing"));
        println!("{}", t!("misc.cp.usage"));
        return Ok(false);
    }
    // `cp <pattern> <dir>`: expand the wildcard; dst must be a directory.
    if operands[0].contains('*') {
        let dst = match resolve_file_operand(&operands[1], nav) {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        copy_wildcard_batch(&operands[0], &operands[1], &dst, force, nav);
        return Ok(false);
    }
    let src = match resolve_file_operand(&operands[0], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    let mut dst = match resolve_file_operand(&operands[1], nav) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    if !src.exists() {
        print_error(&t!("misc.cp.not_exist", operands[0]));
        return Ok(false);
    }
    // `cp file existing_dir/` copies inside the directory (never refuses).
    if dst.is_dir() {
        match src.file_name() {
            Some(name) => dst.push(name),
            None => {
                print_error(&t!("misc.cp.error", "invalid source name"));
                return Ok(false);
            }
        }
    }
    if same_file(&src, &dst) {
        print_error(&t!("misc.cp.same", operands[0]));
        return Ok(false);
    }
    if dst.exists() && !force {
        print_error(&t!("misc.cp.exists", operands[1]));
        return Ok(false);
    }
    if force && dst.exists() {
        let cleared = if dst.is_dir() && !src.is_dir() {
            // Never replace a directory with a file.
            print_error(&t!("misc.cp.error", "cannot overwrite directory with file"));
            return Ok(false);
        } else if dst.is_dir() {
            std::fs::remove_dir_all(&dst).is_ok()
        } else {
            std::fs::remove_file(&dst).is_ok()
        };
        if !cleared {
            print_error(&t!("misc.cp.error", "cannot clear existing target"));
            return Ok(false);
        }
    }

    match copy_path(&src, &dst) {
        Ok(()) => print_success(&t!("misc.cp.copied", operands[0], operands[1])),
        Err(e) => print_error(&t!("misc.cp.error", e)),
    }
    Ok(false)
}

/// True when both paths point at the same file (guards `cp` onto itself).
fn same_file(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// Copy `src` to `dst` then remove `src` (cross-device `mv` fallback).
fn move_by_copy(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_path(src, dst)?;
    if src.is_dir() {
        std::fs::remove_dir_all(src)?;
    } else {
        std::fs::remove_file(src)?;
    }
    Ok(())
}

/// Copy a file or directory tree to `dst` (no removal).
fn copy_path(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_all(src, dst)?;
    } else {
        if let Some(parent) = dst.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::copy(src, dst)?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in walkdir::WalkDir::new(src).min_depth(1) {
        let entry =
            entry.map_err(|e| std::io::Error::other(format!("walk {}: {}", src.display(), e)))?;
        let rel = entry
            .path()
            .strip_prefix(src)
            .map_err(std::io::Error::other)?;
        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Shared wildcard prelude for `cp`/`mv`: expand `src_pat` into files and
/// require `dst` to be a directory (created when missing). Returns the
/// destination directory and the matched sources, or `None` after printing
/// a localized error (`no_match_key` / `dst_dir_key`).
fn expand_wildcard_src(
    src_pat: &str,
    dst: &Path,
    nav: &Navigator,
    no_match_key: &str,
    dst_dir_key: &str,
) -> Option<(PathBuf, Vec<PathBuf>)> {
    let sources = expand_operand_pattern(src_pat, nav);
    if sources.is_empty() {
        print_error(&t!(no_match_key, src_pat));
        return None;
    }
    if dst.is_dir() {
        return Some((dst.to_path_buf(), sources));
    }
    if dst.exists() {
        print_error(&t!(dst_dir_key, dst.display()));
        return None;
    }
    if std::fs::create_dir_all(dst).is_err() {
        print_error(&t!(dst_dir_key, dst.display()));
        return None;
    }
    Some((dst.to_path_buf(), sources))
}

/// `cp <pattern> <dir>`: copy every file matched by a `*` source into
/// `dst_spec` (which must be, or become, a directory). `-f` overwrites an
/// existing target file; a target directory is never replaced by a file.
fn copy_wildcard_batch(src_pat: &str, dst_spec: &str, dst: &Path, force: bool, nav: &Navigator) {
    let Some((dst_dir, sources)) =
        expand_wildcard_src(src_pat, dst, nav, "misc.cp.no_match", "misc.cp.dst_dir")
    else {
        return;
    };
    let mut done = 0usize;
    for src in sources {
        let name = match src.file_name() {
            Some(n) => n,
            None => continue,
        };
        let target = dst_dir.join(name);
        if same_file(&src, &target) {
            print_error(&t!("misc.cp.same", src.display()));
            continue;
        }
        if target.exists() && !force {
            print_error(&t!("misc.cp.exists", target.display()));
            continue;
        }
        if force && target.exists() {
            if target.is_dir() {
                // Never replace a directory with a file.
                print_error(&t!("misc.cp.error", "cannot overwrite directory with file"));
                continue;
            }
            if std::fs::remove_file(&target).is_err() {
                print_error(&t!("misc.cp.error", "cannot clear existing target"));
                continue;
            }
        }
        match copy_path(&src, &target) {
            Ok(()) => done += 1,
            Err(e) => print_error(&t!("misc.cp.error", e)),
        }
    }
    if done > 0 {
        print_success(&t!("misc.cp.wild_done", done, dst_spec));
    }
}

/// `mv <pattern> <dir>`: move every file matched by a `*` source into
/// `dst_spec` (which must be, or become, a directory). Mirrors
/// `copy_wildcard_batch`'s overwrite rules, with a copy+delete fallback.
fn move_wildcard_batch(src_pat: &str, dst_spec: &str, dst: &Path, force: bool, nav: &Navigator) {
    let Some((dst_dir, sources)) =
        expand_wildcard_src(src_pat, dst, nav, "misc.mv.no_match", "misc.mv.dst_dir")
    else {
        return;
    };
    let mut done = 0usize;
    for src in sources {
        let name = match src.file_name() {
            Some(n) => n,
            None => continue,
        };
        let target = dst_dir.join(name);
        if target.exists() && !force {
            print_error(&t!("misc.mv.exists", target.display()));
            continue;
        }
        if force && target.exists() {
            if target.is_dir() {
                print_error(&t!("misc.mv.error", "cannot overwrite directory with file"));
                continue;
            }
            if std::fs::remove_file(&target).is_err() {
                print_error(&t!("misc.mv.error", "cannot clear existing target"));
                continue;
            }
        }
        match std::fs::rename(&src, &target) {
            Ok(()) => done += 1,
            Err(_) => match move_by_copy(&src, &target) {
                Ok(()) => done += 1,
                Err(e) => print_error(&t!("misc.mv.error", e)),
            },
        }
    }
    if done > 0 {
        print_success(&t!("misc.mv.wild_done", done, dst_spec));
    }
}

/// Split `mv` args into operands + force flag.
/// Respects double quotes: `mv "my file.txt" sub/ -f`.
fn parse_mv_args(args: &str) -> (Vec<String>, bool) {
    let mut operands: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut force = false;
    let push_token = |tok: String, operands: &mut Vec<String>, force: &mut bool| {
        if tok == "-f" || tok == "--force" {
            *force = true;
        } else if !tok.is_empty() {
            operands.push(tok);
        }
    };
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    push_token(std::mem::take(&mut current), &mut operands, &mut force);
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        push_token(current, &mut operands, &mut force);
    }
    (operands, force)
}

// In src/shell/commands/misc.rs - Replace the cmd_venva function:

pub fn cmd_venva(args: &str, nav: &mut Navigator) -> Result<bool> {
    use crate::shell::venv::*;

    // Check if already in a venv
    if env::var("VIRTUAL_ENV").is_ok() {
        print_warning(&t!("misc.venva.already_in"));
        return Ok(false);
    }

    let current_path = nav.current_path().to_path_buf();

    let venv_path = if args.is_empty() {
        // Auto-detect venv in current directory
        let venv_names = &["venv", ".venv", "env"];
        let mut found = None;

        for name in venv_names {
            let venv_dir = current_path.join(name);
            if venv_dir.exists() {
                found = Some(venv_dir);
                break;
            }
        }

        if found.is_none() {
            print_error(&t!("misc.venva.not_found"));
            return Ok(false);
        }
        found.unwrap()
    } else {
        let path = Path::new(args);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_path.join(args)
        }
    };

    // Find the activate script
    let activate_script = get_activate_script_path(&venv_path);
    let activate_path = match activate_script {
        Some(p) => p,
        None => {
            print_error(&t!("misc.venva.no_script", venv_path.display()));
            return Ok(false);
        }
    };

    print_info(&t!("misc.venva.activating", venv_path.display()));

    // Parse and apply the environment
    #[cfg(windows)]
    let changes = parse_batch_activate(&activate_path);
    #[cfg(not(windows))]
    let changes = parse_bash_activate(&activate_path);

    match changes {
        Ok(env_changes) => {
            apply_venv_environment(env_changes);

            // Update the navigator's venv name
            let new_venv_name = detect_active_venv();
            nav.update_venv_name(new_venv_name);

            // Verify activation by checking if pip is in PATH
            check_pip_in_path(&venv_path);

            print_success(&t!("misc.venva.activated", venv_path.display()));
            print_info(&t!("misc.venva.active_info"));
        }
        Err(e) => {
            print_error(&t!("misc.venva.failed", e));
        }
    }

    Ok(false)
}

// In src/shell/commands/misc.rs - Add/enhance this function:

pub fn cmd_show_env(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let show_all = args == "--all" || args == "-a";
    let depth: Option<usize> = if let Some(depth_str) = args.strip_prefix("-d ") {
        depth_str.parse::<usize>().ok()
    } else if let Some(depth_str) = args.strip_prefix("--depth ") {
        depth_str.parse::<usize>().ok()
    } else {
        None
    };

    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                         Environment Variables                    ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    // Core venv variables (always shown)
    println!("{}", t!("misc.show_env.venv_header").green().bold());
    match std::env::var("VIRTUAL_ENV") {
        Ok(val) => println!("  VIRTUAL_ENV = {}", val.cyan()),
        Err(_) => println!("  VIRTUAL_ENV = {}", t!("misc.show_env.not_set").dimmed()),
    }
    println!();

    // PATH is always useful to see
    println!("{}", t!("misc.show_env.path_header").yellow().bold());
    if let Ok(path) = std::env::var("PATH") {
        let entries: Vec<&str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        let max_display = depth.unwrap_or(5);

        for (i, entry) in entries.iter().enumerate().take(max_display) {
            let marker =
                if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                    "🐍"
                } else {
                    "  "
                };
            println!(
                "  {}. {} {}",
                (i + 1).to_string().yellow(),
                marker,
                entry.dimmed()
            );
        }

        if entries.len() > max_display {
            let remaining = entries.len() - max_display;
            println!(
                "  {}",
                t!("misc.show_env.and_more", remaining.to_string()).dimmed()
            );
        }

        if show_all {
            println!();
            println!("{}", t!("misc.show_env.path_full").yellow().bold());
            for (i, entry) in entries.iter().enumerate() {
                let marker =
                    if entry.contains("venv") || entry.contains(".venv") || entry.contains("env") {
                        "🐍"
                    } else {
                        "  "
                    };
                println!("  {}. {} {}", (i + 1).to_string().yellow(), marker, entry);
            }
        }
    }
    println!();

    // System information
    println!("{}", t!("misc.show_env.sys_header").blue().bold());
    println!(
        "  {} {}",
        t!("misc.show_env.os_label").cyan(),
        std::env::consts::OS.cyan()
    );
    println!(
        "  {} ntc v{}",
        t!("misc.show_env.shell_label").cyan(),
        env!("CARGO_PKG_VERSION").cyan()
    );
    println!(
        "  {} {}",
        t!("misc.show_env.cwd_label").cyan(),
        _nav.display_path().dimmed()
    );
    println!();

    // Optional: Show other venv-related variables if they exist
    let other_vars = [
        "PYTHONHOME",
        "PYTHONPATH",
        "_OLD_VIRTUAL_PATH",
        "_OLD_VIRTUAL_PYTHONHOME",
        "CONDA_DEFAULT_ENV",
        "CONDA_PREFIX",
    ];

    let mut has_others = false;
    for var in &other_vars {
        if let Ok(val) = std::env::var(var) {
            if !has_others {
                println!("{}", t!("misc.show_env.other_header").magenta().bold());
                has_others = true;
            }
            println!("  {} = {}", var.cyan(), val.dimmed());
        }
    }

    if has_others {
        println!();
    }

    // Summary
    println!("{}", t!("misc.show_env.summary_header").cyan().bold());
    if std::env::var("VIRTUAL_ENV").is_ok() {
        println!(
            "  {} {} {}",
            "✅",
            t!("misc.show_env.active"),
            "ACTIVE".green().bold()
        );
        println!("  {}", t!("misc.show_env.deactivate_hint"));
    } else {
        println!(
            "  {} {} {}",
            "❌",
            t!("misc.show_env.not_active"),
            "NOT ACTIVE".red().bold()
        );
        println!("  {}", t!("misc.show_env.activate_hint"));
    }
    println!();

    Ok(false)
}

// Helper function to verify pip is accessible
fn check_pip_in_path(venv_path: &Path) {
    #[cfg(windows)]
    let pip_path = venv_path.join("Scripts").join("pip.exe");
    #[cfg(not(windows))]
    let pip_path = venv_path.join("bin").join("pip");

    if pip_path.exists() {
        print_info(&t!("misc.pip.found", pip_path.display()));
    } else {
        print_warning(&t!("misc.pip.not_found"));
    }
}

pub fn cmd_venvd(_args: &str, nav: &mut Navigator) -> Result<bool> {
    if env::var("VIRTUAL_ENV").is_err() {
        print_info(&t!("misc.venvd.not_in_venv"));
        return Ok(false);
    }
    clear_venv_environment();
    let current_path = nav.current_path().to_path_buf();
    nav.go_to(&current_path)?; // Refresh display
    print_success(&t!("misc.venvd.deactivated"));
    Ok(false)
}

pub fn cmd_pathcp(_args: &str, nav: &mut Navigator) -> Result<bool> {
    let path = nav.current_path().to_string_lossy().to_string();
    crate::output::copy_to_clipboard(&path, "path")?;
    print_success(&t!("misc.pathcp.copied", path));
    Ok(false)
}

const MAX_SAY_LOOP: i32 = 10000;

fn tokenize_quoted(input: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in input.chars() {
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

pub fn cmd_say(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize_quoted(args);

    let mut text = String::new();
    let mut loop_count: i32 = 1;
    let mut use_tabs = false; // -t → space-separated single line; else newline-separated

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token == "--loop" {
            i += 1;
            if i < tokens.len() {
                if let Ok(n) = tokens[i].parse::<i32>() {
                    if n <= 0 {
                        print_warning(&t!("misc.say.loop_warning"));
                        loop_count = 1;
                    } else if n > MAX_SAY_LOOP {
                        print_warning(&t!("misc.say.loop_exceed", MAX_SAY_LOOP));
                        loop_count = MAX_SAY_LOOP;
                    } else {
                        loop_count = n;
                    }
                } else {
                    print_warning(&t!("misc.say.loop_invalid", tokens[i]));
                }
            } else {
                print_warning(&t!("misc.say.loop_requires"));
            }
        } else if token == "-n" {
            use_tabs = false;
        } else if token == "-t" {
            use_tabs = true;
        } else if token == "--" {
            break;
        } else {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(token);
        }
        i += 1;
    }

    if text.is_empty() {
        // No text given → read piped/redirected input and echo it.
        if let Some(input) = crate::shell::stdin::read_to_string() {
            print!("{}", input);
            if !input.is_empty() && !input.ends_with('\n') {
                println!();
            }
            return Ok(false);
        }
        print_error(&t!("misc.say.usage"));
        print_info(&t!("misc.say.example1"));
        print_info(&t!("misc.say.example2"));
        return Ok(false);
    }

    if use_tabs {
        let line = (0..loop_count)
            .map(|_| text.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", line);
    } else {
        for _ in 0..loop_count {
            println!("{}", text);
        }
    }

    Ok(false)
}

pub fn cmd_fallback(
    input: &str,
    nav: &mut Navigator,
    _watcher_handle: &Option<Arc<watcher::WatcherHandle>>,
) -> Result<bool> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();

    if cmd.starts_with('@') && cmd.len() > 1 {
        let tp_name = &cmd[1..];
        if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
            TeleportManager::jump_by_name(nav, tp_name)?;
            return Ok(false);
        }
    }

    // Try math expression evaluation for inputs that look like math
    let trimmed = input.trim();
    if crate::math::is_math_expression(trimmed) {
        if let Err(e) = crate::math::run(trimmed) {
            print_error(&format!("{}", e));
        }
        return Ok(false);
    }

    let cmd_part = input.trim();
    if !cmd_part.is_empty() {
        if let Err(e) = execute_system_command(cmd_part, nav.current_path()) {
            print_error(&format!("{}", e));
            return Ok(false);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_clear_suppresses_watch_refresh() {
        let _g = crate::utils::test_support::lock_globals();
        let mut nav = Navigator::new().unwrap();
        let _ = crate::shell::helpers::take_watch_refresh_suppressed();
        let mut cfg = Config::default();
        cfg.dir_view_enabled = false;
        Config::with_test_config(cfg, || {
            cmd_clear("", &mut nav).unwrap();
        });
        assert!(crate::shell::helpers::take_watch_refresh_suppressed());
    }

    #[test]
    fn parse_mv_args_basic_and_force() {
        let (ops, force) = parse_mv_args("a.txt b.txt");
        assert_eq!(ops, vec!["a.txt", "b.txt"]);
        assert!(!force);
        let (ops, force) = parse_mv_args("a.txt sub/ -f");
        assert_eq!(ops, vec!["a.txt", "sub/"]);
        assert!(force);
        let (ops, force) = parse_mv_args("--force \"my file.txt\" \"new name.txt\"");
        assert_eq!(ops, vec!["my file.txt", "new name.txt"]);
        assert!(force);
    }

    #[test]
    fn ne_parser_create_modes() {
        assert_eq!(
            parse_ne_action("--init a.py b.html -I a.py"),
            NeAction::Create {
                kind: NeCreateKind::Init,
                targets: vec![PathBuf::from("a.py"), PathBuf::from("b.html")],
                open: Some(0),
            }
        );
        assert_eq!(
            parse_ne_action("--super a.py,b.html -I b.html"),
            NeAction::Create {
                kind: NeCreateKind::Super,
                targets: vec![PathBuf::from("a.py"), PathBuf::from("b.html")],
                open: Some(1),
            }
        );
        assert_eq!(
            parse_ne_action("--init a b"),
            NeAction::Create {
                kind: NeCreateKind::Init,
                targets: vec![PathBuf::from("a"), PathBuf::from("b")],
                open: None,
            }
        );
    }

    #[test]
    fn ne_parser_open_flag_errors() {
        assert_eq!(
            parse_ne_action("--init a b -I"),
            NeAction::Error(NeError::UsageOpen)
        );
        assert_eq!(
            parse_ne_action("--init a b -I a b"),
            NeAction::Error(NeError::UsageOpen)
        );
        assert_eq!(
            parse_ne_action("--init a b -I c"),
            NeAction::Error(NeError::OpenNotCreated("c".to_string()))
        );
        assert_eq!(
            parse_ne_action("--init"),
            NeAction::Error(NeError::MissingCreateTarget)
        );
    }

    #[test]
    fn ne_parser_hyper_and_plain_open() {
        assert_eq!(
            parse_ne_action("--hyper demo --lang c -I"),
            NeAction::Hyper {
                name: "demo".to_string(),
                lang: "c".to_string(),
                open: true,
            }
        );
        assert_eq!(
            parse_ne_action("--hyper demo -I extra"),
            NeAction::Error(NeError::UsageOpen)
        );
        assert_eq!(
            parse_ne_action("--hyper demo extra"),
            NeAction::Error(NeError::HyperUsage)
        );
        assert_eq!(
            parse_ne_action("src/main.rs:42"),
            NeAction::Open {
                path: PathBuf::from("src/main.rs"),
                line: Some(42),
            }
        );
        assert_eq!(parse_ne_action("."), NeAction::Welcome);
    }

    #[test]
    fn parse_mv_args_needs_two_operands() {
        let (ops, _) = parse_mv_args("only-one");
        assert_eq!(ops.len(), 1);
        let (ops, _) = parse_mv_args("");
        assert!(ops.is_empty());
    }

    /// Fixture with two savepoints (`tp1` holding files, empty `tp2`) in a
    /// temp cwd. Returns the fixture root; the caller seeds `cfg.teleports`.
    fn tp_pair_fixture() -> (crate::utils::test_support::TempCwd, std::path::PathBuf) {
        let cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("tp1/file.txt", "hello"), ("tp1/sub/deep.txt", "deep")],
            &["tp1", "tp1/sub", "tp2"],
        );
        let root = std::env::current_dir().unwrap();
        (cwd, root)
    }

    fn tp_pair_config(root: &std::path::Path) -> crate::config::Config {
        let mut cfg = crate::config::Config::default();
        cfg.teleports.insert("tp1".to_string(), root.join("tp1"));
        cfg.teleports.insert("tp2".to_string(), root.join("tp2"));
        cfg
    }

    #[test]
    fn mv_cp_tp_file_into_tp_dir() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            // cp @tp1/file.txt @tp2 -f (src stays).
            assert!(cmd_cp("@tp1/file.txt @tp2 -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("file.txt")).unwrap(),
                b"hello"
            );
            assert!(root.join("tp1").join("file.txt").exists());
            // mv @tp1/file.txt @tp2 -f with a fresh src (mv consumes).
            std::fs::write(root.join("tp1").join("move.txt"), "bye").unwrap();
            assert!(cmd_mv("@tp1/move.txt @tp2 -f", &mut nav).is_ok());
            assert!(!root.join("tp1").join("move.txt").exists());
            assert_eq!(
                std::fs::read(root.join("tp2").join("move.txt")).unwrap(),
                b"bye"
            );
        });
    }

    #[test]
    fn mv_cp_tp_dir_onto_new() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            // cp @tp1 @tp2/new copies the whole tree.
            assert!(cmd_cp("@tp1 @tp2/new", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("new").join("file.txt")).unwrap(),
                b"hello"
            );
            assert_eq!(
                std::fs::read(root.join("tp2").join("new").join("sub").join("deep.txt")).unwrap(),
                b"deep"
            );
            assert!(root.join("tp1").exists());
            // mv @tp1 @tp2/new2 relocates the savepoint dir itself.
            assert!(cmd_mv("@tp1 @tp2/new2", &mut nav).is_ok());
            assert!(!root.join("tp1").exists());
            assert!(root.join("tp2").join("new2").join("file.txt").exists());
        });
    }

    #[test]
    fn mv_cp_tp_file_onto_file() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            std::fs::write(root.join("tp2").join("file.txt"), "old").unwrap();
            // cp -f overwrites.
            assert!(cmd_cp("@tp1/file.txt @tp2/file.txt -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("file.txt")).unwrap(),
                b"hello"
            );
            // mv -f overwrites with a fresh src.
            std::fs::write(root.join("tp1").join("move.txt"), "bye").unwrap();
            std::fs::write(root.join("tp2").join("move.txt"), "old").unwrap();
            assert!(cmd_mv("@tp1/move.txt @tp2/move.txt -f", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("tp2").join("move.txt")).unwrap(),
                b"bye"
            );
            assert!(!root.join("tp1").join("move.txt").exists());
        });
    }

    #[test]
    fn mv_cp_tp_dir_onto_relative() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            // cp @tp1 test — dir onto a bare relative name.
            assert!(cmd_cp("@tp1 test", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("test").join("file.txt")).unwrap(),
                b"hello"
            );
            // cp @tp1 ./test2 — same with an explicit `./` prefix.
            assert!(cmd_cp("@tp1 ./test2", &mut nav).is_ok());
            assert_eq!(
                std::fs::read(root.join("test2").join("sub").join("deep.txt")).unwrap(),
                b"deep"
            );
            assert!(root.join("tp1").exists());
        });
    }

    #[test]
    fn mkf_mkd_rmf_rmd_accept_teleport_savepoints() {
        let (_cwd, root) = tp_pair_fixture();
        crate::config::Config::with_test_config(tp_pair_config(&root), || {
            let mut nav = Navigator::new().unwrap();
            // mkd / mkf create inside the savepoint.
            assert!(cmd_mkd("@tp2/build", &mut nav).is_ok());
            assert!(root.join("tp2").join("build").is_dir());
            assert!(cmd_mkf("@tp2/build/notes.md", &mut nav).is_ok());
            assert!(root.join("tp2").join("build").join("notes.md").is_file());
            // rmf removes a savepoint file and expands a `*` pattern there.
            std::fs::write(root.join("tp2").join("build").join("a.log"), "a").unwrap();
            assert!(cmd_rmf("@tp2/build/*.log", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").join("a.log").exists());
            assert!(cmd_rmf("@tp2/build/notes.md", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").join("notes.md").exists());
            // rmd removes the directory inside the savepoint.
            assert!(cmd_rmd("@tp2/build", &mut nav).is_ok());
            assert!(!root.join("tp2").join("build").exists());
        });
    }

    #[test]
    fn rmf_pattern_expands_to_matching_files_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.log"), "a").unwrap();
        std::fs::write(dir.path().join("b.log"), "b").unwrap();
        std::fs::write(dir.path().join("c.txt"), "c").unwrap();
        std::fs::create_dir(dir.path().join("d.log")).unwrap();

        let nav = Navigator::for_test(dir.path());

        // `*.log` matches files only — the `d.log` directory is skipped.
        let got = expand_rmf_pattern("*.log", &nav);
        assert_eq!(
            got,
            vec![dir.path().join("a.log"), dir.path().join("b.log"),]
        );
        // Prefix form and no-match form.
        assert_eq!(expand_rmf_pattern("a*", &nav).len(), 1);
        assert!(expand_rmf_pattern("*.zzz", &nav).is_empty());

        // `sub/*.log` scopes into a subdirectory.
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/e.log"), "e").unwrap();
        let got = expand_rmf_pattern("sub/*.log", &nav);
        assert_eq!(got, vec![dir.path().join("sub/e.log")]);
    }

    #[test]
    fn cp_pattern_copies_matches_into_dirs() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("a.py", "a"),
                ("b.py", "b"),
                ("c.txt", "c"),
                ("sub/d.py", "d"),
            ],
            &["out"],
        );
        let mut nav = Navigator::new().unwrap();
        // New dst directory is auto-created; only matching files move in.
        assert!(cmd_cp("*.py newdir", &mut nav).is_ok());
        assert_eq!(std::fs::read("newdir/a.py").unwrap(), b"a");
        assert_eq!(std::fs::read("newdir/b.py").unwrap(), b"b");
        assert!(!std::path::Path::new("newdir/c.txt").exists());
        // `<dir>/*.py` scopes into a subdirectory.
        assert!(cmd_cp("sub/*.py out", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/d.py").unwrap(), b"d");
        // Sources stay in place for `cp`.
        assert!(std::path::Path::new("a.py").exists());
        assert!(std::path::Path::new("sub/d.py").exists());
    }

    #[test]
    fn mv_pattern_moves_matches_into_dir() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "a"), ("b.py", "b")],
            &["dest"],
        );
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_mv("*.py dest", &mut nav).is_ok());
        assert!(!std::path::Path::new("a.py").exists());
        assert!(!std::path::Path::new("b.py").exists());
        assert_eq!(std::fs::read("dest/a.py").unwrap(), b"a");
        assert_eq!(std::fs::read("dest/b.py").unwrap(), b"b");
    }

    #[test]
    fn cp_pattern_dst_must_be_a_dir() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "a"), ("dst.txt", "x")],
            &[],
        );
        let mut nav = Navigator::new().unwrap();
        // A plain file destination is refused; nothing is copied.
        assert!(cmd_cp("*.py dst.txt", &mut nav).is_ok());
        assert_eq!(std::fs::read("dst.txt").unwrap(), b"x");
        assert!(std::path::Path::new("a.py").exists());
    }

    #[test]
    fn cp_pattern_no_match_creates_nothing() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(&[("a.py", "a")], &[]);
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_cp("*.zzz out", &mut nav).is_ok());
        // No match → no destination directory is created.
        assert!(!std::path::Path::new("out").exists());
    }

    #[test]
    fn cp_pattern_force_overwrites_targets() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[("a.py", "new"), ("out/a.py", "old")],
            &["out"],
        );
        let mut nav = Navigator::new().unwrap();
        // Without -f the existing target is skipped.
        assert!(cmd_cp("*.py out", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/a.py").unwrap(), b"old");
        // With -f it is overwritten.
        assert!(cmd_cp("*.py out -f", &mut nav).is_ok());
        assert_eq!(std::fs::read("out/a.py").unwrap(), b"new");
    }

    #[test]
    fn rmd_star_expands_to_subdirectories_only() {
        let tmp = tempfile::tempdir().unwrap();
        for d in ["alpha", "beta"] {
            std::fs::create_dir(tmp.path().join(d)).unwrap();
        }
        std::fs::write(tmp.path().join("notes.txt"), "x").unwrap();

        let nav = Navigator::for_test(tmp.path());
        let got = expand_rmd_pattern("*", &nav);
        assert_eq!(got.len(), 2, "got: {got:?}");
        assert!(got.iter().all(|p| p.is_dir()), "got: {got:?}");
        assert!(got.iter().any(|p| p.ends_with("alpha")));
        assert!(got.iter().any(|p| p.ends_with("beta")));
        assert!(!got.iter().any(|p| p.ends_with("notes.txt")));
    }

    #[test]
    fn rmd_pattern_accepts_a_parent_directory() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("keep/one")).unwrap();
        std::fs::create_dir_all(tmp.path().join("keep/two")).unwrap();
        std::fs::create_dir_all(tmp.path().join("other/three")).unwrap();

        let nav = Navigator::for_test(tmp.path());
        let pattern = format!("{}/*", tmp.path().join("keep").display());
        let got = expand_rmd_pattern(&pattern, &nav);
        assert_eq!(got.len(), 2, "got: {got:?}");
        let keep = tmp.path().join("keep");
        assert!(got.iter().all(|p| p.parent() == Some(keep.as_path())));
    }

    #[test]
    fn rmd_pattern_ignores_files_named_like_the_stub() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("old_dir")).unwrap();
        std::fs::write(tmp.path().join("old_file.txt"), "x").unwrap();

        let nav = Navigator::for_test(tmp.path());
        let got = expand_rmd_pattern("old*", &nav);
        assert_eq!(got.len(), 1, "got: {got:?}");
        assert!(got[0].ends_with("old_dir"));
    }

    #[test]
    fn ne_refuses_nested_editor_when_embedded() {
        // Guard must trigger before any editor launch is attempted.
        std::env::set_var("NTC_IN_EDITOR", "1");
        let mut nav = Navigator::new().expect("nav");
        let result = cmd_ne("some_file.txt", &mut nav);
        std::env::remove_var("NTC_IN_EDITOR");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_deadlock_vault_matches_canonical_path() {
        // Isolate first: this test creates files next to `vault_path()`, which
        // must never be the real user vault.
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        let vault = crate::deadlock::vault::vault_path();
        assert!(
            vault.starts_with(home.path()),
            "test isolation failed: {} is outside {}",
            vault.display(),
            home.path().display()
        );
        // Direct path equality.
        assert!(is_deadlock_vault(&vault));
        std::fs::create_dir_all(vault.parent().unwrap()).unwrap();
        std::fs::write(&vault, "x").ok();
        assert!(is_deadlock_vault(&vault));
        // A different file is not the vault.
        let other = vault.with_file_name("not_the_vault.txt");
        std::fs::write(&other, "x").ok();
        assert!(!is_deadlock_vault(&other));
    }

    #[test]
    fn deadlock_vault_path_is_isolated_in_tests() {
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        let vault = crate::deadlock::vault::vault_path();
        assert!(
            vault.starts_with(home.path()),
            "vault path escaped the temp config home: {}",
            vault.display()
        );
        // And it must not point into the real user config directory, even
        // though `dirs::config_dir()` ignores NTC_CONFIG_HOME on Windows.
        if let Some(real) = dirs::config_dir() {
            let real_vault_dir = real.join("ntc").join("deadlock");
            assert!(
                !vault.starts_with(&real_vault_dir),
                "vault path resolves to real user data: {}",
                vault.display()
            );
        }
    }

    #[test]
    fn cmd_rmf_refuses_deadlock_vault_for_non_root() {
        let home = crate::utils::test_support::IsolatedConfigHome::new();
        // Only meaningful when NOT root/admin (root can remove anything).
        if is_root_or_admin() {
            eprintln!("skipped: running as root/admin");
            return;
        }
        let vault = crate::deadlock::vault::vault_path();
        // Hard safety net: never write test garbage into the real vault.
        assert!(
            vault.starts_with(home.path()),
            "refusing to run: {} is outside the isolated config home",
            vault.display()
        );
        std::fs::create_dir_all(vault.parent().unwrap()).ok();
        std::fs::write(&vault, "dummy vault content").ok();
        assert!(vault.exists());

        let mut nav = Navigator::new().unwrap();
        let result = cmd_rmf(&vault.to_string_lossy(), &mut nav);
        assert!(result.is_ok());
        // The vault must survive the attempt.
        assert!(vault.exists());

        // Also blocked via wildcard pattern.
        let glob = vault
            .parent()
            .unwrap()
            .join("deadlock.*")
            .to_string_lossy()
            .to_string();
        let result = cmd_rmf(&glob, &mut nav);
        assert!(result.is_ok());
        assert!(vault.exists());
    }
}
