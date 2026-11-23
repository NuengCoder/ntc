use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.wi.title".into(), "WI คำสั่ง:".into());

    m.insert(
        "help.wi.usage".into(),
        "wi [path] ## ฟีดการเปลี่ยนแปลงไฟล์แบบสด (q=ออก)".into(),
    );

    m.insert(
        "help.wi.example1".into(),
        "wi ## เฝ้าดูการเปลี่ยนแปลงไฟล์แบบสด".into(),
    );
}
