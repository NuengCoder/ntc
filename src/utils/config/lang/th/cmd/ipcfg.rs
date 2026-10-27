use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ipcfg.dns_label".into(), "DNS ......... :".into());

    m.insert(
        "cmd.ipcfg.no_adapters".into(),
        "ไม่พบอะแดปเตอร์เครือข่าย".into(),
    );

    m.insert(
        "cmd.ipcfg.no_such_adapter".into(),
        "ไม่มีอะแดปเตอร์ชื่อ '{}'".into(),
    );

    m.insert("cmd.ipcfg.title".into(), "อะแดปเตอร์เครือข่าย:".into());
}
