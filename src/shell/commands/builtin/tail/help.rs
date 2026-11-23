//! Help section owned by the `tail` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_tail_help_section() {
    print_section(
        "help.tail.title",
        &["help.tail.usage"],
        &[],
        &["help.tail.example1"],
    );
}
