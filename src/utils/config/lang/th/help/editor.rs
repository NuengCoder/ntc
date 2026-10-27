use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.editor.ne".into(),
        "ne, ntceditor <file> ## เปิดไฟล์ที่มีอยู่แล้วในตัวแก้ไขข้อความในตัว".into(),
    );

    m.insert(
        "help.editor.ne_hyper".into(),
        "ne --hyper <ชื่อ> --lang <l> [-I] ## สร้างโปรเจกต์ hyper C/C++ (-I เปิดไดเรกทอรีโปรเจกต์)".into(),
    );

    m.insert(
        "help.editor.ne_init".into(),
        "ne --init <file> [<file> ...] [-I <file>] ## สร้างเทมเพลตไฟล์; -I เปิดไฟล์หนึ่ง".into(),
    );

    m.insert(
        "help.editor.ne_line".into(),
        "ne, ntceditor <file>:<n> ## เปิดไฟล์ที่บรรทัด <n>".into(),
    );

    m.insert(
        "help.editor.ne_super".into(),
        "ne --super <file> [<file> ...] [-I <file>] ## สร้างเทมเพลต super; -I เปิดไฟล์หนึ่ง".into(),
    );

    m.insert(
        "help.editor.ne_welcome".into(),
        "ne, ntceditor ## เปิดหน้าจอต้อนรับในไดเรกทอรีปัจจุบัน".into(),
    );

    m.insert("help.editor.title".into(), "คำสั่งตัวแก้ไข (EDITOR):".into());

    m.insert(
        "help.editor.example1".into(),
        "ne main.rs ## เปิด main.rs ในตัวแก้ไข".into(),
    );

    m.insert(
        "help.editor.example2".into(),
        "ne src/main.rs:42 ## เปิด main.rs ที่บรรทัด 42".into(),
    );

    m.insert(
        "help.editor.example3".into(),
        "ne --init a.py b.html -I a.py ## สร้างเทมเพลตและเปิด a.py".into(),
    );

    m.insert(
        "help.editor.example4".into(),
        "ne --hyper myproject --lang c ## สร้างโปรเจกต์ C hyper".into(),
    );
}
