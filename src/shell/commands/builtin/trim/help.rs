//! Help section owned by the `trim` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_trim_help_section() {
    print_section(
        "help.trim.title",
        &["help.trim.usage"],
        &[],
        &["help.trim.example1"],
    );
}
