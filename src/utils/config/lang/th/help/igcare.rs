use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.igcare.care_dir".into(),
        "care dir <name>[, ...] ## หยุดการละเว้นไดเรกทอรี".into(),
    );

    m.insert(
        "help.igcare.care_ext".into(),
        "care ext <ext>[, ...] ## เพิ่มนามสกุลที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "help.igcare.care_file".into(),
        "care file <name>[, ...] ## เพิ่มไฟล์ที่รองรับเพิ่มเติม".into(),
    );

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## ส่งออกการตั้งค่าทั้งหมดไปยัง <ชื่อ>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## เลือกหมวดหมู่เพื่อส่งออก".into(),
    );

    m.insert(
        "help.igcare.ignore_dir".into(),
        "ignore dir <name>[, ...] ## ละเว้นไดเรกทอรีตามชื่อ".into(),
    );

    m.insert(
        "help.igcare.ignore_ext".into(),
        "ignore ext <ext>[, ...] ## ละเว้นนามสกุลไฟล์".into(),
    );

    m.insert(
        "help.igcare.ignore_file".into(),
        "ignore file <name>[, ...] ## ละเว้นไฟล์เฉพาะ".into(),
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
        "help.igcare.show".into(),
        "igcared ## แสดงการตั้งค่า ignore/care ทั้งหมด".into(),
    );

    m.insert("help.igcare.title".into(), "คำสั่ง IGNORE / CARE:".into());

    m.insert(
        "help.igcare.example1".into(),
        "ignore dir node_modules ## ละเว้น node_modules".into(),
    );

    m.insert(
        "help.igcare.example2".into(),
        "ignore ext .log ## ละเว้นไฟล์ .log".into(),
    );

    m.insert(
        "help.igcare.example3".into(),
        "care dir src ## เลิกละเว้นไดเรกทอรี src".into(),
    );

    m.insert(
        "help.igcare.example4".into(),
        "igcared ## แสดงการตั้งค่า ignore/care ทั้งหมด".into(),
    );
}
