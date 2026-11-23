use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ds.title".into(), "DS คำสั่ง:".into());

    m.insert(
        "help.ds.usage".into(),
        "ds <pattern> [-d <n>] [--no-fuzzy] ## ค้นหาไดเรกทอรีตามชื่อ (--no-fuzzy ปิดการค้นหาแบบ fuzzy)"
            .into(),
    );

    m.insert(
        "help.ds.example1".into(),
        "ds src ## ค้นหาไดเรกทอรีชื่อ 'src'".into(),
    );
}
