use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "encode.binary_hint".into(),
        "ใช้ -o <file> เพื่อบันทึกผลลัพธ์แบบไบนารี.".into(),
    );

    m.insert(
        "encode.binary_warning".into(),
        "ผลลัพธ์เป็นข้อมูลไบนารี. แสดงตัวอย่าง hex:".into(),
    );

    m.insert(
        "encode.hex_even".into(),
        "ความยาวของสตริง hex ต้องเป็นเลขคู่".into(),
    );

    m.insert(
        "encode.input_must_be_text".into(),
        "ข้อมูลนำเข้าต้องเป็นข้อความ".into(),
    );

    m.insert(
        "encode.input_not_text".into(),
        "ข้อมูลนำเข้า {} ต้องเป็นข้อความ (ไม่ใช่ไบนารี)".into(),
    );

    m.insert("encode.invalid_b2".into(), "สตริงไบนารีไม่ถูกต้อง: '{}'".into());

    m.insert("encode.invalid_b10".into(), "ไบต์ทศนิยมไม่ถูกต้อง: '{}'".into());

    m.insert(
        "encode.invalid_b64".into(),
        "ข้อมูล base64 ไม่ถูกต้อง: {}".into(),
    );

    m.insert(
        "encode.invalid_hex_pct".into(),
        "hex ไม่ถูกต้องในการเข้ารหัสเปอร์เซ็นต์: %{}".into(),
    );

    m.insert(
        "encode.invalid_hex_pos".into(),
        "อักขระ hex ไม่ถูกต้องที่ตำแหน่ง {}".into(),
    );

    m.insert(
        "encode.invalid_pct".into(),
        "การเข้ารหัสเปอร์เซ็นต์ไม่ถูกต้อง".into(),
    );

    m.insert(
        "encode.truncated_pct".into(),
        "การเข้ารหัสเปอร์เซ็นต์ถูกตัดทอน".into(),
    );

    m.insert(
        "encode.unknown_format".into(),
        "รูปแบบ encode ไม่รู้จัก: '{}'. ใช้ b2, b10, b64, hex หรือ url.".into(),
    );

    m.insert(
        "encode.unknown_format_fmt".into(),
        "(รูปแบบไม่รู้จัก: {})".into(),
    );

    m.insert(
        "encode.unknown_format_generic".into(),
        "ไม่รู้จักรูปแบบ: '{}'. ใช้ b2, b10, b64, hex หรือ url.".into(),
    );

    m.insert(
        "encode.usage_cmd".into(),
        "การใช้งาน: encode {} <เป้าหมาย> [-o <ไฟล์>]".into(),
    );

    m.insert(
        "encode.usage_encode_b2".into(),
        "  encode b2 <เป้าหมาย> [-o <ไฟล์>] เข้ารหัสไบนารี (0101...)".into(),
    );

    m.insert(
        "encode.usage_encode_b10".into(),
        "  encode b10 <เป้าหมาย> [-o <ไฟล์>] เข้ารหัสทศนิยม (0-255...)".into(),
    );

    m.insert(
        "encode.usage_encode_b64".into(),
        "  encode b64 <เป้าหมาย> [-o <ไฟล์>] เข้ารหัส Base64".into(),
    );

    m.insert(
        "encode.usage_encode_hex".into(),
        "  encode hex <เป้าหมาย> [-o <ไฟล์>] เข้ารหัส Hex".into(),
    );

    m.insert(
        "encode.usage_encode_url".into(),
        "  encode url <เป้าหมาย> [-o <ไฟล์>] เข้ารหัสเปอร์เซ็นต์ URL".into(),
    );

    m.insert("encode.usage_examples".into(), "ตัวอย่าง:".into());

    m.insert("encode.usage_title".into(), "การใช้งาน:".into());

    m.insert("encode.written_to".into(), "เขียนไปที่: {}".into());
}
