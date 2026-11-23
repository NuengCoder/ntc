use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcared.title".into(), "IGCARED ຄຳສັ່ງ:".into());

    m.insert(
        "help.igcared.usage".into(),
        "igcared ## ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );

    m.insert(
        "help.igcared.example1".into(),
        "igcared ## ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );
}
