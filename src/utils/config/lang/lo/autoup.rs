use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "autoup.added".into(),
        "ເພີ່ມ startup entry ແລ້ວ: {} \u{2192} {}".into(),
    );

    m.insert(
        "autoup.already_disabled".into(),
        "Entry '{}' ຖືກປິດໃຊ້ງານແລ້ວ.".into(),
    );

    m.insert(
        "autoup.already_enabled".into(),
        "Entry '{}' ຖືກເປີດໃຊ້ງານແລ້ວ.".into(),
    );

    m.insert(
        "autoup.already_exists".into(),
        "Entry '{}' ມີຢູ່ໃນ user startup ແລ້ວ.".into(),
    );

    m.insert("autoup.cmd_empty".into(), "ຄ່າຄຳສັ່ງບໍ່ສາມາດຫວ່າງເປົ່າໄດ້.".into());

    m.insert("autoup.disabled".into(), "ປິດໃຊ້ງານ startup entry: {}".into());

    m.insert(
        "autoup.disabled_system".into(),
        "ປິດໃຊ້ງານ system startup entry: {}".into(),
    );

    m.insert("autoup.enabled".into(), "ເປີດໃຊ້ງານ startup entry: {}".into());

    m.insert(
        "autoup.enabled_system".into(),
        "ເປີດໃຊ້ງານ system startup entry: {}".into(),
    );

    m.insert(
        "autoup.help_add".into(),
        " autoup add <name> \"<command>\" ເພີ່ມ startup entry (HKCU)".into(),
    );

    m.insert(
        "autoup.help_disable".into(),
        " autoup disable <name> ປິດໃຊ້ງານ startup entry".into(),
    );

    m.insert(
        "autoup.help_enable".into(),
        " autoup enable <name> ເປີດໃຊ້ງານ entry ທີ່ຖືກປິດໄວ້ຄືນ".into(),
    );

    m.insert(
        "autoup.help_help".into(),
        " autoup help ສະແດງ help ນີ້".into(),
    );

    m.insert(
        "autoup.help_list".into(),
        " autoup list ລາຍຊື່ startup entry ທັງໝົດຈາກທຸກແຫຼ່ງ".into(),
    );

    m.insert(
        "autoup.help_rm".into(),
        " autoup rm <name> ລຶບ startup entry".into(),
    );

    m.insert(
        "autoup.hklm_hint".into(),
        "'{}' ແມ່ນ system entry (HKLM). ໃຊ້ 'mode admin' ກ່ອນ, ຈາກນັ້ນລອງໃໝ່.".into(),
    );

    m.insert(
        "autoup.hklm_hint_short".into(),
        "'{}' ແມ່ນ system entry. ໃຊ້ 'mode admin' ກ່ອນ, ຈາກນັ້ນລອງໃໝ່.".into(),
    );

    m.insert("autoup.list_empty".into(), "(ບໍ່ພົບ startup entries)".into());

    m.insert("autoup.list_title".into(), "Windows Startup Entries".into());

    m.insert(
        "autoup.list_total".into(),
        "ທັງໝົດ: {} [A]:{} [D]:{} (CU:{} LM:{} SF:{})".into(),
    );

    m.insert("autoup.not_found".into(), "ບໍ່ພົບ startup entry: {}".into());

    m.insert(
        "autoup.reg_delete_fail".into(),
        "ລຶບ registry entry ບໍ່ສຳເລັດ.".into(),
    );

    m.insert(
        "autoup.reg_disable_fail".into(),
        "ຕັ້ງສະຖານະປິດໃຊ້ງານບໍ່ສຳເລັດ.".into(),
    );

    m.insert(
        "autoup.reg_exec_fail".into(),
        "ເຮັດວຽກ reg.exe ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "autoup.reg_query_fail".into(),
        "ຄົ້ນຫາ registry ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "autoup.reg_write_fail".into(),
        "ຂຽນ registry entry ບໍ່ສຳເລັດ. ລອງໃຊ້ເປັນ Administrator.".into(),
    );

    m.insert("autoup.removed".into(), "ລຶບ startup entry ແລ້ວ: {}".into());

    m.insert(
        "autoup.removed_system".into(),
        "ລຶບ system startup entry ແລ້ວ: {}".into(),
    );

    m.insert(
        "autoup.sf_disable_hint".into(),
        "'{}' ຢູ່ໃນ Startup folder. ຍ້າຍ shortcut ອອກເພື່ອປິດໃຊ້ງານ.".into(),
    );

    m.insert(
        "autoup.sf_rm_hint".into(),
        "'{}' ຢູ່ໃນ Startup folder. ລຶບໄຟລ໌ shortcut ດ້ວຍມື.".into(),
    );

    m.insert(
        "autoup.source_cu".into(),
        " [CU] Current User (HKCU\\...\\Run)".into(),
    );

    m.insert(
        "autoup.source_lm".into(),
        " [LM] Local Machine (HKLM\\...\\Run) \u{2014} requires admin (mode admin)".into(),
    );

    m.insert(
        "autoup.source_sf".into(),
        " [SF] Startup Folder (%APPDATA%\\...\\Startup) \u{2014} read-only".into(),
    );

    m.insert("autoup.source_tags".into(), "Source tags:".into());

    m.insert(
        "autoup.usage_add".into(),
        "ວິທີໃຊ້: autoup add <name> \"<command>\"".into(),
    );

    m.insert(
        "autoup.usage_disable".into(),
        "ວິທີໃຊ້: autoup disable <name>".into(),
    );

    m.insert(
        "autoup.usage_enable".into(),
        "ວິທີໃຊ້: autoup enable <name>".into(),
    );

    m.insert("autoup.usage_examples".into(), "ຕົວຢ່າງ:".into());

    m.insert("autoup.usage_rm".into(), "ວິທີໃຊ້: autoup rm <name>".into());

    m.insert("autoup.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "autoup.use_rm_first".into(),
        "ໃຊ້ 'autoup rm' ກ່ອນ ຖ້າທ່ານຕ້ອງການປ່ຽນແທນມັນ.".into(),
    );

    m.insert(
        "autoup.will_run".into(),
        "entry ນີ້ຈະເຮັດວຽກໃນການເຂົ້າສູ່ລະບົບຄັ້ງຖັດໄປ.".into(),
    );

    m.insert(
        "autoup.windows_only".into(),
        "autoup ແມ່ນຄຸນສົມບັດສະເພາະ Windows ເທົ່ານັ້ນ.".into(),
    );
}
