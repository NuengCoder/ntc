use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "ຍົກເລີກ.".into());

    m.insert("cmd.image.default_label".into(), "ເລີ່ມຕົ້ນ".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. ຍົກເລີກ".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert(
        "cmd.image.desc_markdown".into(),
        "  {}. Markdown (ເລີ່ມຕົ້ນ)".into(),
    );

    m.insert(
        "cmd.image.desc_narrative".into(),
        "  {}. ຄຳອະທິບາຍເລື່ອງລາວ (ສຳລັບ AI)".into(),
    );

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "ໃສ່ເລກ: ".into());

    m.insert(
        "cmd.image.missing_out_format".into(),
        "ຂາດຮູບແບບຫຼັງ 'out'.".into(),
    );

    m.insert(
        "cmd.image.missing_to_format".into(),
        "ຂາດຮູບແບບຫຼັງ 'to'.".into(),
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
        "ວິທີໃຊ້: image <path> [display|show [--sixel]|to|out <format>] [--ocr] [--layout]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "ກຳລັງບັງຄັບ sixel output (ກວດພົບ: {}) — terminal ນີ້ອາດສະແດງບໍ່ຖືກຕ້ອງ".into(),
    );
}
