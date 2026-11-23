//! Help sections owned by the `qsc` command module: `print_qsc_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_qsc_help_section() {
    section_header("help.qsc.title");
    usage("help.qsc.set");
    usage("help.qsc.set_cs");
    usage("help.qsc.off");
    usage("help.qsc.listen");
    usage("help.qsc.stop");
    usage("help.qsc.status");
    usage("help.qsc.startup");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.qsc.example1");
    example_row("help.qsc.example2");
    example_row("help.qsc.example3");
}
