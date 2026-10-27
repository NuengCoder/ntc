use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("entry.goodbye".into(), "再见！".into());

    m.insert(
        "entry.ntpkg_active".into(),
        "ntpkg：{} 个运行时在 PATH 中处于活动状态".into(),
    );

    m.insert(
        "entry.startup_hint".into(),
        "输入 'help' 查看可用命令，输入 'exit' 退出。".into(),
    );

    m.insert("entry.watch_trigger".into(), "监视触发：{}".into());

    m.insert("entry.watcher_failed".into(), "监视器失败：{}".into());

    m.insert(
        "entry.watcher_started".into(),
        "文件监视器已启动（递归）".into(),
    );

    m.insert(
        "entry.welcome_fmt".into(),
        " 欢迎使用 ntc {} - 导航, 工具包, 中心 ".into(),
    );
}
