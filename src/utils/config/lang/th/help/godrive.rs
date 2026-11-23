use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.godrive.title".into(), "GODRIVE คำสั่ง:".into());

    m.insert(
        "help.godrive.usage".into(),
        "godrive ## แสดงรายการและเลือกไดรฟ์ (Windows) หรือเมานต์ใน /mnt (Linux)".into(),
    );

    m.insert(
        "help.godrive.example1".into(),
        "godrive ## เลือกไดรฟ์หรือเมานต์".into(),
    );

    m.insert(
        "help.godrive.example2".into(),
        "godrive c ## ไปที่ไดรฟ์ C: บน Windows หรือ /mnt/c บน Linux".into(),
    );
}
