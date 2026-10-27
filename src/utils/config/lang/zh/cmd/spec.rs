use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.spec.other".into(), "其他".into());

    m.insert("cmd.spec.title".into(), "系统规格".into());

    m.insert("cmd.spec.cpu".into(), "CPU".into());

    m.insert("cmd.spec.vendor".into(), "厂商".into());

    m.insert("cmd.spec.generation".into(), "代数".into());

    m.insert("cmd.spec.model".into(), "型号".into());

    m.insert("cmd.spec.cores".into(), "核心 / 线程".into());

    m.insert("cmd.spec.gpu".into(), "显卡".into());

    m.insert("cmd.spec.ram".into(), "内存".into());

    m.insert("cmd.spec.storage".into(), "存储".into());

    m.insert("cmd.spec.total".into(), "总计".into());

    m.insert("cmd.spec.os".into(), "操作系统".into());

    m.insert("cmd.spec.manufacturer".into(), "制造商".into());

    m.insert("cmd.spec.computer_model".into(), "电脑型号".into());

    m.insert("cmd.spec.serial".into(), "序列号".into());

    m.insert("cmd.spec.hostname".into(), "主机名".into());

    m.insert("cmd.spec.kernel".into(), "内核".into());

    m.insert("cmd.spec.uptime".into(), "运行时间".into());

    m.insert("cmd.spec.unknown".into(), "未知".into());

    m.insert("cmd.spec.custom".into(), "自定义".into());
}
