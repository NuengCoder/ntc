use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tp.title".into(), "传送命令：".into());

    m.insert("help.tp.menu".into(), "tp ## 交互式菜单".into());

    m.insert("help.tp.add".into(), "tp add <名称> ## 保存当前目录".into());

    m.insert(
        "help.tp.add_path".into(),
        "tp add <名称> <路径> ## 保存指定路径".into(),
    );

    m.insert(
        "help.tp.jump".into(),
        "tp jump <名称> ## 传送到已保存位置".into(),
    );

    m.insert(
        "help.tp.jump_num".into(),
        "tp jump <数字> ## 按编号传送".into(),
    );

    m.insert("help.tp.list".into(), "tp list ## 列出所有传送点".into());

    m.insert(
        "help.tp.info".into(),
        "tp info <名称> ## 显示保存点详情".into(),
    );

    m.insert("help.tp.rm".into(), "tp rm <名称> ## 删除传送点".into());

    m.insert(
        "help.tp.rnm".into(),
        "tp rnm <旧名称> to <新名称> ## 重命名保存点".into(),
    );

    m.insert("help.tp.cls".into(), "tp cls ## 清除所有保存点".into());

    m.insert("help.tp.at".into(), "@<名称> ## 快速传送快捷方式".into());

    m.insert(
        "help.tp.example1".into(),
        "tp add web ## 将当前目录保存为 'web'".into(),
    );

    m.insert(
        "help.tp.example2".into(),
        "tp add docs ~/Documents ## 保存指定路径".into(),
    );

    m.insert(
        "help.tp.example3".into(),
        "tp jump web ## 传送到 'web'".into(),
    );

    m.insert(
        "help.tp.example4".into(),
        "tp list ## 列出所有保存点".into(),
    );

    m.insert("help.tp.example5".into(), "@web ## 快速传送快捷方式".into());
}
