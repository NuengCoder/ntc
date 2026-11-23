use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.resetcg.title".into(), "RESETCG คำสั่ง:".into());

    m.insert(
        "help.resetcg.usage".into(),
        "resetcg ## รีเซ็ตการกำหนดค่าเป็นค่าเริ่มต้น (มีตัวเลือกสำรอง)".into(),
    );

    m.insert(
        "help.resetcg.example1".into(),
        "resetcg ## รีเซ็ตการตั้งค่าเป็นค่าเริ่มต้น".into(),
    );
}
