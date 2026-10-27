use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.nav.back".into(),
        "back [n] ## 返回父目录（或n层）".into(),
    );

    m.insert("help.nav.cd".into(), "cd <路径> ## 导航到目录".into());

    m.insert(
        "help.nav.cd_tp".into(),
        "cd to <tp名称> ## 传送到已保存的传送点".into(),
    );

    m.insert(
        "help.nav.fgo".into(),
        "fgo <模式> [-d <n>] ## 查找文件，导航到所选文件的父目录".into(),
    );

    m.insert(
        "help.nav.godrive".into(),
        "godrive ## 列出并选择Windows驱动器".into(),
    );

    m.insert("help.nav.gos".into(), "gos ## 列出子目录并选择一个".into());

    m.insert(
        "help.nav.gosc".into(),
        "gosc ## 连续导航（0退出，v/s/vs查看）".into(),
    );

    m.insert("help.nav.title".into(), "导航命令：".into());

    m.insert(
        "help.nav.where".into(),
        "where [<程序>] ## 显示ntc位置信息，或在PATH中查找程序".into(),
    );

    m.insert(
        "help.nav.example1".into(),
        "cd src ## 导航到 src 目录".into(),
    );

    m.insert("help.nav.example2".into(), "back ## 返回父目录".into());

    m.insert("help.nav.example3".into(), "gos ## 交互式子目录选择".into());

    m.insert(
        "help.nav.example4".into(),
        "cd to web ## 传送到 'web' 保存点".into(),
    );

    m.insert(
        "help.nav.example5".into(),
        "where git ## 在PATH中查找 git 的位置".into(),
    );
}
