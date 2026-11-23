//! `locate` — fuzzy search for files and directories.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::search;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::search::parse_search_args;

pub(crate) fn cmd_locate(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.locate.usage"));
        println!("{}", t!("misc.locate.example1"));
        println!("{}", t!("misc.locate.example2"));
        println!("{}", t!("misc.locate.example3"));
        return Ok(false);
    }
    let (pattern, max_depth, fuzzy) = parse_search_args(args);
    let results = search::search_all_opts(nav.current_path(), &pattern, max_depth, fuzzy);
    if results.is_empty() {
        println!("{}", t!("misc.locate.no_results", pattern).dimmed());
    } else {
        let has_exact = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Exact);
        let has_partial = results
            .iter()
            .any(|r| r.match_kind == search::MatchKind::Partial);
        let label = if has_exact {
            t!("misc.locate.label_exact")
        } else if has_partial {
            t!("misc.locate.label_partial")
        } else {
            t!("misc.locate.label_didyoumean")
        };
        println!();
        println!(
            "{}",
            t!("misc.locate.header", label, pattern, results.len())
                .cyan()
                .bold()
        );
        for (i, r) in results.iter().enumerate() {
            let icon = if r.full_path.is_dir() {
                "[dir]"
            } else {
                "[file]"
            };
            if r.match_kind == search::MatchKind::Fuzzy {
                println!(
                    "  {}. {} {} ({:.0}% match)",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed(),
                    r.score * 100.0
                );
            } else {
                println!(
                    "  {}. {} {}",
                    (i + 1).to_string().yellow(),
                    icon,
                    r.full_path.to_string_lossy().dimmed()
                );
            }
        }
        println!();
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "locate",
    aliases: &[],
    help_topic: Some("locate"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_locate),
};
