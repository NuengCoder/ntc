use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "check.help".into(),
        " check [ไฟล์] - ตีความ {} และรายงานทุกปัญหา".into(),
    );

    m.insert("check.hint".into(), "คำแนะนำ".into());

    m.insert("check.no_files".into(), "ไม่พบไฟล์ {} ใน {}".into());

    m.insert("check.not_found".into(), "ไม่พบ {}".into());

    m.insert("check.ok".into(), "{}: ไม่พบปัญหา".into());

    m.insert("check.read_fail".into(), "อ่าน {} ไม่ได้: {}".into());

    m.insert("check.summary".into(), "พบ {} ปัญหาใน {}".into());

    m.insert("check.title".into(), "ตรวจสอบ {}".into());
}
