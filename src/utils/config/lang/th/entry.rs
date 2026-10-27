use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("entry.goodbye".into(), "ลาก่อน!".into());

    m.insert(
        "entry.ntpkg_active".into(),
        "ntpkg: {} runtime{} ทำงานใน PATH".into(),
    );

    m.insert(
        "entry.startup_hint".into(),
        "พิมพ์ 'help' สำหรับคำสั่งที่มี 'exit' เพื่อออก.".into(),
    );

    m.insert("entry.watch_trigger".into(), "watch trigger: {}".into());

    m.insert("entry.watcher_failed".into(), "ตัวเฝ้าดูไฟล์ล้มเหลว: {}".into());

    m.insert(
        "entry.watcher_started".into(),
        "เริ่มตัวเฝ้าดูไฟล์แล้ว (เรียกซ้ำ)".into(),
    );

    m.insert(
        "entry.welcome_fmt".into(),
        " ยินดีต้อนรับสู่ ntc {} - Navigate, Toolkit, Center ".into(),
    );
}
