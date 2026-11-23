use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cvcode.title".into(), "CVCODE คำสั่ง:".into());

    m.insert(
        "help.cvcode.usage".into(),
        "cvcode <จาก> <ข้อมูล> to <ถึง> ## แปลงระหว่างรูปแบบ".into(),
    );

    m.insert(
        "help.cvcode.example1".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## แปลงระหว่างรูปแบบ".into(),
    );

    m.insert(
        "help.cvcode.example2".into(),
        "cvcode b64 \"aGVsbG8=\" to plaintext ## ถอดรหัสเป็นข้อความ".into(),
    );
}
