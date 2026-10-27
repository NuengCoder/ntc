use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.other.clear".into(), "clear ## ล้างหน้าจอ".into());

    m.insert(
        "help.other.deinit".into(),
        "deinit [--ran|--local] ## ลบไฟล์โปรเจกต์".into(),
    );

    m.insert("help.other.exit".into(), "exit, quit ## ออกจาก ntc".into());

    m.insert("help.other.help".into(), "help ## แสดงความช่วยเหลือนี้".into());

    m.insert("help.other.init".into(), "init [--ran|--local|--opencode] ## เริ่มต้นไฟล์โปรเจกต์ (NTCRANFILE.toml, ntconfig.toml, .opencode/)".into());

    m.insert(
        "help.other.print".into(),
        "print <text> [--loop N] [-n|-t] ## alias ของ say".into(),
    );

    m.insert(
        "help.other.say".into(),
        "say <text> [--loop N] [-n|-t] ## พิมพ์ข้อความ (ไม่จัดเส้นทางไฟล์ใหม่)".into(),
    );

    m.insert("help.other.title".into(), "คำสั่งอื่น ๆ (OTHER):".into());

    m.insert(
        "help.other.tutorial".into(),
        "tutorial ## เริ่มบทเรียนแบบทีละขั้นตอนแบบโต้ตอบ".into(),
    );

    m.insert("help.other.ui".into(), "ui ## แสดงโหมด UI ปัจจุบัน".into());

    m.insert("help.other.version".into(), "version ## แสดงเวอร์ชัน".into());

    m.insert(
        "help.other.example1".into(),
        "init --ran ## เริ่มต้น NTCRANFILE.toml".into(),
    );

    m.insert("help.other.example2".into(), "clear ## ล้างหน้าจอ".into());

    m.insert(
        "help.other.example3".into(),
        "version ## แสดงข้อมูลเวอร์ชัน".into(),
    );

    m.insert(
        "help.other.example4".into(),
        "tutorial ## เริ่มบทเรียนแบบโต้ตอบ".into(),
    );
}
