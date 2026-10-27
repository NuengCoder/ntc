use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("msg.ignore.dir_added".into(), "ກຳລັງບໍ່ສົນໃຈໄດເຣັກທໍຣີ: {}".into());

    m.insert(
        "msg.ignore.dir_removed".into(),
        "ບໍ່ບໍ່ສົນໃຈໄດເຣັກທໍຣີ: {} ອີກແລ້ວ".into(),
    );

    m.insert("msg.ignore.ext_added".into(), "ກຳລັງບໍ່ສົນໃຈໄຟລ໌ .{}".into());

    m.insert("msg.ignore.ext_removed".into(), "ກຳລັງສົນໃຈໄຟລ໌ .{} ອີກ".into());

    m.insert("msg.ignore.file_added".into(), "ກຳລັງບໍ່ສົນໃຈໄຟລ໌: {}".into());

    m.insert(
        "msg.ignore.file_removed".into(),
        "ກຳລັງສົນໃຈໄຟລ໌: {} ອີກ".into(),
    );

    m.insert("msg.ignored.dirs".into(), "ໄດເຣັກທໍຣີທີ່ບໍ່ສົນໃຈ: {}".into());

    m.insert("msg.ignored.extensions".into(), "ນາມສະກຸນທີ່ບໍ່ສົນໃຈ: {}".into());

    m.insert(
        "msg.ignored.extra_extensions".into(),
        "ນາມສະກຸນທີ່ຮອງຮັບເພີ່ມເຕີມ: {}".into(),
    );

    m.insert(
        "msg.ignored.extra_files".into(),
        "ໄຟລ໌ທີ່ຮອງຮັບເພີ່ມເຕີມ: {}".into(),
    );

    m.insert("msg.ignored.files".into(), "ໄຟລ໌ທີ່ບໍ່ສົນໃຈ: {}".into());

    m.insert(
        "msg.image.converted".into(),
        "ແປງຮູບພາບ ແລະ ບັນທຶກໄປທີ່: {}".into(),
    );

    m.insert("msg.image.not_found".into(), "ບໍ່ພົບຮູບພາບ: {}".into());

    m.insert(
        "msg.init.template_created".into(),
        "ສ້າງ template: {}".into(),
    );

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert("msg.list.title".into(), "ntc {} - ຟັງຊັນຄຳສັ່ງທີ່ມີ:\n".into());

    m.insert(
        "msg.report.binary_clipboard_warning".into(),
        "{} report ບໍ່ສາມາດສຳເນົາໃສ່ clipboard ໄດ້ (binary format)".into(),
    );

    m.insert(
        "msg.report.copied_to_clipboard".into(),
        "{} report ສຳເນົາໃສ່ clipboard ແລ້ວ!".into(),
    );

    m.insert("msg.report.file_saved".into(), "ບັນທຶກໄຟລ໌ໄປທີ່: {}".into());

    m.insert("msg.report.path_not_found".into(), "ບໍ່ພົບ path: {}".into());

    m.insert(
        "msg.report.skipped_unsupported".into(),
        "ຂ້າມ (ບໍ່ຮອງຮັບຮູບແບບ): {}".into(),
    );

    m.insert(
        "msg.setA.error".into(),
        "ຄ່າ setA ບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ ON ຫຼື OFF.".into(),
    );

    m.insert("msg.setA.off".into(), "Autosuggest: OFF".into());

    m.insert("msg.setA.on".into(), "Autosuggest: ON".into());

    m.insert(
        "msg.setA.status".into(),
        "Autosuggest (ghost text): {}".into(),
    );

    m.insert(
        "msg.setC.error".into(),
        "ຄ່າ setC ບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ ON ຫຼື OFF.".into(),
    );

    m.insert("msg.setC.status".into(), "Color output: {}".into());

    m.insert("msg.setC.success".into(), "Color: {}".into());

    m.insert(
        "msg.setD.error_out_of_range".into(),
        "ຄ່າ depth ບໍ່ຖືກຕ້ອງ: {}. ຕ້ອງຢູ່ລະຫວ່າງ 1 ແລະ 20.".into(),
    );

    m.insert(
        "msg.setD.error_parse".into(),
        "ຄ່າ depth ບໍ່ຖືກຕ້ອງ: {}. ຕ້ອງເປັນຈຳນວນບວກລະຫວ່າງ 1 ແລະ 20.".into(),
    );

    m.insert("msg.setD.status".into(), "ຄວາມເລິກສູງສຸດປັດຈຸບັນ: {}".into());

    m.insert("msg.setD.success".into(), "ຕັ້ງຄວາມເລິກສູງສຸດເປັນ: {}".into());

    m.insert("msg.setH.off".into(), "History: OFF".into());

    m.insert("msg.setH.on".into(), "History: ON".into());

    m.insert("msg.setH.path".into(), "History path: {}".into());

    m.insert(
        "msg.setH.path_default".into(),
        "History path: default".into(),
    );

    m.insert(
        "msg.setH.path_reset".into(),
        "History path ຣີເຊັດເປັນ default".into(),
    );

    m.insert("msg.setH.path_set".into(), "ຕັ້ງ History path ເປັນ: {}".into());

    m.insert("msg.setH.status".into(), "History: {}".into());

    m.insert(
        "msg.setL.error".into(),
        "ຄ່າ setL ບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ ON ຫຼື OFF.".into(),
    );

    m.insert("msg.setL.status".into(), "Line numbers: {}".into());

    m.insert("msg.setL.success".into(), "Line numbers: {}".into());

    m.insert("msg.setO.status".into(), "Output path ປັດຈຸບັນ: {}".into());

    m.insert("msg.setO.success".into(), "ຕັ້ງ Output path ເປັນ: {}".into());

    m.insert(
        "msg.setT.error_out_of_range".into(),
        "ຈຳນວນ threads ບໍ່ຖືກຕ້ອງ: {}. ຕ້ອງຢູ່ລະຫວ່າງ 1 ແລະ 64.".into(),
    );

    m.insert(
        "msg.setT.error_parse".into(),
        "ຈຳນວນ threads ບໍ່ຖືກຕ້ອງ: {}. ຕ້ອງເປັນຈຳນວນບວກລະຫວ່າງ 1 ແລະ 64.".into(),
    );

    m.insert("msg.setT.status".into(), "Threads ປັດຈຸບັນ: {}".into());

    m.insert("msg.setT.success".into(), "ຕັ້ງ Threads ເປັນ: {}".into());

    m.insert("msg.showcg.box_bottom".into(), "\u{2514}{}\u{2518}".into());

    m.insert("msg.showcg.box_divider".into(), "\u{251C}{}\u{2524}".into());

    m.insert("msg.showcg.box_top".into(), "\u{250C}{}\u{2510}".into());

    m.insert("msg.showcg.label_color".into(), "Color:".into());

    m.insert("msg.showcg.label_history".into(), "History:".into());

    m.insert(
        "msg.showcg.label_line_numbers".into(),
        "Line Numbers:".into(),
    );

    m.insert("msg.showcg.label_max_depth".into(), "Max Depth:".into());

    m.insert("msg.showcg.label_output_path".into(), "Output Path:".into());

    m.insert("msg.showcg.label_threads".into(), "Threads:".into());

    m.insert("msg.showcg.label_watcher".into(), "Watcher:".into());

    m.insert("msg.showcg.label_watch_limit".into(), "Watch limit:".into());

    m.insert("msg.showcg.label_dir_view".into(), "Dir View:".into());

    m.insert(
        "msg.showcg.label_update_check".into(),
        "Update check:".into(),
    );

    m.insert("msg.showcg.label_serve_files".into(), "Serve files:".into());

    m.insert("msg.showcg.label_serve_http".into(), "Serve http:".into());

    m.insert("msg.showcg.label_serve_chess".into(), "Serve chess:".into());

    m.insert("msg.showcg.title".into(), "ການຕັ້ງຄ່າປັດຈຸບັນ".into());

    m.insert("msg.size.bytes_label".into(), "Bytes:".into());

    m.insert("msg.size.human_label".into(), "Human:".into());

    m.insert("msg.size.path_label".into(), "Path: {}".into());

    m.insert("msg.size.time_label".into(), "Time:".into());

    m.insert("msg.size.title".into(), "ຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into());

    m.insert(
        "msg.teleport.error".into(),
        "Teleport ໄປ '{}' ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "msg.teleport.hint_list".into(),
        "ໃຊ້ 'ntc --tp-list' ເພື່ອເບິ່ງ savepoints ທັງໝົດ.".into(),
    );

    m.insert(
        "msg.teleport.not_found".into(),
        "ບໍ່ພົບຈຸດ teleport: '{}'".into(),
    );

    m.insert(
        "msg.teleport.success".into(),
        "\u{2713} Teleported ໄປ '{}' -> {}".into(),
    );

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert(
        "msg.watch.invalid".into(),
        "ໃຊ້ --watch ON ຫຼື --watch OFF".into(),
    );

    m.insert(
        "msg.watch.off".into(),
        "File watcher: OFF (restart ntc ເພື່ອປິດໃຊ້ງານ)".into(),
    );

    m.insert(
        "msg.watch.on".into(),
        "File watcher: ON (restart ntc ເພື່ອເປີດໃຊ້ງານ)".into(),
    );

    m.insert("msg.watch.status".into(), "File watcher: {}".into());

    m.insert(
        "msg.watch.limit_status".into(),
        "Watch display limit: {}".into(),
    );

    m.insert(
        "msg.watch.limit_success".into(),
        "ຕັ້ງ watch display limit ເປັນ {}".into(),
    );

    m.insert(
        "msg.watch.limit_error".into(),
        "watch limit ບໍ່ຖືກຕ້ອງ: '{}'. ໃຊ້ຕົວເລກບວກ.".into(),
    );

    m.insert("msg.where.current_dir".into(), "ໄດເຣັກທໍຣີປັດຈຸບັນ: {}".into());

    m.insert("msg.where.executable".into(), "ntc executable: {}".into());

    m.insert("msg.showcg.label_file_view".into(), "ມຸມມອງໄຟລ໌:".into());

    m.insert("msg.showcg.label_auto_size".into(), "ຂະດັດອັດຕະພາບ:".into());

    m.insert(
        "msg.showcg.auto_size_value".into(),
        "ໄຟລ໌ {}, ໂຟລ້ອກ {}".into(),
    );

    m.insert("msg.setGit.status".into(), "Git integration: {}".into());

    m.insert("msg.setGit.on".into(), "ເປີດ Git integration ແລ້ວ".into());

    m.insert("msg.setGit.off".into(), "ປິດ Git integration ແລ້ວ".into());

    m.insert(
        "msg.setGit.error".into(),
        "ການໃຊ້ງານ: config git ON|OFF".into(),
    );

    m.insert("msg.setDV.status".into(), "Dir view: {}".into());

    m.insert("msg.setDV.on".into(), "ເປີດ Dir view ແລ້ວ".into());

    m.insert("msg.setDV.off".into(), "ປິດ Dir view ແລ້ວ".into());

    m.insert(
        "msg.setDV.error".into(),
        "ການໃຊ້ງານ: config dir_view ON|OFF".into(),
    );

    m.insert("msg.setFV.status".into(), "File view: {}".into());

    m.insert(
        "msg.setFV.on".into(),
        "ເປີດ File view ແລ້ວ ໄຟລ໌ຈະສະແດງຫຼັງຈາກເປລິນຕະພາບ".into(),
    );

    m.insert("msg.setFV.off".into(), "ປິດ File view ແລ້ວ".into());

    m.insert(
        "msg.setFV.error".into(),
        "ການໃຊ້ງານ: config file_view ON|OFF".into(),
    );

    m.insert(
        "msg.setSZ.status".into(),
        "Auto size: files {}, dirs {}".into(),
    );

    m.insert(
        "msg.setSZ.target_status".into(),
        "Auto size ສຳລັບ {}: {}".into(),
    );

    m.insert("msg.setSZ.on".into(), "ເປີດ Auto size ສໍາລັບ {} ແລ້ວ".into());

    m.insert("msg.setSZ.off".into(), "ປິດ Auto size ສໍາລັບ {} ແລ້ວ".into());

    m.insert(
        "msg.setSZ.error".into(),
        "ການຕັ້ງຄ່າຂະດັດບໍ່ຖືກຕ້ອງ ການໃຊ້ງານ: config size all|file|dir ON|OFF".into(),
    );

    m.insert(
        "msg.setSZ.slow_warn".into(),
        "ຄ່າເຕືອນ: Auto size ໃຊ້ --care; ທຸກຄັ້ງ cd/teleport ອາດຊ້າລົ້ຽງ".into(),
    );

    m.insert(
        "msg.setSZ.usage".into(),
        "ການໃຊ້ງານ: config size all|file|dir ON|OFF".into(),
    );

    m.insert("msg.setUC.status".into(), "Update check: {}".into());

    m.insert("msg.setUC.on".into(), "ເປີດແຈ້ງເຕືອນເວີຊັນໃໝ່ລາຍວັນແລ້ວ".into());

    m.insert("msg.setUC.off".into(), "ປິດແຈ້ງເຕືອນເວີຊັນໃໝ່ແລ້ວ".into());

    m.insert(
        "msg.setUC.error".into(),
        "ການໃຊ້ງານ: config update_check ON|OFF".into(),
    );

    m.insert(
        "msg.update.available".into(),
        "ມີ NTC {} ໃໝ່ (ເຈົ້າໃຊ້ {}) — ລັນ 'ntc --upgrade' ໃນ {} ຫຼືເບິ່ງ {}".into(),
    );
}
