use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qsc.listen".into(),
        "qsc listen ## 启动QSC守护进程（后台进程）".into(),
    );

    m.insert(
        "help.qsc.off".into(),
        "config qsc off ## 禁用QSC热键".into(),
    );

    m.insert(
        "help.qsc.set".into(),
        "config qsc \"#c 0\" ## 设置热键（Ctrl+0）".into(),
    );

    m.insert(
        "help.qsc.set_cs".into(),
        "config qsc \"#c #s a\" ## 设置热键（Ctrl+Shift+A）".into(),
    );

    m.insert(
        "help.qsc.startup".into(),
        "qsc startup on|off ## 注册/取消注册为启动应用".into(),
    );

    m.insert(
        "help.qsc.status".into(),
        "qsc status ## 检查QSC守护进程是否在运行".into(),
    );

    m.insert("help.qsc.stop".into(), "qsc stop ## 停止QSC守护进程".into());

    m.insert("help.qsc.title".into(), "QSC（全局热键）命令：".into());

    m.insert(
        "help.qsc.example1".into(),
        "config qsc \"#c 0\" ## 设置 Ctrl+0 热键".into(),
    );

    m.insert(
        "help.qsc.example2".into(),
        "qsc listen ## 启动 QSC 守护进程".into(),
    );

    m.insert(
        "help.qsc.example3".into(),
        "qsc status ## 检查守护进程状态".into(),
    );
}
