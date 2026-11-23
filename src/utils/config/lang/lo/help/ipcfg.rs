use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ipcfg.title".into(), "IPCFG ຄຳສັ່ງ:".into());

    m.insert(
        "help.ipcfg.usage".into(),
        "ipcfg [ຊື່] ## ສະແດງ adapters ແບບ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.ipcfg.example1".into(),
        "ipcfg ## ສະແດງ network adapters".into(),
    );
}
