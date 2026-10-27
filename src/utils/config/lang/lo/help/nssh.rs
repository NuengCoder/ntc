use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nssh.title".into(), "ຄຳສັ່ງ NSSH:".into());

    m.insert(
        "help.nssh.add".into(),
        "nssh add <ຊື່> --host <ໂຮສ> ## ບັນທຶກໂປຣໄຟລ໌ SSH".into(),
    );

    m.insert(
        "help.nssh.ssh".into(),
        "nssh ssh [ຊື່] [--cmd \"ຄຳສັ່ງ\"] ## ເຊວຫຼືຄຳສັ່ງດຽວ".into(),
    );

    m.insert(
        "help.nssh.cp".into(),
        "nssh cp <ໂລຄອລ> <ຊື່>:<ຣີໂໝດ> ## ອັບໂຫຼດ/ດາວໂຫຼດ SFTP".into(),
    );

    m.insert(
        "help.nssh.scan".into(),
        "nssh scan <ໂຮສ> [--top N] ## ສະແກນພອດ TCP".into(),
    );

    m.insert(
        "help.nssh.manage".into(),
        "nssh ls|info|rm|edit ## ຈັດການໂປຣໄຟລ໌".into(),
    );

    m.insert(
        "help.nssh.keygen".into(),
        "nssh keygen ## ສ້າງຄີຜ່ານ ssh-keygen".into(),
    );

    m.insert(
        "help.nssh.example1".into(),
        "nssh add prod --host 203.0.113.10 --user root ## ບັນທຶກໂປຣໄຟລ໌".into(),
    );

    m.insert(
        "help.nssh.example2".into(),
        "nssh ssh prod --cmd \"uptime\" ## ຣັນຄຳສັ່ງຣີໂໝດ".into(),
    );

    m.insert(
        "help.nssh.example3".into(),
        "nssh cp ./app.tar.gz prod:/tmp/ ## ອັບໂຫຼດໄຟລ໌".into(),
    );
}
