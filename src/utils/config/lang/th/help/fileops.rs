use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fileops.cp".into(),
        "cp <src|@tp> <dst|@tp> [-f] ## คัดลอกไฟล์หรือไดเรกทอรี".into(),
    );

    m.insert(
        "help.fileops.mkd".into(),
        "mkd <dir>[, ...] ## สร้างไดเรกทอรีใหม่".into(),
    );

    m.insert(
        "help.fileops.mkf".into(),
        "mkf <file>[, ...] ## สร้างไฟล์ใหม่".into(),
    );

    m.insert(
        "help.fileops.rmd".into(),
        "rmd <dir>[, ...] ## ลบไดเรกทอรี; wildcard ลบทุกที่ตรงกันหลังยืนยัน".into(),
    );

    m.insert(
        "help.fileops.rmf".into(),
        "rmf <file>[, ...] ## ลบไฟล์ (ถามยืนยัน)".into(),
    );

    m.insert(
        "help.fileops.rnmd".into(),
        "rnmd <old> <new> ## เปลี่ยนชื่อไดเรกทอรี".into(),
    );

    m.insert(
        "help.fileops.rnmf".into(),
        "rnmf <old> <new> ## เปลี่ยนชื่อไฟล์".into(),
    );

    m.insert(
        "help.fileops.mv".into(),
        "mv <src|@tp> <dst|@tp> [-f] ## ย้าย/เปลี่ยนชื่อไฟล์หรือไดเรกทอรี".into(),
    );

    m.insert(
        "help.fileops.title".into(),
        "การจัดการไฟล์ (FILE OPERATIONS):".into(),
    );

    m.insert(
        "help.fileops.example1".into(),
        "mkf index.html ## สร้างไฟล์ใหม่".into(),
    );

    m.insert(
        "help.fileops.example2".into(),
        "mkd src/components ## สร้างไดเรกทอรี".into(),
    );

    m.insert(
        "help.fileops.example3".into(),
        "cp file.txt backup.txt ## คัดลอกไฟล์".into(),
    );
}
