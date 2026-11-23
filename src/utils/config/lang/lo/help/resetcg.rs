use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.resetcg.title".into(), "RESETCG ຄຳສັ່ງ:".into());

    m.insert(
        "help.resetcg.usage".into(),
        "resetcg ## ຣີເຊັດ config ເປັນຄ່າເລີ່ມຕົ້ນ (ພ້ອມຕົວເລືອກ backup)".into(),
    );

    m.insert(
        "help.resetcg.example1".into(),
        "resetcg ## ຣີເຊັດການຕັ້ງຄ່າເປັນຄ່າເລີ່ມຕົ້ນ".into(),
    );
}
