use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.clip.title".into(), "ຄຳສັ່ງ CLIPBOARD:".into());

    m.insert(
        "help.clip.show".into(),
        "clip ## ສະແດງ clipboard history".into(),
    );

    m.insert(
        "help.clip.list".into(),
        "clip list ## ສະແດງ clipboard history".into(),
    );

    m.insert(
        "help.clip.select".into(),
        "clip select <N> ## ສຳເນົາລາຍການ N ກັບໃສ່ clipboard".into(),
    );

    m.insert(
        "help.clip.clear".into(),
        "clip clear ## ລຶບ clipboard history".into(),
    );

    m.insert(
        "help.clip.example1".into(),
        "clip ## ສະແດງ clipboard history".into(),
    );

    m.insert(
        "help.clip.example2".into(),
        "clip select 1 ## ສຳເນົາ entry #1 ກັບໃສ່ clipboard".into(),
    );
}
