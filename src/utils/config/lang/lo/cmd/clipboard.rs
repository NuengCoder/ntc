use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.clipboard.access_fail".into(),
        "ເຂົ້າເຖິງ clipboard ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.clipboard.already_empty".into(),
        "Clipboard history ວ່າງເປົ່າຢູ່ແລ້ວ.".into(),
    );

    m.insert("cmd.clipboard.clear_cancelled".into(), "ຍົກເລີກການລຶບ.".into());

    m.insert(
        "cmd.clipboard.clear_error".into(),
        "ລຶບ clipboard history ບໍ່ສຳເລັດ: {}.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_prompt".into(),
        "ລຶບປະຫວັດສະທັງນ້ຳ {} ລາຍການ? [y/N] ".into(),
    );

    m.insert(
        "cmd.clipboard.cleared".into(),
        "ລຶບ Clipboard history ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.clipboard.copied_entry".into(),
        "ສຳເນົາລາຍການ #{} ({}) ໃສ່ clipboard ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.clipboard.empty".into(),
        "Clipboard history ວ່າງເປົ່າ.".into(),
    );

    m.insert(
        "cmd.clipboard.entry_not_found".into(),
        "ບໍ່ພົບລາຍການ #{} (ມີ {} ລາຍການ).".into(),
    );

    m.insert(
        "cmd.clipboard.help_clear".into(),
        "  clip clear         ລຶບ clipboard history".into(),
    );

    m.insert(
        "cmd.clipboard.help_list".into(),
        "  clip               ສະແດງ clipboard history".into(),
    );

    m.insert(
        "cmd.clipboard.help_select".into(),
        "  clip select <N>    ສຳເນົາລາຍການ N ກັບໃສ່ clipboard".into(),
    );

    m.insert(
        "cmd.clipboard.history_header".into(),
        "  Clipboard History (ລ່າສຸດ {}):".into(),
    );

    m.insert(
        "cmd.clipboard.select_usage".into(),
        "ວິທີໃຊ້: clip select <N>".into(),
    );

    m.insert(
        "cmd.clipboard.set_fail".into(),
        "ຕັ້ງ clipboard ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.clipboard.termux_unavailable".into(),
        "termux-clipboard-set ບໍ່ມີ: {}".into(),
    );

    m.insert(
        "cmd.clipboard.unknown_sub".into(),
        "ບໍ່ຮູ້ຈັກ clip subcommand: {}".into(),
    );
}
