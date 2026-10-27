use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("igcare_export.all".into(), "select all".into());

    m.insert("igcare_export.cancel".into(), "cancel".into());

    m.insert(
        "igcare_export.extra_supported_extensions".into(),
        "Extra supported extensions".into(),
    );

    m.insert(
        "igcare_export.extra_supported_files".into(),
        "Extra supported files".into(),
    );

    m.insert("igcare_export.finish".into(), "finish export".into());

    m.insert(
        "igcare_export.ignored_dirs".into(),
        "Ignored directories".into(),
    );

    m.insert(
        "igcare_export.ignored_extensions".into(),
        "Ignored extensions".into(),
    );

    m.insert("igcare_export.ignored_files".into(), "Ignored files".into());

    m.insert("igcare_export.items".into(), "{} item(s)".into());

    m.insert("igcare_export.prompt".into(), "igcare-export>".into());

    m.insert(
        "igcare_export.select".into(),
        "select/deselect by number".into(),
    );

    m.insert("igcare_export.selected".into(), "selected".into());

    m.insert("igcare_export.selected_mark".into(), "(selected)".into());

    m.insert(
        "igcare_export.title".into(),
        "igcare export — Select Categories".into(),
    );

    m.insert(
        "igcare_export.cared_dirs".into(),
        "Cared directories".into(),
    );

    m.insert(
        "igcare_export.cared_extensions".into(),
        "Cared extensions".into(),
    );

    m.insert("igcare_export.cared_files".into(), "Cared files".into());
}
