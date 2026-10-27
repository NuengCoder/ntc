use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("status.none".into(), "none".into());

    m.insert("status.off".into(), "OFF".into());

    m.insert("status.on".into(), "ON".into());
}
