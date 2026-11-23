use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.encode.title".into(), "ENCODE COMMAND:".into());

    m.insert(
        "help.encode.b2".into(),
        "encode b2 <target> [-o <file>] ## Binary encode (0101...)".into(),
    );

    m.insert(
        "help.encode.b10".into(),
        "encode b10 <target> [-o <file>] ## Decimal encode (0-255...)".into(),
    );

    m.insert(
        "help.encode.b64".into(),
        "encode b64 <target> [-o <file>] ## Base64 encode (file or raw text)".into(),
    );

    m.insert(
        "help.encode.hex".into(),
        "encode hex <target> [-o <file>] ## Hex encode".into(),
    );

    m.insert(
        "help.encode.url".into(),
        "encode url <target> [-o <file>] ## URL percent-encode".into(),
    );

    m.insert(
        "help.encode.utf8".into(),
        "encode utf8 <target> [-o <file>] ## UTF-8 as escaped bytes (\\xc3\\xa9)".into(),
    );

    m.insert(
        "help.encode.utf16".into(),
        "encode utf16 <target> [-o <file>] ## UTF-16 as \\uNNNN escapes (\\u00e9)".into(),
    );

    m.insert(
        "help.encode.ascii".into(),
        "encode ascii <target> [-o <file>] ## ASCII text (errors above U+007F)".into(),
    );

    m.insert(
        "help.encode.example1".into(),
        "encode b64 \"hello\" ## Base64 encode a string".into(),
    );

    m.insert(
        "help.encode.example2".into(),
        "encode hex file.bin ## Hex encode a file".into(),
    );
}
