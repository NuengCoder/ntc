use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.encdec.cvcode".into(),
        "cvcode <จาก> <ข้อมูล> to <ถึง> ## แปลงระหว่างรูปแบบ".into(),
    );

    m.insert(
        "help.encdec.decode_b2".into(),
        "decode b2 <target> [-o <file>] ## ถอดรหัสไบนารี".into(),
    );

    m.insert(
        "help.encdec.decode_b10".into(),
        "decode b10 <target> [-o <file>] ## ถอดรหัสทศนิยม".into(),
    );

    m.insert(
        "help.encdec.decode_b64".into(),
        "decode b64 <target> [-o <file>] ## ถอดรหัส Base64".into(),
    );

    m.insert(
        "help.encdec.decode_hex".into(),
        "decode hex <target> [-o <file>] ## ถอดรหัส Hex".into(),
    );

    m.insert(
        "help.encdec.decode_url".into(),
        "decode url <target> [-o <file>] ## ถอดรหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert(
        "help.encdec.encode_b2".into(),
        "encode b2 <target> [-o <file>] ## เข้ารหัสไบนารี (0101...)".into(),
    );

    m.insert(
        "help.encdec.encode_b10".into(),
        "encode b10 <target> [-o <file>] ## เข้ารหัสทศนิยม (0-255...)".into(),
    );

    m.insert(
        "help.encdec.encode_b64".into(),
        "encode b64 <target> [-o <file>] ## เข้ารหัส Base64 (ไฟล์หรือข้อความ)".into(),
    );

    m.insert(
        "help.encdec.encode_hex".into(),
        "encode hex <target> [-o <file>] ## เข้ารหัส Hex".into(),
    );

    m.insert(
        "help.encdec.encode_url".into(),
        "encode url <target> [-o <file>] ## เข้ารหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert("help.encdec.title".into(), "คำสั่ง ENCODE/DECODE:".into());

    m.insert(
        "help.encdec.example1".into(),
        "encode b64 \"hello\" ## เข้ารหัสสตริงด้วย Base64".into(),
    );

    m.insert(
        "help.encdec.example2".into(),
        "encode hex file.bin ## เข้ารหัสไฟล์ด้วย Hex".into(),
    );

    m.insert(
        "help.encdec.example3".into(),
        "decode b64 aGVsbG8= ## ถอดรหัส Base64".into(),
    );

    m.insert(
        "help.encdec.example4".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## แปลงระหว่างรูปแบบ".into(),
    );
}
