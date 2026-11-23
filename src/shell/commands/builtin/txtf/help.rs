//! Help section owned by the `txtf` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_txtf_help_section() {
    print_section(
        "help.txtf.title",
        &["help.txtf.usage"],
        &[],
        &[
            "help.txtf.example1",
            "help.txtf.example2",
            "help.txtf.example3",
            "help.txtf.example4",
        ],
    );
}
