use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ignore.title".into(), "IGNORE ຄຳສັ່ງ:".into());

    m.insert(
        "help.ignore.dir".into(),
        "ignore dir <name>[, ...] ## ບໍ່ສົນໃຈໄດເຣັກທໍຣີຕາມຊື່".into(),
    );

    m.insert(
        "help.ignore.ext".into(),
        "ignore ext <ext>[, ...] ## ບໍ່ສົນໃຈນາມສະກຸນໄຟລ໌".into(),
    );

    m.insert(
        "help.ignore.file".into(),
        "ignore file <name>[, ...] ## ບໍ່ສົນໃຈໄຟລ໌ສະເພາະ".into(),
    );

    m.insert(
        "help.ignore.example1".into(),
        "ignore dir node_modules ## ບໍ່ສົນໃຈ node_modules".into(),
    );

    m.insert(
        "help.ignore.example2".into(),
        "ignore ext .log ## ບໍ່ສົນໃຈໄຟລ໌ .log".into(),
    );
}
