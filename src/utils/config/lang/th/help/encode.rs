use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.encode.title".into(), "ENCODE คำสั่ง:".into());

    m.insert(
        "help.encode.b2".into(),
        "encode b2 <target> [-o <file>] ## เข้ารหัสไบนารี (0101...)".into(),
    );

    m.insert(
        "help.encode.b10".into(),
        "encode b10 <target> [-o <file>] ## เข้ารหัสทศนิยม (0-255...)".into(),
    );

    m.insert(
        "help.encode.b64".into(),
        "encode b64 <target> [-o <file>] ## เข้ารหัส Base64 (ไฟล์หรือข้อความ)".into(),
    );

    m.insert(
        "help.encode.hex".into(),
        "encode hex <target> [-o <file>] ## เข้ารหัส Hex".into(),
    );

    m.insert(
        "help.encode.url".into(),
        "encode url <target> [-o <file>] ## เข้ารหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert(
        "help.encode.utf8".into(),
        "encode utf8 <target> [-o <file>] ## UTF-8 เป็นไบต์ escape (\\xc3\\xa9)".into(),
    );

    m.insert(
        "help.encode.utf16".into(),
        "encode utf16 <target> [-o <file>] ## UTF-16 เป็น escape \\uNNNN (\\u00e9)".into(),
    );

    m.insert(
        "help.encode.ascii".into(),
        "encode ascii <target> [-o <file>] ## ข้อความ ASCII (ผิดพลาดหากเกิน U+007F)".into(),
    );

    m.insert(
        "help.encode.example1".into(),
        "encode b64 \"hello\" ## เข้ารหัสสตริงด้วย Base64".into(),
    );

    m.insert(
        "help.encode.example2".into(),
        "encode hex file.bin ## เข้ารหัสไฟล์ด้วย Hex".into(),
    );
}
