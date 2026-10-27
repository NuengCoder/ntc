use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "qsc.macos_listen_hint".into(),
        "QSC: เรียก 'qsc listen' เมื่อเพิ่มการรองรับแล้ว.".into(),
    );

    m.insert(
        "qsc.macos_not_implemented".into(),
        "QSC: ยังไม่ได้ใช้งานการรองรับปุ่มลัด macOS.".into(),
    );

    m.insert(
        "qsc.unsupported_platform".into(),
        "QSC: แพลตฟอร์มไม่รองรับ".into(),
    );
}
