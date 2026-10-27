use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest ON|OFF ## Show/set autosuggest (ghost text)".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color ON|OFF ## Show/set color output".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## Show/set max depth (1-20)".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|path] ## Show/set history".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [language] ## Show/set interface language".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers ON|OFF ## Show/set line numbers".into(),
    );

    m.insert(
        "help.config.local_deinit".into(),
        "local deinit ## Remove ntconfig.toml from current directory".into(),
    );

    m.insert(
        "help.config.local_help".into(),
        "local help ## Show local command usage".into(),
    );

    m.insert(
        "help.config.local_init".into(),
        "local init ## Create ntconfig.toml template (commented)".into(),
    );

    m.insert(
        "help.config.opencg".into(),
        "opencg ## Open config.toml in editor".into(),
    );

    m.insert(
        "help.config.output".into(),
        "config output [path] ## Show/set output path".into(),
    );

    m.insert(
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## Set QSC global hotkey".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## Disable QSC hotkey".into(),
    );

    m.insert(
        "help.config.resetcg".into(),
        "resetcg ## Reset config to defaults (with backup option)".into(),
    );

    m.insert(
        "help.config.restorecg".into(),
        "restorecg ## Restore config from backup".into(),
    );

    m.insert(
        "help.config.showcg".into(),
        "showcg ## Show current configuration overview".into(),
    );

    m.insert(
        "help.config.showenv".into(),
        "showenv ## Show environment variables (first 5 PATH entries)".into(),
    );

    m.insert(
        "help.config.showenv_all".into(),
        "showenv --all ## Show all PATH entries".into(),
    );

    m.insert(
        "help.config.showenv_d".into(),
        "showenv -d <n> ## Show first n PATH entries".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## Show/set thread count".into(),
    );

    m.insert("help.config.title".into(), "CONFIGURATION COMMANDS:".into());

    m.insert(
        "help.config.watch".into(),
        "config watch ON|OFF ## Enable/disable file watcher".into(),
    );

    m.insert(
        "help.config.watch_trigger".into(),
        "config watch trigger <a> ## Auto-run alias when watcher fires".into(),
    );

    m.insert(
        "help.config.watch_trigger_off".into(),
        "config watch trigger off ## Disable auto-run".into(),
    );

    m.insert(
        "help.config.watch_limit".into(),
        "config watch limit <n> ## Rows shown in watcher change box".into(),
    );

    m.insert(
        "help.config.wi".into(),
        "wi [path] ## Live file-change feed (q=quit)".into(),
    );

    m.insert(
        "help.config.example1".into(),
        "config output ~/Desktop ## Set output path to Desktop".into(),
    );

    m.insert(
        "help.config.example2".into(),
        "config depth 3 ## Set max depth to 3".into(),
    );

    m.insert(
        "help.config.example3".into(),
        "config color ON ## Enable color output".into(),
    );

    m.insert(
        "help.config.example4".into(),
        "config threads 8 ## Set thread count to 8".into(),
    );

    m.insert(
        "help.config.example5".into(),
        "showcg ## Show current configuration overview".into(),
    );

    m.insert(
        "help.config.git".into(),
        "config git ON|OFF ## Enable/disable git integration (prompt + symbols)".into(),
    );

    m.insert(
        "help.config.dir_view".into(),
        "config dir_view ON|OFF ## Show/hide directory tree in shell".into(),
    );

    m.insert(
        "help.config.file_view".into(),
        "config file_view ON|OFF ## Include files in automatic navigation tree (alias: fv)".into(),
    );

    m.insert(
        "help.config.todo".into(),
        "config todo format <txt|md|json|toml|yaml|html> ## Set the todo file format".into(),
    );

    m.insert("help.config.size".into(), "config size all|file|dir ON|OFF ## Auto-calculate sizes in navigation tree; uses --care (alias: s)".into());

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## Show/hide daily new-release notice (alias: uc)".into(),
    );

    m.insert("help.config.serve".into(), "config serve [service] [--port-default N] [--bind-default ADDR] ## Per-service serve defaults (alias: serve config)".into());

    m.insert("help.config.intro_view".into(), "config intro_view ON|OFF ## Show/hide the startup logo, ntpkg and watcher notices (alias: iv)".into());
}
