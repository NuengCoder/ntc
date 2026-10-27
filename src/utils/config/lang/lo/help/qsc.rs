use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.qsc.listen".into(),
        "qsc listen ## ເລີ່ມ QSC daemon (background process)".into(),
    );

    m.insert(
        "help.qsc.off".into(),
        "config qsc off ## ປິດ QSC hotkey".into(),
    );

    m.insert(
        "help.qsc.set".into(),
        "config qsc \"#c 0\" ## ຕັ້ງ hotkey (Ctrl+0)".into(),
    );

    m.insert(
        "help.qsc.set_cs".into(),
        "config qsc \"#c #s a\" ## ຕັ້ງ hotkey (Ctrl+Shift+A)".into(),
    );

    m.insert(
        "help.qsc.startup".into(),
        "qsc startup on|off ## ລົງທະບຽນ/ຍົກເລີກເປັນ startup app".into(),
    );

    m.insert(
        "help.qsc.status".into(),
        "qsc status ## ກວດວ່າ QSC daemon ເຮັດວຽກຢູ່ຫຼືບໍ່".into(),
    );

    m.insert("help.qsc.stop".into(), "qsc stop ## ຢຸດ QSC daemon".into());

    m.insert("help.qsc.title".into(), "ຄຳສັ່ງ QSC (GLOBAL HOTKEY):".into());

    m.insert(
        "help.qsc.example1".into(),
        "config qsc \"#c 0\" ## ຕັ້ງ hotkey Ctrl+0".into(),
    );

    m.insert(
        "help.qsc.example2".into(),
        "qsc listen ## ເລີ່ມ QSC daemon".into(),
    );

    m.insert(
        "help.qsc.example3".into(),
        "qsc status ## ກວດສະຖານະ daemon".into(),
    );
}
