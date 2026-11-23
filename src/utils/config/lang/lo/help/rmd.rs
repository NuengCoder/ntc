use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmd.title".into(), "RMD ຄຳສັ່ງ:".into());

    m.insert(
        "help.rmd.usage".into(),
        "rmd <dir>[, ...] ## ລຶບໄດເຣັກທໍຣີ; wildcard ລຶບທຸກລາຍການທີ່ກົງກັນຫຼັງຢືນຢັນ".into(),
    );

    m.insert(
        "help.rmd.example1".into(),
        "rmd build ## ລຶບໄດເຣັກທໍຣີ build".into(),
    );
}
