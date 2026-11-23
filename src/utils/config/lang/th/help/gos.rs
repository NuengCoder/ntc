use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gos.title".into(), "GOS คำสั่ง:".into());

    m.insert(
        "help.gos.usage".into(),
        "gos ## แสดงรายการไดเรกทอรีย่อยและเลือกหนึ่ง".into(),
    );

    m.insert(
        "help.gos.example1".into(),
        "gos ## เลือกไดเรกทอรีย่อยแบบโต้ตอบ".into(),
    );
}
