use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "กำลังสร้างรายงาน HTML สำหรับ: {}".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "กำลังสร้างรายงาน JSON สำหรับ: {}".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "กำลังสร้างรายงาน Markdown สำหรับ: {}".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "บันทึกรายงาน HTML ไปที่: {} แล้ว".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "บันทึกรายงาน JSON ไปที่: {} แล้ว".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "บันทึกรายงาน Markdown ไปที่: {} แล้ว".into(),
    );

    m.insert("report.saved_to".into(), "บันทึกรายงานไปที่: {} แล้ว".into());
}
