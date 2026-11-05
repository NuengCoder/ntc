use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("up.help.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "up.help.opt_data".into(),
        " -d, --data <body> ອັບໂຫຼດຂໍ້ມູນ inline".into(),
    );

    m.insert(
        "up.help.opt_header".into(),
        " -H, --header <\"Key: Value\"> ເພີ່ມ request header (repeatable)".into(),
    );

    m.insert(
        "up.help.opt_help".into(),
        " -h, --help ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into(),
    );

    m.insert(
        "up.help.opt_post".into(),
        " --post ໃຊ້ POST (ຄ່າເລີ່ມຕົ້ນຂອງການອັບໂຫຼດ)".into(),
    );

    m.insert("up.help.options".into(), "Options:".into());

    m.insert("up.help.title".into(), "Upload (up)".into());

    m.insert("up.help.usage".into(), "Usage:".into());

    m.insert(
        "up.help.usage_data".into(),
        " up -d \"data\" <url> ອັບໂຫຼດຂໍ້ມູນ text inline".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        " up <file> <url> ອັບໂຫຼດໄຟລ໌ແບບ multipart POST".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <file> <url> --put --raw ສົ່ງໄບຕ໌ດິບດ້ວຍ PUT".into(),
    );

    m.insert(
        "up.missing_auth".into(),
        "ຂາດຂໍ້ມູນຢືນຢັນຫຼັງ -a (ຮູບແບບ user:pass)".into(),
    );

    m.insert("up.missing_bearer".into(), "ຂາດ token ຫຼັງ --bearer".into());

    m.insert(
        "up.missing_form".into(),
        "ຂາດ field ຫຼັງ -F (ຮູບແບບ name=value)".into(),
    );

    m.insert(
        "up.invalid_form".into(),
        "field ຟອມບໍ່ຖືກຕ້ອງ: {} (ຮູບແບບ name=value)".into(),
    );

    m.insert(
        "up.missing_timeout".into(),
        "ຂາດຈຳນວນວິນາທີຫຼັງ --timeout".into(),
    );

    m.insert("up.invalid_timeout".into(), "ເວລາໝົດອາຍຸບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "up.conflict".into(),
        "ໃຊ້ໄຟລ໌ ຫຼື ຂໍ້ມູນ -d ຢ່າງໃດຢ່າງໜຶ່ງເທົ່ານັ້ນ".into(),
    );

    m.insert(
        "up.fallback_retry".into(),
        "ເຊີບເວີຕອບ {} - ກຳລັງລອງໃໝ່ດ້ວຍ {}".into(),
    );

    m.insert(
        "up.help.opt_auth".into(),
        " -a, --auth <user:pass> ຂໍ້ມູນຢືນຢັນ Basic auth".into(),
    );

    m.insert(
        "up.help.opt_bearer".into(),
        " --bearer <token> Bearer token".into(),
    );

    m.insert(
        "up.help.opt_form".into(),
        " -F, --form <name=value> field multipart ເພີ່ມເຕີມ (repeatable)".into(),
    );

    m.insert(
        "up.help.opt_raw".into(),
        " --raw ສົ່ງໄບຕ໌ຂອງໄຟລ໌ເປັນ body".into(),
    );

    m.insert(
        "up.help.opt_put".into(),
        " --put ສົ່ງ PUT ແບບດິບແທນ multipart POST".into(),
    );

    m.insert(
        "up.help.opt_timeout".into(),
        " --timeout <ວິນາທີ> ເວລາໝົດອາຍຸ (ເລີ່ມຕົ້ນ 300)".into(),
    );

    m.insert(
        "up.help.opt_no_fallback".into(),
        " --no-fallback ບໍ່ລອງດ້ວຍວິທີຂົນສົ່ງອື່ນ".into(),
    );
}
