use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showenv.title".into(), "SHOWENV ຄຳສັ່ງ:".into());

    m.insert(
        "help.showenv.usage".into(),
        "showenv ## ສະແດງ environment variables (PATH entries ທຳອິດ 5)".into(),
    );

    m.insert(
        "help.showenv.all".into(),
        "showenv --all ## ສະແດງ PATH entries ທັງໝົດ".into(),
    );

    m.insert(
        "help.showenv.depth".into(),
        "showenv -d <n> ## ສະແດງ PATH entries ທຳອິດ n".into(),
    );

    m.insert(
        "help.showenv.example1".into(),
        "showenv --all ## ສະແດງທຸກລາຍການ PATH".into(),
    );
}
