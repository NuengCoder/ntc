use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.autoup.add".into(),
        "autoup add <ชื่อ> \"<cmd>\" ## เพิ่มรายการเริ่มต้นใน HKCU".into(),
    );

    m.insert(
        "help.autoup.disable".into(),
        "autoup disable <ชื่อ> ## ปิดใช้งานรายการ (ใช้ StartupApproved)".into(),
    );

    m.insert(
        "help.autoup.enable".into(),
        "autoup enable <ชื่อ> ## เปิดใช้งานรายการที่ปิดไว้".into(),
    );

    m.insert(
        "help.autoup.list".into(),
        "autoup list ## แสดงรายการเริ่มต้นทั้งหมด (CU/LM/SF)".into(),
    );

    m.insert(
        "help.autoup.rm".into(),
        "autoup rm <ชื่อ> ## ลบรายการเริ่มต้น".into(),
    );

    m.insert(
        "help.autoup.sources".into(),
        "[CU]=HKCU [LM]=HKLM [SF]=โฟลเดอร์ Startup".into(),
    );

    m.insert("help.autoup.title".into(), "ตัวจัดการเริ่มต้น (Windows):".into());

    m.insert(
        "help.autoup.example1".into(),
        "autoup list ## แสดงรายการเริ่มต้น".into(),
    );

    m.insert(
        "help.autoup.example2".into(),
        "autoup add ntc \"C:\\ntc\\ntc.exe\" ## เพิ่มไปยังการเริ่มต้น".into(),
    );

    m.insert(
        "help.autoup.example3".into(),
        "autoup rm ntc ## ลบรายการเริ่มต้น".into(),
    );

    m.insert(
        "help.autoup.example4".into(),
        "autoup disable ntc ## ปิดใช้งานรายการเริ่มต้น".into(),
    );
}
