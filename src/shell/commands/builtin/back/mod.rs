//! `back` — step back through the navigation history.

pub(crate) mod help;

use crate::navigator::{clear_screen, Navigator};
use crate::output::print_error;
use crate::shell::helpers::show_nav_view;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_back(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        match nav.go_back() {
            Ok(()) => {
                clear_screen();
                show_nav_view(nav);
            }
            Err(e) => print_error(&format!("{}", e)),
        }
    } else {
        match args.parse::<usize>() {
            Ok(n) if n > 0 => {
                let mut success = true;
                for i in 0..n {
                    match nav.go_back() {
                        Ok(()) => {}
                        Err(e) => {
                            if i == 0 {
                                print_error(&format!("{}", e));
                            } else {
                                print_error(&t!("cmd.nav.back.invalid_step", i + 1));
                            }
                            success = false;
                            break;
                        }
                    }
                }
                if success {
                    clear_screen();
                    show_nav_view(nav);
                }
            }
            _ => print_error(&t!("cmd.nav.back.invalid_number", args)),
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "back",
    aliases: &["b"],
    help_topic: Some("back"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_back),
};
