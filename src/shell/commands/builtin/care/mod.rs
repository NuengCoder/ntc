//! `care` — add cared directories, extensions, and files.

pub(crate) mod help;

pub(crate) mod completion;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::config_scope::split_scope_flag;
use super::super::support::igcare::apply_igcare;

pub fn cmd_care(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let (scope, args) = split_scope_flag(args);
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.care.dir_usage"),
            Config::add_cared_dir,
            "Un-ignored directories:",
            scope,
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.care.ext_usage"),
            Config::add_cared_extension,
            "Extra supported extensions:",
            scope,
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.care.file_usage"),
            Config::add_cared_file,
            "Extra supported files:",
            scope,
        ),
        _ => {
            println!("{}", t!("cmd.care.title").cyan().bold());
            println!("  {}", t!("cmd.care.dir_desc"));
            println!("  {}", t!("cmd.care.ext_desc"));
            println!("  {}", t!("cmd.care.file_desc"));
            println!();
            println!("{}", t!("cmd.care.scope_help").green());
            println!();
            println!("{}", t!("cmd.ignore.care_examples_title").green());
            println!("  {}", t!("cmd.ignore.care_example_dir").dimmed());
            println!("  {}", t!("cmd.ignore.care_example_ext").dimmed());
            println!("  {}", t!("cmd.ignore.care_example_file").dimmed());
            Ok(false)
        }
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "care",
    aliases: &[],
    help_topic: Some("care"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_care),
};
