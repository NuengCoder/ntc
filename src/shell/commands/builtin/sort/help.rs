//! Help section owned by the `sort` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_sort_help_section() {
    print_section(
        "help.sort.title",
        &["help.sort.usage"],
        &[],
        &["help.sort.example1"],
    );
}
