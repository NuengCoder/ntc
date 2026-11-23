use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ds.title".into(), "DS ຄຳສັ່ງ:".into());

    m.insert(
        "help.ds.usage".into(),
        "ds <pattern> [-d <n>] [--no-fuzzy] ## ຄົ້ນຫາໄດເຣັກທໍຣີຕາມຊື່ (--no-fuzzy ປິດການຄົ້ນຫາແບບ fuzzy)"
            .into(),
    );

    m.insert(
        "help.ds.example1".into(),
        "ds src ## ຊອກຫາໄດເຣັກທໍຣີທີ່ຊື່ 'src'".into(),
    );
}
