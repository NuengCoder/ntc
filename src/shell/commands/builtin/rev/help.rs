//! Help section owned by the `rev` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_rev_help_section() {
    print_section(
        "help.rev.title",
        &["help.rev.usage"],
        &[],
        &["help.rev.example1", "help.rev.example2"],
    );
}
