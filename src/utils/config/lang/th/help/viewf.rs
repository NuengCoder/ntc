use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.viewf.title".into(), "คำสั่ง VIEWF (เฉพาะไฟล์):".into());

    m.insert(
        "help.viewf.tree".into(),
        "viewf [<dir|@tp|@tp/dir/>] ## แสดงต้นไม้เฉพาะไฟล์ที่ความลึก 1".into(),
    );

    m.insert("help.viewf.file_usage".into(), "viewf <file|@tp/file|@tp/dir/file> [--head N|--tail N|--betw A:B] ## แสดงเนื้อหาไฟล์ (เหมือน txtf)".into());

    m.insert(
        "help.viewf.size".into(),
        "viewf -s, --size [ขนาดขั้นต่ำ] ## แสดงขนาดไฟล์; ใส่ค่าจะซ่อนไฟล์ที่เล็กกว่า".into(),
    );

    m.insert(
        "help.viewf.sort".into(),
        "viewf --sort size|char|ext ## เรียงรายการไฟล์".into(),
    );

    m.insert(
        "help.viewf.ext".into(),
        "viewf -e, --ext <นามสกุล...> ## แสดงเฉพาะนามสกุลที่ตรง".into(),
    );

    m.insert(
        "help.viewf.care".into(),
        "viewf -c, --care ## นับทุกอย่างเมื่อคำนวณขนาด".into(),
    );

    m.insert(
        "help.viewf.time".into(),
        "viewf -t, --time ## แสดงเวลาดำเนินการ".into(),
    );

    m.insert(
        "help.viewf.copy".into(),
        "viewf -cp, --copy ## คัดลอกรายการไฟล์ไปยังคลิปบอร์ด".into(),
    );

    m.insert(
        "help.viewf.example1".into(),
        "viewf ## แสดงไฟล์ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.viewf.example2".into(),
        "viewf src ## แสดงไฟล์ใน src".into(),
    );

    m.insert(
        "help.viewf.example3".into(),
        "viewf -e rs,txt --sort size ## กรองนามสกุลและเรียงตามขนาด".into(),
    );

    m.insert(
        "help.viewf.example4".into(),
        "viewf @docs/readme.md --head 20 ## แสดงไฟล์ในจุดเทเลพอร์ต".into(),
    );
}
