use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcared.section_global".into(), "[global]".into());

    m.insert("cmd.igcared.section_local".into(), "[local]".into());

    m.insert("cmd.igcared.cared_dirs".into(), "ໄດເຣັກທໍຣີທີ່ດູແລ:".into());

    m.insert("cmd.igcared.cared_exts".into(), "ນາມສະກຸນທີ່ດູແລ:".into());

    m.insert("cmd.igcared.cared_files".into(), "ໄຟລ໌ທີ່ດູແລ:".into());

    m.insert("cmd.igcared.none".into(), "(ບໍ່ມີ)".into());

    m.insert(
        "cmd.igcared.no_local".into(),
        "(ບໍ່ມີ config local ໃນໂຟນ໌ດີ້ນີ້)".into(),
    );

    m.insert(
        "cmd.igcared.scope_help".into(),
        "ລາຍກาร local ມີຄວາມສຳຄັນກວ່າ ລາຍກาร global ຍັງໃຊ້ໄດ້ ແລະ config global ຈະບໍ່ຖືກແກ້ໄຂ".into(),
    );

    m.insert("cmd.igcared.extra_exts".into(), "ນາມສະກຸນເພີ່ມເຕີມ:".into());

    m.insert("cmd.igcared.extra_files".into(), "ໄຟລ໌ເພີ່ມເຕີມ:".into());

    m.insert(
        "cmd.igcared.ignored_dirs".into(),
        "ໄດເຣັກທໍຣີທີ່ຖືກ ignore:".into(),
    );

    m.insert(
        "cmd.igcared.ignored_exts".into(),
        "ນາມສະກຸນທີ່ຖືກ ignore:".into(),
    );

    m.insert("cmd.igcared.ignored_files".into(), "ໄຟລ໌ທີ່ຖືກ ignore:".into());

    m.insert(
        "cmd.igcared.local_config".into(),
        "   ການຕັ້ງຄ່າທ້ອງຖິ່ນ: {}".into(),
    );
}
