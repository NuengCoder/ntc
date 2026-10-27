use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "gosc.calculating_size".into(),
        "ກຳລັງຄິດໄລ່ຂະໜາດໄດເຣັກທໍຣີ...".into(),
    );

    m.insert("gosc.exiting".into(), "ກຳລັງອອກຈາກ gosc...".into());

    m.insert(
        "gosc.output_zone".into(),
        "─── Output Zone ───────────────────────────────────".into(),
    );

    m.insert(
        "gosc.output_zone_end".into(),
        "──────────────────────────────────────────────────".into(),
    );

    m.insert("gosc.panel.back_1".into(), "go back 1 level".into());

    m.insert("gosc.panel.back_2".into(), "go back 2 levels".into());

    m.insert("gosc.panel.back_n".into(), "go back n levels".into());

    m.insert("gosc.panel.empty".into(), "(ບໍ່ມີ subdirectories)".into());

    m.insert("gosc.panel.exit".into(), "exit gosc".into());

    m.insert(
        "gosc.panel.size_care".into(),
        "size --care (total size)".into(),
    );

    m.insert("gosc.panel.title".into(), "gosc — ນຳທາງຢ່າງຕໍ່ເນື່ອງ".into());

    m.insert(
        "gosc.panel.view_care".into(),
        "view --care -d 1 (files + care)".into(),
    );

    m.insert(
        "gosc.panel.view_size_care".into(),
        "view -s --care -d 1 (files + sizes + care)".into(),
    );

    m.insert("gosc.press_enter".into(), "ກົດ Enter ເພື່ອສືບຕໍ່...".into());

    m.insert("gosc.prompt".into(), "gosc>".into());

    m.insert("gosc.size_bytes".into(), "Bytes:".into());

    m.insert("gosc.size_human".into(), "Human:".into());

    m.insert("gosc.size_title".into(), "ຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into());
}
