//! Help section owned by the `gos` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_gos_help_section() {
    print_section(
        "help.gos.title",
        &["help.gos.usage"],
        &[],
        &["help.gos.example1"],
    );
}
