use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.locate.title".into(), "LOCATE คำสั่ง:".into());

    m.insert(
        "help.locate.usage".into(),
        "locate <pattern> [-d <n>] [--no-fuzzy] ## ค้นหาทั้งไฟล์และไดเรกทอรี".into(),
    );

    m.insert(
        "help.locate.example1".into(),
        "locate README ## ค้นหาไฟล์และไดเรกทอรีสำหรับ 'README'".into(),
    );
}
