use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ne.title".into(), "คำสั่งตัวแก้ไข (EDITOR):".into());

    m.insert(
        "help.ne.open".into(),
        "ne, ntceditor <file> ## เปิดไฟล์ที่มีอยู่แล้วในตัวแก้ไขข้อความในตัว".into(),
    );

    m.insert(
        "help.ne.line".into(),
        "ne, ntceditor <file>:<n> ## เปิดไฟล์ที่บรรทัด <n>".into(),
    );

    m.insert(
        "help.ne.welcome".into(),
        "ne, ntceditor ## เปิดหน้าจอต้อนรับในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert(
        "help.ne.init".into(),
        "ne --init <file> [<file> ...] [-I <file>] ## สร้างเทมเพลตไฟล์; -I เปิดไฟล์หนึ่ง".into(),
    );

    m.insert(
        "help.ne.super".into(),
        "ne --super <file> [<file> ...] [-I <file>] ## สร้างเทมเพลต super; -I เปิดไฟล์หนึ่ง".into(),
    );

    m.insert(
        "help.ne.example1".into(),
        "ne main.rs ## เปิด main.rs ในตัวแก้ไข".into(),
    );

    m.insert(
        "help.ne.example2".into(),
        "ne src/main.rs:42 ## เปิด main.rs ที่บรรทัด 42".into(),
    );

    m.insert(
        "help.ne.example3".into(),
        "ne --init a.py b.html -I a.py ## สร้างเทมเพลตและเปิด a.py".into(),
    );

    m.insert(
        "help.ne.example4".into(),
        "ne --hyper myproject --lang c ## สร้างโปรเจกต์ C hyper".into(),
    );
}
