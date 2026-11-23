//! Help sections owned by the `deadlock` command module: `print_deadlock_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

/// The `deadlock` section of the interactive help.
pub(crate) fn print_deadlock_help_section() {
    section_header("help.deadlock.title");
    usage("help.deadlock.launch");
    usage("help.deadlock.ls");
    usage("help.deadlock.add");
    usage("help.deadlock.rmpw");
    usage("help.deadlock.master");
    usage("help.deadlock.forget");
    usage("help.deadlock.destroy");
    usage("help.deadlock.exit");
    outln!("{}", t!("help.deadlock.note").dimmed());
    outln!("{}", t!("help.examples_header").green());
    example_row("help.deadlock.example1");
    example_row("help.deadlock.example2");
}
