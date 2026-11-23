use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.version.title".into(), "VERSION ຄຳສັ່ງ:".into());

    m.insert("help.version.usage".into(), "version ## ສະແດງເວີຊັນ".into());

    m.insert(
        "help.version.example1".into(),
        "version ## ສະແດງຂໍ້ມູນເວີຊັນ".into(),
    );
}
