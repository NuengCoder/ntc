//! Help sections owned by the `nfe` command module: `print_explorer_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_explorer_help_section() {
    section_header("help.explorer.title");
    usage("help.explorer.launch");
    usage("help.explorer.hint1");
    usage("help.explorer.hint2");
    usage("help.explorer.hint3");
    usage("help.explorer.hint4");
    usage("help.explorer.hint5");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.explorer.example1");
    example_row("help.explorer.example2");
}
