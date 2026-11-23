//! Help sections owned by the `autoup` command module: `print_autoup_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_autoup_help_section() {
    section_header("help.autoup.title");
    usage("help.autoup.list");
    usage("help.autoup.add");
    usage("help.autoup.rm");
    usage("help.autoup.disable");
    usage("help.autoup.enable");
    usage("help.autoup.sources");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.autoup.example1");
    example_row("help.autoup.example2");
    example_row("help.autoup.example3");
    example_row("help.autoup.example4");
}
