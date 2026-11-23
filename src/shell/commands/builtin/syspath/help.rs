//! Help section owned by the `syspath` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_syspath_help_section() {
    print_section(
        "help.syspath.title",
        &[
            "help.syspath.show",
            "help.syspath.add",
            "help.syspath.add_cwd",
            "help.syspath.rm",
            "help.syspath.move",
        ],
        &[],
        &[
            "help.syspath.example1",
            "help.syspath.example2",
            "help.syspath.example3",
        ],
    );
}
