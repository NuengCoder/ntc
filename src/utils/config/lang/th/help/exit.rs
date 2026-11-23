use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.exit.title".into(), "EXIT คำสั่ง:".into());

    m.insert("help.exit.usage".into(), "exit, quit ## ออกจาก ntc".into());

    m.insert("help.exit.example1".into(), "exit ## ออกจาก ntc".into());
}
