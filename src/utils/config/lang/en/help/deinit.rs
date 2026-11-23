use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.deinit.title".into(), "DEINIT COMMAND:".into());

    m.insert(
        "help.deinit.usage".into(),
        "deinit [ran|local|opencode] ## Remove project files (asks before each removal)".into(),
    );

    m.insert(
        "help.deinit.example1".into(),
        "deinit ran ## Remove NTCRANFILE.toml".into(),
    );
}
