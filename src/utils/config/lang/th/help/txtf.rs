use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtf.title".into(), "TXTF คำสั่ง:".into());

    m.insert(
        "help.txtf.usage".into(),
        "txtf [--head N|--tail N|--betw A:B] [file|dir|@tp/file|@tp/dir] ## แสดงเนื้อหาไฟล์หรือเลือกไฟล์จากไดเรกทอรี (เลขบรรทัดเดิม)".into(),
    );

    m.insert(
        "help.txtf.example1".into(),
        "txtf main.rs --head 30 ## 30 บรรทัดแรกของ main.rs".into(),
    );

    m.insert(
        "help.txtf.example2".into(),
        "txtf --betw 10:20 main.rs ## บรรทัด 10-20 ของ main.rs".into(),
    );

    m.insert(
        "help.txtf.example3".into(),
        "txtf @docs/notes.txt ## แสดงไฟล์ในจุดเทเลพอร์ต".into(),
    );

    m.insert(
        "help.txtf.example4".into(),
        "txtf src ## เลือกไฟล์ที่รองรับจากไดเรกทอรี src เพื่อแสดง".into(),
    );
}
