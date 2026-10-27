use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.viewd.title".into(),
        "คำสั่ง VIEWD (เฉพาะไดเรกทอรี):".into(),
    );

    m.insert(
        "help.viewd.usage".into(),
        "viewd [<dir|@tp|@tp/dir/>] [-d N] ## แสดงต้นไม้เฉพาะไดเรกทอรี".into(),
    );

    m.insert(
        "help.viewd.depth".into(),
        "viewd -d, --depth <n> ## แสดงต้นไม้ด้วยความลึกกำหนดเอง".into(),
    );

    m.insert(
        "help.viewd.size".into(),
        "viewd -s, --size [ขนาดขั้นต่ำ] ## แสดงขนาดไดเรกทอรี; ใส่ค่าจะซ่อนที่เล็กกว่า".into(),
    );

    m.insert(
        "help.viewd.care".into(),
        "viewd -c, --care ## นับทุกอย่างเมื่อคำนวณขนาด".into(),
    );

    m.insert(
        "help.viewd.sort".into(),
        "viewd --sort size|char|ext ## เรียงรายการไดเรกทอรี".into(),
    );

    m.insert(
        "help.viewd.time".into(),
        "viewd -t, --time ## แสดงเวลาดำเนินการ".into(),
    );

    m.insert(
        "help.viewd.copy".into(),
        "viewd -cp, --copy ## คัดลอกต้นไม้ไดเรกทอรีไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.viewd.example1".into(),
        "viewd ## แสดงเฉพาะไดเรกทอรีในต้นไม้ปัจจุบัน".into(),
    );

    m.insert(
        "help.viewd.example2".into(),
        "viewd -d 2 --size ## สองระดับพร้อมขนาดไดเรกทอรี".into(),
    );

    m.insert(
        "help.viewd.example3".into(),
        "viewd --sort size ## ไดเรกทอรีที่ใหญ่ที่สุดก่อน".into(),
    );

    m.insert(
        "help.viewd.example4".into(),
        "viewd @docs/ ## แสดงไดเรกทอรีของจุดเทเลพอร์ต".into(),
    );
}
