use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.clipboard.clear".into(),
        "clip clear ## ລຶບ clipboard history".into(),
    );

    m.insert(
        "help.clipboard.list".into(),
        "clip list ## ສະແດງ clipboard history".into(),
    );

    m.insert(
        "help.clipboard.pathcp".into(),
        "pathcp ## ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ clipboard".into(),
    );

    m.insert(
        "help.clipboard.select".into(),
        "clip select <N> ## ສຳເນົາລາຍການ N ກັບໃສ່ clipboard".into(),
    );

    m.insert(
        "help.clipboard.show".into(),
        "clip ## ສະແດງ clipboard history".into(),
    );

    m.insert("help.clipboard.title".into(), "ຄຳສັ່ງ CLIPBOARD:".into());

    m.insert(
        "help.clipboard.example1".into(),
        "clip ## ສະແດງ clipboard history".into(),
    );

    m.insert(
        "help.clipboard.example2".into(),
        "clip select 1 ## ສຳເນົາ entry #1 ກັບໃສ່ clipboard".into(),
    );

    m.insert(
        "help.clipboard.example3".into(),
        "pathcp ## ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
}
