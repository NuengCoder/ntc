use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rev.title".into(), "REV ຄຳສັ່ງ:".into());

    m.insert(
        "help.rev.usage".into(),
        "rev [-u] [file] ## ຍ້ອນຕົວອັກສອນຂອງແຕ່ລະແຖວ ຫຼື ສະຫຼັບລຳດັບແຖວດ້ວຍ -u".into(),
    );

    m.insert(
        "help.rev.example1".into(),
        "rev notes.txt ## ກັບລຳດັບຕົວອັກສອນໃນແຕ່ລະແຖວ".into(),
    );

    m.insert(
        "help.rev.example2".into(),
        "rev -u notes.txt ## ສະຫຼັບລຸ່ມຂຶ້ນເທິງ: ແຖວສຸດທ້າຍຂຶ້ນກ່ອນ, ແຖວທຳອິດໄປທ້າຍ".into(),
    );
}
