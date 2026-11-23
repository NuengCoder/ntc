//! Help section owned by the `rnmd` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_rnmd_help_section() {
    print_section(
        "help.rnmd.title",
        &["help.rnmd.usage"],
        &[],
        &["help.rnmd.example1"],
    );
}
