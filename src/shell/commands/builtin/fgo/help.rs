//! Help section owned by the `fgo` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_fgo_help_section() {
    print_section(
        "help.fgo.title",
        &["help.fgo.usage"],
        &[],
        &["help.fgo.example1"],
    );
}
