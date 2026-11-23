use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.decode.title".into(), "DECODE คำสั่ง:".into());

    m.insert(
        "help.decode.b2".into(),
        "decode b2 <target> [-o <file>] ## ถอดรหัสไบนารี".into(),
    );

    m.insert(
        "help.decode.b10".into(),
        "decode b10 <target> [-o <file>] ## ถอดรหัสทศนิยม".into(),
    );

    m.insert(
        "help.decode.b64".into(),
        "decode b64 <target> [-o <file>] ## ถอดรหัส Base64".into(),
    );

    m.insert(
        "help.decode.hex".into(),
        "decode hex <target> [-o <file>] ## ถอดรหัส Hex".into(),
    );

    m.insert(
        "help.decode.url".into(),
        "decode url <target> [-o <file>] ## ถอดรหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert(
        "help.decode.example1".into(),
        "decode b64 aGVsbG8= ## ถอดรหัส Base64".into(),
    );

    m.insert(
        "help.decode.utf8".into(),
        "decode utf8 <target> [-o <file>] ## ไบต์ escape เป็น UTF-8".into(),
    );

    m.insert(
        "help.decode.utf16".into(),
        "decode utf16 <target> [-o <file>] ## escape \\uNNNN เป็นข้อความ".into(),
    );

    m.insert(
        "help.decode.ascii".into(),
        "decode ascii <target> [-o <file>] ## ข้อความ escape เป็นไบต์ ASCII".into(),
    );
}
