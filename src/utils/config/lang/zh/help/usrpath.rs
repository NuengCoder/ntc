use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.usrpath.title".into(),
        "路径管理器：".into(),
    );

    m.insert(
        "help.usrpath.show".into(),
        "usrpath show ## 显示用户级PATH条目".into(),
    );

    m.insert(
        "help.usrpath.add".into(),
        "usrpath add <路径> ## 将目录添加到用户PATH".into(),
    );

    m.insert(
        "help.usrpath.add_cwd".into(),
        "usrpath add --cwd | --here ## 将当前目录添加到用户PATH".into(),
    );

    m.insert(
        "help.usrpath.rm".into(),
        "usrpath rm <路径或编号> ## 从用户PATH中删除（按路径或索引）".into(),
    );

    m.insert(
        "help.usrpath.move".into(),
        "usrpath move <n> <m> ## 将条目#n移动到位置#m".into(),
    );

    m.insert(
        "help.usrpath.example1".into(),
        "usrpath show ## 显示用户 PATH 条目".into(),
    );

    m.insert(
        "help.usrpath.example2".into(),
        "usrpath add ~/bin ## 添加到用户 PATH".into(),
    );

    m.insert(
        "help.usrpath.example3".into(),
        "usrpath rm 2 ## 从用户 PATH 删除条目 #2".into(),
    );
}
