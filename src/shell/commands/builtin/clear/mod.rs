//! `clear` — redraw the shell welcome header and navigation view.

pub(crate) mod help;

use crate::config::Config;
use crate::navigator::{clear_screen, Navigator};
use crate::shell::helpers::{show_nav_view, suppress_watch_refresh};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_clear(_args: &str, nav: &mut Navigator) -> Result<bool> {
    // `clear` redraws the whole view itself, so the shell must not print a
    // file-watcher change box / refresh the tree after this command.
    suppress_watch_refresh();
    clear_screen();
    println!();
    let show_header =
        Config::global_get_dir_view_enabled() || Config::global_get_file_view_enabled();
    if show_header {
        #[cfg(not(target_os = "android"))]
        {
            let theme = crate::utils::theme::ThemeManager::current();
            let a = |s: &str| crate::utils::terminal::asciify(s);
            let version = env!("CARGO_PKG_VERSION");
            println!(
                "{}",
                a("╔══════════════════════════════════════════════════════════════════╗")
            );
            println!(
                "{}",
                a(&format!("{:^66}", t!("misc.clear.welcome", version)))
                    .color(theme.shell.help_header.to_colored())
                    .bold()
            );
            println!(
                "{}",
                a("╚══════════════════════════════════════════════════════════════════╝")
            );
        }
        #[cfg(target_os = "android")]
        {
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╔══════════════════════════════════════════════════════════════════╗"
                )
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(&format!(
                    "{:^66}",
                    t!("misc.clear.welcome", env!("CARGO_PKG_VERSION"))
                ))
            );
            println!(
                "{}",
                crate::utils::terminal::asciify(
                    "╚══════════════════════════════════════════════════════════════════╝"
                )
            );
        }
    }
    if show_header {
        println!("{}", t!("misc.clear.type_help").dimmed());
    }
    show_nav_view(nav);
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "clear",
    aliases: &[],
    help_topic: Some("clear"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_clear),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_clear_suppresses_watch_refresh() {
        let _g = crate::utils::test_support::lock_globals();
        let mut nav = Navigator::new().unwrap();
        let _ = crate::shell::helpers::take_watch_refresh_suppressed();
        let mut cfg = Config::default();
        cfg.dir_view_enabled = false;
        Config::with_test_config(cfg, || {
            cmd_clear("", &mut nav).unwrap();
        });
        assert!(crate::shell::helpers::take_watch_refresh_suppressed());
    }
}
