use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.image.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert("cmd.image.default_label".into(), "ค่าเริ่มต้น".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. ยกเลิก".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert(
        "cmd.image.desc_markdown".into(),
        "  {}. Markdown (ค่าเริ่มต้น)".into(),
    );

    m.insert(
        "cmd.image.desc_narrative".into(),
        "  {}. คำอธิบายเชิงบรรยาย (สำหรับ AI)".into(),
    );

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "ป้อนตัวเลข: ".into());

    m.insert(
        "cmd.image.missing_out_format".into(),
        "ไม่พบรูปแบบหลัง 'out'.".into(),
    );

    m.insert(
        "cmd.image.missing_to_format".into(),
        "ไม่พบรูปแบบหลัง 'to'.".into(),
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
        "การใช้งาน: image <เส้นทาง> [display|show [--sixel]|to|out <รูปแบบ>] [--ocr] [--layout]"
            .into(),
    );

    m.insert(
        "cmd.image.sixel_forced_warn".into(),
        "กำลังบังคับเอาต์พุต sixel (ตรวจพบ: {}) — เทอร์มินัลนี้อาจแสดงผลเพี้ยน".into(),
    );
}
