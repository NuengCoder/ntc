//! Help section owned by the `doctor` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_doctor_help_section() {
    print_section(
        "help.doctor.title",
        &["help.doctor.usage"],
        &[],
        &["help.doctor.example1"],
    );
}
