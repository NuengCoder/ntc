use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.godrive.title".into(), "GODRIVE ຄຳສັ່ງ:".into());

    m.insert(
        "help.godrive.usage".into(),
        "godrive ## ລາຍຊື່ ແລະ ເລືອກ drives (Windows) ຫຼື mount ໃນ /mnt (Linux)".into(),
    );

    m.insert(
        "help.godrive.example1".into(),
        "godrive ## ເລືອກ drive ຫຼື mount".into(),
    );

    m.insert(
        "help.godrive.example2".into(),
        "godrive c ## ໄປທີ່ drive C: ໃນ Windows ຫຼື /mnt/c ໃນ Linux".into(),
    );
}
