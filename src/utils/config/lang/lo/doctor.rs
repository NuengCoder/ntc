use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("doctor.title".into(), "ntc doctor".into());

    m.insert("doctor.platform".into(), "ntc {} ({}) ເທິງ {}/{}".into());

    m.insert(
        "doctor.no_config_dir".into(),
        "ລະບົບນີ້ບໍ່ມີໄດເຣັກທໍຣີ config".into(),
    );

    m.insert(
        "doctor.config_new".into(),
        "ຍັງບໍ່ໄດ້ສ້າງໄດເຣັກທໍຣີ config: {} (ຈະສ້າງເມື່ອບັນທຶກຄັ້ງທຳອິດ)".into(),
    );

    m.insert("doctor.config_ok".into(), "ຕັ້ງຄ່າ: {}".into());

    m.insert(
        "doctor.config_unwritable".into(),
        "ໄດເຣັກທໍຣີ config ຂຽນບໍ່ໄດ້: {} ({})".into(),
    );

    m.insert("doctor.settings".into(), "ພາສາ: {}, ທີມ: {}".into());

    m.insert("doctor.install".into(), "{} ຢູ່ໃນ PATH: {}".into());

    m.insert(
        "doctor.install_missing".into(),
        "{} ບໍ່ຢູ່ໃນ PATH \u{2014} ມີພຽງໄດເຣັກທໍຣີ build ນີ້ທີ່ຮັນໄດ້".into(),
    );

    m.insert(
        "doctor.ran_none".into(),
        "ran: ບໍ່ມີ ranfile ໃນ {} (ບໍ່ມີຫຍັງໃຫ້ແລ່ນຢູ່ນີ້)".into(),
    );

    m.insert("doctor.ran_ok".into(), "ran: {} ມີ {} target".into());

    m.insert(
        "doctor.ran_env".into(),
        "ran: {} environment variable, {} ຕົວແປ".into(),
    );

    m.insert(
        "doctor.ran_problems".into(),
        "ran: {} ມີ {} ບັນຫາ \u{2014} ຣັນ `{}`".into(),
    );

    m.insert("doctor.ran_error".into(), "ran ຜິດພາດ: {}: {}".into());

    m.insert(
        "doctor.ntpkg_none".into(),
        "ntpkg: ຍັງບໍ່ໄດ້ຕິດຕັ້ງ runtime ໃດໆ".into(),
    );

    m.insert(
        "doctor.ntpkg_ok".into(),
        "ntpkg: ຕິດຕັ້ງ {} ເວີຊັນ, ໃຊ້ງານ {}".into(),
    );

    m.insert("doctor.ntpkg_error".into(), "ntpkg ຜິດພາດ: {}: {}".into());

    m.insert(
        "doctor.ntpkg_path".into(),
        "ntpkg: {} ໄດເຣັກທໍຣີ bin ທີ່ໃຊ້ງານຂາດຈາກ PATH".into(),
    );

    m.insert("doctor.ok".into(), "ບໍ່ພົບບັນຫາ".into());

    m.insert("doctor.issues".into(), "ມີ {} ຢ່າງທີ່ຄວນກວດ".into());
}
