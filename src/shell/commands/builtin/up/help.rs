//! Help section owned by the `up` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_up_help_section() {
    print_section(
        "help.up.title",
        &["help.up.usage"],
        &[],
        &["help.up.example1"],
    );
}
