use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fsc.title".into(), "FSC ຄຳສັ່ງ:".into());

    m.insert(
        "help.fsc.usage".into(),
        "fsc <pattern> [-d <n>] [--no-fuzzy] ## ຄົ້ນຫາໄຟລ໌, ເລືອກເພື່ອສະແດງ ແລະ ສຳເນົາໄດ້".into(),
    );

    m.insert(
        "help.fsc.example1".into(),
        "fsc config ## ຄົ້ນຫາໄຟລ໌ທີ່ກົງແລ້ວສະແດງ".into(),
    );
}
