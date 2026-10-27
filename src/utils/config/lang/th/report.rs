use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "report.generating_html".into(),
        "\u{1F4CA} กำลังสร้างรายงาน HTML สำหรับ: {}".into(),
    );

    m.insert(
        "report.generating_json".into(),
        "\u{1F4CA} กำลังสร้างรายงาน JSON สำหรับ: {}".into(),
    );

    m.insert(
        "report.generating_md".into(),
        "\u{1F4CA} กำลังสร้างรายงาน Markdown สำหรับ: {}".into(),
    );

    m.insert(
        "report.saved_html".into(),
        "\u{2705} บันทึกรายงาน HTML ไปที่: {} แล้ว".into(),
    );

    m.insert(
        "report.saved_json".into(),
        "\u{2705} บันทึกรายงาน JSON ไปที่: {} แล้ว".into(),
    );

    m.insert(
        "report.saved_md".into(),
        "\u{2705} บันทึกรายงาน Markdown ไปที่: {} แล้ว".into(),
    );

    m.insert("report.saved_to".into(), "บันทึกรายงานไปที่: {} แล้ว".into());
}
