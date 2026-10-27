use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.size.care".into(),
        "size --care ## ຄຳນວນຂະໜາດລວມທຸກຢ່າງ (ໂໝດ care)".into(),
    );

    m.insert(
        "help.size.path".into(),
        "size -p, --path <path|@tp> ## ສະແດງຂະໜາດຂອງໄຟລ໌, path ຫຼື teleport savepoint ທີ່ລະບຸ".into(),
    );

    m.insert(
        "help.size.size".into(),
        "size ## ສະແດງຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.size.time".into(),
        "size -t, --time ## ສະແດງເວລາທີ່ໃຊ້".into(),
    );

    m.insert("help.size.title".into(), "ຄຳສັ່ງ SIZE:".into());

    m.insert(
        "help.size.example1".into(),
        "size ## ສະແດງຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.size.example2".into(),
        "size ../src ## ສະແດງຂະໜາດຂອງໄດເຣັກທໍຣີອື່ນ".into(),
    );

    m.insert(
        "help.size.example3".into(),
        "size Cargo.toml ## ສະແດງຂະໜາດຂອງໄຟລ໌".into(),
    );

    m.insert(
        "help.size.example4".into(),
        "size @web --care -t ## ໂໝດ care ພ້ອມເວລາທີ່ໃຊ້ສຳລັບ savepoint".into(),
    );
}
