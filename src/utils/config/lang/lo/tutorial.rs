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

    m.insert("tutorial.tip_label".into(), "Tip:".into());

    m.insert("tutorial.command_label".into(), "ຄຳສັ່ງ:".into());

    m.insert(
        "tutorial.command_usage".into(),
        "ພິມເອງແລ້ວກົດ Enter \u{2014} ntc ຈະບໍ່ຣັນໃຫ້ທ່ານ".into(),
    );

    m.insert(
        "tutorial.already_done".into(),
        "ບົດຮຽນນີ້ສຳເລັດແລ້ວ. ໃຊ້ `tutorial` ເພື່ອເບິ່ງເມນູ, ຫຼື `tutorial restart` ເພື່ອເຮັດຊ້ຳ.".into(),
    );
    m.insert("tutorial.also_try_fmt".into(), "ລອງອີກ: {}".into());
    m.insert(
        "tutorial.blocked_cmd".into(),
        "`{}` ຖືກບລັອກໃນໂໝດບົດຮຽນ. ໃຊ້ຄຳສັ່ງຂອງບົດຮຽນ ຫຼື `tutorial`.".into(),
    );
    m.insert(
        "tutorial.blocked_outside".into(),
        "ທ່ານຢູ່ນອກໂຟນເດີບົດຮຽນ. ໃຊ້ `cd {}` ເພື່ອກັບ, ຫຼື `tutorial exit` ເພື່ອອອກຈາກໂໝດບົດຮຽນ.".into(),
    );
    m.insert("tutorial.goal_label".into(), "ເປົ້າໝາຍ:".into());
    m.insert("tutorial.hint_label".into(), "ຄຳແນະນຳ:".into());
    m.insert(
        "tutorial.lesson_done".into(),
        "ຈົບບົດຮຽນ: {} — ເລືອກບົດຕໍ່ໄປດ້ວຍ `tutorial`.".into(),
    );
    m.insert(
        "tutorial.menu_goal".into(),
        "ພິມ `tutorial <lesson>` ໜຶ່ງຄັ້ງ, ແລ້ວເຮັດເປົ້າໝາຍດ້ວຍຕົນເອງ.".into(),
    );
    m.insert(
        "tutorial.menu_hint".into(),
        "ຕິດຂັດ: tutorial hint · ຂ້າມ: tutorial skip · ສະຖານະ: tutorial status · ອອກ: tutorial exit"
            .into(),
    );
    m.insert("tutorial.menu_title".into(), "ບົດຮຽນ NTC — ເລືອກບົດຮຽນ".into());
    m.insert("tutorial.mode_off".into(), "ປິດໂໝດບົດຮຽນແລ້ວ. ກັບໄປ {}.".into());
    m.insert(
        "tutorial.mode_on".into(),
        "ໂໝດບົດຮຽນເປີດຢູ່ — ທ່ານພິມຄຳສັ່ງເອງ; ntc ພຽງກວດເປົ້າໝາຍ.".into(),
    );
    m.insert(
        "tutorial.not_active".into(),
        "ໂໝດບົດຮຽນບໍ່ໄດ້ເປີດ. ຣັນ `tutorial` ເພື່ອເລີ່ມ.".into(),
    );
    m.insert(
        "tutorial.not_in_lesson".into(),
        "ຍັງບໍ່ໄດ້ເລືອກບົດຮຽນ. ເລືອກດ້ວຍ `tutorial <lesson>`.".into(),
    );
    m.insert("tutorial.prompt_fmt".into(), "tutorial [{} {}/{}] ".into());
    m.insert("tutorial.prompt_none".into(), "tutorial ".into());
    m.insert("tutorial.skipped".into(), "ຂ້າມແລ້ວ.".into());
    m.insert(
        "tutorial.status_fmt".into(),
        "ບົດຮຽນ {} · ຂັ້ນຕອນ {}/{}".into(),
    );
    m.insert("tutorial.step_done".into(), "ຂັ້ນຕອນສຳເລັດ.".into());
    m.insert("tutorial.step_label".into(), "ຂັ້ນຕອນ {}/{}".into());
    m.insert(
        "tutorial.unknown_lesson".into(),
        "ບໍ່ຮູ້ຈັກບົດຮຽນ '{}'. ບົດທີ່ມີ: {}".into(),
    );
    m.insert("tutorial.usage".into(), "ວິທີໃຊ້:\n  tutorial                 ເລີ່ມ sandbox ບົດຮຽນ ແລະ ສະແດງເມນູບົດຮຽນ\n  tutorial <lesson>        ເລີ່ມບົດຮຽນ (ຄີ ຫຼື ໝາຍເລກ)\n  tutorial list            ສະແດງເມນູບົດຮຽນ\n  tutorial hint            ຄຳແນະນຳໜຶ່ງຢ່າງສຳລັບຂັ້ນຕອນປັດຈຸບັນ\n  tutorial skip            ຂ້າມຂັ້ນຕອນປັດຈຸບັນ\n  tutorial restart         ເລີ່ມບົດຮຽນປັດຈຸບັນໃໝ່\n  tutorial status          ບົດຮຽນ ແລະ ຂັ້ນຕອນປັດຈຸບັນ\n  tutorial exit            ອອກຈາກໂໝດບົດຮຽນ\n\nບົດຮຽນ: ຣັນ `tutorial list` ເພື່ອເບິ່ງເມນູທັງໝົດ (ທຸກຄຳສັ່ງ ntc ມີບົດຮຽນ).\nໃນໂໝດບົດຮຽນຈະຣັນແຕ່ຄຳສັ່ງໃນຫ້ອງທົດລອງ; ການອອກຈາກໂຟນເດີບົດຮຽນ\nຈະບລັອກທຸກຢ່າງຍົກເວັ້ນ cd, tutorial, help ແລະ exit.".into());
    m.insert("tutorial.ws_ready".into(), "ພື້ນທີ່ເຮັດວຽກບົດຮຽນ: {}".into());
}
