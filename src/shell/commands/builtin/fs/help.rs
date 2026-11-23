//! Help section owned by the `fs` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_fs_help_section() {
    print_section(
        "help.fs.title",
        &["help.fs.usage"],
        &[],
        &["help.fs.example1"],
    );
}
