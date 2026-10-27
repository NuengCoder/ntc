use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.teleport.add".into(),
        "tp add <name> ## ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.teleport.add_path".into(),
        "tp add <name> <path> ## ບັນທຶກ path ສະເພາະ".into(),
    );

    m.insert(
        "help.teleport.at".into(),
        "@<name> ## ທາງລັດ teleport ດ່ວນ".into(),
    );

    m.insert(
        "help.teleport.cls".into(),
        "tp cls ## ລຶບ savepoints ທັງໝົດ".into(),
    );

    m.insert(
        "help.teleport.info".into(),
        "tp info <name> ## ສະແດງລາຍລະອຽດ savepoint".into(),
    );

    m.insert(
        "help.teleport.jump".into(),
        "tp jump <name> ## Teleport ໄປຕຳແໜ່ງທີ່ບັນທຶກ".into(),
    );

    m.insert(
        "help.teleport.jump_num".into(),
        "tp jump <number> ## Teleport ຕາມເບີ".into(),
    );

    m.insert(
        "help.teleport.list".into(),
        "tp list ## ລາຍຊື່ຈຸດ teleport ທັງໝົດ".into(),
    );

    m.insert("help.teleport.menu".into(), "tp ## ເມນູ interactive".into());

    m.insert(
        "help.teleport.rm".into(),
        "tp rm <name> ## ລຶບຈຸດ teleport".into(),
    );

    m.insert(
        "help.teleport.rnm".into(),
        "tp rnm <old> to <new> ## ປ່ຽນຊື່ savepoint".into(),
    );

    m.insert("help.teleport.title".into(), "ຄຳສັ່ງ TELEPORT:".into());

    m.insert(
        "help.teleport.tpb".into(),
        "tpb ## Teleport ກັບໄປຕຳແໜ່ງກ່ອນໜ້າ (undo tp)".into(),
    );

    m.insert(
        "help.teleport.tpb_clear".into(),
        "tpb clear ## ລຶບ teleport history".into(),
    );

    m.insert(
        "help.teleport.tpb_history".into(),
        "tpb history ## ສະແດງ teleport navigation history".into(),
    );

    m.insert(
        "help.teleport.example1".into(),
        "tp add web ## ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນເປັນ 'web'".into(),
    );

    m.insert(
        "help.teleport.example2".into(),
        "tp add docs ~/Documents ## ບັນທຶກ path ສະເພາະ".into(),
    );

    m.insert(
        "help.teleport.example3".into(),
        "tp jump web ## Teleport ໄປທີ່ 'web'".into(),
    );

    m.insert(
        "help.teleport.example4".into(),
        "tp list ## ລາຍຊື່ savepoints ທັງໝົດ".into(),
    );

    m.insert(
        "help.teleport.example5".into(),
        "@web ## ທາງລັດ teleport ດ່ວນ".into(),
    );
}
