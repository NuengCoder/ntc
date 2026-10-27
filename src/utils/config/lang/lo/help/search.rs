use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.search.ds".into(),
        "ds <pattern> [-d <n>] ## ຄົ້ນຫາໄດເຣັກທໍຣີຕາມຊື່".into(),
    );

    m.insert(
        "help.search.fs".into(),
        "fs <pattern> [-d <n>] ## ຄົ້ນຫາໄຟລ໌ຕາມຊື່ (fuzzy fallback)".into(),
    );

    m.insert(
        "help.search.fsc".into(),
        "fsc <pattern> [-d <n>] ## ຄົ້ນຫາໄຟລ໌, ເລືອກເພື່ອສະແດງ ແລະ ສຳເນົາໄດ້".into(),
    );

    m.insert("help.search.gs".into(), "gs <pattern> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-i] ## ຄົ້ນຫາເນື້ອໃນໄຟລ໌ (grep ງ່າຍໆ)".into());

    m.insert(
        "help.search.gs_interactive".into(),
        "gs -i <pattern> [-d <n>] ## grep interactive (10 ລາຍການ/ໜ້າ, Enter ເພື່ອເປີດ)".into(),
    );

    m.insert(
        "help.search.locate".into(),
        "locate <pattern> [-d <n>] ## ຄົ້ນຫາທັງໄຟລ໌ ແລະ ໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.search.pipe".into(),
        "cmd | gs <pattern> ## ຄົ້ນຫາເນື້ອໃນທີ່ pipe ເຂົ້າມາ (qur -s <pattern> ກໍໄດ້)".into(),
    );

    m.insert(
        "help.search.qur".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] ## ຄົ້ນຫາໄຟລ໌, ເລືອກເນື້ອໃນ, ແທນທີ່"
            .into(),
    );

    m.insert("help.search.title".into(), "ຄຳສັ່ງ SEARCH:".into());

    m.insert(
        "help.search.example1".into(),
        "gs fn main ## ຄົ້ນຫາ 'fn main' ໃນໄຟລ໌".into(),
    );

    m.insert(
        "help.search.example2".into(),
        "gs -i config ## ຜົນການຄົ້ນຫາ interactive".into(),
    );

    m.insert(
        "help.search.example3".into(),
        "fs *.rs ## ຊອກຫາໄຟລ໌ .rs".into(),
    );

    m.insert(
        "help.search.example4".into(),
        "ds src ## ຊອກຫາໄດເຣັກທໍຣີທີ່ຊື່ 'src'".into(),
    );

    m.insert(
        "help.search.example5".into(),
        "locate README ## ຄົ້ນຫາໄຟລ໌ & ໄດເຣັກທໍຣີທີ່ມີ 'README'".into(),
    );

    m.insert(
        "help.search.example6".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + ສະແດງແບບ grep".into(),
    );

    m.insert(
        "help.search.example7".into(),
        "gs TODO < app.log ## ຄົ້ນຫາເນື້ອໃນທີ່ pipe/redirect ເຂົ້າມາ".into(),
    );

    m.insert(
        "help.search.example8".into(),
        "gs error\\|warning ## ຄົ້ນຫາແບບ OR: ກົງກັບຄຳໃດຄຳໜຶ່ງ (a\\|b)".into(),
    );
}
