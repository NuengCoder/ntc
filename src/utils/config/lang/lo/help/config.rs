use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest ON|OFF ## ສະແດງ/ຕັ້ງ autosuggest (ghost text)".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color ON|OFF ## ສະແດງ/ຕັ້ງສີຜົນລັບ".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## ສະແດງ/ຕັ້ງຄວາມເລິກສູງສຸດ (1-20)".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|path] ## ສະແດງ/ຕັ້ງ history".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [language] ## ສະແດງ/ຕັ້ງພາສາອິນເຕີເຟສ".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers ON|OFF ## ສະແດງ/ຕັ້ງເບີແຖວ".into(),
    );

    m.insert(
        "help.config.local_deinit".into(),
        "local deinit ## ລຶບ ntconfig.toml ອອກຈາກໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.config.local_help".into(),
        "local help ## ສະແດງການໃຊ້ງານຄຳສັ່ງ local".into(),
    );

    m.insert(
        "help.config.local_init".into(),
        "local init ## ສ້າງ ntconfig.toml template (commented)".into(),
    );

    m.insert(
        "help.config.opencg".into(),
        "opencg ## ເປີດ config.toml ໃນ editor".into(),
    );

    m.insert(
        "help.config.output".into(),
        "config output [path] ## ສະແດງ/ຕັ້ງ path ຜົນລັບ".into(),
    );

    m.insert(
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## ຕັ້ງ QSC global hotkey".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## ປິດ QSC hotkey".into(),
    );

    m.insert(
        "help.config.resetcg".into(),
        "resetcg ## ຣີເຊັດ config ເປັນຄ່າເລີ່ມຕົ້ນ (ພ້ອມຕົວເລືອກ backup)".into(),
    );

    m.insert(
        "help.config.restorecg".into(),
        "restorecg ## ກູ້ຄືນ config ຈາກ backup".into(),
    );

    m.insert(
        "help.config.showcg".into(),
        "showcg ## ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.config.showenv".into(),
        "showenv ## ສະແດງ environment variables (PATH entries ທຳອິດ 5)".into(),
    );

    m.insert(
        "help.config.showenv_all".into(),
        "showenv --all ## ສະແດງ PATH entries ທັງໝົດ".into(),
    );

    m.insert(
        "help.config.showenv_d".into(),
        "showenv -d <n> ## ສະແດງ PATH entries ທຳອິດ n".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## ສະແດງ/ຕັ້ງຈຳນວນ threads".into(),
    );

    m.insert("help.config.title".into(), "ຄຳສັ່ງ CONFIGURATION:".into());

    m.insert(
        "help.config.watch".into(),
        "config watch ON|OFF ## ເປີດ/ປິດໃຊ້ງານ file watcher".into(),
    );

    m.insert(
        "help.config.watch_trigger".into(),
        "config watch trigger <a> ## ເຮັດວຽກ alias ອັດຕະໂນມັດເມື່ອ watcher ເຮັດວຽກ".into(),
    );

    m.insert(
        "help.config.watch_trigger_off".into(),
        "config watch trigger off ## ປິດການ auto-run".into(),
    );

    m.insert(
        "help.config.watch_limit".into(),
        "config watch limit <n> ## ຈຳນວນແຖວທີ່ສະແດງໃນກ່ອງ watch".into(),
    );

    m.insert(
        "help.config.wi".into(),
        "wi [path] ## ຟีดການປ່ຽນແປງໄຟລ໌ແບບສົດ (q=ອອກ)".into(),
    );

    m.insert(
        "help.config.example1".into(),
        "config output ~/Desktop ## ຕັ້ງ path ຜົນລັບເປັນ Desktop".into(),
    );

    m.insert(
        "help.config.example2".into(),
        "config depth 3 ## ຕັ້ງຄວາມເລິກສູງສຸດເປັນ 3".into(),
    );

    m.insert(
        "help.config.example3".into(),
        "config color ON ## ເປີດການສະແດງສີ".into(),
    );

    m.insert(
        "help.config.example4".into(),
        "config threads 8 ## ຕັ້ງຈຳນວນ threads ເປັນ 8".into(),
    );

    m.insert(
        "help.config.example5".into(),
        "showcg ## ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.config.git".into(),
        "config git ON|OFF ## ເປີດ/ປິດ git integration (prompt + ສັນຍາລັກ)".into(),
    );

    m.insert(
        "help.config.dir_view".into(),
        "config dir_view ON|OFF ## ສະແດງ/ເຊື່ອງຕົ້ນໄມ້ໃນ shell".into(),
    );

    m.insert(
        "help.config.file_view".into(),
        "config file_view ON|OFF ## ສະແດງໄຟລ໌ໃນແຜນຜົນເດີຍອັດຕະພາບ (alias: fv)".into(),
    );

    m.insert(
        "help.config.size".into(),
        "config size all|file|dir ON|OFF ## ຄນນະງະນະຂະດັດໃນແຜນຜົນເດີຍ; ໃຊ້ --care (alias: s)".into(),
    );

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## ສະແດງ/ເຊື່ອງແຈ້ງແຕ່ງເວີຊັນໃໝ່ລາຍວັນ (alias: uc)".into(),
    );

    m.insert("help.config.serve".into(), "config serve [ບໍລິການ] [--port-default ພອດ] [--bind-default ທີ່ຢູ່] ## ຄ່າເລີ່ມຕົ້ນ serve ລາຍບໍລິການ (alias: serve config)".into());
}
