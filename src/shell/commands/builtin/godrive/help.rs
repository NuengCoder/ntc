//! Help section owned by the `godrive` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_godrive_help_section() {
    print_section(
        "help.godrive.title",
        &["help.godrive.usage"],
        &[],
        &["help.godrive.example1", "help.godrive.example2"],
    );
}
