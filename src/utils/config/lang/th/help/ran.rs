use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ran.deinit".into(),
        "ran deinit ## ลบ NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.help".into(),
        "ran help ## แสดงการใช้งาน ran".into(),
    );

    m.insert(
        "help.ran.init".into(),
        "ran init ## สร้างเทมเพลต NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.list".into(),
        "ran list ## แสดง target ทั้งหมดใน NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.run".into(),
        "ran ## เรียก target 'standalone' (ค่าเริ่มต้น)".into(),
    );

    m.insert(
        "help.ran.target".into(),
        "ran <target> ## เรียก target เฉพาะจาก NTCRANFILE.toml".into(),
    );

    m.insert("help.ran.title".into(), "คำสั่ง RAN / TASK RUNNER:".into());

    m.insert(
        "help.ran.example1".into(),
        "ran ## เรียก target standalone เริ่มต้น".into(),
    );

    m.insert(
        "help.ran.example2".into(),
        "ran build ## เรียก target 'build'".into(),
    );

    m.insert(
        "help.ran.example3".into(),
        "ran list ## แสดง target ทั้งหมด".into(),
    );
}
