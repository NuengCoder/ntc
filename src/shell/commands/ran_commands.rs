use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::ranfile::parser::{self, RANFILE_NAME, RANFILE_TEMPLATE};
use crate::ranfile::runner::run_target;
use crate::shell::alias::expand_command_line_interactive;
use crate::t;

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub(crate) fn cmd_ran(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let cwd = nav.current_path().to_path_buf();

    let mut exec = |cmd: &str, path: &Path| -> Result<bool> {
        let trimmed = cmd.trim();
        // "run " prefix → direct system execution, no alias expansion, no built-in check
        if let Some(rest) = trimmed.strip_prefix("run ") {
            return Ok(run_target_cmd(rest, path));
        }
        // No "run " prefix → full ntc pipeline (interactive selects allowed)
        let expanded = match expand_command_line_interactive(cmd, nav)? {
            None => return Ok(false),
            Some(s) => s,
        };
        let lower = expanded.trim().to_lowercase();
        if lower.starts_with("ran ") || lower == "ran" {
            print_error(&t!("ran.calling_not_allowed"));
            return Ok(false);
        }
        match super::execute_command(&expanded, nav, &None) {
            Ok(_) => {}
            Err(e) => {
                print_error(&format!("{}", e));
                return Ok(false);
            }
        }
        Ok(true)
    };

    if args.is_empty() || args == "standalone" {
        if let Err(e) = run_target("standalone", &cwd, &mut exec) {
            print_error(&t!("ran.error_prefix", e));
        }
        return Ok(false);
    }

    let (sub, rest) = match args.split_once(' ') {
        Some((a, b)) => (a, b.trim()),
        None => (args, ""),
    };

    match sub {
        "init" => cmd_ran_init(rest),
        "deinit" => cmd_ran_deinit(rest),
        "help" => {
            print_ran_usage();
            Ok(false)
        }
        "list" | "ls" => cmd_ran_list(nav),
        target => {
            let target = target.to_string();
            if let Err(e) = run_target(&target, &cwd, &mut exec) {
                print_error(&t!("ran.error_prefix", e));
            }
            Ok(false)
        }
    }
}

fn cmd_ran_init(_args: &str) -> Result<bool> {
    let cwd = std::env::current_dir()?;
    let path = cwd.join(RANFILE_NAME);

    if path.exists() {
        print_error(&t!("ran.already_exists", RANFILE_NAME, cwd.display()));
        return Ok(false);
    }

    std::fs::write(&path, RANFILE_TEMPLATE)?;
    print_success(&t!("ran.created", path.display()));
    Ok(false)
}

fn cmd_ran_deinit(_args: &str) -> Result<bool> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!("ran.get_cwd_failed", e));
            return Ok(false);
        }
    };
    let path = cwd.join(RANFILE_NAME);

    if !path.exists() {
        print_error(&t!("ran.not_found", RANFILE_NAME, cwd.display()));
        return Ok(false);
    }

    match std::fs::remove_file(&path) {
        Ok(_) => {
            print_success(&t!("ran.removed", path.display()));
            Ok(false)
        }
        Err(e) => {
            print_error(&t!("ran.remove_failed", path.display(), e));
            Ok(false)
        }
    }
}

fn cmd_ran_list(nav: &mut Navigator) -> Result<bool> {
    let cwd = nav.current_path();
    match parser::parse(cwd) {
        Ok(ranfile) => parser::list_targets(&ranfile),
        Err(e) => print_error(&format!("{}", e)),
    }
    Ok(false)
}

fn run_target_cmd(cmd: &str, cwd: &Path) -> bool {
    use crate::output::print_info;
    print_info(&t!("ran.executing", cmd));
    println!();
    match super::run_system_command(cmd, cwd) {
        Ok(status) => {
            println!();
            if status.success() {
                crate::output::print_success(&t!("ran.completed"));
                true
            } else {
                match status.code() {
                    Some(code) => crate::output::print_error(&t!("ran.exit_code", code)),
                    None => crate::output::print_warning(&t!("ran.terminated")),
                }
                false
            }
        }
        Err(e) => {
            println!();
            crate::output::print_error(&t!("ran.exec_failed", e));
            false
        }
    }
}

fn print_ran_usage() {
    println!("{}", t!("ran.help_title").cyan().bold());
    println!();
    println!("{}", t!("ran.help_usage"));
    println!("{}", t!("ran.help_standalone"));
    println!("{}", t!("ran.help_target"));
    println!("{}", t!("ran.help_init_fmt", RANFILE_NAME));
    println!("{}", t!("ran.help_deinit_fmt", RANFILE_NAME));
    println!("{}", t!("ran.help_list"));
    println!("{}", t!("ran.help_help"));
    println!();
    println!("NTCRANFILE.toml example:");
    println!("  [vars]");
    println!("  CC = \"gcc\"");
    println!("  FLAGS = \"-Wall -O2\"");
    println!();
    println!("  [targets.standalone]");
    println!("  deps = []");
    println!("  cmd = \"echo 'Hello from ran!'\"");
    println!();
    println!("  [targets.build]");
    println!("  deps = [\"lint\"]");
    println!("  cmd = \"$(CC) $(FLAGS) main.c -o app\"");
    println!();
    println!("  [targets.clean]");
    println!("  deps = []");
    println!("  cmd = \"rm -rf *.o app\"");
    println!();
    println!("{}", t!("ran.section_vars").yellow());
    println!("{}", t!("ran.desc_vars"));
    println!("{}", t!("ran.desc_vars2"));
    println!();
    println!("{}", t!("ran.section_deps").yellow());
    println!("{}", t!("ran.desc_deps"));
    println!();
    println!("{}", t!("ran.section_aliases").yellow());
    println!("{}", t!("ran.desc_aliases"));
    println!("{}", t!("ran.desc_aliases2"));
    println!();
    println!("{}", t!("ran.section_reserved").red());
    println!("{}", t!("ran.desc_reserved"));
    println!();
    println!("{}", t!("ran.section_note").yellow());
    println!("{}", t!("ran.desc_note"));
}
