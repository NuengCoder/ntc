//! Help section owned by the `count` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_count_help_section() {
    print_section(
        "help.count.title",
        &["help.count.usage"],
        &[],
        &["help.count.example1", "help.count.example2"],
    );
}
