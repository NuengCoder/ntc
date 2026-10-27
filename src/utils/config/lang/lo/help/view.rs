use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.view.file_usage".into(), "view <file|@tp/file|@tp/dir/file> [--head N|--tail N|--betw A:B] ## ສະແດງເນື້ອໃນໄຟລ໌ (ຄື txtf)".into());

    m.insert(
        "help.view.example10".into(),
        "view main.rs --tail 30 ## 30 ແຖວສຸດທ້າຍຂອງ main.rs".into(),
    );

    m.insert(
        "help.view.example11".into(),
        "view main.rs --betw 10:20 ## ແຖວ 10-20 ຂອງ main.rs".into(),
    );

    m.insert(
        "help.view.example12".into(),
        "view @docs/readme.md ## ສະແດງໄຟລ໌ໃນ teleport".into(),
    );

    m.insert(
        "help.view.care".into(),
        "view -c, --care ## ສະແດງ tree ກັບ care ທຸກຢ່າງ".into(),
    );

    m.insert(
        "help.view.cp".into(),
        "view -cp, --copy ## ສະແດງ tree ແລະ ສຳເນົາໃສ່ clipboard".into(),
    );

    m.insert(
        "help.view.file_only".into(),
        "view -f, --file ## ສະແດງສະເພາະໄຟລ໌ (ຕັດ directory ອອກ)".into(),
    );

    m.insert(
        "help.view.dir_only".into(),
        "view -dir, --directory ## ສະແດງສະເພາະ directory (ຕັດໄຟລ໌ອອກ)".into(),
    );

    m.insert(
        "help.view.depth".into(),
        "view -d, --depth <n> ## ສະແດງ tree ດ້ວຍຄວາມເລິກທີ່ກຳນົດ".into(),
    );

    m.insert(
        "help.view.dirsize".into(),
        "view -ds, --dirsize [ຂະໜາດຂັ້ນຕ່ຳ] ## ສະແດງຂະໜາດ dir; ໃສ່ຄ່າຈະຊ່ອນທີ່ນ້ອຍກວ່າ".into(),
    );

    m.insert(
        "help.view.head".into(),
        "view --head <n> ## ສະແດງພຽງ n ແຖວທຳອິດຂອງ tree".into(),
    );

    m.insert(
        "help.view.tail".into(),
        "view --tail <n> ## ສະແດງພຽງ n ແຖວສຸດທ້າຍຂອງ tree".into(),
    );

    m.insert(
        "help.view.sort".into(),
        "view --sort size|char|ext ## ຮຽງລູກ (dir ຢູ່ກ່ອນ)".into(),
    );

    m.insert(
        "help.view.ext".into(),
        "view -ext, --ext <ນามສະກຸນ...> ## ສະແດງ tree ສະເພາະນามສະກຸນທີ່ກົງກັນ".into(),
    );

    m.insert(
        "help.view.filesize".into(),
        "view -fs, --filesize [ຂະໜາດຂັ້ນຕ່ຳ] ## ສະແດງຂະໜາດໄຟລ໌; ໃສ່ຄ່າຈະຊ່ອນທີ່ນ້ອຍກວ່າ".into(),
    );

    m.insert(
        "help.view.path".into(),
        "view -p, --path <path|@tp|@tp/sub> ## ສະແດງ tree ຂອງ path ຫຼື teleport".into(),
    );

    m.insert(
        "help.view.size".into(),
        "view -s, --size [ຂະໜາດຂັ້ນຕ່ຳ] ## ສະແດງຂະໜາດ; ໃສ່ຄ່າເຊັ່ນ 10KB ຈະຊ່ອນທີ່ນ້ອຍກວ່າ".into(),
    );

    m.insert(
        "help.view.time".into(),
        "view --time, -t ## ສະແດງ tree ພ້ອມເວລາປະຕິບັດງານ".into(),
    );

    m.insert("help.view.title".into(), "ຄຳສັ່ງ VIEW:".into());

    m.insert(
        "help.view.tree".into(),
        "view [<dir|file|@tp|@tp/file|@tp/dir/>] ## ສະແດງ tree ຫຼື ເນື້ອໃນໄຟລ໌".into(),
    );

    m.insert(
        "help.view.example1".into(),
        "view ## ສະແດງ directory tree".into(),
    );

    m.insert(
        "help.view.example2".into(),
        "view --size ## ສະແດງ tree ພ້ອມຂະໜາດ".into(),
    );

    m.insert(
        "help.view.example3".into(),
        "view -d 2 ## ສະແດງ tree ດ້ວຍຄວາມເລິກ 2".into(),
    );

    m.insert(
        "help.view.example4".into(),
        "view -cp ## ສະແດງ tree ແລະ ສຳເນົາໃສ່ clipboard".into(),
    );

    m.insert(
        "help.view.example5".into(),
        "view --time ## ສະແດງ tree ພ້ອມເວລາປະຕິບັດງານ".into(),
    );

    m.insert(
        "help.view.example6".into(),
        "view -p C:\\code\\project ## ສະແດງ tree ຂອງ path ທີ່ລະບຸ".into(),
    );

    m.insert(
        "help.view.example7".into(),
        "view @web ## ສະແດງ tree ຂອງ teleport savepoint".into(),
    );

    m.insert(
        "help.view.example8".into(),
        "view -s 10MB ## ສະແດງສະເພາະໄຟລ໌/dir ຢ່າງໜ້ອຍ 10MB".into(),
    );

    m.insert(
        "help.view.example9".into(),
        "view --sort size --head 20 ## ຮຽງຕາມຂະໜາດສະແດງ 20 ແຖວທຳອິດ".into(),
    );
}
