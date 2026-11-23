//! Help sections owned by the `ran` command module: `print_ran_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_ran_help_section() {
    section_header("help.ran.title");
    usage("help.ran.run");
    usage("help.ran.target");
    usage("help.ran.dry_run");
    usage("help.ran.init");
    usage("help.ran.deinit");
    usage("help.ran.list");
    usage("help.ran.check");
    usage("help.ran.fmt");
    usage("help.ran.help");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.ran.example1");
    example_row("help.ran.example2");
    example_row("help.ran.example3");
}
