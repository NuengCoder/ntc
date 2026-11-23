use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tutorial.title".into(), "TUTORIAL ຄຳສັ່ງ:".into());

    m.insert(
        "help.tutorial.usage".into(),
        "tutorial [ບົດຮຽນ] ## ບົດຮຽນປະຕິບັດໃນ sandbox (cd, view, ne, ral, ran)".into(),
    );

    m.insert(
        "help.tutorial.example1".into(),
        "tutorial ## ເລີ່ມ tutorial interactive".into(),
    );
}
