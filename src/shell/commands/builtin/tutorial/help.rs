//! Help section owned by the `tutorial` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_tutorial_help_section() {
    print_section(
        "help.tutorial.title",
        &["help.tutorial.usage"],
        &[],
        &["help.tutorial.example1"],
    );
}
