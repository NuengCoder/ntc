use crate::cli::helpers::{print_help, spawn_new_terminal};
use crate::navigator::Navigator;
use crate::shell::commands::execute_command;
use crate::t;
use anyhow::Result;
use colored::*;

/// When set to `"1"`, a `ntc <command>` child stays in an interactive shell
/// after running the command (used by `ntc -n <command>`).
pub(crate) const STAY_ENV: &str = "NTC_STAY";

/// Parse command-line arguments and execute the appropriate action.
///
/// Kept surface:
/// - `ntc` (no args) → `Ok(true)`, launch interactive shell
/// - `ntc <command> [args...]` → expand ral aliases, run through the shell, `Ok(false)`
/// - `ntc -n` / `ntc --new [command...]` → open a new terminal (stay interactive), `Ok(false)`
/// - `ntc --upgrade` → self-upgrade, `Ok(false)`
/// - `ntc --version` / `ntc -v` → version, `Ok(false)`
/// - `ntc --help` / `ntc -h` → minimal help, `Ok(false)`
pub fn run_cli() -> Result<bool> {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() <= 1 {
        return Ok(true);
    }
    let args = &raw_args[1..];

    match args[0].as_str() {
        "--version" | "-v" => {
            println!(
                "{}",
                format!(
                    "{} ({})",
                    t!("msg.version.format", env!("CARGO_PKG_VERSION")),
                    env!("BUILD_GIT_HASH")
                )
                .green()
                .bold()
            );
            Ok(false)
        }
        "--upgrade" => crate::shell::commands::upgrade::cmd_upgrade(),
        "--help" | "-h" => {
            print_help();
            Ok(false)
        }
        "-n" | "--new" => {
            let child_args: Vec<String> = args[1..].to_vec();
            let stay = !child_args.is_empty();
            spawn_new_terminal(&child_args, stay)?;
            println!("{}", t!("cli.new.opened"));
            Ok(false)
        }
        _ => {
            let cmd = args.join(" ");
            if cmd.trim().is_empty() {
                return Ok(true);
            }
            run_shell_command_line(&cmd)
        }
    }
}

/// Expand ral aliases (prompting on stdin like the interactive shell),
/// execute each resulting command, and stay interactive when `NTC_STAY=1`.
fn run_shell_command_line(cmd: &str) -> Result<bool> {
    let stay = std::env::var(STAY_ENV).as_deref() == Ok("1");
    if stay {
        std::env::remove_var(STAY_ENV);
    }
    let mut nav = Navigator::new()?;
    let expanded = match crate::shell::alias::expand_command_line_interactive(cmd, &mut nav)? {
        None => return Ok(false),
        Some(cmds) => cmds,
    };
    for c in &expanded {
        if c.trim().is_empty() {
            continue;
        }
        execute_command(c, &mut nav, &None)?;
    }
    if stay {
        crate::shell::run_shell_with_nav(nav)?;
    }
    Ok(false)
}
