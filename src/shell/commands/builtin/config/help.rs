//! Help section owned by the `config` command module.

use crate::shell::help::format::print_section;

pub(crate) fn print_config_help_section() {
    print_section(
        "help.config.title",
        &[
            "help.config.output",
            "help.config.depth",
            "help.config.linenumbers",
            "help.config.color",
            "help.config.threads",
            "help.config.history",
            "help.config.autosuggest",
            "help.config.idle",
            "help.config.git",
            "help.config.dir_view",
            "help.config.intro_view",
            "help.config.file_view",
            "help.config.todo",
            "help.config.size",
            "help.config.watch",
            "help.config.watch_trigger",
            "help.config.watch_trigger_off",
            "help.config.watch_limit",
            "help.config.qsc",
            "help.config.qsc_off",
            "help.config.lang",
            "help.config.env",
            "help.config.env_os",
            "help.config.rmenv",
            "help.config.serve",
            "help.config.update_check",
            "help.config.integration_shell",
            "help.config.code_tools",
        ],
        &[],
        &[
            "help.config.example1",
            "help.config.example2",
            "help.config.example3",
            "help.config.example4",
        ],
    );
}
