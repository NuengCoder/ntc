use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "encode.binary_hint".into(),
        "ໃຊ້ -o <file> ເພື່ອບັນທຶກຜົນລັບ binary.".into(),
    );

    m.insert(
        "encode.binary_warning".into(),
        "ຜົນລັບເປັນຂໍ້ມູນ binary. ກຳລັງສະແດງ hex preview:".into(),
    );

    m.insert(
        "encode.hex_even".into(),
        "ຄວາມຍາວສະຕຣິງ hex ຕ້ອງເປັນຈຳນວນຄູ່".into(),
    );

    m.insert(
        "encode.input_must_be_text".into(),
        "ຂໍ້ມູນທີ່ເຂົ້າຕ້ອງເປັນຂໍ້ຄວາມ".into(),
    );

    m.insert(
        "encode.input_not_text".into(),
        "{} input ຕ້ອງເປັນຂໍ້ຄວາມ (ບໍ່ແມ່ນ binary)".into(),
    );

    m.insert(
        "encode.invalid_b2".into(),
        "ສະຕຣິງ binary ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "encode.invalid_b10".into(),
        "ໄບຕ໌ decimal ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "encode.invalid_b64".into(),
        "base64 input ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "encode.invalid_hex_pct".into(),
        "hex ບໍ່ຖືກຕ້ອງໃນ percent-encoding: %{}".into(),
    );

    m.insert(
        "encode.invalid_hex_pos".into(),
        "ຕົວອັກສອນ hex ບໍ່ຖືກຕ້ອງທີ່ຕຳແໜ່ງ {}".into(),
    );

    m.insert(
        "encode.invalid_pct".into(),
        "percent-encoding ບໍ່ຖືກຕ້ອງ".into(),
    );

    m.insert(
        "encode.truncated_pct".into(),
        "percent-encoding ຖືກຕັດ".into(),
    );

    m.insert(
        "encode.unknown_format".into(),
        "ບໍ່ຮູ້ຈັກ encode format: '{}'. ໃຊ້ b2, b10, b64, hex, url, utf8, utf16, ascii.".into(),
    );

    m.insert(
        "encode.unknown_format_fmt".into(),
        "(ບໍ່ຮູ້ຈັກ format: {})".into(),
    );

    m.insert(
        "encode.unknown_format_generic".into(),
        "ບໍ່ຮູ້ຈັກ format: '{}'. ໃຊ້ b2, b10, b64, hex, url, utf8, utf16, ascii.".into(),
    );

    m.insert(
        "encode.usage_cmd".into(),
        "ວິທີໃຊ້: encode {} <target> [-o <file>]".into(),
    );

    m.insert(
        "encode.usage_encode_b2".into(),
        " encode b2 <target> [-o <file>] Binary encode (0101...)".into(),
    );

    m.insert(
        "encode.usage_encode_b10".into(),
        " encode b10 <target> [-o <file>] Decimal encode (0-255...)".into(),
    );

    m.insert(
        "encode.usage_encode_b64".into(),
        " encode b64 <target> [-o <file>] Base64 encode".into(),
    );

    m.insert(
        "encode.usage_encode_hex".into(),
        " encode hex <target> [-o <file>] Hex encode".into(),
    );

    m.insert(
        "encode.usage_encode_url".into(),
        " encode url <target> [-o <file>] URL percent-encode".into(),
    );

    m.insert("encode.usage_examples".into(), "ຕົວຢ່າງ:".into());

    m.insert("encode.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert("encode.written_to".into(), "ຂຽນໄປທີ່: {}".into());

    m.insert(
        "encode.usage_encode_utf8".into(),
        "  encode utf8 <target> [-o <file>] ເຂົ້າລະຫັດ UTF-8 ເປັນ byte escape (\\xc3\\xa9)".into(),
    );

    m.insert(
        "encode.usage_encode_utf16".into(),
        "  encode utf16 <target> [-o <file>] ເຂົ້າລະຫັດ UTF-16 ເປັນ escape \\uNNNN (\\u00e9)".into(),
    );

    m.insert(
        "encode.usage_encode_ascii".into(),
        "  encode ascii <target> [-o <file>] ເຂົ້າລະຫັດ ASCII (ຜິດພາດຖ້າເກີນ U+007F)".into(),
    );

    m.insert(
        "encode.invalid_escape".into(),
        "ລຳດັບ escape ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "encode.invalid_utf8".into(),
        "byte ທີ່ຖອດລະຫັດບໍ່ແມ່ນ UTF-8 ທີ່ຖືກຕ້ອງ".into(),
    );

    m.insert(
        "encode.invalid_utf16".into(),
        "UTF-16 ທີ່ຖອດລະຫັດບໍ່ຖືກຕ້ອງ (surrogate ບໍ່ຄູ່)".into(),
    );

    m.insert(
        "encode.not_ascii".into(),
        "ອັກຂະລະ '{}' ບໍ່ແມ່ນ ASCII".into(),
    );

    m.insert(
        "encode.not_ascii_byte".into(),
        "byte ທີ່ຖອດລະຫັດ {} ບໍ່ແມ່ນ ASCII".into(),
    );
}
