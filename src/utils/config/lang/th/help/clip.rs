use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clip.title".into(), "คำสั่ง CLIPBOARD:".into());

    m.insert("help.clip.show".into(), "clip ## แสดงประวัติคลิปบอร์ด".into());

    m.insert(
        "help.clip.list".into(),
        "clip list ## แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clip.select".into(),
        "clip select <N> ## คัดลอกรายการ N กลับไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clip.clear".into(),
        "clip clear ## ล้างประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clip.example1".into(),
        "clip ## แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clip.example2".into(),
        "clip select 1 ## คัดลอกรายการ #1 กลับไปยังคลิปบอร์ด".into(),
    );
}
