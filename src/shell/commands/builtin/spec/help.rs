//! Help section owned by the `spec` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_spec_help_section() {
    print_section(
        "help.spec.title",
        &["help.spec.usage"],
        &[],
        &["help.spec.example1"],
    );
}
