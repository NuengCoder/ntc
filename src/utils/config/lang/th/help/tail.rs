use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tail.title".into(), "TAIL คำสั่ง:".into());

    m.insert(
        "help.tail.usage".into(),
        "tail [N] [file] ## แสดง N บรรทัดสุดท้าย (ค่าเริ่มต้น 10)".into(),
    );

    m.insert(
        "help.tail.example1".into(),
        "tail 20 app.log ## 20 บรรทัดสุดท้ายของ app.log".into(),
    );
}
