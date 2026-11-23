use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fgo.title".into(), "FGO คำสั่ง:".into());

    m.insert(
        "help.fgo.usage".into(),
        "fgo <pattern> [-d <n>] ## ค้นหาไฟล์ นำทางไปยังไดเรกทอรีหลักของไฟล์ที่เลือก".into(),
    );

    m.insert(
        "help.fgo.example1".into(),
        "fgo *.rs ## ค้นหาไฟล์ Rust แล้วกระโดดไปที่ไฟล์นั้น".into(),
    );
}
