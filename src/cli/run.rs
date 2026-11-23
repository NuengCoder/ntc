use crate::cli::helpers::{print_help, spawn_new_terminal};
use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::commands::execute_command_with_status;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

/// When set to `"1"`, a `ntc <command>` child stays in an interactive shell
/// after running the command (used by `ntc -n <command>`).
pub(crate) const STAY_ENV: &str = "NTC_STAY";

/// Parse command-line arguments and execute the appropriate action.
///
/// Kept surface:
/// - `ntc` (no args) → `Ok((true, 0))`, launch interactive shell
/// - `ntc <command> [args...]` → expand ral aliases, run through the shell;
///   the second field is the process exit code (0 or 1)
/// - `ntc -n` / `ntc --new [command...]` → open a new terminal (stay interactive), `Ok((false, 0))`
/// - `ntc --upgrade` → self-upgrade, `Ok((false, 0))`
/// - `ntc --version` / `ntc -v` → version, `Ok((false, 0))`
/// - `ntc --here` → interactive shell in the current directory, ignoring the
///   last session directory, `Ok((true, 0))`
/// - `ntc -p/--path <dir>` → interactive shell starting in `<dir>`,
///   `Ok((true, 0))`
/// - `ntc --help` / `ntc -h` → minimal help, `Ok((false, 0))`
pub fn run_cli() -> Result<(bool, i32)> {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() <= 1 {
        return Ok((true, 0));
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
            Ok((false, 0))
        }
        "--upgrade" => crate::shell::commands::support::upgrade::cmd_upgrade().map(|_| (false, 0)),
        "--help" | "-h" => {
            print_help();
            Ok((false, 0))
        }
        "--here" => {
            // Start in the launch directory and skip the session feature for
            // this run (no restore, no save).
            let cwd = std::env::current_dir()?;
            crate::shell::entry::set_startup_dir(cwd);
            crate::shell::entry::disable_session();
            Ok((true, 0))
        }
        "-p" | "--path" => {
            let Some(raw) = args.get(1) else {
                print_error(&t!("cli.path.missing"));
                return Ok((false, 1));
            };
            let expanded = crate::utils::paths::expand_path_str(raw);
            let path = std::path::PathBuf::from(expanded);
            let path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()?.join(path)
            };
            if !path.is_dir() {
                print_error(&t!("cli.path.not_a_directory", path.display()));
                return Ok((false, 1));
            }
            crate::shell::entry::set_startup_dir(path.canonicalize().unwrap_or(path));
            Ok((true, 0))
        }
        "-n" | "--new" => {
            let child_args: Vec<String> = args[1..].to_vec();
            let stay = !child_args.is_empty();
            spawn_new_terminal(&child_args, stay)?;
            println!("{}", t!("cli.new.opened"));
            Ok((false, 0))
        }
        _ => {
            let cmd = quote_argv(args);
            if cmd.trim().is_empty() {
                return Ok((true, 0));
            }
            run_shell_command_line(&cmd)
        }
    }
}

/// Expand ral aliases (prompting on stdin like the interactive shell),
/// execute the resulting command line, and stay interactive when `NTC_STAY=1`.
fn run_shell_command_line(cmd: &str) -> Result<(bool, i32)> {
    let stay = std::env::var(STAY_ENV).as_deref() == Ok("1");
    if stay {
        std::env::remove_var(STAY_ENV);
    }
    let mut nav = Navigator::new()?;

    // Heredocs on the command line read their bodies from process stdin.
    if crate::shell::pipeline::has_heredoc(cmd) {
        let bodies = read_heredocs_stdin(cmd);
        crate::shell::pipeline::set_heredoc_bodies(bodies);
    } else {
        crate::shell::pipeline::set_heredoc_bodies(Vec::new());
    }

    let expanded = match crate::shell::alias::expand_command_line_interactive(cmd, &mut nav)? {
        None => return Ok((false, 0)),
        Some(s) => s,
    };
    let mut exit_code = 0;
    if !expanded.trim().is_empty() {
        let (_, success) = execute_command_with_status(&expanded, &mut nav, &None)?;
        if !success {
            exit_code = 1;
        }
    }

    if stay {
        // The new terminal keeps an interactive shell open; the initial
        // command's status is not the process result anymore.
        crate::shell::run_shell_with_nav(nav)?;
        return Ok((false, 0));
    }
    Ok((false, exit_code))
}

/// Re-quote the OS argv so the shell sees the same argument boundaries the
/// process received. Without this, `ntc nprintf "%s has %d items\n" cart 3`
/// loses its quotes and the format is split on the spaces.
fn quote_argv(args: &[String]) -> String {
    args.iter()
        .map(|arg| quote_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_arg(arg: &str) -> String {
    let needs_quotes = arg.is_empty()
        || arg
            .chars()
            .any(|c| c.is_whitespace() || matches!(c, '"' | '\'' | '[' | '<'))
        || matches!(
            arg,
            "|" | "||" | "&&" | ";" | "<" | "<<" | "<<<" | ">" | ">>"
        );
    if !needs_quotes {
        return arg.to_string();
    }

    let mut out = String::with_capacity(arg.len() + 2);
    out.push('"');
    for c in arg.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Read `<<` heredoc bodies (in declaration order) from process stdin.
fn read_heredocs_stdin(line: &str) -> Vec<Vec<u8>> {
    let delims = crate::shell::pipeline::scan_heredoc_delims(line);
    let mut bodies: Vec<Vec<u8>> = Vec::new();
    if delims.is_empty() {
        return bodies;
    }
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    for delim in delims {
        let mut body = String::new();
        while let Some(Ok(l)) = lines.next() {
            if l.trim_end_matches(['\r', '\n']) == delim {
                break;
            }
            body.push_str(&l);
            body.push('\n');
        }
        bodies.push(body.into_bytes());
    }
    bodies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_shot_argv_keeps_argument_boundaries() {
        assert_eq!(
            quote_argv(&[
                "nprintf".into(),
                "%s has %d items\\n".into(),
                "cart".into(),
                "3".into(),
            ]),
            "nprintf \"%s has %d items\\\\n\" cart 3"
        );
        assert_eq!(
            quote_argv(&["say".into(), "hello world".into(), "".into()]),
            "say \"hello world\" \"\""
        );
        // A bare `*` or `3*4` is not an operator and stays readable.
        assert_eq!(quote_argv(&["math".into(), "3*4".into()]), "math 3*4");
        // Standalone operators become literal arguments.
        assert_eq!(
            quote_argv(&["nprintf".into(), "%s".into(), "|".into()]),
            "nprintf %s \"|\""
        );
    }
}
