//! Help sections owned by the `nsheet` command module: `print_nsheet_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_nsheet_help_section() {
    section_header("help.nsheet.title");
    usage("help.nsheet.open");
    usage("help.nsheet.view");
    usage("help.nsheet.sort");
    usage("help.nsheet.filter");
    usage("help.nsheet.export");
    usage("help.nsheet.tui");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.nsheet.example1");
    example_row("help.nsheet.example2");
    example_row("help.nsheet.example3");
}
