use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.igcare.care_dir".into(),
        "care dir <name>[, ...] ## ຢຸດການບໍ່ສົນໃຈໄດເຣັກທໍຣີ".into(),
    );

    m.insert(
        "help.igcare.care_ext".into(),
        "care ext <ext>[, ...] ## ເພີ່ມນາມສະກຸນທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "help.igcare.care_file".into(),
        "care file <name>[, ...] ## ເພີ່ມໄຟລ໌ທີ່ຮອງຮັບເພີ່ມເຕີມ".into(),
    );

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## ສົ່ງອອກການຕັ້ງຄ່າທັງໝົດໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## ເລືອກປະເພດເພື່ອສົ່ງອອກ".into(),
    );

    m.insert(
        "help.igcare.ignore_dir".into(),
        "ignore dir <name>[, ...] ## ບໍ່ສົນໃຈໄດເຣັກທໍຣີຕາມຊື່".into(),
    );

    m.insert(
        "help.igcare.ignore_ext".into(),
        "ignore ext <ext>[, ...] ## ບໍ່ສົນໃຈນາມສະກຸນໄຟລ໌".into(),
    );

    m.insert(
        "help.igcare.ignore_file".into(),
        "ignore file <name>[, ...] ## ບໍ່ສົນໃຈໄຟລ໌ສະເພາະ".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <file> ## ນຳເຂົ້າການຕັ້ງຄ່າຈາກໄຟລ໌ .ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [name] ## ສ້າງ igcare template ໃນ CWD".into(),
    );

    m.insert(
        "help.igcare.show".into(),
        "igcared ## ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );

    m.insert("help.igcare.title".into(), "ຄຳສັ່ງ IGNORE / CARE:".into());

    m.insert(
        "help.igcare.example1".into(),
        "ignore dir node_modules ## ບໍ່ສົນໃຈ node_modules".into(),
    );

    m.insert(
        "help.igcare.example2".into(),
        "ignore ext .log ## ບໍ່ສົນໃຈໄຟລ໌ .log".into(),
    );

    m.insert(
        "help.igcare.example3".into(),
        "care dir src ## ຍົກເລີກບໍ່ສົນໃຈໄດເຣັກທໍຣີ src".into(),
    );

    m.insert(
        "help.igcare.example4".into(),
        "igcared ## ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );
}
