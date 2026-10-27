use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nav.available_drives".into(), "drives ທີ່ມີ:".into());

    m.insert("nav.cancelled".into(), "ຖືກຍົກເລີກ.".into());

    m.insert("nav.config_exists".into(), " \u{2713} ມີໄຟລ໌ config ຢູ່".into());

    m.insert(
        "nav.drive_prompt".into(),
        "ປ້ອນຕົວອັກສອນ drive ຫຼື ເບີ (ຫຼື 'cancel'): ".into(),
    );

    m.insert("nav.gos_where".into(), "gos ຢູ່ໃສ?".into());
}
