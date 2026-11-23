//! `tpb` — teleport back through the jump history.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::teleport::TeleportManager;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_tpb(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        TeleportManager::teleport_back(nav)?;
    } else {
        let args_lower = args.to_lowercase();
        match args_lower.as_str() {
            "history" | "hist" | "h" => {
                TeleportManager::show_history()?;
            }
            "clear" | "cls" => {
                TeleportManager::clear_stack();
            }
            _ => {
                print_error(&t!("misc.tpb.unknown", args));
                println!("{}", t!("misc.tpb.usage"));
                println!("{}", t!("misc.tpb.usage_basic"));
                println!("{}", t!("misc.tpb.usage_history"));
                println!("{}", t!("misc.tpb.usage_clear"));
            }
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "tpb",
    aliases: &[],
    help_topic: Some("tpb"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_tpb),
};
