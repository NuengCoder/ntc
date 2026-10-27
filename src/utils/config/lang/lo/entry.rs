use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("entry.goodbye".into(), "ລາກ່ອນ!".into());

    m.insert(
        "entry.ntpkg_active".into(),
        "ntpkg: {} runtime{} active ໃນ PATH".into(),
    );

    m.insert(
        "entry.startup_hint".into(),
        "ພິມ 'help' ສຳລັບຄຳສັ່ງທີ່ມີ, 'exit' ເພື່ອອອກ.".into(),
    );

    m.insert("entry.watch_trigger".into(), "watch trigger: {}".into());

    m.insert("entry.watcher_failed".into(), "Watcher ລົ້ມເຫຼວ: {}".into());

    m.insert(
        "entry.watcher_started".into(),
        "ເລີ່ມ file watcher ແລ້ວ (recursive)".into(),
    );

    m.insert(
        "entry.welcome_fmt".into(),
        " ຍິນດີຕ້ອນຮັບສູ່ ntc {} - Navigate, Toolkit, Center ".into(),
    );
}
