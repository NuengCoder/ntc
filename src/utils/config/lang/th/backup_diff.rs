use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup_diff.cancel_opt".into(), "0. ยกเลิก".into());

    m.insert("backup_diff.cancelled".into(), "ยกเลิกการเปรียบเทียบ.".into());

    m.insert(
        "backup_diff.comparing".into(),
        "กำลังเปรียบเทียบโปรเจกต์ปัจจุบันกับแบ็กอัป #{}...".into(),
    );

    m.insert("backup_diff.invalid_input".into(), "อินพุตไม่ถูกต้อง: {}".into());

    m.insert(
        "backup_diff.invalid_selection".into(),
        "ตัวเลือกไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "backup_diff.none".into(),
        "ไม่พบแบ็กอัปสำหรับโปรเจกต์นี้. ใช้ 'bkup' เพื่อสร้าง.".into(),
    );

    m.insert("backup_diff.not_found".into(), "ไม่พบแบ็กอัป #{}.".into());

    m.insert(
        "backup_diff.open_manually".into(),
        "เปิดไฟล์ด้วยตนเอง: {}".into(),
    );

    m.insert(
        "backup_diff.saved_to".into(),
        "บันทึกรายงานการเปรียบเทียบไปยัง: {}".into(),
    );

    m.insert(
        "backup_diff.select_prompt".into(),
        "เลือกแบ็กอัป (1-{}): ".into(),
    );

    m.insert(
        "backup_diff.select_title".into(),
        "\u{1F4CA} เลือกแบ็กอัปเพื่อเปรียบเทียบกับโปรเจกต์ปัจจุบัน".into(),
    );
}
