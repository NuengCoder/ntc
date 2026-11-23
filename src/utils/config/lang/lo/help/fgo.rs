use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fgo.title".into(), "FGO ຄຳສັ່ງ:".into());

    m.insert(
        "help.fgo.usage".into(),
        "fgo <pattern> [-d <n>] ## ຊອກຫາໄຟລ໌, ນຳທາງໄປ parent dir ຂອງໄຟລ໌ທີ່ເລືອກ".into(),
    );

    m.insert(
        "help.fgo.example1".into(),
        "fgo *.rs ## ຄົ້ນຫາໄຟລ໌ Rust ແລ້ວໂດດໄປຫາ".into(),
    );
}
