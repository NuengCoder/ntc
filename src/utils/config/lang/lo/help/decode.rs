use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.decode.title".into(), "DECODE ຄຳສັ່ງ:".into());

    m.insert(
        "help.decode.b2".into(),
        "decode b2 <target> [-o <file>] ## ຖອດລະຫັດ binary".into(),
    );

    m.insert(
        "help.decode.b10".into(),
        "decode b10 <target> [-o <file>] ## ຖອດລະຫັດທົດສະນິຍົມ".into(),
    );

    m.insert(
        "help.decode.b64".into(),
        "decode b64 <target> [-o <file>] ## ຖອດລະຫັດ Base64".into(),
    );

    m.insert(
        "help.decode.hex".into(),
        "decode hex <target> [-o <file>] ## ຖອດລະຫັດ hex".into(),
    );

    m.insert(
        "help.decode.url".into(),
        "decode url <target> [-o <file>] ## ຖອດລະຫັດເປີເຊັນ URL".into(),
    );

    m.insert(
        "help.decode.example1".into(),
        "decode b64 aGVsbG8= ## ຖອດລະຫັດ Base64".into(),
    );

    m.insert(
        "help.decode.utf8".into(),
        "decode utf8 <target> [-o <file>] ## byte escape ເປັນ UTF-8".into(),
    );

    m.insert(
        "help.decode.utf16".into(),
        "decode utf16 <target> [-o <file>] ## escape \\uNNNN ເປັນຂໍ້ຄວາມ".into(),
    );

    m.insert(
        "help.decode.ascii".into(),
        "decode ascii <target> [-o <file>] ## ຂໍ້ຄວາມ escape ເປັນ byte ASCII".into(),
    );
}
