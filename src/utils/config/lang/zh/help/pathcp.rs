use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pathcp.title".into(), "PATHCP 命令：".into());

    m.insert(
        "help.pathcp.usage".into(),
        "pathcp ## 将当前目录路径复制到剪贴板".into(),
    );

    m.insert(
        "help.pathcp.example1".into(),
        "pathcp ## 复制当前目录路径".into(),
    );
}
