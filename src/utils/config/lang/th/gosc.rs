use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "gosc.calculating_size".into(),
        "กำลังคำนวณขนาดไดเรกทอรี...".into(),
    );

    m.insert("gosc.exiting".into(), "กำลังออกจาก gosc...".into());

    m.insert(
        "gosc.output_zone".into(),
        "─── เขตผลลัพธ์ ───────────────────────────────".into(),
    );

    m.insert(
        "gosc.output_zone_end".into(),
        "──────────────────────────────────────────────".into(),
    );

    m.insert("gosc.panel.back_1".into(), "กลับไป 1 ระดับ".into());

    m.insert("gosc.panel.back_2".into(), "กลับไป 2 ระดับ".into());

    m.insert("gosc.panel.back_n".into(), "กลับไป n ระดับ".into());

    m.insert("gosc.panel.empty".into(), "(ไม่มีไดเรกทอรีย่อย)".into());

    m.insert("gosc.panel.exit".into(), "ออกจาก gosc".into());

    m.insert(
        "gosc.panel.size_care".into(),
        "size --care (ขนาดรวม)".into(),
    );

    m.insert("gosc.panel.title".into(), "gosc — นำทางต่อเนื่อง".into());

    m.insert(
        "gosc.panel.view_care".into(),
        "view --care -d 1 (ไฟล์ + care)".into(),
    );

    m.insert(
        "gosc.panel.view_size_care".into(),
        "view -s --care -d 1 (ไฟล์ + ขนาด + care)".into(),
    );

    m.insert(
        "gosc.press_enter".into(),
        "กด Enter เพื่อดำเนินการต่อ...".into(),
    );

    m.insert("gosc.prompt".into(), "gosc>".into());

    m.insert("gosc.size_bytes".into(), "ไบต์:".into());

    m.insert("gosc.size_human".into(), "มนุษย์:".into());

    m.insert("gosc.size_title".into(), "ขนาดไดเรกทอรีปัจจุบัน".into());
}
