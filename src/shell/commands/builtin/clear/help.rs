//! Help section owned by the `clear` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_clear_help_section() {
    print_section(
        "help.clear.title",
        &["help.clear.usage"],
        &[],
        &["help.clear.example1"],
    );
}
