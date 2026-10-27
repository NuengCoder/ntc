use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.encdec.cvcode".into(),
        "cvcode <from> <data> to <to> ## ແປງລະຫວ່າງຮູບແບບ".into(),
    );

    m.insert(
        "help.encdec.decode_b2".into(),
        "decode b2 <target> [-o <file>] ## Binary decode".into(),
    );

    m.insert(
        "help.encdec.decode_b10".into(),
        "decode b10 <target> [-o <file>] ## Decimal decode".into(),
    );

    m.insert(
        "help.encdec.decode_b64".into(),
        "decode b64 <target> [-o <file>] ## Base64 decode".into(),
    );

    m.insert(
        "help.encdec.decode_hex".into(),
        "decode hex <target> [-o <file>] ## Hex decode".into(),
    );

    m.insert(
        "help.encdec.decode_url".into(),
        "decode url <target> [-o <file>] ## URL percent-decode".into(),
    );

    m.insert(
        "help.encdec.encode_b2".into(),
        "encode b2 <target> [-o <file>] ## Binary encode (0101...)".into(),
    );

    m.insert(
        "help.encdec.encode_b10".into(),
        "encode b10 <target> [-o <file>] ## Decimal encode (0-255...)".into(),
    );

    m.insert(
        "help.encdec.encode_b64".into(),
        "encode b64 <target> [-o <file>] ## Base64 encode (file ຫຼື raw text)".into(),
    );

    m.insert(
        "help.encdec.encode_hex".into(),
        "encode hex <target> [-o <file>] ## Hex encode".into(),
    );

    m.insert(
        "help.encdec.encode_url".into(),
        "encode url <target> [-o <file>] ## URL percent-encode".into(),
    );

    m.insert("help.encdec.title".into(), "ຄຳສັ່ງ ENCODE/DECODE:".into());

    m.insert(
        "help.encdec.example1".into(),
        "encode b64 \"hello\" ## Base64 encode ສະຕຣິງ".into(),
    );

    m.insert(
        "help.encdec.example2".into(),
        "encode hex file.bin ## Hex encode ໄຟລ໌".into(),
    );

    m.insert(
        "help.encdec.example3".into(),
        "decode b64 aGVsbG8= ## Base64 decode".into(),
    );

    m.insert(
        "help.encdec.example4".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## ແປງລະຫວ່າງຮູບແບບ".into(),
    );
}
