//! Help section owned by the `rnmf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_rnmf_help_section() {
    print_section(
        "help.rnmf.title",
        &["help.rnmf.usage"],
        &[],
        &["help.rnmf.example1"],
    );
}
