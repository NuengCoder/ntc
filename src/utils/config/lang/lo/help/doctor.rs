use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.doctor.title".into(), "ຄຳສັ່ງ DOCTOR:".into());

    m.insert(
        "help.doctor.usage".into(),
        "doctor ## ກວດ config, PATH, ranfile ແລະ ntpkg ຫາບັນຫາ".into(),
    );

    m.insert(
        "help.doctor.example1".into(),
        "doctor ## ລາຍງານແບບອ່ານຢ່າງດຽວ ບໍ່ແກ້ຫຍັງ".into(),
    );
}
