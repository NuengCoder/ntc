//! Help section owned by the `showcg` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_showcg_help_section() {
    print_section(
        "help.showcg.title",
        &["help.showcg.usage"],
        &[],
        &["help.showcg.example1"],
    );
}
