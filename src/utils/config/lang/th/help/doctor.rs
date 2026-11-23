use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.doctor.title".into(), "คำสั่ง DOCTOR:".into());

    m.insert(
        "help.doctor.usage".into(),
        "doctor ## ตรวจ config, PATH, ranfile และ ntpkg หาปัญหา".into(),
    );

    m.insert(
        "help.doctor.example1".into(),
        "doctor ## รายงานแบบอ่านอย่างเดียว ไม่แก้อะไร".into(),
    );
}
