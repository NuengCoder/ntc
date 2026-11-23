use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ignore.title".into(), "IGNORE คำสั่ง:".into());

    m.insert(
        "help.ignore.dir".into(),
        "ignore dir <name>[, ...] ## ละเว้นไดเรกทอรีตามชื่อ".into(),
    );

    m.insert(
        "help.ignore.ext".into(),
        "ignore ext <ext>[, ...] ## ละเว้นนามสกุลไฟล์".into(),
    );

    m.insert(
        "help.ignore.file".into(),
        "ignore file <name>[, ...] ## ละเว้นไฟล์เฉพาะ".into(),
    );

    m.insert(
        "help.ignore.example1".into(),
        "ignore dir node_modules ## ละเว้น node_modules".into(),
    );

    m.insert(
        "help.ignore.example2".into(),
        "ignore ext .log ## ละเว้นไฟล์ .log".into(),
    );
}
