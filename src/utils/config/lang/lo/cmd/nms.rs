use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nms.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.nms.help_play".into(),
        "nms play <ໄຟລ໌>                                 ເຄື່ອງຫຼິ້ນສຽງເຕັມຈໍ (ວິດີໂອຫຼິ້ນສະເພາະສຽງ)".into(),
    );

    m.insert(
        "cmd.nms.help_scan".into(),
        "nms scan [-d|--depth <n>] [-s|--size] [-l|--long <dur>] [--care]  ສະແດງໄຟລ໌ສຽງ + ວິດີໂອ"
            .into(),
    );

    m.insert(
        "cmd.nms.help_help".into(),
        "nms help                                         ສະແດງຄວາມຊ່ວຍນີ້".into(),
    );

    m.insert("cmd.nms.options_title".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "cmd.nms.help_depth".into(),
        "  -d, --depth <n>   ຄວາມເລິກສູງສຸດ (ເລີ່ມຕົ້ນ: ຈາກ config)".into(),
    );

    m.insert(
        "cmd.nms.help_size".into(),
        "  -s, --size        ສະແດງຂະໜາດໄຟລ໌ແລະລວມທັງໝົດ".into(),
    );

    m.insert(
        "cmd.nms.help_long".into(),
        "  -l, --long <dur>  ສະແດງສະເພາະມີເດຍທີ່ຍາວກວ່າ <dur> (ເຊັ່ນ 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.help_care".into(),
        "  --care            ບໍ່ສົນໃຈກົດ ignore (ໂຫມດ care)".into(),
    );

    m.insert("cmd.nms.keys_title".into(), "ປຸ່ມເຄື່ອງຫຼິ້ນ:".into());

    m.insert(
        "cmd.nms.help_keys".into(),
        "  space ຢຸດ/ຫຼິ້ນຕໍ່ · ←/→ ເລື່ອນ 5 ວິນາທີ · ↑/↓ ລະດັບສຽງ · a ຫຼິ້ນຄືນ · q ອອກ".into(),
    );

    m.insert("cmd.nms.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert("cmd.nms.example_play".into(), "nms play song.mp3".into());

    m.insert("cmd.nms.example_video".into(), "nms play movie.mp4".into());

    m.insert("cmd.nms.example_scan".into(), "nms scan -d 2 -s".into());

    m.insert("cmd.nms.example_long".into(), "nms scan -l 1mn".into());

    m.insert(
        "cmd.nms.play_missing_file".into(),
        "ຂາດໄຟລ໌ ວິທີໃຊ້: nms play <ໄຟລ໌>".into(),
    );

    m.insert("cmd.nms.file_not_found".into(), "ບໍ່ພົບໄຟລ໌ '{}'".into());

    m.insert("cmd.nms.not_media".into(), "ບໍ່ແມ່ນໄຟລ໌ສຽງ/ວິດີໂອ: '{}'".into());

    m.insert(
        "cmd.nms.no_sound_device".into(),
        "ບໍ່ພົບອຸປະກອນສຽງໃນເຄື່ອງນີ້ — ຫຼິ້ນສຽງບໍ່ໄດ້".into(),
    );

    m.insert("cmd.nms.play_failed".into(), "ຫຼິ້ນ '{}' ບໍ່ໄດ້: {}".into());

    m.insert("cmd.nms.unknown_sub".into(), "ບໍ່ຮູ້ຈັກຄຳສັ່ງຍ່ອຍ nms: {}".into());

    m.insert("cmd.nms.unknown_option".into(), "ບໍ່ຮູ້ຈັກຕົວເລືອກ: {}".into());

    m.insert("cmd.nms.depth_usage".into(), "ວິທີໃຊ້: --depth <n>".into());

    m.insert("cmd.nms.invalid_depth".into(), "ຄວາມເລິກບໍ່ຖືກຕ້ອງ: '{}'".into());

    m.insert(
        "cmd.nms.long_usage".into(),
        "ວິທີໃຊ້: --long <dur> (ເຊັ່ນ 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "cmd.nms.invalid_long".into(),
        "ໄລຍະເວລາບໍ່ຖືກຕ້ອງ: '{}' (ເຊັ່ນ 25s, 1mn, 1h)".into(),
    );

    m.insert("cmd.nms.long_none".into(), "ບໍ່ພົບມີເດຍທີ່ຍາວກວ່າ {}".into());

    m.insert("cmd.nms.no_files".into(), "ບໍ່ພົບໄຟລ໌ສຽງ/ວິດີໂອ".into());

    m.insert("cmd.nms.list_title".into(), "ໄຟລ໌ມີເດຍ (ສຽງ + ວິດີໂອ):".into());

    m.insert("cmd.nms.total_files".into(), "{} ໄຟລ໌".into());

    m.insert("cmd.nms.total_files_size".into(), "{} ໄຟລ໌, ລວມ {}".into());
}
