//! `scr` command — run `.ntc` script files.

pub(crate) mod completion;
pub(crate) mod help;
pub(crate) mod script;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::check::{self as check_common, formats as check_formats};
use super::super::{execute_command, run_system_command};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::alias::expand_command_line_interactive;
use crate::t;

use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::cell::Cell;
use std::path::{Path, PathBuf};

// How many `.ntc` scripts are on the stack right now.
thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// A script may call another script by file name, but not forever.
const MAX_SCRIPT_DEPTH: usize = 8;

/// Pops the depth counter however the script exits.
struct DepthGuard;

impl Drop for DepthGuard {
    fn drop(&mut self) {
        DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "scr",
    aliases: &["script"],
    help_topic: Some("scr"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_scr),
};

pub(crate) fn cmd_scr(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    let mut continue_on_err = false;
    if let Some(first) = tokens.first() {
        if *first == "-c" || *first == "--continue" {
            continue_on_err = true;
            i = 1;
        }
    }

    let sub = tokens.get(i).copied().unwrap_or("");
    match sub {
        "" => script::print_usage(),
        "help" | "-h" | "--help" => script::print_usage(),
        "init" => cmd_scr_init(tokens.get(i + 1).copied().unwrap_or("")),
        "ls" | "list" => cmd_scr_ls(nav),
        // Must come before the run-a-file arm so `scr check` never tries to
        // execute a file named `check`.
        "check" => {
            let cwd = nav.current_path().to_path_buf();
            let target = tokens[i + 1..].join(" ");
            check_common::run("scr", &target, &cwd, ".ntc", check_formats::script);
        }
        file => {
            let script_args: Vec<String> = tokens[i + 1..].iter().map(|s| s.to_string()).collect();
            cmd_scr_run(file, &script_args, continue_on_err, nav)?;
        }
    }
    Ok(false)
}

/// `scr init [name]` — create a template script.
fn cmd_scr_init(name: &str) {
    let file = if name.is_empty() {
        "script.ntc".to_string()
    } else if name.ends_with(&format!(".{}", script::SCRIPT_EXT)) {
        name.to_string()
    } else {
        format!("{}.{}", name, script::SCRIPT_EXT)
    };

    let path = match std::env::current_dir() {
        Ok(d) => d.join(&file),
        Err(e) => {
            print_error(&format!("{}", e));
            return;
        }
    };

    if path.exists() {
        print_error(&t!("scr.init_exists", path.display()));
        return;
    }

    match std::fs::write(&path, script::script_template()) {
        Ok(_) => print_success(&t!("scr.init_created", path.display())),
        Err(e) => print_error(&format!("{}", e)),
    }
}

/// `scr ls` — list `*.ntc` scripts in the current directory.
fn cmd_scr_ls(nav: &mut Navigator) {
    let cwd = nav.current_path().to_path_buf();
    match script::list_scripts(&cwd) {
        Ok(scripts) if scripts.is_empty() => {
            print_warning(&t!("scr.ls_empty", cwd.display()));
        }
        Ok(scripts) => {
            println!("{}", t!("scr.ls_title").cyan().bold());
            for path in scripts {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                println!("  {}", name.green());
            }
        }
        Err(e) => print_error(&format!("{}", e)),
    }
}

/// `scr [-c] <file.ntc> [args...]` — run a script.
fn cmd_scr_run(
    file: &str,
    script_args: &[String],
    continue_on_err: bool,
    nav: &mut Navigator,
) -> Result<()> {
    let path = resolve_script_path(file, nav.current_path());
    if !path.is_file() {
        print_error(&t!("scr.not_found", path.display(), ""));
        return Ok(());
    }

    let depth = DEPTH.with(|d| d.get());
    if depth >= MAX_SCRIPT_DEPTH {
        print_error(&t!("scr.nested_too_deep", MAX_SCRIPT_DEPTH));
        return Ok(());
    }
    DEPTH.with(|d| d.set(depth + 1));
    let _guard = DepthGuard;

    let cwd = nav.current_path().to_path_buf();
    let mut exec = |cmd: &str, force: bool| -> Result<bool> {
        let trimmed = cmd.trim();
        let lower = trimmed.to_lowercase();
        if lower == "scr"
            || lower == "script"
            || lower.starts_with("scr ")
            || lower.starts_with("script ")
        {
            print_error(&t!("scr.calling_not_allowed"));
            return Ok(false);
        }
        if force {
            return Ok(run_system_ok(trimmed, &cwd));
        }
        let expanded = match expand_command_line_interactive(trimmed, nav)? {
            None => return Ok(false),
            Some(s) => s,
        };
        if expanded.trim().is_empty() {
            return Ok(true);
        }
        execute_command(&expanded, nav, &None)?;
        Ok(true)
    };

    print_info(&t!("scr.start", path.display()));
    script::runner::run_script(&path, script_args, continue_on_err, &mut exec, &mut read_line)
        .map(|_| ())
}

/// Read one line for the script `input` statement.
///
/// The interactive editor is not running while a script executes, so plain
/// stdin is free here. `None` means end of input.
fn read_line(prompt: &str) -> Option<String> {
    use std::io::Write;
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line.trim_end_matches(['\r', '\n']).to_string()),
    }
}

/// True when the first word of `line` names an existing `*.ntc` script, so a
/// typed `deploy.ntc` runs the script instead of hitting the OS.
pub(crate) fn looks_like_script(line: &str, cwd: &Path) -> bool {
    let first = line.split_whitespace().next().unwrap_or("");
    let token = first.trim_matches('"').trim_matches('\'');
    if !token.to_ascii_lowercase().ends_with(&format!(".{}", script::SCRIPT_EXT)) {
        return false;
    }
    resolve_script_path(token, cwd).is_file()
}

/// Run `line` as a script invocation (`deploy.ntc arg1 arg2`).
pub(crate) fn run_direct(line: &str, nav: &mut Navigator) -> Result<bool> {
    cmd_scr(line, nav)
}

/// Why `line` may not be piped or redirected, when it names a script that
/// declared `set-pipe = fail`.
///
/// The pipeline asks before running the stage: a script that cannot be piped
/// must not start at all, otherwise it would run and its output would be
/// thrown away.
pub(crate) fn pipe_refusal(line: &str, cwd: &Path) -> Option<String> {
    // Both `deploy.ntc …` and `scr deploy.ntc …` name the same file, so the
    // directive cannot be sidestepped by spelling the command in full.
    let mut rest = line.trim();
    let first = rest.split_whitespace().next().unwrap_or("");
    let first_lower = first.to_ascii_lowercase();
    if first_lower == "scr" || first_lower == "script" {
        rest = rest[first.len()..].trim_start();
    }
    if !looks_like_script(rest, cwd) {
        return None;
    }
    let token = rest
        .split_whitespace()
        .next()?
        .trim_matches('"')
        .trim_matches('\'');
    let path = resolve_script_path(token, cwd);
    let content = std::fs::read_to_string(&path).ok()?;
    if script::directives::directives(&content).pipe {
        return None;
    }
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| token.to_string());
    Some(t!("scr.pipe_refused", name))
}

/// Resolve a (possibly relative) script path against the navigator's cwd.
fn resolve_script_path(file: &str, cwd: &Path) -> PathBuf {
    let expanded = crate::utils::paths::expand_path_str(file);
    let p = Path::new(&expanded);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}

/// Run a system command and report success/failure like ran does.
fn run_system_ok(cmd: &str, cwd: &Path) -> bool {
    print_info(&t!("scr.syscmd", cmd));
    println!();
    match run_system_command(cmd, cwd) {
        Ok(status) => {
            println!();
            if status.success() {
                true
            } else {
                match status.code() {
                    Some(code) => print_error(&t!("scr.syscmd_exit", code)),
                    None => print_warning(&t!("scr.syscmd_terminated")),
                }
                false
            }
        }
        Err(e) => {
            println!();
            print_error(&format!("{}", e));
            false
        }
    }
}
