use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.syspath.title".into(), "SYSPATH 命令：".into());

    m.insert(
        "help.syspath.show".into(),
        "syspath show ## 显示系统级PATH条目".into(),
    );

    m.insert(
        "help.syspath.add".into(),
        "syspath add <路径> ## 将目录添加到系统PATH".into(),
    );

    m.insert(
        "help.syspath.add_cwd".into(),
        "syspath add --cwd | --here ## 将当前目录添加到系统PATH".into(),
    );

    m.insert(
        "help.syspath.rm".into(),
        "syspath rm <路径或编号> ## 从系统PATH中删除（按路径或索引）".into(),
    );

    m.insert(
        "help.syspath.move".into(),
        "syspath move <n> <m> ## 将条目#n移动到位置#m".into(),
    );

    m.insert(
        "help.syspath.example1".into(),
        "syspath show ## 显示系统 PATH 条目".into(),
    );

    m.insert(
        "help.syspath.example2".into(),
        "syspath add /usr/local/bin ## 添加到系统 PATH".into(),
    );

    m.insert(
        "help.syspath.example3".into(),
        "syspath move 3 1 ## 移动条目到位置 1".into(),
    );
}
