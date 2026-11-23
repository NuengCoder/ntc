use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.opencg.title".into(), "OPENCG ຄຳສັ່ງ:".into());

    m.insert(
        "help.opencg.usage".into(),
        "opencg ## ເປີດ config.toml ໃນ editor".into(),
    );

    m.insert(
        "help.opencg.example1".into(),
        "opencg ## ແກ້ໄຂ config.toml ໃນ editor".into(),
    );
}
