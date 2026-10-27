use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.mode.admin".into(),
        "mode admin ## ออกและเปิด ntc ใหม่ในฐานะ Administrator".into(),
    );

    m.insert(
        "help.mode.normal".into(),
        "mode normal ## ออกและเปิด ntc ใหม่โดยไม่ยกระดับสิทธิ์".into(),
    );

    m.insert(
        "help.mode.title".into(),
        "โหมด Windows (WINDOWS MODE):".into(),
    );

    m.insert(
        "help.mode.example1".into(),
        "mode admin ## เปิดใหม่ในฐานะ Administrator".into(),
    );
}
