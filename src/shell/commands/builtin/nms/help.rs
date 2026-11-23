//! Help sections owned by the `nms` command module: `print_nms_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_nms_help_section() {
    section_header("help.nms.title");
    usage("help.nms.play");
    usage("help.nms.scan");
    usage("help.nms.long");
    usage("help.nms.keys");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.nms.example1");
    example_row("help.nms.example2");
    example_row("help.nms.example3");
    example_row("help.nms.example4");
}
