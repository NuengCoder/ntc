use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qsc.already_running".into(),
        "QSC daemon is already running. Use 'qsc stop' first.".into(),
    );

    m.insert("cmd.qsc.current_setting".into(), "  Current: {}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert(
        "cmd.qsc.daemon_unknown_err".into(),
        "QSC daemon: unknown ({})".into(),
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

    m.insert(
        "cmd.qsc.help_configure".into(),
        "  First configure the hotkey:".into(),
    );

    m.insert(
        "cmd.qsc.help_listen".into(),
        "  qsc listen              Start the QSC global hotkey daemon".into(),
    );

    m.insert(
        "cmd.qsc.help_status".into(),
        "  qsc status              Check if QSC daemon is running".into(),
    );

    m.insert(
        "cmd.qsc.help_stop".into(),
        "  qsc stop                Stop the QSC global hotkey daemon".into(),
    );

    m.insert("cmd.qsc.hotkey_label".into(), "  Hotkey: {}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert(
        "cmd.qsc.not_configured".into(),
        "No QSC hotkey configured. Use 'config qsc \"#c 0\"' first.".into(),
    );

    m.insert(
        "cmd.qsc.not_running".into(),
        "QSC daemon is not running.".into(),
    );

    m.insert("cmd.qsc.started".into(), "QSC daemon started.".into());

    m.insert(
        "cmd.qsc.startup_registered".into(),
        "QSC daemon registered as a startup app.".into(),
    );

    m.insert(
        "cmd.qsc.startup_removed".into(),
        "QSC daemon removed from startup apps.".into(),
    );

    m.insert(
        "cmd.qsc.startup_usage".into(),
        "Usage: qsc startup on|off".into(),
    );

    m.insert(
        "cmd.qsc.status_running".into(),
        "QSC daemon: running".into(),
    );

    m.insert(
        "cmd.qsc.status_stopped".into(),
        "QSC daemon: stopped".into(),
    );

    m.insert("cmd.qsc.stopped".into(), "QSC daemon stopped.".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC Global Hotkey:".into());
}
