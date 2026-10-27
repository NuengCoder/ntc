use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.clipboard.clear".into(),
        "clip clear ## ล้างประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.list".into(),
        "clip list ## แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.pathcp".into(),
        "pathcp ## คัดลอกเส้นทางไดเรกทอรีปัจจุบันไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.select".into(),
        "clip select <N> ## คัดลอกรายการ N กลับไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.show".into(),
        "clip ## แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert("help.clipboard.title".into(), "คำสั่ง CLIPBOARD:".into());

    m.insert(
        "help.clipboard.example1".into(),
        "clip ## แสดงประวัติคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.example2".into(),
        "clip select 1 ## คัดลอกรายการ #1 กลับไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.clipboard.example3".into(),
        "pathcp ## คัดลอกเส้นทางไดเรกทอรีปัจจุบัน".into(),
    );
}
