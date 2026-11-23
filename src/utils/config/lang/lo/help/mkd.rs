use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkd.title".into(), "MKD ຄຳສັ່ງ:".into());

    m.insert(
        "help.mkd.usage".into(),
        "mkd <dir>[, ...] ## ສ້າງໄດເຣັກທໍຣີໃໝ່".into(),
    );

    m.insert(
        "help.mkd.example1".into(),
        "mkd src/components ## ສ້າງໄດເຣັກທໍຣີ".into(),
    );
}
