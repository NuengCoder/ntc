use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmd.title".into(), "RNMD ຄຳສັ່ງ:".into());

    m.insert(
        "help.rnmd.usage".into(),
        "rnmd <old> <new> ## ປ່ຽນຊື່ໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.rnmd.example1".into(),
        "rnmd src lib ## ປ່ຽນຊື່ໄດເຣັກທໍຣີ".into(),
    );
}
