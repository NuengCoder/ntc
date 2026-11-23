use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.restorecg.title".into(), "RESTORECG คำสั่ง:".into());

    m.insert(
        "help.restorecg.usage".into(),
        "restorecg ## คืนค่าการกำหนดค่าจากข้อมูลสำรอง".into(),
    );

    m.insert(
        "help.restorecg.example1".into(),
        "restorecg ## คืนค่าการตั้งค่าจากข้อมูลสำรอง".into(),
    );
}
