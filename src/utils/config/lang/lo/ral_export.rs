use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ral_export.all".into(), "ເລືອກທັງໝົດ".into());

    m.insert("ral_export.cancel".into(), "ຍົກເລີກ".into());

    m.insert(
        "ral_export.export_cancelled".into(),
        "ການສົ່ງອອກຖືກຍົກເລີກ.".into(),
    );

    m.insert("ral_export.finish".into(), "ສຳເລັດການສົ່ງອອກ".into());

    m.insert("ral_export.prompt".into(), "ral-export>".into());

    m.insert("ral_export.select".into(), "ເລືອກ/ຍົກເລີກຕາມເບີ".into());

    m.insert("ral_export.selected".into(), "ຖືກເລືອກ".into());

    m.insert("ral_export.selected_mark".into(), "(ຖືກເລືອກ)".into());

    m.insert(
        "ral_export.title".into(),
        "ral export — ເລືອກ Aliases ເພື່ອສົ່ງອອກ".into(),
    );
}
