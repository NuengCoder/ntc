use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("up.help.examples".into(), "ตัวอย่าง:".into());

    m.insert(
        "up.help.opt_data".into(),
        "  -d, --data <body> อัปโหลดข้อมูลแบบอินไลน์".into(),
    );

    m.insert(
        "up.help.opt_header".into(),
        "  -H, --header <\"Key: Value\"> เพิ่มส่วนหัวคำขอ (ทำซ้ำได้)".into(),
    );

    m.insert(
        "up.help.opt_help".into(),
        "  -h, --help แสดงความช่วยเหลือนี้".into(),
    );

    m.insert("up.help.opt_post".into(), "  --post ใช้ POST แทน PUT".into());

    m.insert("up.help.options".into(), "ตัวเลือก:".into());

    m.insert("up.help.title".into(), "อัปโหลด (up)".into());

    m.insert("up.help.usage".into(), "การใช้งาน:".into());

    m.insert(
        "up.help.usage_data".into(),
        "  up -d \"data\" <url> อัปโหลดข้อมูลข้อความแบบอินไลน์".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        "  up <file> <url> อัปโหลดไฟล์ผ่าน PUT".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        "  up <file> <url> --post อัปโหลดผ่าน POST".into(),
    );
}
