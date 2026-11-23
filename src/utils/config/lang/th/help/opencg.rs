use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.opencg.title".into(), "OPENCG คำสั่ง:".into());

    m.insert(
        "help.opencg.usage".into(),
        "opencg ## เปิด config.toml ในตัวแก้ไข".into(),
    );

    m.insert(
        "help.opencg.example1".into(),
        "opencg ## แก้ไข config.toml ในตัวแก้ไข".into(),
    );
}
