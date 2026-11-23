//! Help section owned by the `help` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_help_help_section() {
    print_section(
        "help.help.title",
        &["help.help.usage", "help.help.command", "help.help.all"],
        &[],
        &["help.help.example1"],
    );
}
