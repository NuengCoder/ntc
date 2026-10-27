use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.care.scope_help".into(),
        "ໃຊ້ '-g' / '--global' ເພື່ອຂຽນໄປຫາ config global (ໃນໂຟນ໌ດີ້ທີ່ມີ ntconfig.toml)".into(),
    );

    m.insert(
        "cmd.care.dir_desc".into(),
        "care dir <name>[, ...] ຢຸດການບໍ່ສົນໃຈໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "cmd.care.dir_usage".into(),
        "ວິທີໃຊ້: care dir <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.care.ext_desc".into(),
        "care ext <ext>[, ...] ເພີ່ມນາມສະກຸນທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "cmd.care.ext_usage".into(),
        "ວິທີໃຊ້: care ext <ext>[, <ext>, ...]".into(),
    );

    m.insert(
        "cmd.care.file_desc".into(),
        "care file <name>[, ...] ເພີ່ມໄຟລ໌ທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "cmd.care.file_usage".into(),
        "ວິທີໃຊ້: care file <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.care.title".into(),
        "care \u{2014} ຈັດການລາຍການທີ່ໃສ່ໃຈ (ບໍ່ຖືກ ignore)".into(),
    );
}
