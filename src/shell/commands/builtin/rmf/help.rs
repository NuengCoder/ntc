//! Help section owned by the `rmf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_rmf_help_section() {
    print_section(
        "help.rmf.title",
        &["help.rmf.usage"],
        &[],
        &["help.rmf.example1"],
    );
}
