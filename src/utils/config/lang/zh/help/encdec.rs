use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.encdec.cvcode".into(),
        "cvcode <来源> <数据> to <目标> ## 在格式之间转换".into(),
    );

    m.insert(
        "help.encdec.decode_b2".into(),
        "decode b2 <目标> [-o <文件>] ## 二进制解码".into(),
    );

    m.insert(
        "help.encdec.decode_b10".into(),
        "decode b10 <目标> [-o <文件>] ## 十进制解码".into(),
    );

    m.insert(
        "help.encdec.decode_b64".into(),
        "decode b64 <目标> [-o <文件>] ## Base64解码".into(),
    );

    m.insert(
        "help.encdec.decode_hex".into(),
        "decode hex <目标> [-o <文件>] ## 十六进制解码".into(),
    );

    m.insert(
        "help.encdec.decode_url".into(),
        "decode url <目标> [-o <文件>] ## URL百分比解码".into(),
    );

    m.insert(
        "help.encdec.encode_b2".into(),
        "encode b2 <目标> [-o <文件>] ## 二进制编码（0101...）".into(),
    );

    m.insert(
        "help.encdec.encode_b10".into(),
        "encode b10 <目标> [-o <文件>] ## 十进制编码（0-255...）".into(),
    );

    m.insert(
        "help.encdec.encode_b64".into(),
        "encode b64 <目标> [-o <文件>] ## Base64编码（文件或原始文本）".into(),
    );

    m.insert(
        "help.encdec.encode_hex".into(),
        "encode hex <目标> [-o <文件>] ## 十六进制编码".into(),
    );

    m.insert(
        "help.encdec.encode_url".into(),
        "encode url <目标> [-o <文件>] ## URL百分比编码".into(),
    );

    m.insert("help.encdec.title".into(), "编码/解码命令：".into());

    m.insert(
        "help.encdec.example1".into(),
        "encode b64 \"hello\" ## Base64 编码字符串".into(),
    );

    m.insert(
        "help.encdec.example2".into(),
        "encode hex file.bin ## Hex 编码文件".into(),
    );

    m.insert(
        "help.encdec.example3".into(),
        "decode b64 aGVsbG8= ## Base64 解码".into(),
    );

    m.insert(
        "help.encdec.example4".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## 在格式间转换".into(),
    );
}
