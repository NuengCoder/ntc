use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.bench".into(),
        "time --bench <n> <ຄຳສັ່ງ> ## ເຮັດຊ້ຳ n ຄັ້ງ ແລະ ສະແດງ min/max/mean/median".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "time <ຄຳສັ່ງ A> --compare <ຄຳສັ່ງ B> [-w] ## ເຮັດທັງສອງ ແລ້ວປຽບທຽບສະຖານະອອກ ແລະ ຜົນອອກ".into(),
    );

    m.insert(
        "help.time.count".into(),
        "time --count <ໄລຍະເວລາ> ## ນັບຖອຍຫຼັງ (30, 90s, 5m, 1h30m)".into(),
    );

    m.insert(
        "help.time.help".into(),
        "time --help ## ສະແດງຄຳແນະນຳນີ້".into(),
    );

    m.insert(
        "help.time.note".into(),
        "ຄຳສັ່ງດຳເນີນໃນໄດເຣັກທໍຣີປັດຈຸບັນ ແລະ ສາມາດເປັນຄຳສັ່ງ NTC, alias ຫຼື ຄຳສັ່ງພາຍນອກ. --compare ຈະເກັບຜົນອອກ; --bench ຈະເຮັດຄຳສັ່ງຊ້ຳ ດັ່ງນັ້ນຜົນຂ້າງຂອງກໍຈະເກີດຊ້ຳ. ກົດ Ctrl+C ເພື່ອຍົກເລີກການນັບຖອຍຫຼັງ.".into(),
    );

    m.insert(
        "help.time.run".into(),
        "time <ຄຳສັ່ງ> ## ຮັບໃຊ້ຄຳສັ່ງ ແລະ ສະແດງເວລາທີ່ໃຊ້".into(),
    );

    m.insert(
        "help.time.title".into(),
        "ຄຳສັ່ງ TIME - ຈັບເວລາ, ປຽບທຽບ ແລະ ນັບຖອຍຫຼັງ:".into(),
    );
}
