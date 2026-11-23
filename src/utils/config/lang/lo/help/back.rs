use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.back.title".into(), "BACK ຄຳສັ່ງ:".into());

    m.insert(
        "help.back.usage".into(),
        "back [n] ## ກັບໄປໄດເຣັກທໍຣີແມ່ (ຫຼື n ລະດັບ)".into(),
    );

    m.insert("help.back.example1".into(), "back ## ກັບໄປໄດເຣັກທໍຣີແມ່".into());
}
