use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "watcher.file_changed".into(),
        "\u{2190} ตัวเฝ้าดูไฟล์ตรวจพบ".into(),
    );

    m.insert(
        "watcher.more_events".into(),
        "\u{2026} และอีก {} เหตุการณ์".into(),
    );

    m.insert(
        "watcher.tree_refreshed".into(),
        "watcher \u{2192} รีเฟรชต้นไม้แล้ว".into(),
    );
}
