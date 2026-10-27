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

    m.insert("up.help.opt_post".into(), " --post ໃຊ້ POST ແທນ PUT".into());

    m.insert("up.help.options".into(), "Options:".into());

    m.insert("up.help.title".into(), "Upload (up)".into());

    m.insert("up.help.usage".into(), "Usage:".into());

    m.insert(
        "up.help.usage_data".into(),
        " up -d \"data\" <url> ອັບໂຫຼດຂໍ້ມູນ text inline".into(),
    );

    m.insert(
        "up.help.usage_line".into(),
        " up <file> <url> ອັບໂຫຼດໄຟລ໌ຜ່ານ PUT".into(),
    );

    m.insert(
        "up.help.usage_post".into(),
        " up <file> <url> --post ອັບໂຫຼດຜ່ານ POST".into(),
    );
}
