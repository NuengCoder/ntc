//! Help section owned by the `gosc` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_gosc_help_section() {
    print_section(
        "help.gosc.title",
        &["help.gosc.usage"],
        &[],
        &["help.gosc.example1"],
    );
}
