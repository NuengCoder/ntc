use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.config.local.init_all_removed".into(), "'local init --all' ຖືກລຶກແລ້ວ - local config ບໍ່ສາມາດຄັດລອກການຕັ້ງຄ່າ global ໄດ້ອີແລ້ວ ໃຊ້ 'local init' ເພື່ອສ້າງ template".into());

    m.insert(
        "cmd.config.local.precedence_hint".into(),
        "ລາຍກาร local ມີຄວາມສຳຄັນກວ່າ ລາຍກาร global ຍັງໃຊ້ໄດ້ ແລະ config global ຈະບໍ່ຖືກແກ້ໄຂ".into(),
    );

    m.insert(
        "cmd.config.deinit.confirm".into(),
        "ລຶບ {} ບໍ? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.deinit.local_not_found".into(),
        "ບໍ່ພົບ ntconfig.toml, ຂ້າມໄປ.".into(),
    );

    m.insert(
        "cmd.config.deinit.local_remove_failed".into(),
        "ລຶບ ntconfig.toml ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.config.deinit.local_removed".into(),
        "ລຶບ ntconfig.toml ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.config.deinit.not_found".into(),
        "ບໍ່ພົບ {}, ຂ້າມໄປ.".into(),
    );

    m.insert(
        "cmd.config.deinit.remove_failed".into(),
        "ລຶບ {} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("cmd.config.deinit.removed".into(), "ລຶບ {} ແລ້ວ".into());

    m.insert("cmd.config.deinit.skipped".into(), "ຂ້າມໄປ.".into());

    m.insert(
        "cmd.config.deinit.usage".into(),
        "ວິທີໃຊ້: deinit [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.deinit.opencode_not_found".into(),
        "ບໍ່ພົບ scaffold .opencode, ຂ້າມໄປ.".into(),
    );

    m.insert(
        "cmd.config.example_depth".into(),
        " config depth 4  # ຕັ້ງຄວາມເລິກຕົ້ນໄມ້ສູງສຸດເປັນ 4".into(),
    );

    m.insert(
        "cmd.config.example_history".into(),
        " config history off  # ປິດໃຊ້ງານ history".into(),
    );

    m.insert(
        "cmd.config.example_linenumbers".into(),
        " config linenumbers ON  # ເປີດເບີແຖວໃນການສະແດງໄຟລ໌".into(),
    );

    m.insert(
        "cmd.config.example_output".into(),
        " config output C:\\reports  # ຕັ້ງ path ຜົນລັບ".into(),
    );

    m.insert(
        "cmd.config.example_watch".into(),
        " config watch ON  # ເປີດໃຊ້ງານ file watcher".into(),
    );

    m.insert("cmd.config.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.config.init.create_failed".into(),
        "ສ້າງ {} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("cmd.config.init.created".into(), "ສ້າງ {} ແລ້ວ".into());

    m.insert(
        "cmd.config.init.exists_warn".into(),
        "{} ມີຢູ່ແລ້ວ, ຂ້າມໄປ.".into(),
    );

    m.insert(
        "cmd.config.init.local_exists".into(),
        "ntconfig.toml ມີຢູ່ແລ້ວ, ຂ້າມໄປ.".into(),
    );

    m.insert(
        "cmd.config.init.usage".into(),
        "ວິທີໃຊ້: init [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.init.opencode_ready".into(),
        "OpenCode scaffold ພ້ອມແລ້ວ: {}".into(),
    );

    m.insert("cmd.config.lang.active".into(), "{} (active)".into());

    m.insert("cmd.config.lang.available".into(), "ພາສາທີ່ມີ:".into());

    m.insert(
        "cmd.config.lang.coming_soon".into(),
        "{} (set, coming soon)".into(),
    );

    m.insert("cmd.config.lang.set".into(), "ຕັ້ງພາສາເປັນ: {}".into());

    m.insert(
        "cmd.config.lang.set_coming_soon".into(),
        "ຕັ້ງພາສາເປັນ: {} (coming soon)".into(),
    );

    m.insert(
        "cmd.config.lang.unsupported".into(),
        "ພາສາ '{}' ບໍ່ຮອງຮັບ. ທີ່ມີ: {}".into(),
    );

    m.insert(
        "cmd.config.local.create_failed".into(),
        "ສ້າງ ntconfig.toml ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.config.local.created_template".into(),
        "ສ້າງ ntconfig.toml template ໃນ: {}".into(),
    );

    m.insert(
        "cmd.config.local.exists_warning".into(),
        "\u{26a0}\u{fe0f} ntconfig.toml ມີຢູ່ແລ້ວ!".into(),
    );

    m.insert(
        "cmd.config.local.init_cancelled".into(),
        "ຍົກເລີກການເລີ່ມຕົ້ນ.".into(),
    );

    m.insert(
        "cmd.config.local.overwrite_prompt".into(),
        "ຂຽນທັບ? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local.template_edit_hint".into(),
        " ແກ້ໄຂໄຟລ໌ ແລະ ລຶບ '#' ອອກຈາກແຖວທີ່ຕ້ອງການເປີດໃຊ້".into(),
    );

    m.insert(
        "cmd.config.local.template_hint".into(),
        "\u{1f4dd} ນີ້ແມ່ນ COMMENTED TEMPLATE - ຍັງບໍ່ມີການຕັ້ງຄ່າທີ່ເປີດໃຊ້".into(),
    );

    m.insert(
        "cmd.config.local.tip_global".into(),
        "\u{1f4a1} ຄຳແນະນຳ: ການຕັ້ງຄ່າ global (teleports, output path, depth) ຍັງຄົງ global".into(),
    );

    m.insert(
        "cmd.config.local_deinit.cancelled".into(),
        "ຍົກເລີກການຢຸດໃຊ້ງານ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.confirm".into(),
        "ລຶບ ntconfig.toml ບໍ? (y/N): ".into(),
    );

    m.insert(
        "cmd.config.local_deinit.done".into(),
        "ລຶບ ntconfig.toml ແລ້ວ. ການຕັ້ງຄ່າ global ຈະເປັນການຕັ້ງຄ່າທີ່ເປີດໃຊ້ອັນດຽວ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.global_safe".into(),
        "ການຕັ້ງຄ່າ global ຈະບໍ່ຖືກກະທົບ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.lose_settings".into(),
        "ການຕັ້ງຄ່າ ignore/care ທ້ອງຖິ່ນ ແລະ run alias ຈະຖືກລຶບໄປ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.not_found".into(),
        "ບໍ່ພົບ ntconfig.toml ໃນໄດເຣັກທໍຣີປັດຈຸບັນ.".into(),
    );

    m.insert(
        "cmd.config.local_deinit.remove_failed".into(),
        "ລຶບ ntconfig.toml ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.config.local_deinit.title".into(),
        "\u{26A0}\u{FE0F} ການລຶບການຕັ້ງຄ່າທ້ອງຖິ່ນ".into(),
    );

    m.insert(
        "cmd.config.local_deinit.will_delete".into(),
        "ອັນນີ້ຈະລຶບ: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.created".into(),
        "ສ້າງໄຟລ໌ config ທີ່: {}".into(),
    );

    m.insert(
        "cmd.config.opencg.opening".into(),
        "ກຳລັງເປີດ config: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.disabled".into(),
        "ປິດໃຊ້ງານ QSC hotkey ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.config.qsc.hotkey_set".into(),
        "ຕັ້ງ QSC hotkey ເປັນ: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.hotkey_status".into(),
        "QSC hotkey: {}".into(),
    );

    m.insert(
        "cmd.config.qsc.invalid_format".into(),
        "ຮູບແບບ QSC ບໍ່ຖືກຕ້ອງ: '{}'. ໃຊ້ແບບ '#c #s 0'.".into(),
    );

    m.insert("cmd.config.qsc.not_configured".into(), "ຍັງບໍ່ໄດ້ຕັ້ງຄ່າ".into());

    m.insert(
        "cmd.config.qsc.usage1".into(),
        "ວິທີໃຊ້: config qsc \"#c 0\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage2".into(),
        " config qsc \"#c #s a\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage3".into(),
        " config qsc \"#c #s #a F1\"".into(),
    );

    m.insert("cmd.config.qsc.usage4".into(), " config qsc off".into());

    m.insert(
        "cmd.config.resetcg.backup_created".into(),
        "ສ້າງ Backup ແລ້ວ: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_failed".into(),
        "ສ້າງ backup ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_info".into(),
        "ບັນທຶກ backup ແລ້ວ. ໃຊ້ 'restorecg' ເພື່ອກູ້ຄືນຖ້າຈຳເປັນ.".into(),
    );

    m.insert(
        "cmd.config.resetcg.cancelled".into(),
        "ຍົກເລີກການຣີເຊັດ.".into(),
    );

    m.insert(
        "cmd.config.resetcg.config_file_label".into(),
        "ໄຟລ໌ Config:".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_aliases".into(),
        " \u{2022} Run aliases \u{2192} ລຶບແລ້ວ".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_custom_ignores".into(),
        " \u{2022} ignores ທີ່ກຳນົດເອງ \u{2192} ລຶບແລ້ວ".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_depth".into(),
        " \u{2022} ຄວາມເລິກສູງສຸດ \u{2192} 2".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_history".into(),
        " \u{2022} History \u{2192} OFF".into(),
    );

    m.insert("cmd.config.resetcg.default_ignored_dirs".into(), " \u{2022} ໄດເຣັກທໍຣີທີ່ຖືກ ignore \u{2192} target, build, venv, node_modules, installer, logs, .git".into());

    m.insert(
        "cmd.config.resetcg.default_linenumbers".into(),
        " \u{2022} ເບີແຖວ \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_output".into(),
        " \u{2022} path ຜົນລັບ \u{2192} Desktop".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_teleports".into(),
        " \u{2022} Teleports \u{2192} ລຶບແລ້ວ".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_threads".into(),
        " \u{2022} Threads \u{2192} 4".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_watcher".into(),
        " \u{2022} File watcher \u{2192} OFF".into(),
    );

    m.insert(
        "cmd.config.resetcg.not_found".into(),
        "ບໍ່ພົບໄຟລ໌ config. ບໍ່ມີຫຍັງໃຫ້ຣີເຊັດ.".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_backup".into(),
        " \u{2022} ພິມ 'backup' - ສ້າງ backup ກ່ອນຣີເຊັດ".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_no".into(),
        " \u{2022} ພິມ 'no' - ຍົກເລີກ".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_yes".into(),
        " \u{2022} ພິມ 'yes' - ຣີເຊັດ config (ບໍ່ມີ backup)".into(),
    );

    m.insert("cmd.config.resetcg.options_label".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "cmd.config.resetcg.reset_done".into(),
        "ຣີເຊັດ config ເປັນຄ່າເລີ່ມຕົ້ນແລ້ວ!".into(),
    );

    m.insert(
        "cmd.config.resetcg.serialize_failed".into(),
        "serialize default config ບໍ່ສຳເລັດ".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_desc".into(),
        "ອັນນີ້ຈະ RESET ການຕັ້ງຄ່າຂອງທ່ານເປັນຄ່າ DEFAULT:".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_title".into(),
        "\u{26A0}\u{FE0F} ຄຳເຕືອນການຣີເຊັດ config".into(),
    );

    m.insert(
        "cmd.config.resetcg.write_failed".into(),
        "ຂຽນ config ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.config.resetcg.your_choice".into(),
        "ການເລືອກຂອງທ່ານ: ".into(),
    );

    m.insert(
        "cmd.config.restorecg.backed_up".into(),
        "ບັນທຶກ config ປັດຈຸບັນໄວ້ທີ່: {}".into(),
    );

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. ຍົກເລີກ".into());

    m.insert(
        "cmd.config.restorecg.cancelled".into(),
        "ຍົກເລີກການກູ້ຄືນ.".into(),
    );

    m.insert("cmd.config.restorecg.failed".into(), "ກູ້ຄືນບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "cmd.config.restorecg.invalid".into(),
        "ການເລືອກບໍ່ຖືກຕ້ອງ.".into(),
    );

    m.insert("cmd.config.restorecg.none".into(), "ບໍ່ພົບໄຟລ໌ backup.".into());

    m.insert(
        "cmd.config.restorecg.reloaded".into(),
        "ໂຫຼດ config ແລ້ວ! ແລ່ນ 'showcg' ເພື່ອເບິ່ງການຕັ້ງຄ່າທີ່ກູ້ຄືນ.".into(),
    );

    m.insert(
        "cmd.config.restorecg.restored".into(),
        "ກູ້ຄືນ config ຈາກ: {}".into(),
    );

    m.insert(
        "cmd.config.restorecg.title".into(),
        "\u{1F4CB} Backup ທີ່ມີ:".into(),
    );

    m.insert("cmd.config.mgmt_header".into(), "ການຈັດການ:".into());

    m.insert("cmd.config.settings_header".into(), "ການຕັ້ງຄ່າ:".into());

    m.insert(
        "cmd.config.showcg.lang_coming_soon".into(),
        "{} (coming soon)".into(),
    );

    m.insert("cmd.config.showcg.lang_label".into(), "ພາສາ:".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "QSC Hotkey:".into());

    m.insert(
        "cmd.config.title".into(),
        "config \u{2014} ເບິ່ງ ຫຼື ປ່ຽນການຕັ້ງຄ່າ".into(),
    );

    m.insert(
        "cmd.config.watch.alias_not_found".into(),
        "ບໍ່ພົບ alias '{}'. ໃຊ້ 'ral list' ເພື່ອເບິ່ງ aliases.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_cleared".into(),
        "ລຶບ watch trigger alias ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_label".into(),
        "Trigger alias:".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_off_usage".into(),
        "config watch trigger off ປິດການ auto-run".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_set".into(),
        "ຕັ້ງ watch trigger ເປັນ alias: '{}'".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_usage".into(),
        "config watch trigger <alias> ເຮັດວຽກ alias ອັດຕະໂນມັດເມື່ອມີການປ່ຽນແປງ".into(),
    );

    m.insert(
        "cmd.config.watch.limit_usage".into(),
        "config watch limit <n> ຈຳນວນແຖວທີ່ສະແດງໃນກ່ອງ watch".into(),
    );

    m.insert("cmd.config.watch.usage".into(), "ໃຊ້: config watch ON | config watch OFF | config watch trigger <alias> | config watch trigger off | config watch limit <n>".into());

    m.insert(
        "cmd.config.watch.usage_line".into(),
        "ວິທີໃຊ້: config watch ON|OFF".into(),
    );

    m.insert(
        "cmd.config.serve.title".into(),
        "ການຕັ້ງຄ່າ Serve (ຄ່າເລີ່ມຕົ້ນລາຍບໍລິການ)".into(),
    );

    m.insert("cmd.config.serve.status".into(), "  {}: {}:{}".into());

    m.insert("cmd.config.serve.usage".into(), "ວິທີໃຊ້: serve config [files|http-server|chess] [--port-default <ພອດ>] [--bind-default <ທີ່ຢູ່>] (alias: config serve …)".into());

    m.insert(
        "cmd.config.serve.example_show".into(),
        "  serve config                  ສະແດງທຸກບໍລິການ".into(),
    );

    m.insert(
        "cmd.config.serve.example_show_one".into(),
        "  serve config files            ສະແດງຄ່າເລີ່ມຕົ້ນ files".into(),
    );

    m.insert(
        "cmd.config.serve.example_set".into(),
        "  serve config files --port-default 9000   ຕັ້ງພອດເລີ່ມຕົ້ນ files".into(),
    );

    m.insert(
        "cmd.config.serve.saved".into(),
        "ບັນທຶກຄ່າເລີ່ມຕົ້ນ Serve {} ແລ້ວ: {}:{}".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_service".into(),
        "ບໍ່ຮູ້ຈັກບໍລິການ '{}' (ໃຊ້ files|http-server|chess)".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_port".into(),
        "ພອດ '{}' ບໍ່ຖືກຕ້ອງ (ໃຊ້ 1-65535)".into(),
    );

    m.insert(
        "cmd.config.serve.empty_bind".into(),
        "--bind-default ຂາດຄ່າ".into(),
    );

    m.insert("cmd.config.serve.unknown_option".into(), "ບໍ່ຮູ້ຈັກ: {}".into());
}
