use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("interactive.cancelled".into(), "ຖືກຍົກເລີກ.".into());

    m.insert("interactive.invalid_input".into(), "ຂໍ້ມູນເຂົ້າບໍ່ຖືກຕ້ອງ.".into());

    m.insert(
        "interactive.invalid_selection".into(),
        "ການເລືອກບໍ່ຖືກຕ້ອງ.".into(),
    );

    m.insert("interactive.no_items".into(), "(ບໍ່ມີລາຍການ)".into());
}
