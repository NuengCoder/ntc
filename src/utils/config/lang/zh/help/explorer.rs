use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=帮助, b=父目录, f=查找（结果显示路径）".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "g/:=命令（file|dir|line）, /=实时过滤, e=编辑器".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "r=重命名, d=删除, n=新文件, m=新目录, i=切换隐藏文件".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s=切换排序, S=反向排序（含目录）, F5=刷新".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "查找：输入=查询, Tab=hjkl/方向键, Enter=打开, Esc=返回".into(),
    );

    m.insert(
        "help.explorer.launch".into(),
        "explorer, nfe [路径] ## 启动全屏终端文件浏览器".into(),
    );

    m.insert("help.explorer.title".into(), "文件浏览器（NFE）：".into());

    m.insert(
        "help.explorer.example1".into(),
        "explorer ## 在当前目录启动文件管理器".into(),
    );

    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## 在 ~/projects 打开文件管理器".into(),
    );
}
