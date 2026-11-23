use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gosc.title".into(), "GOSC ຄຳສັ່ງ:".into());

    m.insert(
        "help.gosc.usage".into(),
        "gosc ## ນຳທາງຕໍ່ເນື່ອງ (0 ເພື່ອອອກ, v/s/vs ເພື່ອເບິ່ງ)".into(),
    );

    m.insert(
        "help.gosc.example1".into(),
        "gosc ## ເບິ່ງໄດເຣັກທໍຣີຍ່ອຍຕໍ່ເນື່ອງ".into(),
    );
}
