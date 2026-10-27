use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.other.clear".into(), "clear ## ລ້າງໜ້າຈໍ".into());

    m.insert(
        "help.other.deinit".into(),
        "deinit [--ran|--local] ## ລຶບໄຟລ໌ໂຄງການ".into(),
    );

    m.insert("help.other.exit".into(), "exit, quit ## ອອກຈາກ ntc".into());

    m.insert("help.other.help".into(), "help ## ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert("help.other.init".into(), "init [--ran|--local|--opencode] ## ສ້າງໄຟລ໌ໂຄງການ (NTCRANFILE.toml, ntconfig.toml, .opencode/)".into());

    m.insert(
        "help.other.print".into(),
        "print <text> [--loop N] [-n|-t] ## alias ຂອງ say".into(),
    );

    m.insert(
        "help.other.say".into(),
        "say <text> [--loop N] [-n|-t] ## ພິມຂໍ້ຄວາມ (ບໍ່ redirect ໄຟລ໌)".into(),
    );

    m.insert("help.other.title".into(), "ຄຳສັ່ງ OTHER:".into());

    m.insert(
        "help.other.tutorial".into(),
        "tutorial ## ເລີ່ມ tutorial ຂັ້ນຕອນ interactive".into(),
    );

    m.insert("help.other.ui".into(), "ui ## ສະແດງໂໝດ UI ປັດຈຸບັນ".into());

    m.insert("help.other.version".into(), "version ## ສະແດງເວີຊັນ".into());

    m.insert(
        "help.other.example1".into(),
        "init --ran ## ສ້າງ NTCRANFILE.toml".into(),
    );

    m.insert("help.other.example2".into(), "clear ## ລ້າງໜ້າຈໍ".into());

    m.insert(
        "help.other.example3".into(),
        "version ## ສະແດງຂໍ້ມູນເວີຊັນ".into(),
    );

    m.insert(
        "help.other.example4".into(),
        "tutorial ## ເລີ່ມ tutorial interactive".into(),
    );
}
