use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showcg.title".into(), "SHOWCG คำสั่ง:".into());

    m.insert(
        "help.showcg.usage".into(),
        "showcg ## แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );

    m.insert(
        "help.showcg.example1".into(),
        "showcg ## แสดงภาพรวมการกำหนดค่าปัจจุบัน".into(),
    );
}
