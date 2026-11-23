//! Help sections owned by the `nssh` command module: `print_nssh_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_nssh_help_section() {
    section_header("help.nssh.title");
    usage("help.nssh.add");
    usage("help.nssh.ssh");
    usage("help.nssh.cp");
    usage("help.nssh.scan");
    usage("help.nssh.manage");
    usage("help.nssh.keygen");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.nssh.example1");
    example_row("help.nssh.example2");
    example_row("help.nssh.example3");
}
