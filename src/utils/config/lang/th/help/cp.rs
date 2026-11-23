use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cp.title".into(), "CP คำสั่ง:".into());

    m.insert(
        "help.cp.usage".into(),
        "cp <src|@tp> <dst|@tp> [-f] ## คัดลอกไฟล์หรือไดเรกทอรี".into(),
    );

    m.insert(
        "help.cp.example1".into(),
        "cp file.txt backup.txt ## คัดลอกไฟล์".into(),
    );
}
