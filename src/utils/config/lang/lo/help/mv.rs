use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mv.title".into(), "MV ຄຳສັ່ງ:".into());

    m.insert(
        "help.mv.usage".into(),
        "mv <src|@tp> <dst|@tp> [-f] ## ຍ້າຍ/ປ່ຽນຊື່ໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.mv.example1".into(),
        "mv old.txt new.txt ## ປ່ຽນຊື່ໄຟລ໌".into(),
    );
}
