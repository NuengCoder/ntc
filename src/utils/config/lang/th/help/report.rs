use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.report.title".into(), "คำสั่งรายงาน (REPORT):".into());

    m.insert(
        "help.report.project".into(),
        "report project <fmt> [path] --name <file> ## บันทึกรายงานโปรเจกต์ไว้ที่ <file>".into(),
    );

    m.insert(
        "help.report.clip".into(),
        "report clip <n> ## แสดงเนื้อหาคลิปบอร์ดทั้งหมด".into(),
    );

    m.insert(
        "help.report.example1".into(),
        "report project html . --name docs/project ## รายงาน HTML พร้อมต้นไม้ที่พับ/ขยายได้ บันทึกเป็น docs/project.html".into(),
    );

    m.insert("help.report.example2".into(), "report clip 1".into());
}
