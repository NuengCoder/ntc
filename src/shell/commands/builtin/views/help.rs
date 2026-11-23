//! Help section owned by the `views` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_views_help_section() {
    print_section(
        "help.views.title",
        &[
            "help.views.usage",
            "help.views.size",
            "help.views.sort",
            "help.views.newline",
            "help.views.trim",
        ],
        &["help.views.note"],
        &["help.views.example1", "help.views.example2"],
    );
}
