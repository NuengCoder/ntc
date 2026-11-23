//! `gos` — choose a subdirectory interactively.

pub(crate) mod help;

use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success};
use crate::shell::helpers::show_nav_view;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_gos(_args: &str, nav: &mut Navigator) -> Result<bool> {
    let dirs = nav.list_subdirs()?;
    println!();
    println!("{}", t!("nav.gos_where").cyan().bold());
    println!("  {} {}", "0".yellow(), t!("cmd.nav.gos.exit").dimmed());
    if dirs.is_empty() {
        println!("  {}", t!("cmd.nav.gos.empty").dimmed());
    } else {
        for (i, name) in &dirs {
            println!("  {} {}", i.to_string().yellow(), name.bright_magenta());
        }
    }
    println!();
    let labels: Vec<String> = dirs.iter().map(|(_, name)| name.clone()).collect();
    let Some(choice) = crate::utils::interactive::read_number_with_hint(
        "> ",
        |prompt| prompt.green().to_string(),
        &labels,
    )?
    else {
        println!("{}", t!("cmd.nav.gos.staying").dimmed());
        return Ok(false);
    };
    let choice = choice.as_str();

    if choice == "0" || choice.is_empty() {
        println!("{}", t!("cmd.nav.gos.staying").dimmed());
    } else if let Ok(num) = choice.parse::<usize>() {
        if let Some((_, name)) = dirs.iter().find(|(i, _)| *i == num) {
            let new_path = nav.current_path().join(name);
            nav.go_to(&new_path)?;
            clear_screen();
            print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
            show_nav_view(nav);
        } else {
            print_error(&t!("cmd.nav.invalid_number"));
        }
    } else {
        print_error(&t!("cmd.nav.invalid_input"));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "gos",
    aliases: &[],
    help_topic: Some("gos"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_gos),
};
