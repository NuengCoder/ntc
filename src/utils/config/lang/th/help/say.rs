use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.say.title".into(), "SAY คำสั่ง:".into());

    m.insert(
        "help.say.usage".into(),
        "say <text> [--loop N] [-n|-t] ## พิมพ์ข้อความ (ไม่จัดเส้นทางไฟล์ใหม่)".into(),
    );

    m.insert(
        "help.say.example1".into(),
        "say \"hello\" ## พิมพ์ข้อความ".into(),
    );
}
