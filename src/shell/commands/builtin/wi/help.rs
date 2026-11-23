//! Help section owned by the `wi` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_wi_help_section() {
    print_section(
        "help.wi.title",
        &["help.wi.usage"],
        &[],
        &["help.wi.example1"],
    );
}
