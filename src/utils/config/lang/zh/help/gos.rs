use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gos.title".into(), "GOS 命令：".into());

    m.insert(
        "help.gos.usage".into(),
        "gos ## 列出子目录并选择一个".into(),
    );

    m.insert("help.gos.example1".into(), "gos ## 交互式子目录选择".into());
}
