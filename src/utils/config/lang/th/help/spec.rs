use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.spec.title".into(), "SPEC คำสั่ง:".into());

    m.insert(
        "help.spec.usage".into(),
        "spec ## แสดงสเปกคอมพิวเตอร์/ระบบ (CPU, GPU, RAM, พื้นที่เก็บ, OS, เลขซีเรียล)".into(),
    );

    m.insert(
        "help.spec.example1".into(),
        "spec ## แสดงสเปกของระบบ".into(),
    );
}
