use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.teleport.add".into(),
        "tp add <名称> ## 保存当前目录".into(),
    );

    m.insert(
        "help.teleport.add_path".into(),
        "tp add <名称> <路径> ## 保存指定路径".into(),
    );

    m.insert(
        "help.teleport.at".into(),
        "@<名称> ## 快速传送快捷方式".into(),
    );

    m.insert(
        "help.teleport.cls".into(),
        "tp cls ## 清除所有保存点".into(),
    );

    m.insert(
        "help.teleport.info".into(),
        "tp info <名称> ## 显示保存点详情".into(),
    );

    m.insert(
        "help.teleport.jump".into(),
        "tp jump <名称> ## 传送到已保存位置".into(),
    );

    m.insert(
        "help.teleport.jump_num".into(),
        "tp jump <数字> ## 按编号传送".into(),
    );

    m.insert(
        "help.teleport.list".into(),
        "tp list ## 列出所有传送点".into(),
    );

    m.insert("help.teleport.menu".into(), "tp ## 交互式菜单".into());

    m.insert(
        "help.teleport.rm".into(),
        "tp rm <名称> ## 删除传送点".into(),
    );

    m.insert(
        "help.teleport.rnm".into(),
        "tp rnm <旧名称> to <新名称> ## 重命名保存点".into(),
    );

    m.insert("help.teleport.title".into(), "传送命令：".into());

    m.insert(
        "help.teleport.tpb".into(),
        "tpb ## 传送到上一个位置（撤销传送）".into(),
    );

    m.insert(
        "help.teleport.tpb_clear".into(),
        "tpb clear ## 清除传送历史".into(),
    );

    m.insert(
        "help.teleport.tpb_history".into(),
        "tpb history ## 显示传送导航历史".into(),
    );

    m.insert(
        "help.teleport.example1".into(),
        "tp add web ## 将当前目录保存为 'web'".into(),
    );

    m.insert(
        "help.teleport.example2".into(),
        "tp add docs ~/Documents ## 保存指定路径".into(),
    );

    m.insert(
        "help.teleport.example3".into(),
        "tp jump web ## 传送到 'web'".into(),
    );

    m.insert(
        "help.teleport.example4".into(),
        "tp list ## 列出所有保存点".into(),
    );

    m.insert(
        "help.teleport.example5".into(),
        "@web ## 快速传送快捷方式".into(),
    );
}
