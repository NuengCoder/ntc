use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cvcode.formats".into(),
        "Formats: b2, b10, b64 (base64), hex, url".into(),
    );

    m.insert("cvcode.missing_data".into(), "ຂາດຂໍ້ມູນທີ່ຈະແປງ".into());

    m.insert("cvcode.missing_to".into(), "ຂາດຮູບແບບເປົ້າໝາຍຫຼັງ 'to'".into());

    m.insert(
        "cvcode.usage".into(),
        " cvcode <from> <data> to <to> [-o <file>]".into(),
    );

    m.insert(
        "cvcode.usage_detailed".into(),
        "ວິທີໃຊ້: cvcode <from> <data> to <to> [-o <file>]".into(),
    );
}
