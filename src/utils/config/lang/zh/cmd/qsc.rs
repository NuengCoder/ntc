use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qsc.already_running".into(),
        "QSC 守护进程已在运行。先使用 'qsc stop'。".into(),
    );

    m.insert("cmd.qsc.current_setting".into(), "  当前：{}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert(
        "cmd.qsc.daemon_unknown_err".into(),
        "QSC 守护进程：未知（{}）".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex1".into(),
        "    config qsc \\\"#c 0\\\"        Set hotkey to Ctrl+0".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex2".into(),
        "    config qsc \\\"#c #s a\\\"     Set hotkey to Ctrl+Shift+A".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex3".into(),
        "    config qsc \\\"#c #s #a F1\\\"  Set hotkey to Ctrl+Shift+Alt+F1".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_off".into(),
        "    config qsc off            Disable QSC hotkey".into(),
    );

    m.insert("cmd.qsc.help_configure".into(), "  请先配置热键：".into());

    m.insert(
        "cmd.qsc.help_listen".into(),
        "  qsc listen              启动 QSC 全局热键守护进程".into(),
    );

    m.insert(
        "cmd.qsc.help_status".into(),
        "  qsc status              检查 QSC 守护进程是否运行".into(),
    );

    m.insert(
        "cmd.qsc.help_stop".into(),
        "  qsc stop                停止 QSC 全局热键守护进程".into(),
    );

    m.insert("cmd.qsc.hotkey_label".into(), "  热键：{}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert(
        "cmd.qsc.not_configured".into(),
        "未配置 QSC 热键。先使用 'config qsc \"#c 0\"'。".into(),
    );

    m.insert(
        "cmd.qsc.not_running".into(),
        "QSC 守护进程未在运行。".into(),
    );

    m.insert("cmd.qsc.started".into(), "QSC 守护进程已启动。".into());

    m.insert(
        "cmd.qsc.startup_registered".into(),
        "QSC 守护进程已注册为启动应用程序。".into(),
    );

    m.insert(
        "cmd.qsc.startup_removed".into(),
        "QSC 守护进程已从启动应用程序中删除。".into(),
    );

    m.insert(
        "cmd.qsc.startup_usage".into(),
        "用法：qsc startup on|off".into(),
    );

    m.insert(
        "cmd.qsc.status_running".into(),
        "QSC 守护进程：运行中".into(),
    );

    m.insert(
        "cmd.qsc.status_stopped".into(),
        "QSC 守护进程：已停止".into(),
    );

    m.insert("cmd.qsc.stopped".into(), "QSC 守护进程已停止。".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC 全局热键：".into());
}
