use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ranfile.available_title".into(), "targets ທີ່ມີ:".into());

    m.insert("ranfile.depends_on".into(), " ─▶ ຂຶ້ນກັບ: ".into());

    m.insert(
        "ranfile.runner.hint".into(),
        "ເຮັດວຽກ {} ເພື່ອເບິ່ງ targets ທີ່ມີ.".into(),
    );
}
