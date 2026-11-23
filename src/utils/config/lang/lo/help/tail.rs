use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tail.title".into(), "TAIL ຄຳສັ່ງ:".into());

    m.insert(
        "help.tail.usage".into(),
        "tail [N] [file] ## ສະແດງ N ແຖວສຸດທ້າຍ (ຄ່າເລີ່ມຕົ້ນ 10)".into(),
    );

    m.insert(
        "help.tail.example1".into(),
        "tail 20 app.log ## 20 ແຖວສຸດທ້າຍຂອງ app.log".into(),
    );
}
