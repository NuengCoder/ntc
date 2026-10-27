use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.time.title".into(),
        "ຄຳສັ່ງ TIME - ຈັບເວລາ ແລະ ນັບຖອຍຫຼັງ:".into(),
    );

    m.insert(
        "help.time.run".into(),
        "  time run <ຄຳສັ່ງ>                     ຮັບໃຊ້ຄຳສັ່ງ NTC, alias ຫຼື ຄຳສັ່ງພາຍນອກ ແລະ ລາຍງານເວລາ"
            .into(),
    );

    m.insert(
        "help.time.alias".into(),
        "  time -r|--run <ຄຳສັ່ງ>                ຮູບສັ້ນຂອງ run".into(),
    );

    m.insert(
        "help.time.compare".into(),
        "  time -r <ຄຳສັ່ງ> --compare <ຄຳສັ່ງ>   ເຮັດທັງສອງແລ້ວປຽບທຽບຜົນທີ່ບັນທຶກໄວ້".into(),
    );

    m.insert(
        "help.time.compare_with".into(),
        "  time compare <ຄຳສັ່ງ A> --with <ຄຳສັ່ງ B> [-w]  ປຽບທຽບສອງຄຳສັ່ງ (alias: -r A --compare B)"
            .into(),
    );

    m.insert(
        "help.time.bench".into(),
        "  time bench [--runs N] <ຄຳສັ່ງ>        ຊ້ຳ N ຄັ້ງ (ຄ່າເລີ່ມຕົ້ນ 5) ແລະ ສະແດງ min/max/mean/median"
            .into(),
    );

    m.insert(
        "help.time.count".into(),
        "  time count <ໄລຍະເວລາ>                ນັບຖອຍຫຼັງ (ເຊັ່ນ 30, 90s, 5m, 1h30m)".into(),
    );

    m.insert(
        "help.time.help".into(),
        "  time help                            ສະແດງຄຳແນະນຳນີ້".into(),
    );

    m.insert(
        "help.time.note".into(),
        "ຄຳສັ່ງດຳເນີນຕິດຕໍ່ກັນໃນໄດເຣັກທໍຣີປັດຈຸບັນ ແລະ ສາມາດເປັນຄຳສັ່ງ NTC, alias ຫຼື ຄຳສັ່ງພາຍນອກ. ການປຽບທຽບ ແລະ bench ຈະເກັບຜົນອອກມາ; ຜົນຂ້າງຂອງຄຳສັ່ງຍັງຄົງຢູ່. ກົດ Ctrl+C ເພື່ອຍົກເລີກການນັບຖອຍຫຼັງ.".into(),
    );
}
