use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.wi.title".into(), "WI ຄຳສັ່ງ:".into());

    m.insert(
        "help.wi.usage".into(),
        "wi [path] ## ຟีดການປ່ຽນແປງໄຟລ໌ແບບສົດ (q=ອອກ)".into(),
    );

    m.insert(
        "help.wi.example1".into(),
        "wi ## ເຝົ້າເບິ່ງການປ່ຽນແປງໄຟລ໌ແບບສົດ".into(),
    );
}
