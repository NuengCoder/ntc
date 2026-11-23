use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fs.title".into(), "FS คำสั่ง:".into());

    m.insert(
        "help.fs.usage".into(),
        "fs <pattern> [-d <n>] [--no-fuzzy] ## ค้นหาไฟล์ตามชื่อ (สำรองแบบ fuzzy เว้นแต่ใส่ --no-fuzzy)"
            .into(),
    );

    m.insert("help.fs.example1".into(), "fs *.rs ## ค้นหาไฟล์ .rs".into());
}
