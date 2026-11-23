//! `version` — print the running ntc version.

pub(crate) mod help;

use crate::navigator::Navigator;
#[cfg(not(target_os = "android"))]
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_version(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(not(target_os = "android"))]
    {
        let theme = crate::utils::theme::ThemeManager::current();
        println!(
            "ntc {}",
            env!("CARGO_PKG_VERSION")
                .color(theme.shell.success.to_colored())
                .bold()
        );
    }
    #[cfg(target_os = "android")]
    {
        println!("ntc {}", env!("CARGO_PKG_VERSION"));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "version",
    aliases: &[],
    help_topic: Some("version"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_version),
};
