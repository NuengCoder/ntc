use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.ignore.scope_local".into(), "config local".into());

    m.insert("cmd.ignore.scope_global".into(), "config global".into());

    m.insert(
        "cmd.ignore.scope_help".into(),
        "ໃຊ້ '-g' / '--global' ເພື່ອຂຽນໄປຫາ config global (ໃນໂຟນ໌ດີ້ທີ່ມີ ntconfig.toml)".into(),
    );

    m.insert("cmd.ignore.scope_target".into(), "ເປົ້າຫມາຍ: {}".into());

    m.insert("cmd.ignore.applied".into(), "{} {} ລາຍການ".into());

    m.insert(
        "cmd.ignore.care_example_dir".into(),
        "  care dir build,dist".into(),
    );

    m.insert(
        "cmd.ignore.care_example_ext".into(),
        "  care ext txt,md,json".into(),
    );

    m.insert(
        "cmd.ignore.care_example_file".into(),
        "  care file .env".into(),
    );

    m.insert(
        "cmd.ignore.care_examples_title".into(),
        "ຕົວຢ່າງ care:".into(),
    );

    m.insert(
        "cmd.ignore.check_hint".into(),
        "ແລ່ນ 'igcared' ເພື່ອເບິ່ງການຕັ້ງຄ່າ ignore/care ປັດຈຸບັນ".into(),
    );

    m.insert(
        "cmd.ignore.dir_desc".into(),
        "ignore dir <name>[, ...] ບໍ່ສົນໃຈໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "cmd.ignore.dir_usage".into(),
        "ວິທີໃຊ້: ignore dir <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.example_dir".into(),
        "  ignore dir temp,logs".into(),
    );

    m.insert(
        "cmd.ignore.example_ext".into(),
        "  ignore ext log,tmp,bak".into(),
    );

    m.insert(
        "cmd.ignore.example_file".into(),
        "  ignore file Cargo.lock".into(),
    );

    m.insert("cmd.ignore.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.ignore.ext_desc".into(),
        "ignore ext <ext>[, ...] ບໍ່ສົນໃຈນາມສະກຸນໄຟລ໌".into(),
    );

    m.insert(
        "cmd.ignore.ext_usage".into(),
        "ວິທີໃຊ້: ignore ext <ext>[, <ext>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.file_desc".into(),
        "ignore file <name>[, ...] ບໍ່ສົນໃຈໄຟລ໌ສະເພາະ".into(),
    );

    m.insert(
        "cmd.ignore.file_usage".into(),
        "ວິທີໃຊ້: ignore file <name>[, <name>, ...]".into(),
    );

    m.insert(
        "cmd.ignore.title".into(),
        "ignore \u{2014} ຈັດການລາຍການທີ່ຖືກບໍ່ສົນໃຈ".into(),
    );
}
