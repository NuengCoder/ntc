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
        "未知的编码格式：'{}'。使用 b2, b10, b64, hex 或 url。".into(),
    );

    m.insert(
        "encode.unknown_format_fmt".into(),
        "（未知格式：{}）".into(),
    );

    m.insert(
        "encode.unknown_format_generic".into(),
        "未知格式：'{}'。使用 b2, b10, b64, hex 或 url。".into(),
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
}
