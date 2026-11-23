use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cp.title".into(), "CP ຄຳສັ່ງ:".into());

    m.insert(
        "help.cp.usage".into(),
        "cp <src|@tp> <dst|@tp> [-f] ## ສຳເນົາໄຟລ໌ຫຼືໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.cp.example1".into(),
        "cp file.txt backup.txt ## ສຳເນົາໄຟລ໌".into(),
    );
}
