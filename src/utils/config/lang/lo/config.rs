use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "config.general.autosuggest".into(),
        " config autosuggest ON|OFF ສະແດງ/ຕັ້ງ autosuggest".into(),
    );

    m.insert(
        "config.general.color".into(),
        " config color ON|OFF ສະແດງ/ຕັ້ງສີ".into(),
    );

    m.insert(
        "config.general.depth".into(),
        " config depth [n] ສະແດງ/ຕັ້ງຄວາມເລິກສູງສຸດ (1-20)".into(),
    );

    m.insert("config.general.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "config.general.history".into(),
        " config history [on|off|path] ສະແດງ/ຕັ້ງ history".into(),
    );

    m.insert(
        "config.general.lang".into(),
        " config lang [language] ສະແດງ/ຕັ້ງພາສາອິນເຕີເຟສ".into(),
    );

    m.insert(
        "config.general.linenumbers".into(),
        " config linenumbers ON|OFF ສະແດງ/ຕັ້ງເບີແຖວ".into(),
    );

    m.insert(
        "config.general.output".into(),
        " config output [path] ສະແດງ/ຕັ້ງ path ຜົນລັບ".into(),
    );

    m.insert(
        "config.general.qsc".into(),
        " config qsc \"#c 0\" ສະແດງ/ຕັ້ງ QSC global hotkey".into(),
    );

    m.insert("config.general.settings".into(), "ການຕັ້ງຄ່າ:".into());

    m.insert(
        "config.general.threads".into(),
        " config threads [n] ສະແດງ/ຕັ້ງຈຳນວນ threads".into(),
    );

    m.insert(
        "config.general.title".into(),
        "config \u{2014} ເບິ່ງ ຫຼື ປ່ຽນການຕັ້ງຄ່າ".into(),
    );

    m.insert(
        "config.general.watch".into(),
        " config watch ON|OFF ສະແດງ/ຕັ້ງ file watcher".into(),
    );

    m.insert("config.lang.set".into(), "ຕັ້ງພາສາເປັນ: {}".into());
}
