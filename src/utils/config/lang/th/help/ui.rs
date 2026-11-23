use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ui.title".into(), "UI คำสั่ง:".into());

    m.insert("help.ui.usage".into(), "ui ## แสดงโหมด UI ปัจจุบัน".into());

    m.insert("help.ui.example1".into(), "ui ## แสดงโหมด UI ปัจจุบัน".into());
}
