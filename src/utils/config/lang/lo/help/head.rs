use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.head.title".into(), "HEAD ຄຳສັ່ງ:".into());

    m.insert(
        "help.head.usage".into(),
        "head [N] [file] ## ສະແດງ N ແຖວທຳອິດ (ຄ່າເລີ່ມຕົ້ນ 10)".into(),
    );

    m.insert(
        "help.head.example1".into(),
        "head 20 app.log ## 20 ແຖວທຳອິດຂອງໄຟລ໌ (ຫຼື input ທີ່ pipe ເຂົ້າມາ)".into(),
    );
}
