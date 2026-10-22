use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::script;
use crate::shell::alias::expand_command_line_interactive;
use crate::t;

use anyhow::Result;
use colored::Colorize;
use std::path::{Path, PathBuf};

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
        super::execute_command(&expanded, nav, &None)?;
        Ok(true)
    };

    print_info(&t!("scr.start", path.display()));
    script::runner::run_script(&path, script_args, continue_on_err, &mut exec)?;
    Ok(())
}

/// Resolve a (possibly relative) script path against the navigator's cwd.
fn resolve_script_path(file: &str, cwd: &Path) -> PathBuf {
    let p = Path::new(file);
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
    match super::run_system_command(cmd, cwd) {
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
