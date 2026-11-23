use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmf.title".into(), "RMF คำสั่ง:".into());

    m.insert(
        "help.rmf.usage".into(),
        "rmf <file>[, ...] ## ลบไฟล์ (ถามยืนยัน)".into(),
    );

    m.insert("help.rmf.example1".into(), "rmf old.log ## ลบไฟล์".into());
}
