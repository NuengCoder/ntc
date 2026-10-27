use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "ບໍ່ຮູ້ຈັກ decode format: '{}'. ໃຊ້ b2, b10, b64, hex, ຫຼື url.".into(),
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
}
