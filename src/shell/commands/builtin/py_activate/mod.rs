//! `py_activate` — activate a Python virtual environment.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::paths;
use anyhow::Result;
use std::env;
use std::path::Path;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_venva(args: &str, nav: &mut Navigator) -> Result<bool> {
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
        let expanded = paths::expand_path_str(args);
        let path = Path::new(&expanded);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_path.join(path)
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

/// Verify pip is accessible from the activated environment.
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "py_activate",
    aliases: &[],
    help_topic: Some("py_activate"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_venva),
};
