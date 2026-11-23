use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ipcfg.title".into(), "IPCFG COMMAND:".into());

    m.insert(
        "help.ipcfg.usage".into(),
        "ipcfg [name] ## Show adapters like ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.ipcfg.example1".into(),
        "ipcfg ## Show network adapters".into(),
    );
}
