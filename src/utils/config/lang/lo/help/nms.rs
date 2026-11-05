use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "ຄຳສັ່ງ NMS (NTC Music):".into());

    m.insert(
        "help.nms.play".into(),
        "nms play [-sf <dur>] <ໄຟລ໌> ## ຫຼິ້ນສຽງໃນ terminal (Ctrl+C ຢຸດ; -sf ເລີ່ມຈາກເວລາທີ່ກຳນົດ)".into(),
    );

    m.insert(
        "help.nms.scan".into(),
        "nms scan [-d <n>] [-s] [-l <dur>] [--care] ## ສະແດງໄຟລ໌ສຽງ + ວິດີໂອ (dur: 25s, 1mn, 1h)"
            .into(),
    );

    m.insert(
        "help.nms.long".into(),
        "nms scan -l <dur> ## ສະແດງສະເພາະມີເດຍທີ່ຍາວກວ່າ <dur> (ເຊັ່ນ 25s, 1mn, 1h)".into(),
    );

    m.insert(
        "help.nms.keys".into(),
        "ຫຼິ້ນໃນ terminal ພ້ອມນັບຕຳແໜ່ງ / ເວລາທີ່ເຫຼືອຖອຍຫຼັງ · ວິດີໂອຫຼິ້ນສະເພາະສຽງ (ຜ່ານ ffmpeg) · ກົດ Ctrl+C ເພື່ອຢຸດ".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play -sf 30 song.mp3 ## ເລີ່ມຫຼິ້ນໄຟລ໌ສຽງທີ່ 0:30".into(),
    );

    m.insert(
        "help.nms.example2".into(),
        "nms play movie.mp4 ## ຫຼິ້ນສຽງຈາກວິດີໂອ".into(),
    );

    m.insert(
        "help.nms.example3".into(),
        "nms scan -d 2 -s ## ສະແດງໄຟລ໌ມີເດຍພ້ອມຂະໜາດ".into(),
    );

    m.insert(
        "help.nms.example4".into(),
        "nms scan -l 1mn ## ສະແດງມີເດຍທີ່ຍາວກວ່າ 1 ນາທີ".into(),
    );
}
