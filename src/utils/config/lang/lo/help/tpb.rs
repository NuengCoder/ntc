use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tpb.title".into(), "TPB ຄຳສັ່ງ:".into());

    m.insert(
        "help.tpb.usage".into(),
        "tpb ## Teleport ກັບໄປຕຳແໜ່ງກ່ອນໜ້າ (undo tp)".into(),
    );

    m.insert(
        "help.tpb.history".into(),
        "tpb history ## ສະແດງ teleport navigation history".into(),
    );

    m.insert(
        "help.tpb.clear".into(),
        "tpb clear ## ລຶບ teleport history".into(),
    );

    m.insert("help.tpb.example1".into(), "tpb ## ກັບໄປຕຳແໜ່ງກ່ອນໜ້າ".into());
}
