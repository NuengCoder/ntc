use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmd.title".into(), "RNMD คำสั่ง:".into());

    m.insert(
        "help.rnmd.usage".into(),
        "rnmd <old> <new> ## เปลี่ยนชื่อไดเรกทอรี".into(),
    );

    m.insert(
        "help.rnmd.example1".into(),
        "rnmd src lib ## เปลี่ยนชื่อไดเรกทอรี".into(),
    );
}
