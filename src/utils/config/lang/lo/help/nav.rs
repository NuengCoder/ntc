use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.nav.back".into(),
        "back [n] ## ກັບໄປໄດເຣັກທໍຣີແມ່ (ຫຼື n ລະດັບ)".into(),
    );

    m.insert("help.nav.cd".into(), "cd <path> ## ນຳທາງໄປໄດເຣັກທໍຣີ".into());

    m.insert(
        "help.nav.cd_tp".into(),
        "cd to <tp_name> ## Teleport ໄປຈຸດທີ່ບັນທຶກໄວ້".into(),
    );

    m.insert(
        "help.nav.fgo".into(),
        "fgo <pattern> [-d <n>] ## ຊອກຫາໄຟລ໌, ນຳທາງໄປ parent dir ຂອງໄຟລ໌ທີ່ເລືອກ".into(),
    );

    m.insert(
        "help.nav.godrive".into(),
        "godrive ## ລາຍຊື່ ແລະ ເລືອກ Windows drives".into(),
    );

    m.insert(
        "help.nav.gos".into(),
        "gos ## ລາຍຊື່ subdirectories ແລະ ເລືອກ".into(),
    );

    m.insert(
        "help.nav.gosc".into(),
        "gosc ## ນຳທາງຕໍ່ເນື່ອງ (0 ເພື່ອອອກ, v/s/vs ເພື່ອເບິ່ງ)".into(),
    );

    m.insert("help.nav.title".into(), "ຄຳສັ່ງ NAVIGATION:".into());

    m.insert(
        "help.nav.where".into(),
        "where [<ໂປຣແກຣມ>] ## ສະແດງຕຳແໜ່ງຂອງ ntc ຫຼື ຄົ້ນຫາໂປຣແກຣມໃນ PATH".into(),
    );

    m.insert(
        "help.nav.example1".into(),
        "cd src ## ນຳທາງໄປທີ່ໄດເຣັກທໍຣີ src".into(),
    );

    m.insert("help.nav.example2".into(), "back ## ກັບໄປໄດເຣັກທໍຣີແມ່".into());

    m.insert(
        "help.nav.example3".into(),
        "gos ## ການເລືອກ subdirectory ແບບ interactive".into(),
    );

    m.insert(
        "help.nav.example4".into(),
        "cd to web ## Teleport ໄປຈຸດບັນທຶກ 'web'".into(),
    );

    m.insert(
        "help.nav.example5".into(),
        "where git ## ຄົ້ນຫາຕຳແໜ່ງຂອງ git ໃນ PATH".into(),
    );
}
