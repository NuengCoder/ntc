use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pathcp.title".into(), "PATHCP ຄຳສັ່ງ:".into());

    m.insert(
        "help.pathcp.usage".into(),
        "pathcp ## ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນໃສ່ clipboard".into(),
    );

    m.insert(
        "help.pathcp.example1".into(),
        "pathcp ## ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
}
