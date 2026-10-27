use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "watcher.file_changed".into(),
        "\u{2190} 检测到文件变化".into(),
    );

    m.insert("watcher.more_events".into(), "\u{2026} 还有 {} 个".into());

    m.insert(
        "watcher.tree_refreshed".into(),
        "监视器 \u{2192} 树已刷新".into(),
    );
}
