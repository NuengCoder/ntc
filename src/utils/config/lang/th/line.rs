use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.line.usage".into(),
        "การใช้งาน: line [-A <คำนำหน้า>] [-B <ตัวคั่น>] [ไฟล์]  (ใส่เลขทุกบรรทัด; อ่านจาก pipe เมื่อไม่มีไฟล์)"
            .into(),
    );

    m.insert(
        "misc.line.bad_arg".into(),
        "line: อาร์กิวเมนต์ไม่คาดคิด '{}' (การใช้งาน: line [-A <คำนำหน้า>] [-B <ตัวคั่น>] [ไฟล์])".into(),
    );
}
