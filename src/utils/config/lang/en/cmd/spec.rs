use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.spec.other".into(), "Other".into());

    m.insert("cmd.spec.title".into(), "SYSTEM SPECIFICATIONS".into());

    m.insert("cmd.spec.cpu".into(), "CPU".into());

    m.insert("cmd.spec.vendor".into(), "Vendor".into());

    m.insert("cmd.spec.generation".into(), "Generation".into());

    m.insert("cmd.spec.model".into(), "Model".into());

    m.insert("cmd.spec.cores".into(), "Cores / Threads".into());

    m.insert("cmd.spec.gpu".into(), "GPU".into());

    m.insert("cmd.spec.ram".into(), "RAM".into());

    m.insert("cmd.spec.storage".into(), "Storage".into());

    m.insert("cmd.spec.total".into(), "Total".into());

    m.insert("cmd.spec.os".into(), "OS".into());

    m.insert("cmd.spec.manufacturer".into(), "Manufacturer".into());

    m.insert("cmd.spec.computer_model".into(), "Computer Model".into());

    m.insert("cmd.spec.serial".into(), "Serial Number".into());

    m.insert("cmd.spec.hostname".into(), "Hostname".into());

    m.insert("cmd.spec.kernel".into(), "Kernel".into());

    m.insert("cmd.spec.uptime".into(), "Uptime".into());

    m.insert("cmd.spec.unknown".into(), "Unknown".into());

    m.insert("cmd.spec.custom".into(), "Custom".into());
}
