use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.local.title".into(), "LOCAL ຄຳສັ່ງ:".into());

    m.insert(
        "help.local.init".into(),
        "local init ## ສ້າງ ntconfig.toml template (commented)".into(),
    );

    m.insert(
        "help.local.help".into(),
        "local help ## ສະແດງການໃຊ້ງານຄຳສັ່ງ local".into(),
    );

    m.insert(
        "help.local.deinit".into(),
        "local deinit ## ລຶບ ntconfig.toml ອອກຈາກໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.local.example1".into(),
        "local init ## ສ້າງ template ntconfig.toml ຢູ່ນີ້".into(),
    );
}
