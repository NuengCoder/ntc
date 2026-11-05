use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=help, b=ไดเรกทอรีหลัก, f=ค้นหา (แสดงเส้นทางผลลัพธ์)".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "g/:=คำสั่ง (file|dir|line), /=กรองทันที, e=ตัวแก้ไข".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "r=เปลี่ยนชื่อ, d=ลบ, n=ไฟล์ใหม่, m=ไดเรกทอรีใหม่, i=ซ่อน/แสดง".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s=โหมดเรียง, S=ทิศทางการเรียง (รวมไดเรกทอรี), F5=รีเฟรช".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "ค้นหา: พิมพ์=คำค้น, Tab=hjkl/ลูกศร, Enter=เปิด, Esc=กลับ".into(),
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
