use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcare.title".into(), "คำสั่ง IGCARE:".into());

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## ส่งออกการตั้งค่าทั้งหมดไปยัง <ชื่อ>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## เลือกหมวดหมู่เพื่อส่งออก".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <file> ## นำเข้าการตั้งค่าจากไฟล์ .ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [name] ## สร้างเทมเพลต igcare ใน CWD".into(),
    );

    m.insert(
        "help.igcare.example1".into(),
        "igcare make ## สร้างเทมเพลต igcare".into(),
    );
}
