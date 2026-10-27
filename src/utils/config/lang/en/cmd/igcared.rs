use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcared.extra_exts".into(), "Extra extensions:".into());

    m.insert("cmd.igcared.extra_files".into(), "Extra files:".into());

    m.insert(
        "cmd.igcared.ignored_dirs".into(),
        "Ignored directories:".into(),
    );

    m.insert(
        "cmd.igcared.ignored_exts".into(),
        "Ignored extensions:".into(),
    );

    m.insert("cmd.igcared.ignored_files".into(), "Ignored files:".into());

    m.insert(
        "cmd.igcared.local_config".into(),
        "   Local config: {}".into(),
    );

    m.insert("cmd.igcared.section_global".into(), "[global]".into());

    m.insert("cmd.igcared.section_local".into(), "[local]".into());

    m.insert("cmd.igcared.cared_dirs".into(), "Cared directories:".into());

    m.insert("cmd.igcared.cared_exts".into(), "Cared extensions:".into());

    m.insert("cmd.igcared.cared_files".into(), "Cared files:".into());

    m.insert("cmd.igcared.none".into(), "(none)".into());

    m.insert(
        "cmd.igcared.no_local".into(),
        "(no local config in this directory)".into(),
    );

    m.insert("cmd.igcared.scope_help".into(), "Local entries take priority; global entries keep working and the global config is never changed.".into());
}
