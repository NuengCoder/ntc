use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.back.title".into(), "BACK คำสั่ง:".into());

    m.insert(
        "help.back.usage".into(),
        "back [n] ## กลับไปยังไดเรกทอรีหลัก (หรือ n ระดับ)".into(),
    );

    m.insert(
        "help.back.example1".into(),
        "back ## กลับไปยังไดเรกทอรีหลัก".into(),
    );
}
