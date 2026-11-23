use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkd.title".into(), "MKD คำสั่ง:".into());

    m.insert(
        "help.mkd.usage".into(),
        "mkd <dir>[, ...] ## สร้างไดเรกทอรีใหม่".into(),
    );

    m.insert(
        "help.mkd.example1".into(),
        "mkd src/components ## สร้างไดเรกทอรี".into(),
    );
}
