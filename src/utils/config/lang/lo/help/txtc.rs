use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtc.title".into(), "TXTC ຄຳສັ່ງ:".into());

    m.insert(
        "help.txtc.usage".into(),
        "txtc [--line|--no-line] <ຟາຍ> ## ສຳເນົາເນື້ອໃນໃສ່ clipboard (ເປີດ/ປິດເລກແຖວ)".into(),
    );

    m.insert(
        "help.txtc.example1".into(),
        "txtc main.rs ## ສຳເນົາ main.rs ໄປ clipboard".into(),
    );
}
