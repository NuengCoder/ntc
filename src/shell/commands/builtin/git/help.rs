//! Help sections owned by the `git` command module: `print_git_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_git_help_section() {
    section_header("help.git.title");
    usage("help.git.status");
    usage("help.git.branch");
    usage("help.git.log");
    usage("help.git.diff");
    usage("help.git.passthrough");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.git.example1");
    example_row("help.git.example2");
    example_row("help.git.example3");
}
