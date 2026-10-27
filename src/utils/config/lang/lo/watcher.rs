use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "watcher.file_changed".into(),
        "\u{2190} file watcher ກວດພົບ".into(),
    );

    m.insert("watcher.more_events".into(), "\u{2026} ແລະ {} ອີກ".into());

    m.insert(
        "watcher.tree_refreshed".into(),
        "watcher \u{2192} ອັບເດດ tree ແລ້ວ".into(),
    );
}
