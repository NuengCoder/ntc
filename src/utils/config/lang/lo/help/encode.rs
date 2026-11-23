use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.encode.title".into(), "ENCODE ຄຳສັ່ງ:".into());

    m.insert(
        "help.encode.b2".into(),
        "encode b2 <target> [-o <file>] ## ເຂົ້າລະຫັດ binary (0101...)".into(),
    );

    m.insert(
        "help.encode.b10".into(),
        "encode b10 <target> [-o <file>] ## ເຂົ້າລະຫັດທົດສະນິຍົມ (0-255...)".into(),
    );

    m.insert(
        "help.encode.b64".into(),
        "encode b64 <target> [-o <file>] ## Base64 encode (file ຫຼື raw text)".into(),
    );

    m.insert(
        "help.encode.hex".into(),
        "encode hex <target> [-o <file>] ## ເຂົ້າລະຫັດ hex".into(),
    );

    m.insert(
        "help.encode.url".into(),
        "encode url <target> [-o <file>] ## ເຂົ້າລະຫັດເປີເຊັນ URL".into(),
    );

    m.insert(
        "help.encode.utf8".into(),
        "encode utf8 <target> [-o <file>] ## UTF-8 ເປັນ byte escape (\\xc3\\xa9)".into(),
    );

    m.insert(
        "help.encode.utf16".into(),
        "encode utf16 <target> [-o <file>] ## UTF-16 ເປັນ escape \\uNNNN (\\u00e9)".into(),
    );

    m.insert(
        "help.encode.ascii".into(),
        "encode ascii <target> [-o <file>] ## ຂໍ້ຄວາມ ASCII (ຜິດພາດຖ້າເກີນ U+007F)".into(),
    );

    m.insert(
        "help.encode.example1".into(),
        "encode b64 \"hello\" ## Base64 encode ສະຕຣິງ".into(),
    );

    m.insert(
        "help.encode.example2".into(),
        "encode hex file.bin ## Hex encode ໄຟລ໌".into(),
    );
}
