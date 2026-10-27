use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=help, l=ไปที่บรรทัด, b=ไดเรกทอรีหลัก, f=ค้นหา".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "/=กรอง, e=ตัวแก้ไข, r=เปลี่ยนชื่อ, d=ลบ".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "n=ไฟล์ใหม่, m=ไดเรกทอรีใหม่, i=สลับการซ่อน".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s/สลับการเรียง, S=เรียงย้อนกลับ, F5=รีเฟรช".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "q/Esc=ออก, เมาส์=คลิก/เลื่อน".into(),
    );

    m.insert(
        "help.explorer.launch".into(),
        "explorer, nfe [PATH] ## เปิดตัวสำรวจไฟล์แบบเต็มหน้าจอ".into(),
    );

    m.insert("help.explorer.title".into(), "ตัวสำรวจไฟล์ (NFE):".into());

    m.insert(
        "help.explorer.example1".into(),
        "explorer ## เปิดตัวสำรวจไฟล์ในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## เปิดตัวสำรวจใน ~/projects".into(),
    );
}
