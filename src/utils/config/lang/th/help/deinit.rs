use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.deinit.title".into(), "DEINIT คำสั่ง:".into());

    m.insert(
        "help.deinit.usage".into(),
        "deinit [ran|local|opencode] ## ลบไฟล์โปรเจกต์ (ถามยืนยันก่อนลบทุกครั้ง)".into(),
    );

    m.insert(
        "help.deinit.example1".into(),
        "deinit ran ## ลบ NTCRANFILE.toml".into(),
    );
}
