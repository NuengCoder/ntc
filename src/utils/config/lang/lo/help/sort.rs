use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.sort.title".into(), "SORT ຄຳສັ່ງ:".into());

    m.insert(
        "help.sort.usage".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## ຈັດລຳດັບແຖວ (ຍ້ອນກັບ/ຕົວເລກ/ບໍ່ຊ້ຳ/ບໍ່ສົນຕົວພິມ)".into(),
    );

    m.insert(
        "help.sort.example1".into(),
        "gs error | sort -u | head 10 ## ຈັດລຳດັບຜົນທີ່ບໍ່ຊ້ຳ ແລ້ວເອົາ 10 ແຖວທຳອິດ".into(),
    );
}
