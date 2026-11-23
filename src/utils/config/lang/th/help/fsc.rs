use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fsc.title".into(), "FSC คำสั่ง:".into());

    m.insert(
        "help.fsc.usage".into(),
        "fsc <pattern> [-d <n>] [--no-fuzzy] ## ค้นหาไฟล์ เลือกหนึ่งเพื่อแสดงและคัดลอกได้".into(),
    );

    m.insert(
        "help.fsc.example1".into(),
        "fsc config ## ค้นหาไฟล์ที่ตรงและแสดง".into(),
    );
}
