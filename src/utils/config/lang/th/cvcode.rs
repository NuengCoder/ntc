use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cvcode.formats".into(),
        "รูปแบบ: b2, b10, b64 (base64), hex, url".into(),
    );

    m.insert("cvcode.missing_data".into(), "ไม่พบข้อมูลสำหรับแปลง".into());

    m.insert(
        "cvcode.missing_to".into(),
        "ไม่พบรูปแบบเป้าหมายหลัง 'to'".into(),
    );

    m.insert(
        "cvcode.usage".into(),
        "  cvcode <จาก> <ข้อมูล> to <ถึง> [-o <ไฟล์>]".into(),
    );

    m.insert(
        "cvcode.usage_detailed".into(),
        "การใช้งาน: cvcode <จาก> <ข้อมูล> to <ถึง> [-o <ไฟล์>]".into(),
    );
}
