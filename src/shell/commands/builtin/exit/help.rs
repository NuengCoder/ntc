//! Help section owned by the `exit` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_exit_help_section() {
    print_section(
        "help.exit.title",
        &["help.exit.usage"],
        &[],
        &["help.exit.example1"],
    );
}
