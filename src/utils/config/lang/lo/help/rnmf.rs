use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmf.title".into(), "RNMF ຄຳສັ່ງ:".into());

    m.insert(
        "help.rnmf.usage".into(),
        "rnmf <old> <new> ## ປ່ຽນຊື່ໄຟລ໌".into(),
    );

    m.insert(
        "help.rnmf.example1".into(),
        "rnmf a.txt b.txt ## ປ່ຽນຊື່ໄຟລ໌".into(),
    );
}
