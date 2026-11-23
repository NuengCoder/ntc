use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.encode.title".into(), "ENCODE 命令：".into());

    m.insert(
        "help.encode.b2".into(),
        "encode b2 <目标> [-o <文件>] ## 二进制编码（0101...）".into(),
    );

    m.insert(
        "help.encode.b10".into(),
        "encode b10 <目标> [-o <文件>] ## 十进制编码（0-255...）".into(),
    );

    m.insert(
        "help.encode.b64".into(),
        "encode b64 <目标> [-o <文件>] ## Base64编码（文件或原始文本）".into(),
    );

    m.insert(
        "help.encode.hex".into(),
        "encode hex <目标> [-o <文件>] ## 十六进制编码".into(),
    );

    m.insert(
        "help.encode.url".into(),
        "encode url <目标> [-o <文件>] ## URL百分比编码".into(),
    );

    m.insert(
        "help.encode.utf8".into(),
        "encode utf8 <target> [-o <file>] ## UTF-8 转义字节（\\xc3\\xa9）".into(),
    );

    m.insert(
        "help.encode.utf16".into(),
        "encode utf16 <target> [-o <file>] ## UTF-16 \\uNNNN 转义（\\u00e9）".into(),
    );

    m.insert(
        "help.encode.ascii".into(),
        "encode ascii <target> [-o <file>] ## ASCII 文本（超过 U+007F 报错）".into(),
    );

    m.insert(
        "help.encode.example1".into(),
        "encode b64 \"hello\" ## Base64 编码字符串".into(),
    );

    m.insert(
        "help.encode.example2".into(),
        "encode hex file.bin ## Hex 编码文件".into(),
    );
}
