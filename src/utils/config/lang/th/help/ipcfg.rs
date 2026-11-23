use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ipcfg.title".into(), "IPCFG คำสั่ง:".into());

    m.insert(
        "help.ipcfg.usage".into(),
        "ipcfg [ชื่อ] ## แสดงอะแดปเตอร์เครือข่ายแบบ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.ipcfg.example1".into(),
        "ipcfg ## แสดงอะแดปเตอร์เครือข่าย".into(),
    );
}
