//! Help sections owned by the `skill` command module: `print_skill_help`.

use crate::shell::help::format::*;
use crate::shell::help::outln;
use crate::t;
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn print_skill_help_section() {
    section_header("help.skill.title");
    usage("help.skill.download");
    usage("help.skill.add");
    usage("help.skill.remove");
    usage("help.skill.delete");
    usage("help.skill.edit");
    usage("help.skill.list");
    usage("help.skill.list_all");
    usage("help.skill.avail");
    usage("help.skill.info");
    usage("help.skill.sub");
    usage("help.skill.help");
    outln!("{}", t!("help.examples_header").green());
    example_row("help.skill.example1");
    example_row("help.skill.example2");
    example_row("help.skill.example3");
    example_row("help.skill.example4");
    example_row("help.skill.example5");
    example_row("help.skill.example6");
    example_row("help.skill.example7");
    example_row("help.skill.example8");
    example_row("help.skill.example9");
    example_row("help.skill.example10");
}
