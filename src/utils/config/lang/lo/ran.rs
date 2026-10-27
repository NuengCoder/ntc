use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ran.already_exists".into(), "{} ມີຢູ່ແລ້ວໃນ {}".into());

    m.insert(
        "ran.calling_not_allowed".into(),
        "ການເອີ້ນ 'ran' ຈາກພາຍໃນ NTCRANFILE target ບໍ່ອະນຸຍາດ".into(),
    );

    m.insert("ran.completed".into(), "ຄຳສັ່ງສຳເລັດຢ່າງຖືກຕ້ອງ.".into());

    m.insert("ran.created".into(), "ສ້າງ {} ແລ້ວ".into());

    m.insert(
        "ran.desc_aliases".into(),
        " ຄຳສັ່ງຖືກຂະຫຍາຍຜ່ານລະບົບ run-alias ຂອງ ntc, ດັ່ງນັ້ນ aliases ກໍເຮັດວຽກ".into(),
    );

    m.insert(
        "ran.desc_aliases2".into(),
        " ຕົ້ນດ້ວຍ 'run ' ເພື່ອຂ້າມການຂະຫຍາຍ alias ແລະ ເຮັດວຽກໂດຍກົງເປັນຄຳສັ່ງລະບົບ.".into(),
    );

    m.insert(
        "ran.desc_deps".into(),
        " Targets ສາມາດຂຶ້ນກັບ targets ອື່ນ. Dependencies ເຮັດວຽກກ່ອນ, ຕາມລຳດັບ.".into(),
    );

    m.insert(
        "ran.desc_note".into(),
        " ການເອີ້ນ 'ran' ຈາກພາຍໃນຄຳສັ່ງ NTCRANFILE target ບໍ່ອະນຸຍາດ.".into(),
    );

    m.insert(
        "ran.desc_reserved".into(),
        " init, deinit, help, list, ls \u{2014} ສິ່ງເຫຼົ່ານີ້ບໍ່ສາມາດໃຊ້ເປັນຊື່ target.".into(),
    );

    m.insert(
        "ran.desc_vars".into(),
        " ກຳນົດຕົວແປພາຍໃຕ້ [vars] ແລະ ອ້າງອີງດ້ວຍ $(VAR_NAME) ໃນຄຳສັ່ງ.".into(),
    );

    m.insert(
        "ran.desc_vars2".into(),
        " ພວກມັນເຮັດວຽກຄືຕົວແປ Make ແຕ່ໃຊ້ syntax $(...).".into(),
    );

    m.insert("ran.error_prefix".into(), "ran: {}".into());

    m.insert("ran.exec_failed".into(), "ເຮັດວຽກຄຳສັ່ງບໍ່ສຳເລັດ: {}".into());

    m.insert("ran.executing".into(), "ກຳລັງເຮັດວຽກ: {}".into());

    m.insert("ran.exit_code".into(), "ຄຳສັ່ງຈົບດ້ວຍ code: {}".into());

    m.insert(
        "ran.get_cwd_failed".into(),
        "ດຶງໄດເຣັກທໍຣີປັດຈຸບັນບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "ran.help_deinit_fmt".into(),
        " ran deinit ລຶບ {} ຈາກໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert("ran.help_help".into(), " ran help ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert(
        "ran.help_init_fmt".into(),
        " ran init ສ້າງ {} ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "ran.help_list".into(),
        " ran list ລາຍຊື່ targets ທີ່ມີທັງໝົດ".into(),
    );

    m.insert(
        "ran.help_standalone".into(),
        " ran ເຮັດວຽກ target 'standalone' (default)".into(),
    );

    m.insert(
        "ran.help_target".into(),
        " ran <target> ເຮັດວຽກ target ສະເພາະ".into(),
    );

    m.insert(
        "ran.help_title".into(),
        "ran \u{2014} ntc task runner (ຄື Make ແຕ່ ntc-native)".into(),
    );

    m.insert("ran.help_usage".into(), "Usage:".into());

    m.insert("ran.not_found".into(), "ບໍ່ພົບ {} ໃນ {}".into());

    m.insert("ran.remove_failed".into(), "ລຶບ {} ບໍ່ສຳເລັດ: {}".into());

    m.insert("ran.removed".into(), "ລຶບ {} ແລ້ວ".into());

    m.insert("ran.section_aliases".into(), "Run aliases:".into());

    m.insert("ran.section_deps".into(), "Dependencies:".into());

    m.insert("ran.section_note".into(), "Note:".into());

    m.insert("ran.section_reserved".into(), "ຊື່ target ທີ່ສະຫງວນ:".into());

    m.insert("ran.section_vars".into(), "Variables:".into());

    m.insert("ran.terminated".into(), "ຄຳສັ່ງຖືກຢຸດ (Ctrl+C)".into());
}
