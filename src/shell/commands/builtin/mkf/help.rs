//! Help section owned by the `mkf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_mkf_help_section() {
    print_section(
        "help.mkf.title",
        &["help.mkf.usage"],
        &[],
        &["help.mkf.example1"],
    );
}
