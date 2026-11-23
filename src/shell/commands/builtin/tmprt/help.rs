//! Help sections owned by the `tmprt` command module: `print_tmprt_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_tmprt_help_section() {
    section_header("help.tmprt.title");
    usage("help.tmprt.com");
    usage("help.tmprt.weather");
    usage("help.tmprt.key");
    usage("help.tmprt.city");
    usage("help.tmprt.sensors");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.tmprt.example1");
    example_row("help.tmprt.example2");
    example_row("help.tmprt.example3");
    example_row("help.tmprt.example4");
}
