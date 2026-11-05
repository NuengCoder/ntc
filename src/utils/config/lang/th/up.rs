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

    m.insert(
        "up.help.opt_post".into(),
        "  --post ใช้ POST (ค่าเริ่มต้นของการอัปโหลด)".into(),
    );

    m.insert("up.help.options".into(), "ตัวเลือก:".into());

    m.insert("up.help.title".into(), "อัปโหลด (up)".into());

    m.insert("up.help.usage".into(), "การใช้งาน:".into());

    m.insert(
        "up.help.usage_data".into(),
        "  up -d \"data\" <url> อัปโหลดข้อมูลข้อความแบบอินไลน์".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        "  up <file> <url> อัปโหลดไฟล์แบบ multipart POST".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        "  up <file> <url> --put --raw ส่งไบต์ดิบด้วย PUT".into(),
    );

    m.insert(
        "up.missing_auth".into(),
        "ไม่มีข้อมูลรับรองหลัง -a (รูปแบบ user:pass)".into(),
    );

    m.insert("up.missing_bearer".into(), "ไม่มีโทเค็นหลัง --bearer".into());

    m.insert(
        "up.missing_form".into(),
        "ไม่มีฟิลด์หลัง -F (รูปแบบ name=value)".into(),
    );

    m.insert(
        "up.invalid_form".into(),
        "ฟิลด์ฟอร์มไม่ถูกต้อง: {} (รูปแบบ name=value)".into(),
    );

    m.insert(
        "up.missing_timeout".into(),
        "ไม่มีจำนวนวินาทีหลัง --timeout".into(),
    );

    m.insert("up.invalid_timeout".into(), "เวลาหมดอายุไม่ถูกต้อง: {}".into());

    m.insert(
        "up.conflict".into(),
        "ใช้ไฟล์หรือข้อมูล -d อย่างใดอย่างหนึ่งเท่านั้น".into(),
    );

    m.insert(
        "up.fallback_retry".into(),
        "เซิร์ฟเวอร์ตอบ {} - กำลังลองใหม่ด้วย {}".into(),
    );

    m.insert(
        "up.help.opt_auth".into(),
        "  -a, --auth <user:pass> ข้อมูลรับรอง Basic auth".into(),
    );

    m.insert(
        "up.help.opt_bearer".into(),
        "  --bearer <token> โทเค็น Bearer".into(),
    );

    m.insert(
        "up.help.opt_form".into(),
        "  -F, --form <name=value> ฟิลด์ multipart เพิ่มเติม (ทำซ้ำได้)".into(),
    );

    m.insert(
        "up.help.opt_raw".into(),
        "  --raw ส่งไบต์ของไฟล์เป็นเนื้อหาคำขอ".into(),
    );

    m.insert(
        "up.help.opt_put".into(),
        "  --put ส่ง PUT แบบดิบแทน multipart POST".into(),
    );

    m.insert(
        "up.help.opt_timeout".into(),
        "  --timeout <วินาที> เวลาหมดอายุคำขอ (ค่าเริ่มต้น 300)".into(),
    );

    m.insert(
        "up.help.opt_no_fallback".into(),
        "  --no-fallback ไม่ลองด้วยวิธีขนส่งอื่น".into(),
    );
}
