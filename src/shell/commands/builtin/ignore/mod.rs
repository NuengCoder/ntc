//! `ignore` — add ignored directories, extensions, and files.

pub(crate) mod completion;
pub(crate) mod help;

use crate::config::Config;
use crate::navigator::Navigator;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::config_scope::split_scope_flag;
use super::super::support::igcare::apply_igcare;

pub fn cmd_ignore(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let (scope, args) = split_scope_flag(args);
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd {
        "dir" => apply_igcare(
            subargs,
            &t!("cmd.ignore.dir_usage"),
            Config::add_ignored_dir,
            "Ignored directories:",
            scope,
        ),
        "ext" => apply_igcare(
            subargs,
            &t!("cmd.ignore.ext_usage"),
            Config::add_ignored_extension,
            "Ignored extensions:",
            scope,
        ),
        "file" => apply_igcare(
            subargs,
            &t!("cmd.ignore.file_usage"),
            Config::add_ignored_file,
            "Ignored files:",
            scope,
        ),
        _ => {
            println!("{}", t!("cmd.ignore.title").cyan().bold());
            println!("  {}", t!("cmd.ignore.dir_desc"));
            println!("  {}", t!("cmd.ignore.ext_desc"));
            println!("  {}", t!("cmd.ignore.file_desc"));
            println!();
            println!("{}", t!("cmd.ignore.scope_help").green());
            println!();
            println!("{}", t!("cmd.ignore.examples_title").green());
            println!("  {}", t!("cmd.ignore.example_dir").dimmed());
            println!("  {}", t!("cmd.ignore.example_ext").dimmed());
            println!("  {}", t!("cmd.ignore.example_file").dimmed());
            Ok(false)
        }
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ignore",
    aliases: &[],
    help_topic: Some("ignore"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ignore),
};
