use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pathman.syspath_add".into(),
        "syspath add <路径> ## 将目录添加到系统PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_add_cwd".into(),
        "syspath add --cwd | --here ## 将当前目录添加到系统PATH".into(),
    );

    m.insert(
        "help.pathman.syspath_move".into(),
        "syspath move <n> <m> ## 将条目#n移动到位置#m".into(),
    );

    m.insert(
        "help.pathman.syspath_rm".into(),
        "syspath rm <路径或编号> ## 从系统PATH中删除（按路径或索引）".into(),
    );

    m.insert(
        "help.pathman.syspath_show".into(),
        "syspath show ## 显示系统级PATH条目".into(),
    );

    m.insert(
        "help.pathman.title".into(),
        "路径管理器（Windows）：".into(),
    );

    m.insert(
        "help.pathman.usrpath_add".into(),
        "usrpath add <路径> ## 将目录添加到用户PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_add_cwd".into(),
        "usrpath add --cwd | --here ## 将当前目录添加到用户PATH".into(),
    );

    m.insert(
        "help.pathman.usrpath_move".into(),
        "usrpath move <n> <m> ## 将条目#n移动到位置#m".into(),
    );

    m.insert(
        "help.pathman.usrpath_rm".into(),
        "usrpath rm <路径或编号> ## 从用户PATH中删除（按路径或索引）".into(),
    );

    m.insert(
        "help.pathman.usrpath_show".into(),
        "usrpath show ## 显示用户级PATH条目".into(),
    );

    m.insert(
        "help.pathman.example1".into(),
        "usrpath show ## 显示用户 PATH 条目".into(),
    );

    m.insert(
        "help.pathman.example2".into(),
        "usrpath add C:\\tools ## 添加到用户 PATH".into(),
    );

    m.insert(
        "help.pathman.example3".into(),
        "usrpath rm 2 ## 从用户 PATH 删除条目 #2".into(),
    );

    m.insert(
        "help.pathman.example4".into(),
        "syspath show ## 显示系统 PATH 条目".into(),
    );

    m.insert(
        "help.pathman.example5".into(),
        "syspath add C:\\bin ## 添加到系统 PATH".into(),
    );

    m.insert(
        "help.pathman.example6".into(),
        "syspath move 3 1 ## 移动条目到位置 1".into(),
    );
}
