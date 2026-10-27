use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "config.general.autosuggest".into(),
        " config autosuggest ON|OFF Show/set autosuggest".into(),
    );

    m.insert(
        "config.general.color".into(),
        " config color ON|OFF Show/set color".into(),
    );

    m.insert(
        "config.general.depth".into(),
        " config depth [n] Show/set max depth (1-20)".into(),
    );

    m.insert("config.general.examples".into(), "Examples:".into());

    m.insert(
        "config.general.history".into(),
        " config history [on|off|path] Show/set history".into(),
    );

    m.insert(
        "config.general.lang".into(),
        " config lang [language] Show/set interface language".into(),
    );

    m.insert(
        "config.general.linenumbers".into(),
        " config linenumbers ON|OFF Show/set line numbers".into(),
    );

    m.insert(
        "config.general.output".into(),
        " config output [path] Show/set output path".into(),
    );

    m.insert(
        "config.general.qsc".into(),
        " config qsc \"#c 0\" Show/set QSC global hotkey".into(),
    );

    m.insert("config.general.settings".into(), "Settings:".into());

    m.insert(
        "config.general.threads".into(),
        " config threads [n] Show/set thread count".into(),
    );

    m.insert(
        "config.general.title".into(),
        "config \u{2014} View or change configuration settings".into(),
    );

    m.insert(
        "config.general.watch".into(),
        " config watch ON|OFF Show/set file watcher".into(),
    );

    m.insert("config.lang.set".into(), "Language set to: {}".into());
}
