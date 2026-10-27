use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "decode.unknown_format".into(),
        "未知的解码格式：'{}'。使用 b2, b10, b64, hex 或 url。".into(),
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
}
