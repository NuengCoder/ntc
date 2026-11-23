//! Help section owned by the `head` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_head_help_section() {
    print_section(
        "help.head.title",
        &["help.head.usage"],
        &[],
        &["help.head.example1"],
    );
}
