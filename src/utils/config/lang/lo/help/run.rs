use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.run.title".into(), "RUN ຄຳສັ່ງ:".into());

    m.insert(
        "help.run.usage".into(),
        "run <alias> ## ເຮັດວຽກ alias ດ້ວຍ 'run'".into(),
    );

    m.insert(
        "help.run.direct".into(),
        "<alias> ## ເຮັດວຽກ alias ໂດຍກົງ".into(),
    );

    m.insert(
        "help.run.param".into(),
        "<alias>(<arg>) ## ເຮັດວຽກ parameterised alias".into(),
    );

    m.insert(
        "help.run.example1".into(),
        "build ## ເຮັດວຽກ alias ໂດຍກົງ".into(),
    );
}
