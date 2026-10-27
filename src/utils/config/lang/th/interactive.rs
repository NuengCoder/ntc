use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("interactive.cancelled".into(), "ยกเลิกแล้ว.".into());

    m.insert(
        "interactive.invalid_input".into(),
        "ข้อมูลนำเข้าไม่ถูกต้อง.".into(),
    );

    m.insert(
        "interactive.invalid_selection".into(),
        "การเลือกไม่ถูกต้อง.".into(),
    );

    m.insert("interactive.no_items".into(), "(ไม่มีรายการ)".into());
}
