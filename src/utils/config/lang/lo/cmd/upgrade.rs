use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.upgrade.after_restart".into(),
        "ເວີຊັນໃໝ່ຈະພ້ອມໃຊ້ຫຼັງຈາກ restart.".into(),
    );

    m.insert(
        "cmd.upgrade.ahead".into(),
        "ເວີຊັນທ້ອງຖິ່ນ ({}) ແມ່ນກ່ອນ release ລ່າສຸດ ({}).".into(),
    );

    m.insert(
        "cmd.upgrade.already_latest".into(),
        "ntc {} ເປັນເວີຊັນລ່າສຸດແລ້ວ.".into(),
    );

    m.insert(
        "cmd.upgrade.api_parse_fail".into(),
        "parse ການຕອບສະໜອງ GitHub API ບໍ່ສຳເລັດ".into(),
    );

    m.insert(
        "cmd.upgrade.api_read_fail".into(),
        "ອ່ານການຕອບສະໜອງ GitHub API ບໍ່ສຳເລັດ".into(),
    );

    m.insert(
        "cmd.upgrade.api_request_fail".into(),
        "ຄຳຂໍ GitHub API ລົ້ມເຫຼວ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.check_fail".into(),
        "ກວດຫາການອັບເດດບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.checking".into(),
        "ກຳລັງກວດຫາການອັບເດດ (ເວີຊັນປັດຈຸບັນ: {})...".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail".into(),
        "ສຳເນົາ binary ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail_android".into(),
        "ສຳເນົາ binary ບໍ່ສຳເລັດ. ກວດພື້ນທີ່ແຜ່ນ ຫຼື ສິດອະນຸຍາດ.".into(),
    );

    m.insert(
        "cmd.upgrade.dev_build".into(),
        "ທ່ານອາດຈະກຳລັງແລ່ນ development build.".into(),
    );

    m.insert(
        "cmd.upgrade.download_complete".into(),
        "ດາວໂຫຼດສຳເລັດ. ກຳລັງຕິດຕັ້ງການອັບເດດ...".into(),
    );

    m.insert("cmd.upgrade.download_fail".into(), "ດາວໂຫຼດລົ້ມເຫຼວ: {}".into());

    m.insert(
        "cmd.upgrade.downloading".into(),
        "ກຳລັງດາວໂຫຼດການອັບເດດ...".into(),
    );

    m.insert(
        "cmd.upgrade.downloading_url".into(),
        "ກຳລັງດາວໂຫຼດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.exe_path_fail".into(),
        "ໄດ້ path executable ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("cmd.upgrade.install_fail".into(), "ການຕິດຕັ້ງລົ້ມເຫຼວ: {}".into());

    m.insert(
        "cmd.upgrade.missing_tag".into(),
        "ຂາດ 'tag_name' ໃນການຕອບສະໜອງ GitHub release".into(),
    );

    m.insert("cmd.upgrade.new_version".into(), "ມີເວີຊັນໃໝ່: ntc {}".into());

    m.insert(
        "cmd.upgrade.no_assets_release".into(),
        "ບໍ່ພົບ assets ໃນ release v{}".into(),
    );

    m.insert(
        "cmd.upgrade.perm_fail".into(),
        "ຕັ້ງສິດ executable ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.progress_bytes".into(),
        "ດາວໂຫຼດແລ້ວ: {} bytes".into(),
    );

    m.insert("cmd.upgrade.progress_pct".into(), "Progress: {}%".into());

    m.insert(
        "cmd.upgrade.restart".into(),
        "restart ntc ເພື່ອໃຊ້ເວີຊັນໃໝ່.".into(),
    );

    m.insert(
        "cmd.upgrade.stream_fail".into(),
        "ອ່ານ download stream ບໍ່ສຳເລັດ".into(),
    );

    m.insert(
        "cmd.upgrade.sudo_hint".into(),
        "ແລ່ນດ້ວຍສິດ sudo/administrator".into(),
    );

    m.insert(
        "cmd.upgrade.time_remaining".into(),
        "ເວລາທີ່ເຫຼືອ: {:>3} seconds  ".into(),
    );

    m.insert(
        "cmd.upgrade.tmpdir_fail".into(),
        "ສ້າງ temp directory ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.unsupported_platform".into(),
        "platform ບໍ່ຮອງຮັບ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.updated".into(),
        "ອັບເດດ ntc ເປັນ v{} ແລ້ວ!".into(),
    );

    m.insert(
        "cmd.upgrade.write_fail".into(),
        "ຂຽນໄຟລ໌ທີ່ດາວໂຫຼດມາບໍ່ສຳເລັດ".into(),
    );

    m.insert("cmd.upgrade.extracting".into(), "ກຳລັງແຍກ binary...".into());

    m.insert(
        "cmd.upgrade.extract_fail".into(),
        "ບໍ່ສາມາດອ່ານ/ແຍກໄຟລ໌ທີ່ດາວໂຫຼດ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_file_in_archive".into(),
        "ບໍ່ພົບ '{}' ໃນໄຟລ໌ທີ່ດາວໂຫຼດ".into(),
    );

    m.insert(
        "cmd.upgrade.no_installer_asset".into(),
        "ບໍ່ພົບ Windows installer ສຳລັບອັບເດດ ntc".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch".into(),
        "ກຳລັງເປີດຕົວຕິດຕັ້ງ ntc (ເງິບ)...".into(),
    );

    m.insert(
        "cmd.upgrade.installer_launch_fail".into(),
        "ບໍ່ສາມາດເປີດຕົວຕິດຕັ້ງ: {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_arch_asset".into(),
        "ບໍ່ມີ binary ntc ສຳລັບสถาปัตยกรรมນີ້ ({}) ສ້າງຈາກ source: cargo build --release".into(),
    );

    m.insert(
        "cmd.upgrade.no_asset_match".into(),
        "ບໍ່ພົບ asset ທີ່ກົງກັບ platform key '{}'".into(),
    );

    m.insert(
        "cmd.upgrade.installed_user_dir".into(),
        "ຕິດຕັ້ງ ntc ໄປທີ່ {} (ຕິດຕັ້ງສຳລັບຜູ້ໃຊ້, ບໍ່ຕ້ອງການ admin/sudo).".into(),
    );

    m.insert(
        "cmd.upgrade.home_fail".into(),
        "ບໍ່ສາມາດກຳນົດ home directory".into(),
    );

    m.insert(
        "cmd.upgrade.path_exported".into(),
        "ອັບເດດ PATH ໃນ {}".into(),
    );
}
