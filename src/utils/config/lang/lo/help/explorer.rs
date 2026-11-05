use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=help, b=parent dir, f=find (ສະແດງເສັ້ນທາງຜົນ)".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "g/:=ຄຳສັ່ງ (file|dir|line), /=filter ທັນທີ, e=editor".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "r=rename, d=delete, n=new file, m=new dir, i=hidden toggle".into(),
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

    m.insert("help.explorer.title".into(), "FILE EXPLORER (NFE):".into());

    m.insert(
        "help.explorer.example1".into(),
        "explorer ## ເປີດ file explorer ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## ເປີດ explorer ໃນ ~/projects".into(),
    );
}
