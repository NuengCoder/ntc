use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mkf.title".into(), "MKF ຄຳສັ່ງ:".into());

    m.insert(
        "help.mkf.usage".into(),
        "mkf <file>[, ...] ## ສ້າງໄຟລ໌ໃໝ່".into(),
    );

    m.insert(
        "help.mkf.example1".into(),
        "mkf index.html ## ສ້າງໄຟລ໌ໃໝ່".into(),
    );
}
