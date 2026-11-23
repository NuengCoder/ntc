use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.example1".into(),
        "explorer ## ເປີດ file explorer ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## ເປີດ explorer ໃນ ~/projects".into(),
    );

    m.insert(
        "help.explorer.hint1".into(),
        "h=help, b=ໄດເຣັກທໍຣີແມ່, f=ຄົ້ນຫາ (ສະແດງເສັ້ນທາງຜົນ)".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "g/:=ຄຳສັ່ງ (file|dir|line), /=filter ທັນທີ, e=editor".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "r=ປ່ຽນຊື່, d=ລຶບ, n=ໄຟລ໌ໃໝ່, m=ໄດເຣັກທໍຣີໃໝ່, i=ສະຫຼັບການເຊື່ອງ".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s=sort mode, S=sort direction (ລວມໂຟນເດີ), F5=refresh".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "find: ພິມ=ຄົ້ນຫາ, Tab=hjkl/ລູກສອນ, Enter=ເປີດ, Esc=ກັບ".into(),
    );

    m.insert(
        "help.explorer.launch".into(),
        "explorer, nfe [PATH] ## ເປີດ terminal file explorer ເຕັມຈໍ".into(),
    );

    m.insert("help.explorer.title".into(), "ຕົວສຳຫຼວດໄຟລ໌ (NFE):".into());
}
