use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qsc.listen".into(),
        "qsc listen ## Start QSC daemon (background process)".into(),
    );

    m.insert(
        "help.qsc.off".into(),
        "config qsc off ## Disable QSC hotkey".into(),
    );

    m.insert(
        "help.qsc.set".into(),
        "config qsc \"#c 0\" ## Set hotkey (Ctrl+0)".into(),
    );

    m.insert(
        "help.qsc.set_cs".into(),
        "config qsc \"#c #s a\" ## Set hotkey (Ctrl+Shift+A)".into(),
    );

    m.insert(
        "help.qsc.startup".into(),
        "qsc startup on|off ## Register/unregister as startup app".into(),
    );

    m.insert(
        "help.qsc.status".into(),
        "qsc status ## Check if QSC daemon is running".into(),
    );

    m.insert("help.qsc.stop".into(), "qsc stop ## Stop QSC daemon".into());

    m.insert(
        "help.qsc.title".into(),
        "QSC (GLOBAL HOTKEY) COMMANDS:".into(),
    );

    m.insert(
        "help.qsc.example1".into(),
        "config qsc \"#c 0\" ## Set Ctrl+0 hotkey".into(),
    );

    m.insert(
        "help.qsc.example2".into(),
        "qsc listen ## Start QSC daemon".into(),
    );

    m.insert(
        "help.qsc.example3".into(),
        "qsc status ## Check daemon status".into(),
    );
}
