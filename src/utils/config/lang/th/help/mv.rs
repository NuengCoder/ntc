use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mv.title".into(), "MV คำสั่ง:".into());

    m.insert(
        "help.mv.usage".into(),
        "mv <src|@tp> <dst|@tp> [-f] ## ย้าย/เปลี่ยนชื่อไฟล์หรือไดเรกทอรี".into(),
    );

    m.insert(
        "help.mv.example1".into(),
        "mv old.txt new.txt ## เปลี่ยนชื่อไฟล์".into(),
    );
}
