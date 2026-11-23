use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.betw.title".into(), "BETW ຄຳສັ່ງ:".into());

    m.insert(
        "help.betw.usage".into(),
        "betw A:B [file] ## ສະແດງແຖວ A ຫາ B (ເລີ່ມທີ່ 1 ລວມປາຍ)".into(),
    );

    m.insert(
        "help.betw.example1".into(),
        "betw 10:20 app.log ## ສະແດງແຖວ 10-20 ຂອງ app.log".into(),
    );
}
