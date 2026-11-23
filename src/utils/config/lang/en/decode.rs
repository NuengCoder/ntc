use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "Unknown decode format: '{}'. Use b2, b10, b64, hex, url, utf8, utf16, or ascii.".into(),
    );

    m.insert(
        "decode.usage_cmd".into(),
        "Usage: decode {} <target> [-o <file>]".into(),
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
        " decode utf8 <target> [-o <file>] Escaped bytes to UTF-8 (\\xc3\\xa9)".into(),
    );

    m.insert(
        "decode.usage_decode_utf16".into(),
        " decode utf16 <target> [-o <file>] \\uNNNN escapes to text (\\u00e9)".into(),
    );

    m.insert(
        "decode.usage_decode_ascii".into(),
        " decode ascii <target> [-o <file>] Escaped text to ASCII bytes".into(),
    );
}
