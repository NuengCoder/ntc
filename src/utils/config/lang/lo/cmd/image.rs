use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "ຍົກເລີກ.".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. ຍົກເລີກ".into());

    m.insert("cmd.image.enter_number".into(), "ໃສ່ເລກ: ".into());

    m.insert(
        "cmd.image.no_ansi".into(),
        "terminal ນີ້ບໍ່ຮອງຮັບສີ ANSI ຈຶ່ງສະແດງ '{}' ໃນ terminal ບໍ່ໄດ້".into(),
    );

    m.insert(
        "cmd.image.no_files".into(),
        "ບໍ່ພົບໄຟລ໌ຮູບພາບໃນໄດເຣັກທໍຣີປັດຈຸບັນ.".into(),
    );

    m.insert("cmd.image.select_image".into(), "ເລືອກຮູບພາບ:".into());

    m.insert(
        "cmd.image.displaying".into(),
        "ກຳລັງສະແດງ {} ({}x{}) ຜ່ານ {}".into(),
    );

    m.insert(
        "cmd.image.usage".into(),
        "ວິທີໃຊ້: image [display|show] [--sixel] [<path>]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "ກຳລັງບັງຄັບ sixel output (ກວດພົບ: {}) — terminal ນີ້ອາດສະແດງບໍ່ຖືກຕ້ອງ".into(),
    );
}
