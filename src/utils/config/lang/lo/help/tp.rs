use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tp.title".into(), "ຄຳສັ່ງ TELEPORT:".into());

    m.insert("help.tp.menu".into(), "tp ## ເມນູ interactive".into());

    m.insert(
        "help.tp.add".into(),
        "tp add <name> ## ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.tp.add_path".into(),
        "tp add <name> <path> ## ບັນທຶກ path ສະເພາະ".into(),
    );

    m.insert(
        "help.tp.jump".into(),
        "tp jump <name> ## Teleport ໄປຕຳແໜ່ງທີ່ບັນທຶກ".into(),
    );

    m.insert(
        "help.tp.jump_num".into(),
        "tp jump <number> ## Teleport ຕາມເບີ".into(),
    );

    m.insert(
        "help.tp.list".into(),
        "tp list ## ລາຍຊື່ຈຸດ teleport ທັງໝົດ".into(),
    );

    m.insert(
        "help.tp.info".into(),
        "tp info <name> ## ສະແດງລາຍລະອຽດ savepoint".into(),
    );

    m.insert("help.tp.rm".into(), "tp rm <name> ## ລຶບຈຸດ teleport".into());

    m.insert(
        "help.tp.rnm".into(),
        "tp rnm <old> to <new> ## ປ່ຽນຊື່ savepoint".into(),
    );

    m.insert("help.tp.cls".into(), "tp cls ## ລຶບ savepoints ທັງໝົດ".into());

    m.insert("help.tp.at".into(), "@<name> ## ທາງລັດ teleport ດ່ວນ".into());

    m.insert(
        "help.tp.example1".into(),
        "tp add web ## ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນເປັນ 'web'".into(),
    );

    m.insert(
        "help.tp.example2".into(),
        "tp add docs ~/Documents ## ບັນທຶກ path ສະເພາະ".into(),
    );

    m.insert(
        "help.tp.example3".into(),
        "tp jump web ## Teleport ໄປທີ່ 'web'".into(),
    );

    m.insert(
        "help.tp.example4".into(),
        "tp list ## ລາຍຊື່ savepoints ທັງໝົດ".into(),
    );

    m.insert(
        "help.tp.example5".into(),
        "@web ## ທາງລັດ teleport ດ່ວນ".into(),
    );
}
