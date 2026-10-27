use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.qsc.already_running".into(),
        "QSC daemon ເຮັດວຽກຢູ່ແລ້ວ. ໃຊ້ 'qsc stop' ກ່ອນ.".into(),
    );

    m.insert("cmd.qsc.current_setting".into(), "  ປັດຈຸບັນ: {}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert(
        "cmd.qsc.daemon_unknown_err".into(),
        "QSC daemon: ບໍ່ຮູ້ຈັກ ({})".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex1".into(),
        "    config qsc \"#c 0\"        ຕັ້ງ hotkey ເປັນ Ctrl+0".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex2".into(),
        "    config qsc \"#c #s a\"     ຕັ້ງ hotkey ເປັນ Ctrl+Shift+A".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_ex3".into(),
        "    config qsc \"#c #s #a F1\"  ຕັ້ງ hotkey ເປັນ Ctrl+Shift+Alt+F1".into(),
    );

    m.insert(
        "cmd.qsc.help_conf_off".into(),
        "    config qsc off            ປິດ QSC hotkey".into(),
    );

    m.insert(
        "cmd.qsc.help_configure".into(),
        "  ກ່ອນອື່ນຕ້ອງຕັ້ງຄ່າ hotkey ກ່ອນ:".into(),
    );

    m.insert(
        "cmd.qsc.help_listen".into(),
        "  qsc listen              ເລີ່ມ QSC global hotkey daemon".into(),
    );

    m.insert(
        "cmd.qsc.help_status".into(),
        "  qsc status              ກວດວ່າ QSC daemon ເຮັດວຽກຢູ່ບໍ".into(),
    );

    m.insert(
        "cmd.qsc.help_stop".into(),
        "  qsc stop                ຢຸດ QSC global hotkey daemon".into(),
    );

    m.insert("cmd.qsc.hotkey_label".into(), "  Hotkey: {}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert(
        "cmd.qsc.not_configured".into(),
        "ຍັງບໍ່ໄດ້ຕັ້ງຄ່າ QSC hotkey. ໃຊ້ 'config qsc \"#c 0\"' ກ່ອນ.".into(),
    );

    m.insert("cmd.qsc.not_running".into(), "QSC daemon ບໍ່ເຮັດວຽກ.".into());

    m.insert("cmd.qsc.started".into(), "ເລີ່ມ QSC daemon ແລ້ວ.".into());

    m.insert(
        "cmd.qsc.startup_registered".into(),
        "ລົງທະບຽນ QSC daemon ເປັນ startup app ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.qsc.startup_removed".into(),
        "ລຶບ QSC daemon ອອກຈາກ startup apps ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.qsc.startup_usage".into(),
        "ວິທີໃຊ້: qsc startup on|off".into(),
    );

    m.insert("cmd.qsc.status_running".into(), "QSC daemon: ເຮັດວຽກ".into());

    m.insert("cmd.qsc.status_stopped".into(), "QSC daemon: ຢຸດ".into());

    m.insert("cmd.qsc.stopped".into(), "ຢຸດ QSC daemon ແລ້ວ.".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC Global Hotkey:".into());
}
