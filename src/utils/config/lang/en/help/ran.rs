use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ran.deinit".into(),
        "ran deinit ## Remove NTCRANFILE.toml".into(),
    );

    m.insert("help.ran.help".into(), "ran help ## Show ran usage".into());

    m.insert(
        "help.ran.init".into(),
        "ran init ## Create a NTCRANFILE.toml template".into(),
    );

    m.insert(
        "help.ran.list".into(),
        "ran list ## List all targets in NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.run".into(),
        "ran ## Run the 'standalone' target (default)".into(),
    );

    m.insert(
        "help.ran.target".into(),
        "ran <target> ## Run a specific target from NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.title".into(),
        "RAN / TASK RUNNER COMMANDS:".into(),
    );

    m.insert(
        "help.ran.example1".into(),
        "ran ## Run default standalone target".into(),
    );

    m.insert(
        "help.ran.example2".into(),
        "ran build ## Run 'build' target".into(),
    );

    m.insert(
        "help.ran.example3".into(),
        "ran list ## List all targets".into(),
    );
}
