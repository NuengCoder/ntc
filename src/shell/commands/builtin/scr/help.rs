//! Help sections owned by the `scr` command module: `print_scr_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_scr_help_section() {
    section_header("help.scr.title");
    usage("help.scr.run");
    usage("help.scr.direct");
    usage("help.scr.continue");
    usage("help.scr.init");
    usage("help.scr.ls");
    usage("help.scr.help");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.scr.example1");
    example_row("help.scr.example2");
    example_row("help.scr.example3");
}
