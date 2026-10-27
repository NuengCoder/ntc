use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.ran.deinit".into(),
        "ran deinit ## ລຶບ NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.help".into(),
        "ran help ## ສະແດງການໃຊ້ງານ ran".into(),
    );

    m.insert(
        "help.ran.init".into(),
        "ran init ## ສ້າງ NTCRANFILE.toml template".into(),
    );

    m.insert(
        "help.ran.list".into(),
        "ran list ## ລາຍຊື່ targets ທັງໝົດໃນ NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.ran.run".into(),
        "ran ## ເຮັດວຽກ target 'standalone' (default)".into(),
    );

    m.insert(
        "help.ran.target".into(),
        "ran <target> ## ເຮັດວຽກ target ສະເພາະຈາກ NTCRANFILE.toml".into(),
    );

    m.insert("help.ran.title".into(), "ຄຳສັ່ງ RAN / TASK RUNNER:".into());

    m.insert(
        "help.ran.example1".into(),
        "ran ## ເຮັດວຽກ standalone target ເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.ran.example2".into(),
        "ran build ## ເຮັດວຽກ target 'build'".into(),
    );

    m.insert(
        "help.ran.example3".into(),
        "ran list ## ລາຍຊື່ targets ທັງໝົດ".into(),
    );
}
