use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.viewd.title".into(), "VIEWD 命令（仅目录）：".into());

    m.insert(
        "help.viewd.usage".into(),
        "viewd [<目录|@tp|@tp/目录/>] [-d N] ## 显示仅目录树".into(),
    );

    m.insert(
        "help.viewd.depth".into(),
        "viewd -d, --depth <n> ## 使用自定义深度".into(),
    );

    m.insert(
        "help.viewd.size".into(),
        "viewd -s, --size [最小值] ## 显示目录大小；带值时隐藏更小的目录".into(),
    );

    m.insert(
        "help.viewd.care".into(),
        "viewd -c, --care ## 计算大小时包含所有项".into(),
    );

    m.insert(
        "help.viewd.sort".into(),
        "viewd --sort size|char|ext ## 排序目录项".into(),
    );

    m.insert(
        "help.viewd.time".into(),
        "viewd -t, --time ## 显示执行时间".into(),
    );

    m.insert(
        "help.viewd.copy".into(),
        "viewd -cp, --copy ## 复制目录树到剪贴板".into(),
    );

    m.insert(
        "help.viewd.example1".into(),
        "viewd ## 仅显示当前目录树中的目录".into(),
    );

    m.insert(
        "help.viewd.example2".into(),
        "viewd -d 2 --size ## 显示两层目录及大小".into(),
    );

    m.insert(
        "help.viewd.example3".into(),
        "viewd --sort size ## 最大的目录在前".into(),
    );

    m.insert(
        "help.viewd.example4".into(),
        "viewd @docs/ ## 显示传送点中的目录".into(),
    );
}
