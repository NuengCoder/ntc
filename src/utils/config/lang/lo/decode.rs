use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "ບໍ່ຮູ້ຈັກ decode format: '{}'. ໃຊ້ b2, b10, b64, hex, url, utf8, utf16, ascii.".into(),
    );

    m.insert(
        "decode.usage_cmd".into(),
        "ວິທີໃຊ້: decode {} <target> [-o <file>]".into(),
    );

    m.insert(
        "decode.usage_decode_b2".into(),
        " decode b2 <target> [-o <file>] Binary decode".into(),
    );

    m.insert(
        "decode.usage_decode_b10".into(),
        " decode b10 <target> [-o <file>] Decimal decode".into(),
    );

    m.insert(
        "decode.usage_decode_b64".into(),
        " decode b64 <target> [-o <file>] Base64 decode".into(),
    );

    m.insert(
        "decode.usage_decode_hex".into(),
        " decode hex <target> [-o <file>] Hex decode".into(),
    );

    m.insert(
        "decode.usage_decode_url".into(),
        " decode url <target> [-o <file>] URL percent-decode".into(),
    );

    m.insert(
        "decode.usage_decode_utf8".into(),
        "  decode utf8 <target> [-o <file>] byte escape ເປັນ UTF-8 (\\xc3\\xa9)".into(),
    );

    m.insert(
        "decode.usage_decode_utf16".into(),
        "  decode utf16 <target> [-o <file>] escape \\uNNNN ເປັນຂໍ້ຄວາມ (\\u00e9)".into(),
    );

    m.insert(
        "decode.usage_decode_ascii".into(),
        "  decode ascii <target> [-o <file>] ຂໍ້ຄວາມ escape ເປັນ byte ASCII".into(),
    );
}
