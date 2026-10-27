use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.mode.admin".into(),
        "mode admin ## ອອກ ແລະ ເລີ່ມ ntc ໃໝ່ເປັນ Administrator".into(),
    );

    m.insert(
        "help.mode.normal".into(),
        "mode normal ## ອອກ ແລະ ເລີ່ມ ntc ໃໝ່ໂດຍບໍ່ມີ elevation".into(),
    );

    m.insert("help.mode.title".into(), "WINDOWS MODE (Windows):".into());

    m.insert(
        "help.mode.example1".into(),
        "mode admin ## ເລີ່ມໃໝ່ເປັນ Administrator".into(),
    );
}
