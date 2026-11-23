use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtc.title".into(), "TXTC คำสั่ง:".into());

    m.insert(
        "help.txtc.usage".into(),
        "txtc [--line|--no-line] <ไฟล์> ## คัดลอกเนื้อหาไปยังคลิปบอร์ด (เปิด/ปิดเลขบรรทัด)".into(),
    );

    m.insert(
        "help.txtc.example1".into(),
        "txtc main.rs ## คัดลอก main.rs ไปยังคลิปบอร์ด".into(),
    );
}
