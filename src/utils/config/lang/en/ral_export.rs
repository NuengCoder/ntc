use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ral_export.all".into(), "select all".into());

    m.insert("ral_export.cancel".into(), "cancel".into());

    m.insert(
        "ral_export.export_cancelled".into(),
        "Export cancelled.".into(),
    );

    m.insert("ral_export.finish".into(), "finish export".into());

    m.insert("ral_export.prompt".into(), "ral-export>".into());

    m.insert(
        "ral_export.select".into(),
        "select/deselect by number".into(),
    );

    m.insert("ral_export.selected".into(), "selected".into());

    m.insert("ral_export.selected_mark".into(), "(selected)".into());

    m.insert(
        "ral_export.title".into(),
        "ral export — Select Aliases to Export".into(),
    );
}
