//! `fgo` — find a file and jump to its directory.

pub(crate) mod help;

use crate::navigator::{clear_screen, Navigator};
use crate::output::{print_error, print_success};
use crate::shell::helpers::show_nav_view;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::search::parse_search_args;

pub(crate) fn cmd_fgo(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.nav.fgo_usage"));
        println!("{}", t!("cmd.nav.fgo.example1"));
        println!("{}", t!("cmd.nav.fgo.example2"));
        return Ok(false);
    }
    let (pattern, max_depth, fuzzy) = parse_search_args(args);
    let results = crate::search::search_files_opts(nav.current_path(), &pattern, max_depth, fuzzy);
    if results.is_empty() {
        let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
        print!("{}", output);
        return Ok(false);
    }
    println!();
    println!(
        "{}",
        t!("cmd.nav.fgo.found", results.len(), pattern)
            .cyan()
            .bold()
    );
    for (i, r) in results.iter().enumerate() {
        let path_str = r.full_path.to_string_lossy();
        println!("  {}. {}", (i + 1).to_string().yellow(), path_str.dimmed());
    }
    println!("  {}", t!("cmd.nav.fgo.cancel").red());
    println!();
    let labels: Vec<String> = results
        .iter()
        .map(|result| {
            result
                .full_path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| result.full_path.display().to_string())
        })
        .collect();
    let Some(input) = crate::utils::interactive::read_number_with_hint(
        &t!("cmd.nav.fgo.select"),
        |text| text.green().to_string(),
        &labels,
    )?
    else {
        println!("{}", t!("cmd.nav.fgo.cancelled").dimmed());
        return Ok(false);
    };
    let input = input.trim();
    if input == "0" || input.is_empty() {
        println!("{}", t!("cmd.nav.fgo.cancelled").dimmed());
        return Ok(false);
    }
    if let Ok(n) = input.parse::<usize>() {
        if n >= 1 && n <= results.len() {
            let target = &results[n - 1].full_path;
            let parent = target.parent().unwrap_or(target);
            nav.go_to(parent)?;
            clear_screen();
            print_success(&t!("cmd.nav.cd.navigated", nav.display_path()));
            show_nav_view(nav);
        } else {
            print_error(&t!("cmd.nav.invalid_selection"));
        }
    } else {
        print_error(&t!("cmd.nav.invalid_input"));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "fgo",
    aliases: &[],
    help_topic: Some("fgo"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_fgo),
};
