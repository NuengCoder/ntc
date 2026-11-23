//! `fs` — search files by name.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::search::parse_search_args;

pub(crate) fn cmd_fs(args: &str, nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("misc.fs.usage"));
        println!("{}", t!("misc.fs.example1"));
        println!("{}", t!("misc.fs.example2"));
        println!("{}", t!("misc.fs.example3"));
        return Ok(false);
    }
    let (pattern, max_depth, fuzzy) = parse_search_args(args);
    let results = crate::search::search_files_opts(nav.current_path(), &pattern, max_depth, fuzzy);
    let output = crate::search::format_search_results(&results, &pattern, max_depth, true);
    print!("{}", output);
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "fs",
    aliases: &[],
    help_topic: Some("fs"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_fs),
};
