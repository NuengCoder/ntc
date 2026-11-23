use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rnmf.title".into(), "RNMF คำสั่ง:".into());

    m.insert(
        "help.rnmf.usage".into(),
        "rnmf <old> <new> ## เปลี่ยนชื่อไฟล์".into(),
    );

    m.insert(
        "help.rnmf.example1".into(),
        "rnmf a.txt b.txt ## เปลี่ยนชื่อไฟล์".into(),
    );
}
