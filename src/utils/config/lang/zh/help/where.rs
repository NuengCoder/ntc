use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.where.title".into(), "WHERE 命令：".into());

    m.insert(
        "help.where.usage".into(),
        "where [-a|--all] [<程序>] ## 显示 ntc 可执行文件，或在 PATH 中查找程序".into(),
    );

    m.insert(
        "help.where.example1".into(),
        "where -a ## 显示所有 ntc 配置、会话、软件包和保险库路径".into(),
    );

    m.insert(
        "help.where.example2".into(),
        "where git ## 显示 PATH 中的第一个 git（-a 列出全部）".into(),
    );
}
