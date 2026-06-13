use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::ranfile::parser::{self, RANFILE_NAME, RANFILE_TEMPLATE};
use crate::ranfile::runner::run_target;
use crate::shell::alias::expand_command_line;

use anyhow::Result;
use std::path::Path;
use colored::Colorize;

pub(crate) fn cmd_ran(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let cwd = nav.current_path().to_path_buf();

    let mut exec = |cmd: &str, path: &Path| -> Result<bool> {
        let trimmed = cmd.trim();
        // "run " prefix → direct system execution, no alias expansion, no built-in check
        if let Some(rest) = trimmed.strip_prefix("run ") {
            return Ok(run_target_cmd(rest, path));
        }
        // No "run " prefix → full ntc pipeline
        let expanded = expand_command_line(cmd);
        for part in expanded {
            let lower = part.trim().to_lowercase();
            if lower.starts_with("ran ") || lower == "ran" {
                print_error("Calling 'ran' from within a NTCRANFILE target is not allowed");
                return Ok(false);
            }
            match super::execute_command(&part, nav, &None) {
                Ok(_) => {}
                Err(e) => {
                    print_error(&format!("{}", e));
                    return Ok(false);
                }
            }
        }
        Ok(true)
    };

    if args.is_empty() || args == "standalone" {
        if let Err(e) = run_target("standalone", &cwd, &mut exec) {
            print_error(&format!("ran: {}", e));
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
        "help" => { print_ran_usage(); Ok(false) }
        "list" | "ls" => cmd_ran_list(nav),
        target => {
            let target = target.to_string();
            if let Err(e) = run_target(&target, &cwd, &mut exec) {
                print_error(&format!("ran: {}", e));
            }
            Ok(false)
        }
    }
}

fn cmd_ran_init(_args: &str) -> Result<bool> {
    let cwd = std::env::current_dir()?;
    let path = cwd.join(RANFILE_NAME);

    if path.exists() {
        print_error(&format!("{} already exists in {}", RANFILE_NAME, cwd.display()));
        return Ok(false);
    }

    std::fs::write(&path, RANFILE_TEMPLATE)?;
    print_success(&format!("Created {}", path.display()));
    Ok(false)
}

fn cmd_ran_deinit(_args: &str) -> Result<bool> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&format!("Failed to get current directory: {}", e));
            return Ok(false);
        }
    };
    let path = cwd.join(RANFILE_NAME);

    if !path.exists() {
        print_error(&format!("No {} found in {}", RANFILE_NAME, cwd.display()));
        return Ok(false);
    }

    match std::fs::remove_file(&path) {
        Ok(_) => {
            print_success(&format!("Removed {}", path.display()));
            Ok(false)
        }
        Err(e) => {
            print_error(&format!("Failed to remove {}: {}", path.display(), e));
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

/// Run a command for a ran target, return true if the exit status was success.
fn run_target_cmd(cmd: &str, cwd: &Path) -> bool {
    use crate::output::print_info;
    print_info(&format!("Executing: {}", cmd));
    println!();
    match super::run_system_command(cmd, cwd) {
        Ok(status) => {
            println!();
            if status.success() {
                crate::output::print_success("Command completed successfully.");
                true
            } else {
                match status.code() {
                    Some(code) => crate::output::print_error(&format!("Command exited with code: {}", code)),
                    None => crate::output::print_warning("Command terminated (Ctrl+C)"),
                }
                false
            }
        }
        Err(e) => {
            println!();
            crate::output::print_error(&format!("Failed to execute command: {}", e));
            false
        }
    }
}

fn print_ran_usage() {
    println!("{}", "ran — ntc task runner (like Make but ntc-native)".cyan().bold());
    println!();
    println!("{}", "Usage:");
    println!("  ran                        Run the 'standalone' target (default)");
    println!("  ran <target>               Run a specific target");
    println!("  ran init                   Create a {} in the current directory", RANFILE_NAME);
    println!("  ran deinit                 Remove {} from the current directory", RANFILE_NAME);
    println!("  ran list                   List all available targets");
    println!("  ran help                   Show this help");
    println!();
    println!("{}", "NTCRANFILE.toml example:");
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
    println!("{}", "Variables:".yellow());
    println!("  Define variables under [vars] and reference them with $(VAR_NAME) in commands.");
    println!("  They work like Make variables but use $(...) syntax.");
    println!();
    println!("{}", "Dependencies:".yellow());
    println!("  Targets can depend on other targets. Dependencies run first, in order.");
    println!();
    println!("{}", "Run aliases:".yellow());
    println!("  Commands are expanded through ntc's run-alias system, so aliases work too.");
    println!("  Prefix with 'run ' to bypass alias expansion and run directly as a system command.");
    println!();
    println!("{}", "Reserved target names:".red());
    println!("  init, deinit, help, list, ls — these cannot be used as target names.");
    println!();
    println!("{}", "Note:".yellow());
    println!("  Calling 'ran' from within a NTCRANFILE target command is not allowed.");
}
