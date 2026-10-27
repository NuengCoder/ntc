use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.size.care".into(),
        "size --care ## คำนวณขนาดรวมทุกอย่าง (โหมด care)".into(),
    );

    m.insert(
        "help.size.path".into(),
        "size -p, --path <path|@tp> ## แสดงขนาดของไฟล์ เส้นทาง หรือจุดเทเลพอร์ตที่ระบุ".into(),
    );

    m.insert(
        "help.size.size".into(),
        "size ## แสดงขนาดไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.size.time".into(),
        "size -t, --time ## แสดงเวลาที่ใช้".into(),
    );

    m.insert("help.size.title".into(), "คำสั่งขนาด (SIZE):".into());

    m.insert(
        "help.size.example1".into(),
        "size ## แสดงขนาดไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.size.example2".into(),
        "size ../src ## แสดงขนาดของไดเรกทอรีอื่น".into(),
    );

    m.insert(
        "help.size.example3".into(),
        "size Cargo.toml ## แสดงขนาดของไฟล์".into(),
    );

    m.insert(
        "help.size.example4".into(),
        "size @web --care -t ## โหมด care พร้อมเวลาที่ใช้สำหรับจุดเทเลพอร์ต".into(),
    );
}
