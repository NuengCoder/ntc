use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cvcode.title".into(), "CVCODE ຄຳສັ່ງ:".into());

    m.insert(
        "help.cvcode.usage".into(),
        "cvcode <from> <data> to <to> ## ແປງລະຫວ່າງຮູບແບບ".into(),
    );

    m.insert(
        "help.cvcode.example1".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## ແປງລະຫວ່າງຮູບແບບ".into(),
    );

    m.insert(
        "help.cvcode.example2".into(),
        "cvcode b64 \"aGVsbG8=\" to plaintext ## ຖອດລະຫັດເປັນຂໍ້ຄວາມ".into(),
    );
}
