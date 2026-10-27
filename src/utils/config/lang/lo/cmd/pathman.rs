use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.pathman.access_denied".into(),
        "ປະຕິເສດການເຂົ້າເຖິງ — ຕ້ອງການສິດ administrator.".into(),
    );

    m.insert(
        "cmd.pathman.added_to_path".into(),
        "ເພີ່ມໃສ່ {} ແລ້ວ: {}".into(),
    );

    m.insert(
        "cmd.pathman.already_in_path".into(),
        "ຢູ່ໃນ PATH ແລ້ວ: {}".into(),
    );

    m.insert("cmd.pathman.empty_path".into(), "║  {}".into());

    m.insert(
        "cmd.pathman.example_add".into(),
        "  {} add C:\\Tools".into(),
    );

    m.insert(
        "cmd.pathman.example_add_cwd".into(),
        "  {} add --cwd".into(),
    );

    m.insert(
        "cmd.pathman.example_move".into(),
        "  {} move 3 1               ຍ້າຍລາຍການ #3 ໄປດ້ານເທິງ".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_idx".into(),
        "  {} rm 3                    ລຶບລາຍການ #3 ອອກຈາກລາຍຊື່".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_path".into(),
        "  {} rm C:\\OldTools".into(),
    );

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.pathman.help_add".into(),
        "  {} add <path>               ເພີ່ມໄດເຣັກທໍຣີໃສ່ PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_add_cwd".into(),
        "  {} add --cwd | --here       ເພີ່ມໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_move".into(),
        "  {} move <n> <m>             ຍ້າຍລາຍການ #n ໄປຕຳແໜ່ງ #m".into(),
    );

    m.insert(
        "cmd.pathman.help_rm".into(),
        "  {} rm <path-or-number>      ລຶບອອກຈາກ PATH (ດ້ວຍ path ຫຼື index)".into(),
    );

    m.insert(
        "cmd.pathman.help_show".into(),
        "  {} show                     ສະແດງ PATH entries ພ້ອມ indices".into(),
    );

    m.insert(
        "cmd.pathman.idx_out_of_range".into(),
        "Index ເກີນຂອບເຂດ: {} (1-{})".into(),
    );

    m.insert(
        "cmd.pathman.indices_numbers".into(),
        "Indices ຕ້ອງເປັນເລກ".into(),
    );

    m.insert(
        "cmd.pathman.mode_admin_hint".into(),
        "ຈາກນັ້ນແລ່ນຄຳສັ່ງອີກຄັ້ງ. ຫຼື ໃຊ້: mode admin".into(),
    );

    m.insert(
        "cmd.pathman.move_usage".into(),
        "ວິທີໃຊ້: {} move <n> <m>".into(),
    );

    m.insert(
        "cmd.pathman.moved".into(),
        "ຍ້າຍໄປຕຳແໜ່ງ #{} ໃນ {} ແລ້ວ: {}".into(),
    );

    m.insert(
        "cmd.pathman.new_terminal".into(),
        "ການປ່ຽນແປງຈະມີຜົນກັບໜ້າຕ່າງ terminal ໃໝ່.".into(),
    );

    m.insert(
        "cmd.pathman.not_found_in_path".into(),
        "ບໍ່ພົບໃນ PATH: {}".into(),
    );

    m.insert(
        "cmd.pathman.path_update_fail".into(),
        "ອັບເດດ PATH ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_query_fail".into(),
        "ຄົ້ນຫາ registry ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.pathman.reg_write_fail".into(),
        "ຂຽນ registry ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.pathman.removed_from_path".into(),
        "ລຶບອອກຈາກ {} ແລ້ວ: {}".into(),
    );

    m.insert(
        "cmd.pathman.rm_usage".into(),
        "ວິທີໃຊ້: usrpath rm <path-or-number>".into(),
    );

    m.insert(
        "cmd.pathman.run_as_admin".into(),
        "ແລ່ນ terminal ຂອງທ່ານເປັນ Administrator (ຄລິກຂວາ \u{2192} \\\"Run as administrator\\\"),"
            .into(),
    );

    m.insert(
        "cmd.pathman.same_source_target".into(),
        "ແຫຼ່ງທີ່ມາ ແລະ ເປົ້າໝາຍເປັນອັນດຽວກັນ \u{2014} ບໍ່ມີຫຍັງໃຫ້ຍ້າຍ".into(),
    );

    m.insert("cmd.pathman.system_level".into(), "System".into());

    m.insert(
        "cmd.pathman.total_entries".into(),
        "  ທັງໝົດ: {} entries".into(),
    );

    m.insert("cmd.pathman.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert("cmd.pathman.user_level".into(), "User".into());

    m.insert(
        "cmd.pathman.windows_only".into(),
        "usrpath/syspath ແມ່ນຄຸນສົມບັດສະເພາະ Windows ເທົ່ານັ້ນ.".into(),
    );
}
