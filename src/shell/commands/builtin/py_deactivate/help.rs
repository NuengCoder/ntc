//! Help section owned by the `py_deactivate` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_py_deactivate_help_section() {
    print_section(
        "help.py_deactivate.title",
        &["help.py_deactivate.usage"],
        &[],
        &["help.py_deactivate.example1"],
    );
}
