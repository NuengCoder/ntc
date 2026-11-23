use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.version.title".into(), "VERSION คำสั่ง:".into());

    m.insert("help.version.usage".into(), "version ## แสดงเวอร์ชัน".into());

    m.insert(
        "help.version.example1".into(),
        "version ## แสดงข้อมูลเวอร์ชัน".into(),
    );
}
