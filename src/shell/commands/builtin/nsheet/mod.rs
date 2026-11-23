//! `nsheet` command: terminal sheet viewer for CSV/TSV/JSON/JSONL.
//!
//! ```text
//! nsheet open users.csv                  # fullscreen TUI (arrows, /, s, f, e, q)
//! nsheet view users.csv -l 20 --sort age --desc --filter "city~york"
//! nsheet users.csv --filter "age>30" --export json -o out.json
//! ```
//!
//! Parsing/filter/sort/export live in [`crate::shell::commands::builtin::nsheet::nsheet`]; this module is the
//! shell front-end: argument parsing, the non-interactive `view` printer
//! (reusing `ndb`'s box table) and the fullscreen crossterm TUI.

pub(crate) mod completion;
pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod nsheet;

mod args;
mod tui;
mod view;

#[cfg(test)]
mod tests;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use anyhow::Result;

use args::parse_nsheet_args;
use tui::cmd_open;
use view::cmd_view;

fn tokenize(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut single = false;
    let mut double = false;
    for ch in input.chars() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            ' ' | '\t' if !single && !double => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    // Strip one layer of surrounding quotes (tokenize kept them).
    out.into_iter().map(|t| strip_wrapping_quotes(&t)).collect()
}

fn strip_wrapping_quotes(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}
fn print_usage() {
    let theme = crate::utils::theme::ThemeManager::current();
    let heading = |key: &str| {
        theme
            .shell
            .paint_bold(crate::utils::theme::ShellRole::HelpHeader, &t!(key))
    };
    let example = |key: &str| {
        theme
            .shell
            .paint(crate::utils::theme::ShellRole::HelpExample, &t!(key))
    };
    println!("{}", heading("cmd.nsheet.usage_title"));
    println!("  {}", t!("cmd.nsheet.help_open"));
    println!("  {}", t!("cmd.nsheet.help_view"));
    println!();
    println!("{}", heading("cmd.nsheet.options_title"));
    println!("  {}", t!("cmd.nsheet.help_delimiter"));
    println!("  {}", t!("cmd.nsheet.help_header"));
    println!("  {}", t!("cmd.nsheet.help_limit"));
    println!("  {}", t!("cmd.nsheet.help_sort"));
    println!("  {}", t!("cmd.nsheet.help_filter"));
    println!("  {}", t!("cmd.nsheet.help_export"));
    println!("  {}", t!("cmd.nsheet.help_output"));
    println!("  {}", t!("cmd.nsheet.help_help"));
    println!();
    println!("{}", example("cmd.nsheet.examples_title"));
    println!("    {}", t!("cmd.nsheet.example_open"));
    println!("    {}", t!("cmd.nsheet.example_view"));
    println!("    {}", t!("cmd.nsheet.example_filter"));
    println!("    {}", t!("cmd.nsheet.example_export"));
}
/// Dispatch an `nsheet` command line.
pub fn cmd_nsheet(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    let first = tokens[0].to_lowercase();
    if first == "help" || first == "-h" || first == "--help" {
        print_usage();
        return Ok(false);
    }
    let (is_view, rest) = if first == "open" {
        (false, &tokens[1..])
    } else if first == "view" {
        (true, &tokens[1..])
    } else if first.starts_with('-') {
        print_usage();
        return Ok(false);
    } else {
        // Bare `nsheet <file> [flags]` behaves like `open`.
        (false, &tokens[..])
    };
    let parsed = match parse_nsheet_args(rest) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            println!();
            print_usage();
            return Ok(false);
        }
    };
    if is_view {
        cmd_view(nav, &parsed)
    } else {
        cmd_open(nav, &parsed)
    }
}

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "nsheet",
    aliases: &[],
    help_topic: Some("nsheet"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_nsheet),
};
