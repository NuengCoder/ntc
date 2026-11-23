//! Help sections owned by the `qr` command module: `print_qr_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_qr_help_section() {
    section_header("help.qr.title");
    usage("help.qr.encode");
    usage("help.qr.save");
    usage("help.qr.image");
    usage("help.qr.decode");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.qr.example1");
    example_row("help.qr.example2");
    example_row("help.qr.example4");
    example_row("help.qr.example3");
}
