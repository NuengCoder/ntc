use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.exit.title".into(), "EXIT COMMAND:".into());

    m.insert("help.exit.usage".into(), "exit, quit ## Exit ntc".into());

    m.insert("help.exit.example1".into(), "exit ## Leave ntc".into());
}
