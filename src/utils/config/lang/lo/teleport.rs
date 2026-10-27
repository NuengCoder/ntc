use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("teleport.added".into(), "ສ້າງ Savepoint '{}' -> {}".into());

    m.insert(
        "teleport.all_cleared".into(),
        "ລຶບ savepoints ທັງໝົດແລ້ວ.".into(),
    );

    m.insert(
        "teleport.back_hint".into(),
        "ໃຊ້ 'tpb' ເພື່ອກັບໄປຕຳແໜ່ງກ່ອນໜ້າ.".into(),
    );

    m.insert("teleport.back_opt".into(), "b. Teleport ກັບ".into());

    m.insert("teleport.cancel_opt".into(), "0. ຍົກເລີກ".into());

    m.insert(
        "teleport.choose_prompt".into(),
        "ປ້ອນເບີເພື່ອ teleport (1-{}) ຫຼື b/h/0: ".into(),
    );

    m.insert("teleport.clear_cancelled".into(), "ການລຶບຖືກຍົກເລີກ.".into());

    m.insert(
        "teleport.clear_confirm".into(),
        "\u{26A0} ທ່ານແນ່ໃຈບໍວ່າຕ້ອງການລຶບ savepoints ທັງໝົດ? (y/N): ".into(),
    );

    m.insert(
        "teleport.clear_hint".into(),
        "ໃຊ້ 'tp clear' ເພື່ອລຶບ history.".into(),
    );

    m.insert(
        "teleport.fs_failed".into(),
        "ເຂົ້າເຖິງ file system ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "teleport.history_cleared".into(),
        "ລຶບ teleport history ແລ້ວ.".into(),
    );

    m.insert("teleport.history_opt".into(), "h. ສະແດງ History".into());

    m.insert(
        "teleport.history_title".into(),
        "\u{1F4DC} Teleport History (ລ່າສຸດຢູ່ທ້າຍ)".into(),
    );

    m.insert(
        "teleport.invalid_char".into(),
        "ຕົວອັກສອນບໍ່ຖືກຕ້ອງໃນຊື່: '{}'".into(),
    );

    m.insert(
        "teleport.invalid_index".into(),
        "index ບໍ່ຖືກຕ້ອງ: {}. ໃຊ້ 1-{}".into(),
    );

    m.insert(
        "teleport.invalid_input_interactive".into(),
        "ຂໍ້ມູນເຂົ້າບໍ່ຖືກຕ້ອງ: {}. ປ້ອນເບີ, 'b', ຫຼື 'h'.".into(),
    );

    m.insert(
        "teleport.invalid_name".into(),
        "ຊື່ບໍ່ຖືກຕ້ອງ: '{}' ຖືກສະຫງວນ ຫຼື ວ່າງເປົ່າ".into(),
    );

    m.insert(
        "teleport.jump_failed".into(),
        "teleport ບໍ່ສຳເລັດ: '{}'".into(),
    );

    m.insert(
        "teleport.jump_hint".into(),
        "ໃຊ້ 'tp jump <name>' ເພື່ອ teleport.".into(),
    );

    m.insert("teleport.jumped".into(), "Teleported ໄປ '{}' -> {}".into());

    m.insert(
        "teleport.list_title".into(),
        "\u{1F4CC} ຈຸດ Teleport ຂອງທ່ານ (global)".into(),
    );

    m.insert(
        "teleport.menu_title".into(),
        "\u{1F4CC} Teleport - ຈຸດ Teleport ຂອງທ່ານ (global)".into(),
    );

    m.insert(
        "teleport.name_exists".into(),
        "ປ່ຽນຊື່ບໍ່ໄດ້: '{}' ມີຢູ່ເປັນ savepoint ແລ້ວ".into(),
    );

    m.insert(
        "teleport.name_taken".into(),
        "Savepoint '{}' ມີຢູ່ແລ້ວ. ໃຊ້ 'tp edit' ເພື່ອອັບເດດ.".into(),
    );

    m.insert(
        "teleport.name_taken_edit".into(),
        "Savepoint '{}' ມີຢູ່ແລ້ວ".into(),
    );

    m.insert(
        "teleport.no_history_first".into(),
        "ບໍ່ມີ teleport history. ໃຊ້ 'tp jump <name>' ກ່ອນເພື່ອສ້າງ history.".into(),
    );

    m.insert(
        "teleport.no_history_start".into(),
        "ບໍ່ມີ teleport history. ໃຊ້ 'tp jump <name>' ເພື່ອເລີ່ມສ້າງ history.".into(),
    );

    m.insert(
        "teleport.no_savepoints_to_clear".into(),
        "ບໍ່ມີ savepoints ໃຫ້ລຶບ.".into(),
    );

    m.insert(
        "teleport.no_savepoints_yet".into(),
        "ຍັງບໍ່ມີ savepoints. ໃຊ້ 'tp add <name>' ເພື່ອສ້າງ.".into(),
    );

    m.insert("teleport.not_found".into(), "ບໍ່ພົບ savepoint: '{}'".into());

    m.insert(
        "teleport.overwrite_cancelled".into(),
        "ການຂຽນທັບຖືກຍົກເລີກ.".into(),
    );

    m.insert(
        "teleport.overwrite_prompt".into(),
        "Savepoint '{}' ມີຢູ່ແລ້ວທີ່ຕຳແໜ່ງອື່ນ. ຂຽນທັບບໍ? (y/N): ".into(),
    );

    m.insert("teleport.prev_gone".into(), "ຕຳແໜ່ງກ່ອນໜ້າບໍ່ມີຢູ່ອີກ: {}".into());

    m.insert("teleport.removed".into(), "ລຶບ savepoint: '{}' ແລ້ວ".into());

    m.insert(
        "teleport.removing_from_history".into(),
        "ກຳລັງລຶບຈາກ history. ໃຊ້ 'tp history' ເພື່ອເບິ່ງລາຍການທີ່ເຫຼືອ.".into(),
    );

    m.insert(
        "teleport.rename_failed".into(),
        "ປ່ຽນຊື່ savepoint ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("teleport.rename_same".into(), "ຊື່ໃໝ່ຄືກັນກັບຊື່ເກົ່າ".into());

    m.insert(
        "teleport.renamed".into(),
        "ປ່ຽນຊື່ savepoint '{}' \u{2192} '{}'".into(),
    );

    m.insert(
        "teleport.renamed_with_path".into(),
        "ປ່ຽນຊື່ savepoint '{}' -> '{}' (-> {})".into(),
    );

    m.insert(
        "teleport.teleport_cancelled".into(),
        "ການ teleport ຖືກຍົກເລີກ.".into(),
    );

    m.insert(
        "teleport.teleported_back".into(),
        "Teleported ກັບໄປ: {}".into(),
    );

    m.insert(
        "teleport.tpb_hint".into(),
        "ໃຊ້ 'tpb' ເພື່ອກັບໄປຕຳແໜ່ງກ່ອນໜ້າ.".into(),
    );

    m.insert(
        "teleport.updated".into(),
        "ອັບເດດ Savepoint '{}': {} -> {}".into(),
    );
}
