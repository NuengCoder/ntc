use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nav.available_drives".into(), "ไดรฟ์ที่มี:".into());

    m.insert("nav.available_mounts".into(), "เมานต์ที่มี (/mnt):".into());

    m.insert("nav.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert("nav.config_exists".into(), " ไฟล์การกำหนดค่ามีอยู่".into());

    m.insert(
        "nav.drive_prompt".into(),
        "ป้อนตัวอักษรไดรฟ์หรือหมายเลข (หรือ 'cancel'): ".into(),
    );

    m.insert(
        "nav.mount_prompt".into(),
        "ป้อนชื่อเมานต์หรือหมายเลข (หรือ 'cancel'): ".into(),
    );

    m.insert("nav.gos_where".into(), "gos อยู่ที่ไหน?".into());
}
