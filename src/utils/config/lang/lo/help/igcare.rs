use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcare.title".into(), "ຄຳສັ່ງ IGCARE:".into());

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## ສົ່ງອອກການຕັ້ງຄ່າທັງໝົດໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## ເລືອກປະເພດເພື່ອສົ່ງອອກ".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <file> ## ນຳເຂົ້າການຕັ້ງຄ່າຈາກໄຟລ໌ .ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [name] ## ສ້າງ igcare template ໃນ CWD".into(),
    );

    m.insert(
        "help.igcare.example1".into(),
        "igcare make ## ສ້າງ template igcare".into(),
    );
}
