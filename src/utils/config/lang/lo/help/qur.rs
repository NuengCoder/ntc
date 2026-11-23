use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.qur.title".into(), "QUR ຄຳສັ່ງ:".into());

    m.insert(
        "help.qur.usage".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] ## ຄົ້ນຫາໄຟລ໌, ເລືອກເນື້ອໃນ, ແທນທີ່"
            .into(),
    );

    m.insert(
        "help.qur.example1".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + ສະແດງແບບ grep".into(),
    );
}
