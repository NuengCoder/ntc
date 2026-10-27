use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.mode.admin_relaunch".into(),
        "正在以管理员身份重新启动 ntc...".into(),
    );

    m.insert(
        "cmd.mode.elevate_fail".into(),
        "启动提权进程失败：{}".into(),
    );

    m.insert(
        "cmd.mode.exe_fail".into(),
        "获取可执行文件路径失败：{}".into(),
    );

    m.insert("cmd.mode.launch_fail".into(), "启动进程失败：{}".into());

    m.insert(
        "cmd.mode.normal_relaunch".into(),
        "正在以普通模式重新启动 ntc...".into(),
    );

    m.insert(
        "cmd.mode.windows_only".into(),
        "mode 是仅限 Windows 的功能。".into(),
    );
}
