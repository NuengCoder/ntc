use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.exit.title".into(), "EXIT ຄຳສັ່ງ:".into());

    m.insert("help.exit.usage".into(), "exit, quit ## ອອກຈາກ ntc".into());

    m.insert("help.exit.example1".into(), "exit ## ອອກຈາກ ntc".into());
}
