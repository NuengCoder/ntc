use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.decode.title".into(), "DECODE 命令：".into());

    m.insert(
        "help.decode.b2".into(),
        "decode b2 <目标> [-o <文件>] ## 二进制解码".into(),
    );

    m.insert(
        "help.decode.b10".into(),
        "decode b10 <目标> [-o <文件>] ## 十进制解码".into(),
    );

    m.insert(
        "help.decode.b64".into(),
        "decode b64 <目标> [-o <文件>] ## Base64解码".into(),
    );

    m.insert(
        "help.decode.hex".into(),
        "decode hex <目标> [-o <文件>] ## 十六进制解码".into(),
    );

    m.insert(
        "help.decode.url".into(),
        "decode url <目标> [-o <文件>] ## URL百分比解码".into(),
    );

    m.insert(
        "help.decode.example1".into(),
        "decode b64 aGVsbG8= ## Base64 解码".into(),
    );

    m.insert(
        "help.decode.utf8".into(),
        "decode utf8 <target> [-o <file>] ## 转义字节转 UTF-8".into(),
    );

    m.insert(
        "help.decode.utf16".into(),
        "decode utf16 <target> [-o <file>] ## \\uNNNN 转义转文本".into(),
    );

    m.insert(
        "help.decode.ascii".into(),
        "decode ascii <target> [-o <file>] ## 转义文本转 ASCII 字节".into(),
    );
}
