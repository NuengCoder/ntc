//! `py_deactivate` — leave the active Python virtual environment.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_info, print_success};
use crate::shell::venv::clear_venv_environment;
use crate::t;
use anyhow::Result;
use std::env;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_venvd(_args: &str, nav: &mut Navigator) -> Result<bool> {
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

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "py_deactivate",
    aliases: &[],
    help_topic: Some("py_deactivate"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_venvd),
};
