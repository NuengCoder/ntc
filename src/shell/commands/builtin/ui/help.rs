//! Help section owned by the `ui` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ui_help_section() {
    print_section(
        "help.ui.title",
        &["help.ui.usage"],
        &[],
        &["help.ui.example1"],
    );
}
