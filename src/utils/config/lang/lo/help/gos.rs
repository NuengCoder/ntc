use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gos.title".into(), "GOS ຄຳສັ່ງ:".into());

    m.insert(
        "help.gos.usage".into(),
        "gos ## ລາຍຊື່ subdirectories ແລະ ເລືອກ".into(),
    );

    m.insert(
        "help.gos.example1".into(),
        "gos ## ການເລືອກ subdirectory ແບບ interactive".into(),
    );
}
