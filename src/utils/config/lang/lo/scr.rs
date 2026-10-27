use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "scr.calling_not_allowed".into(),
        "ບໍ່ອະນຸຍາດໃຫ້ເອີ້ນ 'scr' ພາຍໃນສະຄຣິບ".into(),
    );

    m.insert("scr.cmd_failed".into(), "ຄຳສັ່ງລົ້ມເຫຼວ".into());

    m.insert("scr.completed".into(), "ສະຄຣິບ '{}' ສຳເລັດ ({} ຄຳສັ່ງ)".into());

    m.insert("scr.continue_warn".into(), "ສືບຕໍ່ຫຼັງຂໍ້ຜິດພາດທີ່ບັນທັດ {}".into());

    m.insert("scr.desc_args".into(), " $1..$9, $@ ອາກິວເມັນຕຳແໜ່ງ".into());

    m.insert(
        "scr.desc_force".into(),
        " @ <cmd> ບັງຄັບຄຳສັ່ງລະບົບ (ຂ້າມຄຳສັ່ງ ntc)".into(),
    );

    m.insert(
        "scr.desc_interp".into(),
        " $var / ${var} / ${var:-default} ການແທນຄ່າຕົວແປ".into(),
    );

    m.insert("scr.desc_set".into(), " set NAME = value ກຳນົດຕົວແປ".into());

    m.insert(
        "scr.done_errors".into(),
        "ສະຄຣິບ '{}' ສຳເລັດ ມີ {} ຂໍ້ຜິດພາດ".into(),
    );

    m.insert("scr.error_at".into(), "ຂໍ້ຜິດພາດທີ່ {}:{} \u{2014} {}".into());

    m.insert("scr.executing".into(), "[{}] ກຳລັງແລ່ນ: {}".into());

    m.insert(
        "scr.failed_abort".into(),
        "ສະຄຣິບ '{}' ຢຸດທີ່ບັນທັດ {} ({} ຂໍ້ຜິດພາດ)".into(),
    );

    m.insert(
        "scr.help_continue".into(),
        " scr -c <file.ntc> [args...] ສືບຕໍ່ຫຼັງຂໍ້ຜິດພາດ".into(),
    );

    m.insert("scr.help_help".into(), " scr help ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert(
        "scr.help_init".into(),
        " scr init [name] ສ້າງ template ສະຄຣິບ".into(),
    );

    m.insert(
        "scr.help_ls".into(),
        " scr ls ລາຍຊື່ *.ntc ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "scr.help_run".into(),
        " scr <file.ntc> [args...] ແລ່ນສະຄຣິບອັດຕະໂນມັດ".into(),
    );

    m.insert(
        "scr.help_title".into(),
        "scr \u{2014} ຕົວແລ່ນສະຄຣິບອັດຕະໂນມັດຂອງ ntc".into(),
    );

    m.insert("scr.help_usage".into(), "ການໃຊ້ງານ:".into());

    m.insert("scr.init_created".into(), "ສ້າງ template {} ແລ້ວ".into());

    m.insert("scr.init_exists".into(), "{} ມີຢູ່ແລ້ວ".into());

    m.insert("scr.ls_empty".into(), "ບໍ່ພົບ *.ntc ໃນ {}".into());

    m.insert("scr.ls_title".into(), "ສະຄຣິບອັດຕະໂນມັດ:".into());

    m.insert("scr.no_commands".into(), "{} ບໍ່ມີຄຳສັ່ງໃຫ້ແລ່ນ".into());

    m.insert("scr.not_found".into(), "ບໍ່ພົບສະຄຣິບ: {} ({})".into());

    m.insert("scr.start".into(), "ກຳລັງແລ່ນສະຄຣິບ {}...".into());

    m.insert("scr.syscmd".into(), "ກຳລັງແລ່ນ: {}".into());

    m.insert("scr.syscmd_exit".into(), "ຄຳສັ່ງອອກດ້ວຍລະຫັດ: {}".into());

    m.insert("scr.syscmd_terminated".into(), "ຄຳສັ່ງສິ້ນສຸດ (Ctrl+C)".into());
}
