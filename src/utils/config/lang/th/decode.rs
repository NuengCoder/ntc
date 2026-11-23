use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "รูปแบบ decode ไม่รู้จัก: '{}'. ใช้ b2, b10, b64, hex, url, utf8, utf16, ascii.".into(),
    );

    m.insert(
        "decode.usage_cmd".into(),
        "การใช้งาน: decode {} <เป้าหมาย> [-o <ไฟล์>]".into(),
    );

    m.insert(
        "decode.usage_decode_b2".into(),
        "  decode b2 <เป้าหมาย> [-o <ไฟล์>] ถอดรหัสไบนารี".into(),
    );

    m.insert(
        "decode.usage_decode_b10".into(),
        "  decode b10 <เป้าหมาย> [-o <ไฟล์>] ถอดรหัสทศนิยม".into(),
    );

    m.insert(
        "decode.usage_decode_b64".into(),
        "  decode b64 <เป้าหมาย> [-o <ไฟล์>] ถอดรหัส Base64".into(),
    );

    m.insert(
        "decode.usage_decode_hex".into(),
        "  decode hex <เป้าหมาย> [-o <ไฟล์>] ถอดรหัส Hex".into(),
    );

    m.insert(
        "decode.usage_decode_url".into(),
        "  decode url <เป้าหมาย> [-o <ไฟล์>] ถอดรหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert(
        "decode.usage_decode_utf8".into(),
        "  decode utf8 <เป้าหมาย> [-o <ไฟล์>] ไบต์ escape เป็น UTF-8 (\\xc3\\xa9)".into(),
    );

    m.insert(
        "decode.usage_decode_utf16".into(),
        "  decode utf16 <เป้าหมาย> [-o <ไฟล์>] escape \\uNNNN เป็นข้อความ (\\u00e9)".into(),
    );

    m.insert(
        "decode.usage_decode_ascii".into(),
        "  decode ascii <เป้าหมาย> [-o <ไฟล์>] ข้อความ escape เป็นไบต์ ASCII".into(),
    );
}
