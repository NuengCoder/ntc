use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.restorecg.title".into(), "RESTORECG ຄຳສັ່ງ:".into());

    m.insert(
        "help.restorecg.usage".into(),
        "restorecg ## ກູ້ຄືນ config ຈາກ backup".into(),
    );

    m.insert(
        "help.restorecg.example1".into(),
        "restorecg ## ກູ້ຄືນການຕັ້ງຄ່າຈາກ backup".into(),
    );
}
