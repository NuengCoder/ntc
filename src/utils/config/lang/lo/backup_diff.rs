use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup_diff.cancel_opt".into(), "0. ຍົກເລີກ".into());

    m.insert("backup_diff.cancelled".into(), "ຍົກເລີກການປຽບທຽບ.".into());

    m.insert(
        "backup_diff.comparing".into(),
        "ກຳລັງປຽບທຽບໂຄງການປັດຈຸບັນກັບ backup #{}...".into(),
    );

    m.insert(
        "backup_diff.invalid_input".into(),
        "ຂໍ້ມູນທີ່ໃສ່ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "backup_diff.invalid_selection".into(),
        "ການເລືອກບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "backup_diff.none".into(),
        "ບໍ່ພົບ backup ຂອງໂຄງການນີ້. ໃຊ້ 'bkup' ເພື່ອສ້າງ.".into(),
    );

    m.insert("backup_diff.not_found".into(), "ບໍ່ພົບ Backup #{}.".into());

    m.insert("backup_diff.open_manually".into(), "ເປີດໄຟລ໌ດ້ວຍມື: {}".into());

    m.insert(
        "backup_diff.saved_to".into(),
        "ບັນທຶກບົດລາຍງານ diff ໄວ້ທີ່: {}".into(),
    );

    m.insert(
        "backup_diff.select_prompt".into(),
        "ເລືອກ backup (1-{}): ".into(),
    );

    m.insert(
        "backup_diff.select_title".into(),
        "\u{1F4CA} ເລືອກ backup ເພື່ອປຽບທຽບກັບໂຄງການປັດຈຸບັນ".into(),
    );
}
