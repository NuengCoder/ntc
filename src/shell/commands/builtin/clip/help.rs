//! Help section owned by the `clip` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_clip_help_section() {
    print_section(
        "help.clip.title",
        &[
            "help.clip.show",
            "help.clip.list",
            "help.clip.select",
            "help.clip.clear",
        ],
        &[],
        &["help.clip.example1", "help.clip.example2"],
    );
}
