//! Help sections owned by the `fmt` command module: `print_fmt_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_fmt_help_section() {
    section_header("help.fmt.title");
    usage("cmd.fmt.usage");
    outln!("{}", t!("help.examples_header").green());
    example_row("fmt.help_example1");
    example_row("fmt.help_example2");
    example_row("fmt.help_example4");
    example_row("fmt.help_example5");
    example_row("fmt.help_example6");
    example_row("fmt.help_example7");
    example_row("fmt.help_example8");
    example_row("fmt.help_example9");
    example_row("fmt.help_example10");
}
