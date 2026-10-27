use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.nav.back.invalid_number".into(),
        "ເລກບໍ່ຖືກຕ້ອງ: {}. ວິທີໃຊ້: back [n]".into(),
    );

    m.insert(
        "cmd.nav.back.invalid_step".into(),
        "Null parent ທີ່ຂັ້ນຕອນ {} - ບໍ່ມີບ່ອນໃຫ້ back".into(),
    );

    m.insert("cmd.nav.cd.example1".into(), "ຕົວຢ່າງ: cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), " cd subdir".into());

    m.insert(
        "cmd.nav.cd.example3".into(),
        " cd to work # Teleport ໄປທີ່ savepoint 'work'".into(),
    );

    m.insert("cmd.nav.cd.navigated".into(), "ນຳທາງໄປທີ່: {}".into());

    m.insert(
        "cmd.nav.cd.tp_list".into(),
        "ໃຊ້ 'tp list' ເພື່ອເບິ່ງ savepoints ທັງໝົດ.".into(),
    );

    m.insert(
        "cmd.nav.cd.tp_not_found".into(),
        "ບໍ່ພົບ teleport point: '{}'".into(),
    );

    m.insert(
        "cmd.nav.cd.usage1".into(),
        "ວິທີໃຊ້: cd <directory_path>".into(),
    );

    m.insert(
        "cmd.nav.cd.usage2".into(),
        " cd to <tp_name> Teleport ໄປທີ່ savepoint".into(),
    );

    m.insert(
        "cmd.nav.drive_windows_only".into(),
        "ການນຳທາງ drive ຮອງຮັບພຽງ Windows ເທົ່ານັ້ນ.".into(),
    );

    m.insert("cmd.nav.fgo.cancel".into(), "0. ຍົກເລີກ".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "ຍົກເລີກ.".into());

    m.insert(
        "cmd.nav.fgo.example1".into(),
        " fgo main.rs # ຄົ້ນຫາ ແລະ ເລືອກໄຟລ໌ເພື່ອນຳທາງໄປທີ່ parent ຂອງມັນ".into(),
    );

    m.insert(
        "cmd.nav.fgo.example2".into(),
        " fgo main.rs -d 5 # ຄົ້ນຫາລົງເລິກສູງສຸດ 5 ລະດັບ".into(),
    );

    m.insert(
        "cmd.nav.fgo.found".into(),
        "\u{1f50d} ພົບ {} ໄຟລ໌ສຳລັບ \"{}\":".into(),
    );

    m.insert(
        "cmd.nav.fgo.select".into(),
        "ເລືອກໄຟລ໌ເພື່ອນຳທາງໄປຍັງໄດເຣັກທໍຣີ parent: ".into(),
    );

    m.insert(
        "cmd.nav.fgo_usage".into(),
        "ວິທີໃຊ້: fgo <pattern> [-d <depth>]".into(),
    );

    m.insert("cmd.nav.gos.empty".into(), "(ບໍ່ມີ subdirectories)".into());

    m.insert("cmd.nav.gos.exit".into(), "exit".into());

    m.insert("cmd.nav.gos.staying".into(), "ຢູ່ບ່ອນເດີມ.".into());

    m.insert("cmd.nav.invalid_drive".into(), "ເລກ drive ບໍ່ຖືກຕ້ອງ.".into());

    m.insert("cmd.nav.invalid_input".into(), "ຂໍ້ມູນທີ່ໃສ່ບໍ່ຖືກຕ້ອງ.".into());

    m.insert("cmd.nav.invalid_number".into(), "ເລກບໍ່ຖືກຕ້ອງ.".into());

    m.insert("cmd.nav.invalid_selection".into(), "ການເລືອກບໍ່ຖືກຕ້ອງ.".into());

    m.insert(
        "cmd.nav.where.config_file".into(),
        "\u{2699}\u{fe0f} ໄຟລ໌ Config:".into(),
    );

    m.insert(
        "cmd.nav.where.config_missing".into(),
        "\u{26a0} ບໍ່ພົບໄຟລ໌ config (ຈະຖືກສ້າງເມື່ອບັນທຶກຄັ້ງທຳອິດ)".into(),
    );

    m.insert(
        "cmd.nav.where.config_size".into(),
        "\u{1f4cf} ຂະໜາດ config:".into(),
    );

    m.insert(
        "cmd.nav.where.current_dir".into(),
        "\u{1f4c2} ໄດເຣັກທໍຣີປັດຈຸບັນ:".into(),
    );

    m.insert(
        "cmd.nav.where.executable".into(),
        "\u{1f4c1} Executable:".into(),
    );

    m.insert(
        "cmd.nav.where.not_found".into(),
        "\u{2717} ບໍ່ພົບ '{}' ໃນ PATH".into(),
    );

    m.insert(
        "cmd.nav.where.path_title".into(),
        "\u{1f50d} ຕຳແໜ່ງຂອງ '{}' ໃນ PATH:".into(),
    );

    m.insert("cmd.nav.where.title".into(), "ຂໍ້ມູນບ່ອນຕັ້ງ ntc".into());
}
