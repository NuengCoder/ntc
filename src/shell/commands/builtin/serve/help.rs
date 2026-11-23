//! Help section owned by the `serve` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_serve_help_section() {
    print_section(
        "help.serve.title",
        &[
            "help.serve.service",
            "help.serve.chess",
            "help.serve.git",
            "help.serve.arc",
            "help.serve.ndb",
            "help.serve.theme",
            "help.serve.fake",
            "help.serve.config",
            "help.serve.backup",
        ],
        &[],
        &["help.serve.example1"],
    );
}
