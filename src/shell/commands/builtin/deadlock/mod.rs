//! Shell command wrapper for the `deadlock` password-vault shell.
//!
//! `deadlock` itself is implemented in `crate::shell::commands::builtin::deadlock::deadlock` (see
//! `src/deadlock/mod.rs`); this module only bridges the ntc shell dispatcher
//! to that nested shell.

pub(crate) mod completion;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod deadlock;
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;

use anyhow::Result;

/// `deadlock` — open the encrypted password vault.
pub fn cmd_deadlock(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();

    if args == "--help" || args == "-h" || args == "help" {
        crate::shell::help::print_deadlock_help_section();
        return Ok(false);
    }

    if !args.is_empty() {
        print_error(&t!("deadlock.no_args"));
        return Ok(false);
    }

    crate::shell::commands::builtin::deadlock::deadlock::run_deadlock_shell()?;
    Ok(false)
}

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "deadlock",
    aliases: &[],
    help_topic: Some("deadlock"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_deadlock),
};
