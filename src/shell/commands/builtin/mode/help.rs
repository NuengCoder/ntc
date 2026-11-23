//! Help sections owned by the `mode` command module: `print_mode_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_mode_help_section() {
    section_header("help.mode.title");
    usage("help.mode.admin");
    usage("help.mode.normal");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.mode.example1");
}
