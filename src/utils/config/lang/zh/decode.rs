use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "未知的解码格式：'{}'。使用 b2, b10, b64, hex, url, utf8, utf16, ascii。".into(),
    );

    m.insert(
        "decode.usage_cmd".into(),
        "用法：decode {} <目标> [-o <文件>]".into(),
    );

    m.insert(
        "decode.usage_decode_b2".into(),
        " decode b2 <目标> [-o <文件>] 二进制解码".into(),
    );

    m.insert(
        "decode.usage_decode_b10".into(),
        " decode b10 <目标> [-o <文件>] 十进制解码".into(),
    );

    m.insert(
        "decode.usage_decode_b64".into(),
        " decode b64 <目标> [-o <文件>] Base64 解码".into(),
    );

    m.insert(
        "decode.usage_decode_hex".into(),
        " decode hex <目标> [-o <文件>] 十六进制解码".into(),
    );

    m.insert(
        "decode.usage_decode_url".into(),
        " decode url <目标> [-o <文件>] URL 百分比解码".into(),
    );

    m.insert(
        "decode.usage_decode_utf8".into(),
        " decode utf8 <目标> [-o <文件>] 转义字节转 UTF-8（\\xc3\\xa9）".into(),
    );

    m.insert(
        "decode.usage_decode_utf16".into(),
        " decode utf16 <目标> [-o <文件>] \\uNNNN 转义转文本（\\u00e9）".into(),
    );

    m.insert(
        "decode.usage_decode_ascii".into(),
        " decode ascii <目标> [-o <文件>] 转义文本转 ASCII 字节".into(),
    );
}
