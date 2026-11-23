use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.help.title".into(), "HELP คำสั่ง:".into());

    m.insert("help.help.usage".into(), "help ## แสดงความช่วยเหลือนี้".into());

    m.insert(
        "help.help.example1".into(),
        "help cd ## แสดงวิธีใช้ของคำสั่งเดียว".into(),
    );
    m.insert(
        "help.help.command".into(),
        "help <คำสั่ง> ## แสดงคำอธิบายของคำสั่งเดียว".into(),
    );
    m.insert(
        "help.help.all".into(),
        "help --all ## พิมพ์คำอธิบายของทุกคำสั่ง".into(),
    );
}
