use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ipcfg.title".into(), "IPCFG 命令：".into());

    m.insert(
        "help.ipcfg.usage".into(),
        "ipcfg [名称] ## 显示网卡信息（类似 ipconfig/ifconfig）".into(),
    );

    m.insert(
        "help.ipcfg.example1".into(),
        "ipcfg ## 显示网络适配器".into(),
    );
}
