//! `fake` command: deterministic, seedable fake-data generation.
//!
//! ```text
//! ntc> fake gen name,email,city -n 5
//! ntc> fake gen name,email -f sql -n 3
//! ntc> fake types -s emial
//! ntc> fake group add users [username as name, year as age]
//! ntc> fake gen -g users -f sql -n 10
//! ```

pub(crate) mod completion;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod fake;
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::commands::builtin::fake::fake::{gen, group, types};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

/// Dispatch `fake …`. Never changes navigation.
pub fn cmd_fake(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let (sub, rest) = match args.split_once(char::is_whitespace) {
        Some((a, b)) => (a.to_lowercase(), b.trim()),
        None => (args.to_lowercase(), ""),
    };

    match sub.as_str() {
        "" | "help" | "-h" | "--help" => print_usage(),
        "gen" | "generate" | "g" => gen::run(rest)?,
        "check" => {
            let cwd = nav.current_path().to_path_buf();
            crate::shell::commands::support::check::run(
                "fake",
                rest,
                &cwd,
                ".ntc_fake",
                crate::shell::commands::support::check::formats::fake,
            );
        }
        "types" | "type" | "t" => types::cmd_types(rest)?,
        "locales" | "locale" | "loc" => types::cmd_locales()?,
        "group" | "grp" | "groups" => group::cmd_group(rest)?,
        other => {
            print_error(&t!("cmd.fake.unknown_sub", other));
            println!();
            print_usage();
        }
    }
    Ok(false)
}

fn print_usage() {
    println!("{}", t!("cmd.fake.usage_title").cyan().bold());
    println!("  {}", t!("cmd.fake.usage_gen"));
    println!("  {}", t!("cmd.fake.usage_types"));
    println!("  {}", t!("cmd.fake.usage_locales"));
    println!("  {}", t!("cmd.fake.usage_group"));
    println!("  {}", t!("cmd.fake.usage_help"));
    println!("  {}", t!("check.help", "fake"));
    println!();
    println!("{}", t!("cmd.fake.options_title").yellow().bold());
    println!("  {}", t!("cmd.fake.opt_count"));
    println!("  {}", t!("cmd.fake.opt_format"));
    println!("  {}", t!("cmd.fake.opt_seed"));
    println!("  {}", t!("cmd.fake.opt_output"));
    println!("  {}", t!("cmd.fake.opt_group"));
    println!("  {}", t!("cmd.fake.opt_locale"));
    println!("  {}", t!("cmd.fake.opt_unique"));
    println!();
    println!("{}", t!("cmd.fake.examples_header").green());
    println!("  {}", t!("cmd.fake.example1"));
    println!("  {}", t!("cmd.fake.example2"));
    println!("  {}", t!("cmd.fake.example3"));
    println!("  {}", t!("cmd.fake.example4"));
}

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "fake",
    aliases: &[],
    help_topic: Some("fake"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_fake),
};
