use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.other.clear".into(), "clear ## Clear screen".into());

    m.insert(
        "help.other.deinit".into(),
        "deinit [--ran|--local] ## Remove project files".into(),
    );

    m.insert("help.other.exit".into(), "exit, quit ## Exit ntc".into());

    m.insert("help.other.help".into(), "help ## Show this help".into());

    m.insert("help.other.init".into(), "init [--ran|--local|--opencode] ## Initialize project files (NTCRANFILE.toml, ntconfig.toml, .opencode/)".into());

    m.insert(
        "help.other.print".into(),
        "print <text> [--loop N] [-n|-t] ## Alias for say".into(),
    );

    m.insert(
        "help.other.say".into(),
        "say <text> [--loop N] [-n|-t] ## Print text (no file redirect)".into(),
    );

    m.insert("help.other.title".into(), "OTHER COMMANDS:".into());

    m.insert(
        "help.other.tutorial".into(),
        "tutorial ## Start the interactive step-by-step tutorial".into(),
    );

    m.insert("help.other.ui".into(), "ui ## Show current UI mode".into());

    m.insert(
        "help.other.version".into(),
        "version ## Show version".into(),
    );

    m.insert(
        "help.other.example1".into(),
        "init --ran ## Initialize NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.other.example2".into(),
        "clear ## Clear the screen".into(),
    );

    m.insert(
        "help.other.example3".into(),
        "version ## Show version info".into(),
    );

    m.insert(
        "help.other.example4".into(),
        "tutorial ## Start interactive tutorial".into(),
    );
}
