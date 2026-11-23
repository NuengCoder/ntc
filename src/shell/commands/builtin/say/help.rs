//! Help section owned by the `say` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_say_help_section() {
    print_section(
        "help.say.title",
        &["help.say.usage"],
        &[],
        &["help.say.example1"],
    );
}
