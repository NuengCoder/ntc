//! Help sections owned by the `tl` command module: `print_tl_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_tl_help_section() {
    section_header("help.tl.title");
    usage("help.tl.file");
    usage("help.tl.ext");
    usage("help.tl.dir");
    usage("help.tl.sort");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.tl.example1");
    example_row("help.tl.example2");
    example_row("help.tl.example3");
    example_row("help.tl.example4");
}
