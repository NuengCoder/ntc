//! `cd` — change directory, including teleport savepoints.

pub(crate) mod help;

use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success};
use crate::shell::helpers::show_nav_view;
use crate::t;
use crate::teleport::TeleportManager;
use anyhow::Result;
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_cd(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    if args.is_empty() {
        match crate::utils::paths::home_dir() {
            Some(home) => cd_to(nav, home)?,
            None => print_error(&t!("cmd.nav.cd.home_unavailable")),
        }
    } else {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].to_lowercase() == "to" {
            let tp_name = parts[1];
            if TeleportManager::get_all().contains_key(&tp_name.to_lowercase()) {
                TeleportManager::jump_by_name(nav, tp_name)?;
            } else {
                print_error(&t!("cmd.nav.cd.tp_not_found", tp_name));
                println!("{}", t!("cmd.nav.cd.tp_list"));
            }
        } else {
            let expanded = crate::utils::paths::expand_path_str(args);
            let target = PathBuf::from(expanded);
            let target = if target.is_absolute() {
                target
            } else {
                nav.current_path().join(target)
            };
            cd_to(nav, target)?;
        }
    }
    Ok(false)
}

/// Navigate, then remember the old directory in the teleport history stack so
/// `cd` participates in `tpb` like every `tp` jump does.
fn cd_to(nav: &mut Navigator, target: PathBuf) -> Result<()> {
    let previous = nav.current_path().to_path_buf();
    nav.go_to(&target)?;
    TeleportManager::remember_location(previous);
    clear_screen();
    print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
    show_nav_view(nav);
    Ok(())
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "cd",
    aliases: &[],
    help_topic: Some("cd"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_cd),
};
