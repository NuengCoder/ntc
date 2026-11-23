//! Help section owned by the `py_activate` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_py_activate_help_section() {
    print_section(
        "help.py_activate.title",
        &[
            "help.py_activate.activate",
            "help.py_activate.activate_path",
        ],
        &[],
        &["help.py_activate.example1", "help.py_activate.example2"],
    );
}
