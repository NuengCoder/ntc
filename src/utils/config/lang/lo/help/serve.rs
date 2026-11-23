use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.serve.title".into(), "SERVE ຄຳສັ່ງ:".into());

    m.insert(
        "help.serve.service".into(),
        "serve <service> ## ເລືອກບໍລິການ (`serve` ດຽວຈະສະແດງລາຍການ)".into(),
    );

    m.insert(
        "help.serve.chess".into(),
        "serve chess [--online] ## ໝາກຮຸກ hotseat/AI/coach (--online: lobby + ຫ້ອງ)".into(),
    );

    m.insert(
        "help.serve.git".into(),
        "serve git [ໂຟນເດີ] ## Git GUI: status, diff, stage, commit, branch, tag, stash, push/pull"
            .into(),
    );

    m.insert(
        "help.serve.arc".into(),
        "serve arc [ໂຟນເດີ] ## Archive GUI: ສ້າງ, ເບິ່ງ ແລະ ແຕກໄຟລ໌ບີບອັດ".into(),
    );

    m.insert(
        "help.serve.ndb".into(),
        "serve ndb ## ເບິ່ງ SQLite: ເລືອກຖານຂໍ້ມູນໃນໂຟນເດີນີ້, ເບິ່ງໂຄງສ້າງ/ຂໍ້ມູນ, ເລືອກ --write".into(),
    );

    m.insert(
        "help.serve.theme".into(),
        "serve theme ## Theme GUI: ແກ້ໄຂ, ເບິ່ງຕົວຢ່າງ, ບັນທຶກ ແລະ ນຳໃຊ້ທີມ".into(),
    );

    m.insert(
        "help.serve.fake".into(),
        "serve fake ## UI ຂໍ້ມູນຈຳລອງ: ຊະນິດ ຫຼື ກຸ່ມທີ່ບັນທຶກໄວ້, ເບິ່ງຕົວຢ່າງ txt/json/csv/sql".into(),
    );

    m.insert(
        "help.serve.config".into(),
        "serve config ## UI ການຕັ້ງຄ່າ: ຄ່າຕ່າງໆ ແລະ ຄ່າເລີ່ມຕົ້ນຂອງ serve (CLI ຍັງຢູ່ config serve)".into(),
    );

    m.insert(
        "help.serve.backup".into(),
        "serve backup [path] ## UI ສຳຮອງຂໍ້ມູນ: ສ້າງ, diff, ກູ້ຄືນ, ຍ້ອນກັບ, ລຶບໂຟນເດີນີ້".into(),
    );

    m.insert(
        "help.serve.example1".into(),
        "serve files --port 8080 ## ເລີ່ມ file-drive server ເທິງ port 8080".into(),
    );
}
