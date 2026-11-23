use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.decode.title".into(), "DECODE COMMAND:".into());

    m.insert(
        "help.decode.b2".into(),
        "decode b2 <target> [-o <file>] ## Binary decode".into(),
    );

    m.insert(
        "help.decode.b10".into(),
        "decode b10 <target> [-o <file>] ## Decimal decode".into(),
    );

    m.insert(
        "help.decode.b64".into(),
        "decode b64 <target> [-o <file>] ## Base64 decode".into(),
    );

    m.insert(
        "help.decode.hex".into(),
        "decode hex <target> [-o <file>] ## Hex decode".into(),
    );

    m.insert(
        "help.decode.url".into(),
        "decode url <target> [-o <file>] ## URL percent-decode".into(),
    );

    m.insert(
        "help.decode.utf8".into(),
        "decode utf8 <target> [-o <file>] ## Escaped bytes to UTF-8".into(),
    );

    m.insert(
        "help.decode.utf16".into(),
        "decode utf16 <target> [-o <file>] ## \\uNNNN escapes to text".into(),
    );

    m.insert(
        "help.decode.ascii".into(),
        "decode ascii <target> [-o <file>] ## Escaped text to ASCII bytes".into(),
    );

    m.insert(
        "help.decode.example1".into(),
        "decode b64 aGVsbG8= ## Base64 decode".into(),
    );
}
