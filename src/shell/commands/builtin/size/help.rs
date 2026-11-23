//! Help sections owned by the `size` command module: `print_size_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_size_help_section() {
    section_header("help.size.title");
    usage("help.size.size");
    usage("help.size.path");
    usage("help.size.care");
    usage("help.size.time");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.size.example1");
    example_row("help.size.example2");
    example_row("help.size.example3");
    example_row("help.size.example4");
}
