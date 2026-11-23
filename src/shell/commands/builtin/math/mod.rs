//! `math` — evaluate a math expression, or piped input.

pub(crate) mod completion;
pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod math;

use crate::navigator::Navigator;
use crate::utils::output::print_error;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_math(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let mut expr = args.trim().to_string();
    // `math -` (or bare `math`) evaluates piped / redirected input.
    if expr.is_empty() || expr == "-" {
        if let Some(s) = crate::shell::stdin::read_to_string() {
            expr = s.trim().to_string();
        }
    }
    // A bad expression is a user-input error, not a fatal one: report it and
    // let `&&` / `||` and the CLI exit status treat it as a failed command.
    if let Err(e) = crate::shell::commands::builtin::math::math::run(&expr) {
        print_error(&format!("{}", e));
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "math",
    aliases: &[],
    help_topic: Some("math"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_math),
};
