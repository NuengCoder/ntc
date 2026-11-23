use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.local.title".into(), "LOCAL คำสั่ง:".into());

    m.insert(
        "help.local.init".into(),
        "local init ## สร้างเทมเพลต ntconfig.toml (แบบ comment)".into(),
    );

    m.insert(
        "help.local.help".into(),
        "local help ## แสดงการใช้งานคำสั่ง local".into(),
    );

    m.insert(
        "help.local.deinit".into(),
        "local deinit ## ลบ ntconfig.toml จากไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.local.example1".into(),
        "local init ## สร้างเทมเพลต ntconfig.toml ที่นี่".into(),
    );
}
