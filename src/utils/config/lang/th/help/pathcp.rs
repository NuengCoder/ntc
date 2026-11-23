use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pathcp.title".into(), "PATHCP คำสั่ง:".into());

    m.insert(
        "help.pathcp.usage".into(),
        "pathcp ## คัดลอกเส้นทางไดเรกทอรีปัจจุบันไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.pathcp.example1".into(),
        "pathcp ## คัดลอกเส้นทางไดเรกทอรีปัจจุบัน".into(),
    );
}
