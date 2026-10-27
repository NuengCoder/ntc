use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.care.dir_desc".into(),
        "care dir <name>[, ...] Stop ignoring directories".into(),
    );

    m.insert(
        "cmd.care.dir_usage".into(),
        "Usage: care dir <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.care.ext_desc".into(),
        "care ext <ext>[, ...] Add extra supported extensions".into(),
    );

    m.insert(
        "cmd.care.ext_usage".into(),
        "Usage: care ext <ext>[, <ext>, ...]".into(),
    );

    m.insert(
        "cmd.care.file_desc".into(),
        "care file <name>[, ...] Add extra supported files".into(),
    );

    m.insert(
        "cmd.care.file_usage".into(),
        "Usage: care file <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.care.title".into(),
        "care \u{2014} Manage cared (un-ignored) items".into(),
    );

    m.insert(
        "cmd.care.scope_help".into(),
        "Use '-g' / '--global' to target the global config (inside a folder with ntconfig.toml)."
            .into(),
    );
}
