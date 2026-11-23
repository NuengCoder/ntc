use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.care.title".into(), "CARE ຄຳສັ່ງ:".into());

    m.insert(
        "help.care.care_dir".into(),
        "care dir <name>[, ...] ## ຢຸດການບໍ່ສົນໃຈໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.care.care_ext".into(),
        "care ext <ext>[, ...] ## ເພີ່ມນາມສະກຸນທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "help.care.care_file".into(),
        "care file <name>[, ...] ## ເພີ່ມໄຟລ໌ທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "help.care.example1".into(),
        "care dir src ## ຍົກເລີກບໍ່ສົນໃຈໄດເຣັກທໍຣີ src".into(),
    );
}
