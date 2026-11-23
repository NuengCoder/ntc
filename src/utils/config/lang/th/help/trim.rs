use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trim.title".into(), "TRIM คำสั่ง:".into());

    m.insert(
        "help.trim.usage".into(),
        "trim <pattern> [ข้อความ] ## ลบทุกครั้งที่พบ <pattern> (จากข้อความหรืออินพุตไปป์)".into(),
    );

    m.insert(
        "help.trim.example1".into(),
        "echo nice ... bro ... | trim \" ...\" ## ตัดแพตเทิร์นออกจากข้อความที่ส่งผ่านไปป์ (แบบอินไลน์: trim \" ...\" \"nice ... bro ...\")".into(),
    );
}
