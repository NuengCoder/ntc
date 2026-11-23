//! `serve` — dispatcher for the browser-facing services.
//!
//! Each service lives in its own module; [`service::SERVICES`] is the single
//! source of truth for which services exist, what `serve` help prints and
//! what `showcg` / `config serve` list.

pub(crate) mod help;

pub(crate) mod arc;
pub(crate) mod backup;
pub(crate) mod chess;
pub(crate) mod config;
pub(crate) mod fake;
pub(crate) mod files;
pub(crate) mod git;
pub(crate) mod http_server;
pub(crate) mod mdview;
pub(crate) mod ndb;
pub(crate) mod service;
pub(crate) mod share;
pub(crate) mod theme;
pub(crate) mod tunnel;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::commands::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::shell::commands::support::net::tokenize;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

pub fn cmd_serve(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    // Bare `serve` never runs anything — it lists the services.
    if tokens.is_empty() || matches!(tokens[0].as_str(), "help" | "--help" | "-h") {
        print_serve_services();
        return Ok(false);
    }
    match service::find(&tokens[0]) {
        Some(spec) => {
            let rest = tokens[1..].join(" ");
            (spec.run)(&rest, nav)
        }
        None => {
            print_error(&t!("cmd.serve.unknown_option", tokens[0]));
            print_serve_services();
            Ok(false)
        }
    }
}

/// Top-level `serve` help: service list only, never runs a server.
/// Per-service details live in `serve <service> --help`.
fn print_serve_services() {
    println!("{}", t!("cmd.serve.services_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.services_usage"));
    println!();
    println!("{}", t!("cmd.serve.services_header").cyan().bold());
    for spec in service::SERVICES {
        println!("  {}", t!(spec.row_key));
    }
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.services_example_files"));
    println!("  {}", t!("cmd.serve.services_example_http"));
    println!("  {}", t!("cmd.serve.services_example_fake"));
    println!("  {}", t!("cmd.serve.services_example_backup"));
    println!("  {}", t!("cmd.serve.services_example_config"));
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "serve",
    aliases: &[],
    help_topic: None,
    flags: FlagPolicy::CommandOwnsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_serve),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_serve_lists_services_without_running() {
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_serve("", &mut nav).is_ok());
        assert!(cmd_serve("help", &mut nav).is_ok());
        assert!(cmd_serve("--help", &mut nav).is_ok());
        assert!(cmd_serve("-h", &mut nav).is_ok());
    }

    #[test]
    fn unknown_service_reports_and_lists() {
        let mut nav = Navigator::new().unwrap();
        assert!(cmd_serve("nope", &mut nav).is_ok());
    }

    #[test]
    fn every_service_id_dispatches_through_the_table() {
        // Dispatch tokens must be resolvable, so `serve <id> --help` never
        // falls through to the unknown-service branch.
        for spec in service::SERVICES {
            assert!(
                service::find(spec.id).is_some(),
                "{} is not dispatchable",
                spec.id
            );
        }
    }
}
