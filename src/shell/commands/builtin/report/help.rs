//! Help section owned by the `report` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_report_help_section() {
    print_section(
        "help.report.title",
        &["help.report.project", "help.report.clip"],
        &[],
        &["help.report.example1", "help.report.example2"],
    );
}
