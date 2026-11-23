//! Help section owned by the `back` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_back_help_section() {
    print_section(
        "help.back.title",
        &["help.back.usage"],
        &[],
        &["help.back.example1"],
    );
}
