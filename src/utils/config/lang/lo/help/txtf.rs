use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.txtf.title".into(), "TXTF ຄຳສັ່ງ:".into());

    m.insert(
        "help.txtf.usage".into(),
        "txtf [--head N|--tail N|--betw A:B] [file|dir|@tp/file|@tp/dir] ## ສະແດງເນື້ອໃນໄຟລ໌ ຫຼື ເລືອກໄຟລ໌ຈາກ directory (ເລກແຖວເດີມ)".into(),
    );

    m.insert(
        "help.txtf.example1".into(),
        "txtf main.rs --head 30 ## 30 ແຖວທຳອິດຂອງ main.rs".into(),
    );

    m.insert(
        "help.txtf.example2".into(),
        "txtf --betw 10:20 main.rs ## ແຖວ 10-20 ຂອງ main.rs".into(),
    );

    m.insert(
        "help.txtf.example3".into(),
        "txtf @docs/notes.txt ## ສະແດງໄຟລ໌ໃນ teleport".into(),
    );

    m.insert(
        "help.txtf.example4".into(),
        "txtf src ## ເລືອກໄຟລ໌ທີ່ຮອງຮັບຈາກ directory src ເພື່ອສະແດງ".into(),
    );
}
