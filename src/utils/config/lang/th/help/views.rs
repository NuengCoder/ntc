use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.views.title".into(),
        "คำสั่ง VIEWS (รายการแบนสั้น):".into(),
    );

    m.insert(
        "help.views.usage".into(),
        "views [<dir|@tp|@tp/dir/>] ## แสดงหนึ่งระดับของไดเรกทอรี แบบแบนเหมือน ls".into(),
    );

    m.insert(
        "help.views.size".into(),
        "views -s, --size [ขนาดขั้นต่ำ] ## แสดงขนาด; ใส่ค่าจะซ่อนรายการที่เล็กกว่า (1024, 10KB, 5MB)".into(),
    );

    m.insert(
        "help.views.sort".into(),
        "views --sort <name|ext|size> ## เรียงรายการ (ค่าเริ่มต้น: name)".into(),
    );

    m.insert(
        "help.views.newline".into(),
        "views -n ## หนึ่งรายการต่อบรรทัดแทนการจัดกลุ่มแบบไหลต่อเนื่อง".into(),
    );

    m.insert(
        "help.views.note".into(),
        "ไม่มีแท็ก [Directory]/[ignored] - สีบอกชนิดของรายการ ความลึกคงที่ที่ 1 และคำนวณขนาดด้วยความหมายของ --care เสมอ ใช้ --size เพื่อแสดงขนาด".into(),
    );

    m.insert(
        "help.views.example1".into(),
        "views ## แสดงรายการแบนของไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.views.example2".into(),
        "views src --size 1KB --sort size ## รายการ >= 1KB ใหญ่สุดก่อน แสดงขนาด".into(),
    );
}
