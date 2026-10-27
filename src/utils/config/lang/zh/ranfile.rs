use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ranfile.available_title".into(), "可用目标：".into());

    m.insert("ranfile.depends_on".into(), " ─▶ 依赖：".into());

    m.insert(
        "ranfile.runner.hint".into(),
        "运行 {} 查看可用目标。".into(),
    );
}
