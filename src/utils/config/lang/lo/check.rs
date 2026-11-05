use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "check.help".into(),
        " check [ໄຟລ໌] - ຕີຄວາມ {} ແລະ ລາຍງານທຸກບັນຫາ".into(),
    );

    m.insert("check.hint".into(), "ຄຳແນະນຳ".into());

    m.insert("check.no_files".into(), "ບໍ່ພົບໄຟລ໌ {} ໃນ {}".into());

    m.insert("check.not_found".into(), "ບໍ່ພົບ {}".into());

    m.insert("check.ok".into(), "{}: ບໍ່ມີບັນຫາ".into());

    m.insert("check.read_fail".into(), "ອ່ານ {} ບໍ່ໄດ້: {}".into());

    m.insert("check.summary".into(), "ພົບ {} ບັນຫາໃນ {}".into());

    m.insert("check.title".into(), "ກວດສອບ {}".into());
}
