use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "encode.binary_hint".into(),
        "使用 -o <文件> 保存二进制输出。".into(),
    );

    m.insert(
        "encode.binary_warning".into(),
        "输出为二进制数据。显示十六进制预览：".into(),
    );

    m.insert(
        "encode.hex_even".into(),
        "十六进制字符串长度必须为偶数".into(),
    );

    m.insert(
        "encode.input_must_be_text".into(),
        "输入数据必须是文本".into(),
    );

    m.insert(
        "encode.input_not_text".into(),
        "{} 输入必须是文本（非二进制）".into(),
    );

    m.insert(
        "encode.invalid_b2".into(),
        "无效的二进制字符串：'{}'".into(),
    );

    m.insert("encode.invalid_b10".into(), "无效的十进制字节：'{}'".into());

    m.insert("encode.invalid_b64".into(), "无效的 base64 输入：{}".into());

    m.insert(
        "encode.invalid_hex_pct".into(),
        "百分比编码中的无效十六进制：%{}".into(),
    );

    m.insert(
        "encode.invalid_hex_pos".into(),
        "位置 {} 处无效的十六进制字符".into(),
    );

    m.insert("encode.invalid_pct".into(), "无效的百分比编码".into());

    m.insert("encode.truncated_pct".into(), "截断的百分比编码".into());

    m.insert(
        "encode.unknown_format".into(),
        "未知的编码格式：'{}'。使用 b2, b10, b64, hex, url, utf8, utf16, ascii。".into(),
    );

    m.insert(
        "encode.unknown_format_fmt".into(),
        "（未知格式：{}）".into(),
    );

    m.insert(
        "encode.unknown_format_generic".into(),
        "未知格式：'{}'。使用 b2, b10, b64, hex, url, utf8, utf16, ascii。".into(),
    );

    m.insert(
        "encode.usage_cmd".into(),
        "用法：encode {} <目标> [-o <文件>]".into(),
    );

    m.insert(
        "encode.usage_encode_b2".into(),
        " encode b2 <目标> [-o <文件>] 二进制编码（0101...）".into(),
    );

    m.insert(
        "encode.usage_encode_b10".into(),
        " encode b10 <目标> [-o <文件>] 十进制编码（0-255...）".into(),
    );

    m.insert(
        "encode.usage_encode_b64".into(),
        " encode b64 <目标> [-o <文件>] Base64 编码".into(),
    );

    m.insert(
        "encode.usage_encode_hex".into(),
        " encode hex <目标> [-o <文件>] 十六进制编码".into(),
    );

    m.insert(
        "encode.usage_encode_url".into(),
        " encode url <目标> [-o <文件>] URL 百分比编码".into(),
    );

    m.insert("encode.usage_examples".into(), "示例：".into());

    m.insert("encode.usage_title".into(), "用法：".into());

    m.insert("encode.written_to".into(), "已写入：{}".into());

    m.insert(
        "encode.usage_encode_utf8".into(),
        " encode utf8 <目标> [-o <文件>] UTF-8 转义字节编码（\\xc3\\xa9）".into(),
    );

    m.insert(
        "encode.usage_encode_utf16".into(),
        " encode utf16 <目标> [-o <文件>] UTF-16 \\uNNNN 转义编码（\\u00e9）".into(),
    );

    m.insert(
        "encode.usage_encode_ascii".into(),
        " encode ascii <目标> [-o <文件>] ASCII 编码（超过 U+007F 报错）".into(),
    );

    m.insert(
        "encode.invalid_escape".into(),
        "无效的转义序列：'{}'".into(),
    );

    m.insert(
        "encode.invalid_utf8".into(),
        "解码后的字节不是有效的 UTF-8".into(),
    );

    m.insert(
        "encode.invalid_utf16".into(),
        "解码后的 UTF-16 无效（代理项未配对）".into(),
    );

    m.insert(
        "encode.not_ascii".into(),
        "字符 '{}' 不是 ASCII".into(),
    );

    m.insert(
        "encode.not_ascii_byte".into(),
        "解码后的字节 {} 不是 ASCII".into(),
    );
}
