//! Help sections owned by the `math` command module: `print_math_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_math_help_section() {
    section_header("help.math.title");
    usage("help.math.eval");
    usage("help.math.timer");
    usage("help.math.fun");
    usage("help.math.file");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.math.example1");
    example_row("help.math.example2");
    example_row("help.math.example3");
    example_row("help.math.example4");
    example_row("help.math.example5");
    example_row("help.math.example6");
}
