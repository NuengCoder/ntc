use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. ยกเลิก".into());

    m.insert("cmd.image.enter_number".into(), "ป้อนตัวเลข: ".into());

    m.insert(
        "cmd.image.no_ansi".into(),
        "เทอร์มินัลนี้ไม่รองรับสี ANSI จึงแสดง '{}' ในเทอร์มินัลไม่ได้".into(),
    );

    m.insert(
        "cmd.image.no_files".into(),
        "ไม่พบไฟล์ภาพในไดเรกทอรีปัจจุบัน.".into(),
    );

    m.insert("cmd.image.select_image".into(), "เลือกภาพ:".into());

    m.insert(
        "cmd.image.displaying".into(),
        "กำลังแสดง {} ({}x{}) ผ่าน {}".into(),
    );

    m.insert(
        "cmd.image.usage".into(),
        "การใช้งาน: image [display|show] [--sixel] [<เส้นทาง>]".into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "กำลังบังคับเอาต์พุต sixel (ตรวจพบ: {}) — เทอร์มินัลนี้อาจแสดงผลเพี้ยน".into(),
    );
}
