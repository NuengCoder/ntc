use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.arc.title".into(), "ຄຳສັ່ງ ARCHIVE:".into());

    m.insert(
        "help.arc.create".into(),
        "arc create <out.zip|tar.gz|tar> <ໄຟລ໌...> ## ສ້າງໄຟລ໌ບີບอัด".into(),
    );

    m.insert(
        "help.arc.extract".into(),
        "arc extract <ໄຟລ໌> [ປາຍທາງ] ## ແຕກໄຟລ໌".into(),
    );

    m.insert(
        "help.arc.list".into(),
        "arc list <ໄຟລ໌> ## ສະແດງເນື້ອໃນ".into(),
    );

    m.insert(
        "help.arc.example1".into(),
        "arc create backup.zip src Cargo.toml ## ສ້າງ zip".into(),
    );

    m.insert(
        "help.arc.example2".into(),
        "arc list backup.zip ## ສະແດງເນື້ອໃນ".into(),
    );

    m.insert(
        "help.arc.example3".into(),
        "arc extract backup.zip restored ## ແຕກໄຟລ໌".into(),
    );
}
