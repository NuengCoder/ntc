use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkf.title".into(), "MKF คำสั่ง:".into());

    m.insert(
        "help.mkf.usage".into(),
        "mkf <file>[, ...] ## สร้างไฟล์ใหม่".into(),
    );

    m.insert(
        "help.mkf.example1".into(),
        "mkf index.html ## สร้างไฟล์ใหม่".into(),
    );
}
