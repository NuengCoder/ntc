use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clear.title".into(), "CLEAR คำสั่ง:".into());

    m.insert("help.clear.usage".into(), "clear ## ล้างหน้าจอ".into());

    m.insert("help.clear.example1".into(), "clear ## ล้างหน้าจอ".into());
}
