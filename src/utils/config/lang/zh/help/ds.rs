use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ds.title".into(), "DS 命令：".into());

    m.insert(
        "help.ds.usage".into(),
        "ds <模式> [-d <n>] [--no-fuzzy] ## 按名称搜索目录（--no-fuzzy 仅精确/部分匹配）".into(),
    );

    m.insert(
        "help.ds.example1".into(),
        "ds src ## 查找名为 'src' 的目录".into(),
    );
}
