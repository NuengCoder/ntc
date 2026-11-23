//! Help section owned by the `version` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_version_help_section() {
    print_section(
        "help.version.title",
        &["help.version.usage"],
        &[],
        &["help.version.example1"],
    );
}
