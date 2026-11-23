//! Help section owned by the `fsc` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_fsc_help_section() {
    print_section(
        "help.fsc.title",
        &["help.fsc.usage"],
        &[],
        &["help.fsc.example1"],
    );
}
