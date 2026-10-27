use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.size.care".into(),
        "size --care ## 计算所有文件的大小（care 模式）".into(),
    );

    m.insert(
        "help.size.path".into(),
        "size -p, --path <文件|路径|@tp> ## 显示指定文件、路径或传送点的大小".into(),
    );

    m.insert("help.size.size".into(), "size ## 显示当前目录大小".into());

    m.insert(
        "help.size.time".into(),
        "size -t, --time ## 显示耗时".into(),
    );

    m.insert("help.size.title".into(), "大小命令：".into());

    m.insert(
        "help.size.example1".into(),
        "size ## 显示当前目录大小".into(),
    );

    m.insert(
        "help.size.example2".into(),
        "size ../src ## 显示其他目录的大小".into(),
    );

    m.insert(
        "help.size.example3".into(),
        "size Cargo.toml ## 显示文件的大小".into(),
    );

    m.insert(
        "help.size.example4".into(),
        "size @web --care -t ## 传送点使用 care 模式并显示耗时".into(),
    );
}
