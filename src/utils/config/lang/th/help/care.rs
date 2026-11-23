use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.care.title".into(), "CARE คำสั่ง:".into());

    m.insert(
        "help.care.care_dir".into(),
        "care dir <name>[, ...] ## หยุดการละเว้นไดเรกทอรี".into(),
    );

    m.insert(
        "help.care.care_ext".into(),
        "care ext <ext>[, ...] ## เพิ่มนามสกุลที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "help.care.care_file".into(),
        "care file <name>[, ...] ## เพิ่มไฟล์ที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "help.care.example1".into(),
        "care dir src ## เลิกละเว้นไดเรกทอรี src".into(),
    );
}
