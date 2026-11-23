//! `tmprt` command: hardware temperature monitor (`tmprt com`) and
//! OpenWeather current weather (`tmprt weather`).
//!
//! Subcommands:
//! - `tmprt com [secs]`          live HWMonitor-style sensor view
//! - `tmprt weather [city]`      current weather via OpenWeather API
//! - `tmprt key [API_KEY]`       save/display the OpenWeather API key
//! - `tmprt city [name]`         save/display the default city
//! - `tmprt sensors <sub>`       manage hardware sensor backends (LHM)
//!
//! The sensor TUI lives in [`com`], backend/install plumbing in [`sensors`],
//! weather and config helpers in [`weather`], formatting in [`ui`].

pub(crate) mod completion;
pub(crate) mod help;

mod com;
mod sensors;
mod ui;
mod weather;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub fn cmd_tmprt(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let sub = parts.first().copied().unwrap_or("").to_lowercase();
    let rest = parts.get(1).unwrap_or(&"").trim();

    match sub.as_str() {
        "com" | "computer" | "hw" | "hardware" => com::cmd_tmprt_com(rest),
        "weather" | "w" => weather::cmd_tmprt_weather(rest),
        "key" => weather::cmd_tmprt_key(rest),
        "city" => weather::cmd_tmprt_city(rest),
        "sensors" | "sensor" => sensors::cmd_tmprt_sensors(rest),
        "" | "help" | "--help" | "-h" => {
            print_usage();
            Ok(false)
        }
        _ => {
            print_error(&t!("cmd.tmprt.unknown_sub", sub));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

fn print_usage() {
    println!("{}", t!("cmd.tmprt.usage_title").cyan().bold());
    println!("  {}", t!("cmd.tmprt.usage_com"));
    println!("  {}", t!("cmd.tmprt.usage_weather"));
    println!("  {}", t!("cmd.tmprt.usage_key"));
    println!("  {}", t!("cmd.tmprt.usage_city"));
    println!("  {}", t!("cmd.tmprt.usage_sensors"));
    println!();
    println!("{}", t!("cmd.tmprt.examples_title").green());
    println!("  {}", t!("cmd.tmprt.example1"));
    println!("  {}", t!("cmd.tmprt.example2"));
    println!("  {}", t!("cmd.tmprt.example3"));
    println!("  {}", t!("cmd.tmprt.example4"));
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "tmprt",
    aliases: &[],
    help_topic: Some("tmprt"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_tmprt),
};
