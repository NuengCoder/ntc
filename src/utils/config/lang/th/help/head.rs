use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.head.title".into(), "HEAD คำสั่ง:".into());

    m.insert(
        "help.head.usage".into(),
        "head [N] [file] ## แสดง N บรรทัดแรก (ค่าเริ่มต้น 10)".into(),
    );

    m.insert(
        "help.head.example1".into(),
        "head 20 app.log ## 20 บรรทัดแรกของไฟล์ (หรืออินพุตที่ pipe เข้ามา)".into(),
    );
}
