use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ipcfg.dns_label".into(), "DNS ......... :".into());

    m.insert(
        "cmd.ipcfg.no_adapters".into(),
        "ບໍ່ພົບ network adapters.".into(),
    );

    m.insert(
        "cmd.ipcfg.no_such_adapter".into(),
        "ບໍ່ມີ adapter ຊື່ '{}'.".into(),
    );

    m.insert("cmd.ipcfg.title".into(), "Network adapters:".into());
}
