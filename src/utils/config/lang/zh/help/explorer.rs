use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=帮助, l=跳转到行, b=父目录, f=查找".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "/=过滤, e=编辑器, r=重命名, d=删除".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "n=新文件, m=新目录, i=切换隐藏文件".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s=切换排序, S=反向排序, F5=刷新".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "q/Esc=退出, 鼠标=点击/滚动".into(),
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
