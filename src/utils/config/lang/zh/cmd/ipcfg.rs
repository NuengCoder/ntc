use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ipcfg.dns_label".into(), "DNS ......... :".into());

    m.insert("cmd.ipcfg.no_adapters".into(), "未找到网络适配器。".into());

    m.insert(
        "cmd.ipcfg.no_such_adapter".into(),
        "没有这个网卡：'{}'。".into(),
    );

    m.insert("cmd.ipcfg.title".into(), "网络适配器：".into());
}
