use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fs.title".into(), "FS ຄຳສັ່ງ:".into());

    m.insert(
        "help.fs.usage".into(),
        "fs <pattern> [-d <n>] [--no-fuzzy] ## ຄົ້ນຫາໄຟລ໌ຕາມຊື່ (fuzzy fallback ເວັ້ນເສຍແຕ່ໃສ່ --no-fuzzy)"
            .into(),
    );

    m.insert("help.fs.example1".into(), "fs *.rs ## ຊອກຫາໄຟລ໌ .rs".into());
}
