use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.skip.title".into(), "SKIP ຄຳສັ່ງ:".into());

    m.insert(
        "help.skip.usage1".into(),
        "skip [N] [file] ## ສະແດງທຸກແຖວຫຼັງ N ແຖວທຳອິດ (ຄ່າເລີ່ມຕົ້ນ 10)".into(),
    );

    m.insert(
        "help.skip.usage2".into(),
        "skip -x <lines> [file] ## ສະແດງທຸກແຖວ ເວັ້ນແຖວທີ່ລະບຸ (ເລີ່ມຈາກ 1)".into(),
    );

    m.insert(
        "help.skip.note".into(),
        "ເລກແຖວໃນ -x ຄັ່ນດ້ວຍຈຸລະພາຄ ຫຼື ຊ່ອງວ່າງ (2,5,8 ຫຼື 2 5 8); input ມາຈາກໄຟລ໌ ຫຼື pipe".into(),
    );

    m.insert(
        "help.skip.example1".into(),
        "skip 2 app.log ## ສະແດງ app.log ໂດຍບໍ່ມີ 2 ແຖວທຳອິດ".into(),
    );

    m.insert(
        "help.skip.example2".into(),
        "gs TODO | skip -x 2,5,8 ## ຂ້າມແຖວທີ 2, 5 ແລະ 8 ຂອງຜົນຄົ້ນຫາ".into(),
    );
}
