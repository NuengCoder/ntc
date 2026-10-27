use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.theme.add".into(),
        "theme add <name> ## สร้างธีมใหม่ (เปิดในตัวแก้ไข)".into(),
    );

    m.insert(
        "help.theme.current".into(),
        "theme current ## แสดงชื่อและรายละเอียดธีมปัจจุบัน".into(),
    );

    m.insert(
        "help.theme.edit".into(),
        "theme edit [name] ## แก้ไขธีม (หรือเปิดไดเรกทอรีธีม)".into(),
    );

    m.insert(
        "help.theme.export".into(),
        "theme export <name> ## ส่งออกธีมไปยังไฟล์ .ntc_theme".into(),
    );

    m.insert(
        "help.theme.import".into(),
        "theme import <file> ## นำเข้าธีมจากไฟล์ .ntc_theme".into(),
    );

    m.insert(
        "help.theme.info".into(),
        "theme info <name> ## แสดงข้อมูลเมตาและจำนวนสีของธีม".into(),
    );

    m.insert(
        "help.theme.interactive".into(),
        "theme ## ตัวจัดการธีมแบบโต้ตอบ (เมนู)".into(),
    );

    m.insert("help.theme.list".into(), "theme list ## แสดงธีมทั้งหมด".into());

    m.insert(
        "help.theme.make".into(),
        "theme make [name] ## สร้างเทมเพลตธีมใน CWD".into(),
    );

    m.insert(
        "help.theme.reload".into(),
        "theme reload ## โหลดธีมจากดิสก์ใหม่".into(),
    );

    m.insert("help.theme.rm".into(), "theme rm <name> ## ลบธีม".into());

    m.insert(
        "help.theme.rnm".into(),
        "theme rnm <old> to <new> ## เปลี่ยนชื่อธีม".into(),
    );

    m.insert(
        "help.theme.switch".into(),
        "theme <name> ## สลับไปใช้ธีมตามชื่อ".into(),
    );

    m.insert("help.theme.title".into(), "คำสั่งธีม (THEME):".into());

    m.insert(
        "help.theme.where".into(),
        "theme where ## แสดงตำแหน่งไดเรกทอรีธีม".into(),
    );

    m.insert(
        "help.theme.example1".into(),
        "theme list ## แสดงธีมทั้งหมด".into(),
    );

    m.insert(
        "help.theme.example2".into(),
        "theme dark ## สลับไปใช้ธีม 'dark'".into(),
    );

    m.insert(
        "help.theme.example3".into(),
        "theme current ## แสดงข้อมูลธีมปัจจุบัน".into(),
    );

    m.insert(
        "help.theme.example4".into(),
        "theme add mytheme ## สร้างธีมใหม่".into(),
    );
}
