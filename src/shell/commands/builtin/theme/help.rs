//! Help sections owned by the `theme` command module: `print_theme_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_theme_help_section() {
    section_header("help.theme.title");
    usage("help.theme.interactive");
    usage("help.theme.list");
    usage("help.theme.current");
    usage("help.theme.switch");
    usage("help.theme.add");
    usage("help.theme.rm");
    usage("help.theme.edit");
    usage("help.theme.where");
    usage("help.theme.info");
    usage("help.theme.export");
    usage("help.theme.import");
    usage("help.theme.install");
    usage("help.theme.rnm");
    usage("help.theme.reload");
    usage("help.theme.make");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.theme.example1");
    example_row("help.theme.example2");
    example_row("help.theme.example3");
    example_row("help.theme.example4");
}
