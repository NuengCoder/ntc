use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.config.title".into(), "ຄຳສັ່ງ CONFIGURATION:".into());

    m.insert(
        "help.config.output".into(),
        "config output [path] ## ສະແດງ/ຕັ້ງ path ຜົນລັບ".into(),
    );

    m.insert(
        "help.config.depth".into(),
        "config depth [n] ## ສະແດງ/ຕັ້ງຄວາມເລິກສູງສຸດ (1-20)".into(),
    );

    m.insert(
        "help.config.linenumbers".into(),
        "config linenumbers ON|OFF ## ສະແດງ/ຕັ້ງເບີແຖວ".into(),
    );

    m.insert(
        "help.config.color".into(),
        "config color ON|OFF ## ສະແດງ/ຕັ້ງສີຜົນລັບ".into(),
    );

    m.insert(
        "help.config.threads".into(),
        "config threads [n] ## ສະແດງ/ຕັ້ງຈຳນວນ threads".into(),
    );

    m.insert(
        "help.config.history".into(),
        "config history [on|off|path] ## ສະແດງ/ຕັ້ງ history".into(),
    );

    m.insert(
        "help.config.autosuggest".into(),
        "config autosuggest ON|OFF ## ສະແດງ/ຕັ້ງ autosuggest (ghost text)".into(),
    );

    m.insert(
        "help.config.idle".into(),
        "config idle ON|OFF ## ສະແດງ/ຕັ້ງຄ່າຄຳແນະນຳຕອນທີ່ບໍ່ໄດ້ພິມ".into(),
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
        "help.config.intro_view".into(),
        "config intro_view ON|OFF ## ສະແດງ/ເຊື່ອງໂລໂກ້ຕອນເລີ່ມ, ແຈ້ງເຕືອນ ntpkg ແລະ watcher (ຊື່ຫຍໍ້: iv)"
            .into(),
    );

    m.insert(
        "help.config.file_view".into(),
        "config file_view ON|OFF ## ສະແດງໄຟລ໌ໃນແຜນຜົນເດີຍອັດຕະພາບ (alias: fv)".into(),
    );

    m.insert(
        "help.config.todo".into(),
        "config todo format <txt|md|json|toml|yaml|html> ## ຕັ້ງຮູບແບບໄຟລ໌ todo".into(),
    );

    m.insert(
        "help.config.size".into(),
        "config size all|file|dir ON|OFF ## ຄນນະງະນະຂະດັດໃນແຜນຜົນເດີຍ; ໃຊ້ --care (alias: s)".into(),
    );

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
        "help.config.qsc".into(),
        "config qsc \"#c 0\" ## ຕັ້ງ QSC global hotkey".into(),
    );

    m.insert(
        "help.config.qsc_off".into(),
        "config qsc off ## ປິດ QSC hotkey".into(),
    );

    m.insert(
        "help.config.lang".into(),
        "config lang [language] ## ສະແດງ/ຕັ້ງພາສາອິນເຕີເຟສ".into(),
    );

    m.insert(
        "help.config.env".into(),
        "config env NAME value [-g|-l|-os|-osg] ## ຕັ້ງຕົວແປສະພາບແວດລ້ອມ NTC (ຮອງຮັບ NAME=value; ບໍ່ໃສ່ຄ່າ: ສະແດງລາຍການ; ໃສ່ຊື່: ສະແດງຄ່າ)".into(),
    );

    m.insert(
        "help.config.env_os".into(),
        "  -g global | -l local | -os ເຊສຊັນນີ້ | -osg ບັນທຶກຖາວອນໃນ OS ## ຄ່າເລີ່ມຕົ້ນ: local ຖ້າມີ ntconfig.toml, ບໍ່ດັ່ງນັ້ນ global".into(),
    );

    m.insert(
        "help.config.rmenv".into(),
        "config rmenv NAME [-g|-l|-os|-osg|-all] ## ລຶບຕົວແປສະພາບແວດລ້ອມ NTC (-all: ທຸກບ່ອນເກັບ + ເຊສຊັນ)"
            .into(),
    );

    m.insert(
        "help.config.serve".into(),
        "config serve [ບໍລິການ] [--port-default ພອດ] [--bind-default ທີ່ຢູ່] ## ຄ່າເລີ່ມຕົ້ນ serve ລາຍບໍລິການ (CLI; `serve config` ແມ່ນ GUI ໃນເບົາເຊີ)".into(),
    );

    m.insert(
        "help.config.update_check".into(),
        "config update_check ON|OFF ## ສະແດງ/ເຊື່ອງແຈ້ງແຕ່ງເວີຊັນໃໝ່ລາຍວັນ (alias: uc)".into(),
    );

    m.insert(
        "help.config.integration_shell".into(),
        "config integration_shell ON|OFF ## ເຕີມຊື່ shell ທີ່ຕິດຕັ້ງ (powershell ເທິງ Windows, bash ເທິງ Linux desktop, cmd, …), ແຟລັກໂຕເອີ້ນ ແລະ ເປີດໂໝດ ntc ps ເທິງ Windows / ntc <distro> ເທິງ Linux (alias: is)".into(),
    );

    m.insert(
        "help.config.code_tools".into(),
        "config code_tools ON|OFF ## ເຕີມເຄື່ອງມືຂຽນໂປຣແກຣມ/ຄອມໄພເລີທີ່ຕິດຕັ້ງ (g++, python, java, cargo, …) ແລະ ແຟລັກ (alias: ct)".into(),
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
}
