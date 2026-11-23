//! Help section owned by the `run` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_run_help_section() {
    print_section(
        "help.run.title",
        &["help.run.usage", "help.run.direct", "help.run.param"],
        &[],
        &["help.run.example1"],
    );
}
