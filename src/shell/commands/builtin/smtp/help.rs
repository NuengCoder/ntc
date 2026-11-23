//! Help section owned by the `smtp` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_smtp_help_section() {
    print_section(
        "help.smtp.title",
        &["help.smtp.usage"],
        &[],
        &["help.smtp.example1"],
    );
}
