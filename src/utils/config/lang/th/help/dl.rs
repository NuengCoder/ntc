use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.dl.title".into(), "DL คำสั่ง:".into());

    m.insert(
        "help.dl.usage".into(),
        "dl <url> ## ดาวน์โหลดไฟล์ (--resume, --stop, --history, --cancel)".into(),
    );

    m.insert(
        "help.dl.example1".into(),
        "dl https://example.com/file.zip ## ดาวน์โหลดพร้อมความคืบหน้า".into(),
    );
}
