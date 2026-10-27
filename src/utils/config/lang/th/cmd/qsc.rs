use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qsc.already_running".into(),
        "เดม่อน QSC ทำงานอยู่แล้ว. ใช้ 'qsc stop' ก่อน.".into(),
    );

    m.insert("cmd.qsc.current_setting".into(), "  ปัจจุบัน: {}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert(
        "cmd.qsc.daemon_unknown_err".into(),
        "เดม่อน QSC: ไม่รู้จัก ({})".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex1".into(),
        "    config qsc \\\"#c 0\\\"        ตั้งปุ่มลัดเป็น Ctrl+0".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex2".into(),
        "    config qsc \\\"#c #s a\\\"     ตั้งปุ่มลัดเป็น Ctrl+Shift+A".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex3".into(),
        "    config qsc \\\"#c #s #a F1\\\"  ตั้งปุ่มลัดเป็น Ctrl+Shift+Alt+F1".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_off".into(),
        "    config qsc off            ปิดปุ่มลัด QSC".into(),
    );

    m.insert("cmd.qsc.help_configure".into(), "  ขั้นแรกให้ตั้งค่าปุ่มลัด:".into());

    m.insert(
        "cmd.qsc.help_listen".into(),
        "  qsc listen              เริ่มเดม่อนปุ่มลัด QSC".into(),
    );

    m.insert(
        "cmd.qsc.help_status".into(),
        "  qsc status              ตรวจสอบว่าเดม่อน QSC ทำงานหรือไม่".into(),
    );

    m.insert(
        "cmd.qsc.help_stop".into(),
        "  qsc stop                หยุดเดม่อนปุ่มลัด QSC".into(),
    );

    m.insert("cmd.qsc.hotkey_label".into(), "ปุ่มลัด: {}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert(
        "cmd.qsc.not_configured".into(),
        "ยังไม่ได้ตั้งค่าปุ่มลัด QSC. ใช้ 'config qsc \"#c 0\"' ก่อน.".into(),
    );

    m.insert("cmd.qsc.not_running".into(), "เดม่อน QSC ไม่ทำงาน.".into());

    m.insert("cmd.qsc.started".into(), "เริ่มเดม่อน QSC แล้ว.".into());

    m.insert(
        "cmd.qsc.startup_registered".into(),
        "ลงทะเบียนเดม่อน QSC เป็นโปรแกรมเริ่มต้นแล้ว.".into(),
    );

    m.insert(
        "cmd.qsc.startup_removed".into(),
        "ลบเดม่อน QSC ออกจากโปรแกรมเริ่มต้นแล้ว.".into(),
    );

    m.insert(
        "cmd.qsc.startup_usage".into(),
        "การใช้งาน: qsc startup on|off".into(),
    );

    m.insert(
        "cmd.qsc.status_running".into(),
        "เดม่อน QSC: กำลังทำงาน".into(),
    );

    m.insert("cmd.qsc.status_stopped".into(), "เดม่อน QSC: หยุดแล้ว".into());

    m.insert("cmd.qsc.stopped".into(), "หยุดเดม่อน QSC แล้ว.".into());

    m.insert(
        "cmd.qsc.usage_title".into(),
        "ปุ่มลัด QSC (Global Hotkey):".into(),
    );
}
