use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "tutorial.complete_msg".into(),
        "ຕອນນີ້ທ່ານຮູ້ສິ່ງຈຳເປັນຂອງ NTC ແລ້ວ:".into(),
    );

    m.insert(
        "tutorial.complete_title".into(),
        " Tutorial Complete! ".into(),
    );

    m.insert("tutorial.demonstrating".into(), "ກຳລັງສາທິດ:".into());

    m.insert(
        "tutorial.exit_hint".into(),
        "ພິມ 'exit' ເພື່ອອອກຈາກ NTC.".into(),
    );

    m.insert(
        "tutorial.help_hint".into(),
        "ພິມ 'help' ສຳລັບລາຍອ້າງອີງຄຳສັ່ງທັງໝົດ.".into(),
    );

    m.insert("tutorial.prompt_continue".into(), "ກົດ Enter ເພື່ອສືບຕໍ່".into());

    m.insert("tutorial.prompt_exit_label".into(), "exit".into());

    m.insert("tutorial.prompt_skip_label".into(), "skip".into());

    m.insert(
        "tutorial.return_hint".into(),
        "ກົດ Enter ເພື່ອກັບຄືນ shell".into(),
    );

    m.insert(
        "tutorial.step_title".into(),
        " NTC Tutorial \u{2014} ຂັ້ນຕອນ {}/{} ".into(),
    );

    m.insert("tutorial.tip_label".into(), "\u{1F4A1} Tip:".into());
}
