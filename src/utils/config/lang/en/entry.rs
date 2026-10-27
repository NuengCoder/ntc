use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("entry.goodbye".into(), "Goodbye!".into());

    m.insert(
        "entry.ntpkg_active".into(),
        "ntpkg: {} runtime{} active in PATH".into(),
    );

    m.insert(
        "entry.startup_hint".into(),
        "Type 'help' for available commands, 'exit' to quit.".into(),
    );

    m.insert("entry.watch_trigger".into(), "watch trigger: {}".into());

    m.insert("entry.watcher_failed".into(), "Watcher failed: {}".into());

    m.insert(
        "entry.watcher_started".into(),
        "File watcher started (recursive)".into(),
    );

    m.insert(
        "entry.welcome_fmt".into(),
        " Welcome to ntc {} - Navigate, Toolkit, Center ".into(),
    );
}
