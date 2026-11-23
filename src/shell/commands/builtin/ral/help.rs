//! Help section owned by the `ral` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_ral_help_section() {
    print_section(
        "help.ral.title",
        &[
            "help.ral.add",
            "help.ral.add_param",
            "help.ral.env_ref",
            "help.ral.add_defaults",
            "help.ral.add_array",
            "help.ral.add_rest",
            "help.ral.edit",
            "help.ral.rnm",
            "help.ral.rm",
            "help.ral.list",
            "help.ral.info",
            "help.ral.cls",
            "help.ral.export_all",
            "help.ral.export_select",
            "help.ral.import",
            "help.ral.install",
            "help.ral.make",
            "help.ral.hintn",
        ],
        &[],
        &[
            "help.ral.example1",
            "help.ral.example2",
            "help.ral.example3",
            "help.ral.example4",
            "help.ral.example5",
            "help.ral.example6",
        ],
    );
}
