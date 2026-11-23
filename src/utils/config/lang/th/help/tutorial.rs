use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tutorial.title".into(), "TUTORIAL คำสั่ง:".into());

    m.insert(
        "help.tutorial.usage".into(),
        "tutorial [บทเรียน] ## บทเรียนลงมือทำในแซนด์บ็อกซ์ (cd, view, ne, ral, ran)".into(),
    );

    m.insert(
        "help.tutorial.example1".into(),
        "tutorial ## เริ่มบทเรียนแบบโต้ตอบ".into(),
    );
}
