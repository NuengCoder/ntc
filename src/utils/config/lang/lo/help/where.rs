use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.where.title".into(), "WHERE ຄຳສັ່ງ:".into());

    m.insert(
        "help.where.usage".into(),
        "where [-a|--all] [<ໂປຣແກຣມ>] ## ສະແດງ executable ຂອງ ntc ຫຼື ຄົ້ນຫາໂປຣແກຣມໃນ PATH".into(),
    );

    m.insert(
        "help.where.example1".into(),
        "where -a ## ສະແດງທຸກ path ຂອງ config, session, package ແລະ vault ຂອງ ntc".into(),
    );

    m.insert(
        "help.where.example2".into(),
        "where git ## ສະແດງ git ທຳອິດໃນ PATH (-a ສະແດງທັງໝົດ)".into(),
    );
}
