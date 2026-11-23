//! Help section owned by the `where` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_where_help_section() {
    print_section(
        "help.where.title",
        &["help.where.usage"],
        &[],
        &["help.where.example1", "help.where.example2"],
    );
}
