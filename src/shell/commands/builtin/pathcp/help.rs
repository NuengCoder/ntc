//! Help section owned by the `pathcp` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_pathcp_help_section() {
    print_section(
        "help.pathcp.title",
        &["help.pathcp.usage"],
        &[],
        &["help.pathcp.example1"],
    );
}
