use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.spec.title".into(), "SPEC ຄຳສັ່ງ:".into());

    m.insert(
        "help.spec.usage".into(),
        "spec ## ສະແດງສະເປັກຄອມພິວເຕີ/ລະບົບ (CPU, GPU, RAM, ບ່ອນເກັບຂໍ້ມູນ, OS, ເລກຊີຣຽວ)".into(),
    );

    m.insert(
        "help.spec.example1".into(),
        "spec ## ສະແດງສະເປັກຂອງລະບົບ".into(),
    );
}
