//! Help section owned by the `care` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_care_help_section() {
    print_section(
        "help.care.title",
        &[
            "help.care.care_dir",
            "help.care.care_ext",
            "help.care.care_file",
        ],
        &[],
        &["help.care.example1"],
    );
}
