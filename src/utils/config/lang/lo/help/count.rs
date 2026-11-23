use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.count.title".into(), "COUNT ຄຳສັ່ງ:".into());

    m.insert(
        "help.count.usage".into(),
        "count <needle> [text] [-l|--line] ## ນັບຈຳນວນຄັ້ງທີ່ <needle> ປາກົດ; -l|--line ນັບຈຳນວນແຖວທີ່ pipe ເຂົ້າມາ".into(),
    );

    m.insert(
        "help.count.example1".into(),
        "gs TODO | count TODO ## ນັບຈຳນວນທີ່ກົງໃນຂໍ້ຄວາມທີ່ pipe ເຂົ້າມາ".into(),
    );

    m.insert(
        "help.count.example2".into(),
        "gs TODO | count -l ## ນັບຈຳນວນແຖວທັງໝົດຂອງຜົນລັບທີ່ pipe ເຂົ້າມາ".into(),
    );
}
