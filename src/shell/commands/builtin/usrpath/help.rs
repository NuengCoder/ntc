//! Help section owned by the `usrpath` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_usrpath_help_section() {
    print_section(
        "help.usrpath.title",
        &[
            "help.usrpath.show",
            "help.usrpath.add",
            "help.usrpath.add_cwd",
            "help.usrpath.rm",
            "help.usrpath.move",
        ],
        &[],
        &[
            "help.usrpath.example1",
            "help.usrpath.example2",
            "help.usrpath.example3",
        ],
    );
}
