use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcared.title".into(), "IGCARED คำสั่ง:".into());

    m.insert(
        "help.igcared.usage".into(),
        "igcared ## แสดงการตั้งค่า ignore/care ทั้งหมด".into(),
    );

    m.insert(
        "help.igcared.example1".into(),
        "igcared ## แสดงการตั้งค่า ignore/care ทั้งหมด".into(),
    );
}
