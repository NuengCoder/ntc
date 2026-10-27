use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.theme.add".into(),
        "theme add <name> ## ສ້າງ theme ໃໝ່ (ເປີດໃນ editor)".into(),
    );

    m.insert(
        "help.theme.current".into(),
        "theme current ## ສະແດງຊື່ ແລະ ລາຍລະອຽດ theme ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.theme.edit".into(),
        "theme edit [name] ## ແກ້ໄຂ theme (ຫຼື ເປີດໄດເຣັກທໍຣີ theme)".into(),
    );

    m.insert(
        "help.theme.export".into(),
        "theme export <name> ## ສົ່ງອອກ theme ເປັນໄຟລ໌ .ntc_theme".into(),
    );

    m.insert(
        "help.theme.import".into(),
        "theme import <file> ## ນຳເຂົ້າ theme ຈາກໄຟລ໌ .ntc_theme".into(),
    );

    m.insert(
        "help.theme.info".into(),
        "theme info <name> ## ສະແດງ metadata theme ແລະ ຈຳນວນສີ".into(),
    );

    m.insert(
        "help.theme.interactive".into(),
        "theme ## ຕົວຈັດການ theme interactive (ເມນູ)".into(),
    );

    m.insert(
        "help.theme.list".into(),
        "theme list ## ລາຍຊື່ themes ທີ່ມີທັງໝົດ".into(),
    );

    m.insert(
        "help.theme.make".into(),
        "theme make [name] ## ສ້າງ theme template ໃນ CWD".into(),
    );

    m.insert(
        "help.theme.reload".into(),
        "theme reload ## ໂຫຼດ themes ຄືນໃໝ່ຈາກ disk".into(),
    );

    m.insert("help.theme.rm".into(), "theme rm <name> ## ລຶບ theme".into());

    m.insert(
        "help.theme.rnm".into(),
        "theme rnm <old> to <new> ## ປ່ຽນຊື່ theme".into(),
    );

    m.insert(
        "help.theme.switch".into(),
        "theme <name> ## ສະຫຼັບໄປ theme ຕາມຊື່".into(),
    );

    m.insert("help.theme.title".into(), "ຄຳສັ່ງ THEME:".into());

    m.insert(
        "help.theme.where".into(),
        "theme where ## ສະແດງຕຳແໜ່ງໄດເຣັກທໍຣີ theme".into(),
    );

    m.insert(
        "help.theme.example1".into(),
        "theme list ## ລາຍຊື່ themes ທີ່ມີທັງໝົດ".into(),
    );

    m.insert(
        "help.theme.example2".into(),
        "theme dark ## ສະຫຼັບໄປ theme 'dark'".into(),
    );

    m.insert(
        "help.theme.example3".into(),
        "theme current ## ສະແດງຂໍ້ມູນ theme ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.theme.example4".into(),
        "theme add mytheme ## ສ້າງ theme ໃໝ່".into(),
    );
}
