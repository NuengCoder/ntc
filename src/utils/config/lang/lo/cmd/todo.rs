use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.todo.add_usage".into(),
        "ວິທີໃຊ້: todo add \"task name\" [-> N]".into(),
    );

    m.insert("cmd.todo.all_cleared".into(), "ລຶບງານທັງໝົດແລ້ວ.".into());

    m.insert("cmd.todo.cancelled".into(), "ຍົກເລີກ.".into());

    m.insert(
        "cmd.todo.clear_prompt".into(),
        "ລຶບທັງໝົດ {} ງານບໍ? [y/N] ".into(),
    );

    m.insert("cmd.todo.empty_name".into(), "ຊື່ງານບໍ່ສາມາດຫວ່າງເປົ່າໄດ້".into());

    m.insert(
        "cmd.todo.help_add".into(),
        "  todo add \"name\" [-> N]  ເພີ່ມງານ (ທາງເລືອກທີ່ຕຳແໜ່ງ N)".into(),
    );

    m.insert(
        "cmd.todo.help_clear".into(),
        "  todo clear              ລຶບງານທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "cmd.todo.help_info".into(),
        "  todo info [N]           ສະແດງງານທັງໝົດ ຫຼື ງານທີ່ N".into(),
    );

    m.insert(
        "cmd.todo.help_rm".into(),
        "  todo rm <N>             ລຶບງານທີ່ N".into(),
    );

    m.insert(
        "cmd.todo.help_show".into(),
        "  todo                    ສະແດງງານທັງໝົດ".into(),
    );

    m.insert(
        "cmd.todo.info_usage".into(),
        "ວິທີໃຊ້: todo info <N>  — N ຕ້ອງເປັນເລກ".into(),
    );

    m.insert(
        "cmd.todo.invalid_position".into(),
        "ຕຳແໜ່ງບໍ່ຖືກຕ້ອງ: '{}' ຕ້ອງເປັນເລກ".into(),
    );

    m.insert("cmd.todo.list_title".into(), "TODO LIST:".into());

    m.insert(
        "cmd.todo.missing_position".into(),
        "ຂາດເລກຕຳແໜ່ງຫຼັງ '->'".into(),
    );

    m.insert("cmd.todo.missing_quote".into(), "ຂາດເຄື່ອງໝາຍອ້າງອີງປິດ".into());

    m.insert(
        "cmd.todo.must_be_quoted".into(),
        "ຊື່ງານຕ້ອງໃສ່ເຄື່ອງໝາຍອ້າງອີງ".into(),
    );

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "ບໍ່ມີງານໃຫ້ລຶບ.".into());

    m.insert(
        "cmd.todo.one_based".into(),
        "ເລກງານເລີ່ມຈາກ 1 (ຕ່ຳສຸດ 1)".into(),
    );

    m.insert(
        "cmd.todo.position_one_based".into(),
        "ຕຳແໜ່ງຕ້ອງເລີ່ມຈາກ 1 (ຕ່ຳສຸດ 1)".into(),
    );

    m.insert("cmd.todo.rm_usage".into(), "ວິທີໃຊ້: todo rm <N>".into());

    m.insert(
        "cmd.todo.rm_usage_num".into(),
        "ວິທີໃຊ້: todo rm <N>  — N ຕ້ອງເປັນເລກ".into(),
    );

    m.insert("cmd.todo.task_added".into(), "ເພີ່ມງານແລ້ວ: {}".into());

    m.insert(
        "cmd.todo.task_not_found".into(),
        "ບໍ່ພົບງານ #{} (ມີ {} ງານ)".into(),
    );

    m.insert("cmd.todo.task_removed".into(), "ລຶບງານແລ້ວ: {}".into());

    m.insert(
        "cmd.todo.unexpected_text".into(),
        "ຂໍ້ຄວາມທີ່ບໍ່ຄາດຫວັງຫຼັງຊື່ງານ: '{}'".into(),
    );

    m.insert(
        "cmd.todo.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກ todo subcommand: '{}'".into(),
    );

    m.insert("cmd.todo.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.todo.help_done".into(),
        "  todo done <N>           ໝາຍງານ N ວ່າສຳເລັດ".into(),
    );

    m.insert(
        "cmd.todo.help_undo".into(),
        "  todo undo <N>           ເປີດງານ N ໃໝ່".into(),
    );

    m.insert(
        "cmd.todo.help_edit".into(),
        "  todo edit <N> \"name\"    ປ່ຽນຊื่องານ N".into(),
    );

    m.insert("cmd.todo.done_usage".into(), "ວິທີໃຊ້: todo done <N>".into());

    m.insert("cmd.todo.undo_usage".into(), "ວິທີໃຊ້: todo undo <N>".into());

    m.insert(
        "cmd.todo.edit_usage".into(),
        "ວິທີໃຊ້: todo edit <N> \"ຊື່ໃໝ່\"".into(),
    );

    m.insert("cmd.todo.task_done".into(), "ງານສຳເລັດແລ້ວ: {}".into());

    m.insert("cmd.todo.task_undone".into(), "ເປີດງານໃໝ່ແລ້ວ: {}".into());

    m.insert("cmd.todo.task_edited".into(), "ອັບເດດງານແລ້ວ: {}".into());

    m.insert("cmd.todo.already_done".into(), "ງານ #{} ສຳເລັດແລ້ວ".into());

    m.insert("cmd.todo.already_open".into(), "ງານ #{} ຍັງເປີດຢູ່".into());
}
