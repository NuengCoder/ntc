use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ignore.applied".into(), "{} {} item(s)".into());

    m.insert(
        "cmd.ignore.care_example_dir".into(),
        "  care dir build,dist".into(),
    );

    m.insert(
        "cmd.ignore.care_example_ext".into(),
        "  care ext txt,md,json".into(),
    );

    m.insert(
        "cmd.ignore.care_example_file".into(),
        "  care file .env".into(),
    );

    m.insert(
        "cmd.ignore.care_examples_title".into(),
        "Care examples:".into(),
    );

    m.insert(
        "cmd.ignore.check_hint".into(),
        "Run 'igcared' to see current ignore/care settings".into(),
    );

    m.insert(
        "cmd.ignore.dir_desc".into(),
        "ignore dir <name>[, ...] Ignore directories".into(),
    );

    m.insert(
        "cmd.ignore.dir_usage".into(),
        "Usage: ignore dir <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.example_dir".into(),
        "  ignore dir temp,logs".into(),
    );

    m.insert(
        "cmd.ignore.example_ext".into(),
        "  ignore ext log,tmp,bak".into(),
    );

    m.insert(
        "cmd.ignore.example_file".into(),
        "  ignore file Cargo.lock".into(),
    );

    m.insert("cmd.ignore.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.ignore.ext_desc".into(),
        "ignore ext <ext>[, ...] Ignore file extensions".into(),
    );

    m.insert(
        "cmd.ignore.ext_usage".into(),
        "Usage: ignore ext <ext>[, <ext>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.file_desc".into(),
        "ignore file <name>[, ...] Ignore specific files".into(),
    );

    m.insert(
        "cmd.ignore.file_usage".into(),
        "Usage: ignore file <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.title".into(),
        "ignore \u{2014} Manage ignored items".into(),
    );

    m.insert("cmd.ignore.scope_local".into(), "local config".into());

    m.insert("cmd.ignore.scope_global".into(), "global config".into());

    m.insert(
        "cmd.ignore.scope_help".into(),
        "Use '-g' / '--global' to target the global config (inside a folder with ntconfig.toml)."
            .into(),
    );

    m.insert("cmd.ignore.scope_target".into(), "Target: {}".into());
}
