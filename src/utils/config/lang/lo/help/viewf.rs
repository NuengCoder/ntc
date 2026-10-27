use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.viewf.title".into(), "ຄຳສັ່ງ VIEWF (ສະເພາະໄຟລ໌):".into());

    m.insert(
        "help.viewf.tree".into(),
        "viewf [<dir|@tp|@tp/dir/>] ## ສະແດງ tree ສະເພາະໄຟລ໌ທີ່ depth 1".into(),
    );

    m.insert("help.viewf.file_usage".into(), "viewf <file|@tp/file|@tp/dir/file> [--head N|--tail N|--betw A:B] ## ສະແດງເນື້ອໃນໄຟລ໌ (ຄື txtf)".into());

    m.insert(
        "help.viewf.size".into(),
        "viewf -s, --size [min-size] ## ສະແດງຂະໜາດໄຟລ໌; ໃສ່ຄ່າຈະເຊື່ອງໄຟລ໌ນ້ອຍກວ່າ".into(),
    );

    m.insert(
        "help.viewf.sort".into(),
        "viewf --sort size|char|ext ## ຈັດລຳດັບລາຍການໄຟລ໌".into(),
    );

    m.insert(
        "help.viewf.ext".into(),
        "viewf -e, --ext <ext...> ## ສະແດງສະເພາະນາມສະກຸນທີ່ຕົງ".into(),
    );

    m.insert(
        "help.viewf.care".into(),
        "viewf -c, --care ## ນັບທຸກຢ່າງເມື່ອຄິດຂະໜາດ".into(),
    );

    m.insert(
        "help.viewf.time".into(),
        "viewf -t, --time ## ສະແດງເວລາດຳເນີນການ".into(),
    );

    m.insert(
        "help.viewf.copy".into(),
        "viewf -cp, --copy ## ສຳເນົາລາຍການໄຟລ໌ໃສ່ clipboard".into(),
    );

    m.insert(
        "help.viewf.example1".into(),
        "viewf ## ສະແດງໄຟລ໌ໃນ directory ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.viewf.example2".into(),
        "viewf src ## ສະແດງໄຟລ໌ໃນ src".into(),
    );

    m.insert(
        "help.viewf.example3".into(),
        "viewf -e rs,txt --sort size ## ກັ່ນນາມສະກຸນແລະຈັດຕາມຂະໜາດ".into(),
    );

    m.insert(
        "help.viewf.example4".into(),
        "viewf @docs/readme.md --head 20 ## ສະແດງໄຟລ໌ໃນ teleport".into(),
    );
}
