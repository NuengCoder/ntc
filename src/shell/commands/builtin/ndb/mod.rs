//! Shell command wrapper for the `ndb` database shell.
//!
//! `ndb` itself is implemented in the `ndb` submodule (see `ndb/mod.rs`); this
//! module only bridges the ntc shell dispatcher to that nested shell.

pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod ndb;

use crate::navigator::Navigator;
use crate::t;

use anyhow::Result;

/// `ndb` — open the NTC Database shell for the current directory.
pub fn cmd_ndb(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();

    if args == "--help" || args == "-h" {
        crate::shell::help::print_ndb_help_section();
        return Ok(false);
    }

    if !args.is_empty() {
        crate::output::print_error(&t!("ndb.no_args"));
        return Ok(false);
    }

    crate::shell::commands::builtin::ndb::ndb::run_ndb_shell(nav)?;
    Ok(false)
}

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ndb",
    aliases: &[],
    help_topic: Some("ndb"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ndb),
};
