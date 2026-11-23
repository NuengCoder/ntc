use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.qur.title".into(), "QUR COMMAND:".into());

    m.insert(
        "help.qur.usage".into(),
        "qur <glob> [-s \"pat\"] [-rp \"new\"] [-dp] [-d <n>] ## Query files, select content, replace".into(),
    );

    m.insert(
        "help.qur.example1".into(),
        "qur *.rs -s \"TODO\" -dp ## Query glob + grep-like display".into(),
    );
}
