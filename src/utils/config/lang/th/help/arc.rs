use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.arc.title".into(), "คำสั่ง ARCHIVE:".into());

    m.insert(
        "help.arc.create".into(),
        "arc create <out.zip|tar.gz|tar> <ไฟล์...> ## สร้างไฟล์บีบอัด".into(),
    );

    m.insert(
        "help.arc.extract".into(),
        "arc extract <ไฟล์> [ปลายทาง] ## แตกไฟล์".into(),
    );

    m.insert("help.arc.list".into(), "arc list <ไฟล์> ## แสดงเนื้อหา".into());

    m.insert(
        "help.arc.example1".into(),
        "arc create backup.zip src Cargo.toml ## สร้าง zip".into(),
    );

    m.insert(
        "help.arc.example2".into(),
        "arc list backup.zip ## แสดงเนื้อหา".into(),
    );

    m.insert(
        "help.arc.example3".into(),
        "arc extract backup.zip restored ## แตกไฟล์".into(),
    );
}
