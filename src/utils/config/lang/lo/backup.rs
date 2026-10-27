use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup.and_more".into(), " ... ແລະອີກ {} ອັນ".into());

    m.insert(
        "backup.available_title".into(),
        "  Backup ທີ່ມີຢູ່ (ໃໝ່ສຸດກ່ອນ)".into(),
    );

    m.insert("backup.cancel_opt".into(), " 0. ຍົກເລີກ".into());

    m.insert("backup.cancelled".into(), "ຍົກເລີກການລຶບ.".into());

    m.insert(
        "backup.clear_history_prompt".into(),
        "\u{26A0} ລຶບ undo history? ອັນນີ້ບໍ່ສາມາດກັບຄືນໄດ້. (y/N):".into(),
    );

    m.insert("backup.cleared".into(), "ລຶບ backup ທັງໝົດແລ້ວ.".into());

    m.insert(
        "backup.cleared_count".into(),
        "ລຶບ {} backup ຂອງໂຄງການນີ້.".into(),
    );

    m.insert("backup.confirm".into(), "ພິມ 'yes' ເພື່ອຢືນຢັນ: ".into());

    m.insert(
        "backup.create_hint".into(),
        "ໃຊ້ 'backup create' ເພື່ອສ້າງ backup ທຳອິດ.".into(),
    );

    m.insert(
        "backup.created".into(),
        "ສ້າງ backup ແລ້ວ: #{} \u{2014} {} \u{2014} {} ໄຟລ໌".into(),
    );

    m.insert(
        "backup.created_detail".into(),
        "ສ້າງ Backup #{} ແລ້ວ: {} ({} ໄຟລ໌, {})".into(),
    );

    m.insert("backup.creating".into(), "ກຳລັງສ້າງ backup...".into());

    m.insert(
        "backup.delete_extra_failed".into(),
        "ລຶບໄຟລ໌ພິເສດ {} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "backup.deleted_since".into(),
        "{} ຖືກລຶບໄປນັບແຕ່ສ້າງ backup.".into(),
    );

    m.insert(
        "backup.entry_file_count".into(),
        " Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} ໄຟລ໌".into(),
    );

    m.insert("backup.exists".into(), "Backups ມີຢູ່: {} backup ທີ່ {}".into());

    m.insert(
        "backup.failed_to_backup".into(),
        "backup {} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "backup.file_deleted".into(),
        "{} ຖືກລຶບຫຼັງຈາກສ້າງ backup.".into(),
    );

    m.insert(
        "backup.file_modified".into(),
        "{} ມີການປ່ຽນແປງຫຼັງຈາກບັນທຶກລ່າສຸດ.".into(),
    );

    m.insert(
        "backup.help_clear".into(),
        " backup clear ລຶບ backup ທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "backup.help_clear_force".into(),
        " backup clear --force ລຶບ backup ທັງໝົດ (ບໍ່ຢືນຢັນ)".into(),
    );

    m.insert(
        "backup.help_create".into(),
        " backup create ສ້າງ backup ໃໝ່".into(),
    );

    m.insert(
        "backup.help_diff".into(),
        " backup diff [num] ປຽບທຽບໂຄງການປັດຈຸບັນກັບ backup".into(),
    );

    m.insert("backup.help_examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "backup.help_list".into(),
        " backup list ລາຍຊື່ backup ທັງໝົດ".into(),
    );

    m.insert(
        "backup.help_recreate".into(),
        " backup recreate ລຶບທັງໝົດ + undo history + ສ້າງສົດ".into(),
    );

    m.insert(
        "backup.help_recreate_force".into(),
        " backup recreate --force ຄືກັນ, ຂ້າມການຢືນຢັນທັງໝົດ".into(),
    );

    m.insert(
        "backup.help_restore".into(),
        " backup restore [num] ກູ້ຄືນ backup (interactive ຖ້າບໍ່ມີ num)".into(),
    );

    m.insert(
        "backup.help_restore_choose".into(),
        " backup restore --choose ເລືອກ backup ຈາກເມນູ interactive".into(),
    );

    m.insert(
        "backup.help_restore_choose_deleted".into(),
        " backup restore --choose --deleted ເລືອກ + ກູ້ຄືນສິ່ງທີ່ຖືກລຶບ".into(),
    );

    m.insert(
        "backup.help_restore_deleted".into(),
        " backup restore --deleted ກູ້ຄືນໄຟລ໌ທີ່ຖືກລຶບໄປນັບແຕ່ backup".into(),
    );

    m.insert(
        "backup.help_title".into(),
        "backup \u{2014} ຈັດການ backup ຂອງໂຄງການ".into(),
    );

    m.insert(
        "backup.help_undo".into(),
        " backup undo ຍົກເລີກການຄືນລ່າສຸດ".into(),
    );

    m.insert(
        "backup.help_undo_cls".into(),
        " backup undo --cls ລຶບ undo history".into(),
    );

    m.insert(
        "backup.help_undo_force".into(),
        " backup undo --force ລຶບ undo history (ບໍ່ຢືນຢັນ)".into(),
    );

    m.insert("backup.invalid_arg".into(), "ອາກິວເມນບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "backup.invalid_diff_arg".into(),
        "ອາກິວເມນບໍ່ຖືກຕ້ອງ: {}. ວິທີໃຊ້: backup diff <number>".into(),
    );

    m.insert(
        "backup.invalid_selection".into(),
        "ການເລືອກບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "backup.list_entry".into(),
        " #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert("backup.list_location".into(), "ສະຖານທີ່: {}".into());

    m.insert(
        "backup.list_title".into(),
        "Backups ຂອງໂຄງການນີ້: ພົບ {} ລາຍການ".into(),
    );

    m.insert("backup.menu_item".into(), "Backup #{}".into());

    m.insert(
        "backup.modified_since".into(),
        "{} ມີການປ່ຽນແປງຫຼັງຈາກສ້າງ backup.".into(),
    );

    m.insert(
        "backup.no_deleted_files".into(),
        "ບໍ່ມີໄຟລ໌ທີ່ຖືກລຶບໃຫ້ກູ້ຄືນ \u{2014} ໂຄງການກົງກັບ snapshot ຂອງ backup ແລ້ວ.".into(),
    );

    m.insert("backup.none".into(), "ບໍ່ພົບ backup ຂອງໂຄງການນີ້.".into());

    m.insert("backup.not_found_num".into(), "ບໍ່ພົບ Backup #{}.".into());

    m.insert(
        "backup.pick_detail".into(),
        "{} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert("backup.pick_prompt".into(), "ໃສ່ເລກ (1-{}) ຫຼື 0: ".into());

    m.insert("backup.pick_title".into(), "ເລືອກ backup ເພື່ອກູ້ຄືນ".into());

    m.insert(
        "backup.previous_state_saved".into(),
        "ບັນທຶກສະພາບກ່ອນໜ້າແລ້ວ. ໃຊ້ 'unpd' ເພື່ອຍົກເລີກການກູ້ຄືນນີ້.".into(),
    );

    m.insert("backup.recreate_proceed".into(), "ດຳເນີນຕໍ່ບໍ? (y/N): ".into());

    m.insert(
        "backup.recreate_prompt".into(),
        "ບໍ່ພົບ backup ທີ່ມີຢູ່. ສ້າງ backup ສົດຕໍ່ໄປບໍ? (y/N): ".into(),
    );

    m.insert(
        "backup.recreate_step1".into(),
        " 1. ລຶບ backup ທີ່ມີຢູ່ທັງໝົດ".into(),
    );

    m.insert("backup.recreate_step2".into(), " 2. ລຶບ undo history".into());

    m.insert(
        "backup.recreate_step3".into(),
        " 3. ສ້າງ backup ສົດຂອງສະພາບປັດຈຸບັນ".into(),
    );

    m.insert(
        "backup.recreate_will_title".into(),
        "\u{26A0}\u{FE0F} RECREATE ຈະ:".into(),
    );

    m.insert(
        "backup.recreated".into(),
        "ສ້າງໂຄງການໃໝ່ແລ້ວ: ລຶບ backup ເກົ່າ, ສ້າງ backup ສົດ.".into(),
    );

    m.insert("backup.restore_add_count".into(), "{} ໄຟລ໌ຈະຖືກກູ້ຄືນ.".into());

    m.insert("backup.restore_cancelled".into(), "ຍົກເລີກການກູ້ຄືນ.".into());

    m.insert(
        "backup.restore_confirm".into(),
        "ດຳເນີນການກູ້ຄືນບໍ? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_create_title".into(),
        "\u{26A0} ໄຟລ໌ຕໍ່ໄປນີ້ຈະຖືກສ້າງ ({}):".into(),
    );

    m.insert(
        "backup.restore_delete_title".into(),
        "\u{26A0} ໄຟລ໌ຕໍ່ໄປນີ້ຈະຖືກລຶບ (ບໍ່ຢູ່ໃນ backup) ({}):".into(),
    );

    m.insert(
        "backup.restore_deleted_confirm".into(),
        "ດຳເນີນການກູ້ຄືນສິ່ງທີ່ຖືກລຶບບໍ? (y/N): ".into(),
    );

    m.insert(
        "backup.restore_deleted_title".into(),
        " {} ໄຟລ໌ຈະຖືກກູ້ຄືນ (ມີຢູ່ໃນ backup ແຕ່ໝົດໄປຕອນນີ້):".into(),
    );

    m.insert("backup.restore_failed".into(), "ກູ້ຄືນ {} ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "backup.restore_overwrite_count".into(),
        "{} ໄຟລ໌ຈະຖືກຂຽນທັບ.".into(),
    );

    m.insert(
        "backup.restore_overwrite_title".into(),
        "\u{26A0} ໄຟລ໌ຕໍ່ໄປນີ້ຈະຖືກຂຽນທັບ ({}):".into(),
    );

    m.insert(
        "backup.restore_preview_title".into(),
        "ເບິ່ງລ່ວງໜ້າການກູ້ຄືນ".into(),
    );

    m.insert(
        "backup.restored_backup".into(),
        "ກູ້ຄືນສຳເລັດ: Backup #{} \u{2014} ກູ້ຄືນ {} ໄຟລ໌".into(),
    );

    m.insert("backup.restored_date".into(), "ດຳເນີນການກູ້ຄືນເມື່ອ: {}".into());

    m.insert(
        "backup.restored_deleted".into(),
        "ກູ້ຄືນໄຟລ໌ທີ່ຖືກລຶບຈາກ Backup #{}: ກູ້ຄືນ {} ໄຟລ໌".into(),
    );

    m.insert(
        "backup.restored_result".into(),
        "ກູ້ຄືນ backup #{} ({})".into(),
    );

    m.insert(
        "backup.select_entry".into(),
        " {}. Backup #{} \u{2014} {} \u{2014} {} \u{2014} {} files".into(),
    );

    m.insert(
        "backup.select_prompt".into(),
        "ເລືອກ backup ເພື່ອກູ້ຄືນ (1-{}): ".into(),
    );

    m.insert(
        "backup.skip_large".into(),
        "ຂ້າມ {} ({} MB ເກີນຂີດຈຳກັດ 50 MB)".into(),
    );

    m.insert(
        "backup.skip_metadata".into(),
        "ຂ້າມ {} (metadata ມີຂໍ້ຜິດພາດ: {})".into(),
    );

    m.insert(
        "backup.skip_rel_path".into(),
        "ຂ້າມ {} (ບໍ່ສາມາດຊອກຫາ relative path ໄດ້)".into(),
    );

    m.insert(
        "backup.skip_symlink".into(),
        "ຂ້າມ {} (symlink \u{2014} ບໍ່ຕິດຕາມ)".into(),
    );

    m.insert(
        "backup.skipped_count".into(),
        "ຂ້າມ {} ໄຟລ໌ \u{2014} ເບິ່ງ ignore.txt ສຳລັບລາຍລະອຽດ".into(),
    );

    m.insert("backup.skipped_error".into(), "ຂ້າມ {} ({})".into());

    m.insert(
        "backup.skipped_large".into(),
        "ຂ້າມ {} ({} \u{2014} ໃຫຍ່ເກີນໄປ)".into(),
    );

    m.insert(
        "backup.skipped_unsupported".into(),
        "ຂ້າມ {} (ປະເພດໄຟລ໌ບໍ່ຮອງຮັບ)".into(),
    );

    m.insert(
        "backup.snapshot_failed".into(),
        "ບໍ່ສາມາດ snapshot {} ສຳລັບ undo ໄດ້ ({}), ດຳເນີນຕໍ່".into(),
    );

    m.insert(
        "backup.some_restore_failed".into(),
        "ບາງໄຟລ໌ບໍ່ສາມາດກູ້ຄືນໄດ້. ເກັບຮັກສາ undo ໄວ້".into(),
    );

    m.insert("backup.stored_in".into(), "ເກັບ backup ໄວ້ທີ່: {}".into());

    m.insert(
        "backup.undo_cleared".into(),
        "ລຶບ undo history ແລ້ວ (ຍົກເວັ້ນ {})".into(),
    );

    m.insert(
        "backup.undo_complete".into(),
        "ຍົກເລີກສຳເລັດ: ກູ້ຄືນ {} ໄຟລ໌ຈາກກ່ອນການກູ້ຄືນລ່າສຸດ".into(),
    );

    m.insert(
        "backup.undo_complete_detail".into(),
        "ຍົກເລີກການກູ້ຄືນ backup #{} (ກູ້ຄືນ {}, ລຶບ {}, ກູ້ຄືນອີກ {})".into(),
    );

    m.insert(
        "backup.undo_delete_failed".into(),
        "ບໍ່ສາມາດລຶບໄຟລ໌ທີ່ກູ້ຄືນ {}: {}".into(),
    );

    m.insert(
        "backup.undo_missing_original".into(),
        "ບໍ່ສາມາດກູ້ຄືນ {} (ຕົ້ນສະບັບຫາຍໄປຈາກ undo storage)".into(),
    );

    m.insert(
        "backup.undo_missing_snapshot".into(),
        "ບໍ່ສາມາດກູ້ຄືນອີກ {} (snapshot ຫາຍໄປຈາກ undo storage)".into(),
    );

    m.insert("backup.undo_no_history".into(), "ບໍ່ພົບ undo history.".into());

    m.insert("backup.undo_none".into(), "ບໍ່ມີການກູ້ຄືນໃຫ້ຍົກເລີກ.".into());

    m.insert(
        "backup.undo_rerestore_failed".into(),
        "ບໍ່ສາມາດກູ້ຄືນອີກສຳລັບໄຟລ໌ທີ່ຖືກລຶບ {}: {}".into(),
    );

    m.insert(
        "backup.unknown_clear_option".into(),
        "ບໍ່ຮູ້ຈັກຕົວເລືອກ backup clear: {}".into(),
    );

    m.insert(
        "backup.unknown_create_option".into(),
        "ບໍ່ຮູ້ຈັກຕົວເລືອກ backup create: {}".into(),
    );

    m.insert(
        "backup.unknown_recreate_option".into(),
        "ບໍ່ຮູ້ຈັກຕົວເລືອກ backup recreate: {}".into(),
    );

    m.insert(
        "backup.unknown_undo_option".into(),
        "ບໍ່ຮູ້ຈັກຕົວເລືອກ backup undo: {}".into(),
    );

    m.insert(
        "backup.usage_clear".into(),
        "ວິທີໃຊ້: backup clear [--force]".into(),
    );

    m.insert(
        "backup.usage_recreate".into(),
        "ວິທີໃຊ້: backup recreate [--force]".into(),
    );

    m.insert(
        "backup.usage_restore".into(),
        "ວິທີໃຊ້: backup restore [--choose] [--deleted] [num]".into(),
    );

    m.insert("backup.warning_count".into(), "ທ່ານມີ {} backup:".into());

    m.insert(
        "backup.warning_title".into(),
        "\u{26A0}\u{FE0F} ຄຳເຕືອນ: ອັນນີ້ຈະລຶບ backup ທັງໝົດຂອງໂຄງການນີ້!".into(),
    );

    m.insert(
        "backup.would_overwrite".into(),
        "{} ຈະຖືກຂຽນທັບ (ໃຊ້ --force ເພື່ອຂ້າມ)".into(),
    );

    m.insert(
        "backup.would_overwrite_deleted".into(),
        "ກຳລັງກູ້ຄືນໄຟລ໌ທີ່ຖືກລຶບ: {}".into(),
    );
}
