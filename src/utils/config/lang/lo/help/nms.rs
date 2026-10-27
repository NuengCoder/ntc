use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nms.title".into(), "ຄຳສັ່ງ NMS (NTC Music):".into());

    m.insert(
        "help.nms.play".into(),
        "nms play <ໄຟລ໌> ## ເຄື່ອງຫຼິ້ນສຽງເຕັມຈໍ (space ຢຸດ · ←/→ ເລື່ອນ · ↑/↓ ລະດັບສຽງ · a ຫຼິ້ນຄືນ)".into(),
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
        "ໜ້າຈໍເຄື່ອງຫຼິ້ນ: ຄື້ນສຽງຈິງ + ມາດຕະວັດລະດັບສູງ/ຕ່ຳ · ວິດີໂອຫຼິ້ນສະເພາະສຽງ · ຈົບແລ້ວຈະຢຸດນິ້ງ ກົດ a ເພື່ອຫຼິ້ນຄືນ".into(),
    );

    m.insert(
        "help.nms.example1".into(),
        "nms play song.mp3 ## ຫຼິ້ນໄຟລ໌ສຽງ".into(),
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
