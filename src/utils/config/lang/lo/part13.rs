use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.theme.title".into(), "ຄຳສັ່ງ THEME:".into());
    m.insert(
        "help.theme.where".into(),
        "theme where ## ສະແດງຕຳແໜ່ງໄດເຣັກທໍຣີ theme".into(),
    );
    m.insert(
        "help.tl.dir".into(),
        "tl dir <path> [-s <-n|-l|-r>] ## ສະແດງນາມສະກຸນທີ່ຮອງຮັບ ແລະ ຈຳນວນແຖວໃນໄດເຣັກທໍຣີ (sort ໄດ້)"
            .into(),
    );
    m.insert("help.tl.ext".into(), "tl ext <ext> [-s <-n|-l|-r>] ## ສະແດງໄຟລ໌ <ext> ທັງໝົດ ແລະ ຈຳນວນແຖວລວມ (ເຄົາລົບ igcare, sort ໄດ້)".into());
    m.insert(
        "help.tl.file".into(),
        "tl file <path> ## ນັບແຖວໃນໄຟລ໌ເດີ່ມ".into(),
    );
    m.insert("help.tl.sort".into(), "tl ext|dir <target> -s <-n|-l|-r> ## Sort: -n ປົກກະຕິ, -l ຕາມແຖວ (ນ້ອຍຫາໃຫຍ່), -r ກົງກັນຂ້າມ (ໃຫຍ່ຫານ້ອຍ)".into());
    m.insert("help.tl.title".into(), "ຄຳສັ່ງ TOTAL LINE:".into());
    m.insert(
        "help.todo.add".into(),
        "todo add \"name\" ## ເພີ່ມວຽກໃໝ່".into(),
    );
    m.insert(
        "help.todo.add_at".into(),
        "todo add \"name\" -> N ## ໃສ່ວຽກທີ່ຕຳແໜ່ງ N".into(),
    );
    m.insert(
        "help.todo.clear".into(),
        "todo clear ## ລຶບວຽກທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );
    m.insert(
        "help.todo.info".into(),
        "todo info [N] ## ສະແດງວຽກທັງໝົດ ຫຼື ລາຍລະອຽດວຽກ N".into(),
    );
    m.insert("help.todo.rm".into(), "todo rm <N> ## ລຶບວຽກ N".into());
    m.insert(
        "help.todo.show".into(),
        "todo ## ສະແດງວຽກທັງໝົດ (ຈາກ future_todo.txt ໃນ CWD)".into(),
    );
    m.insert("help.todo.title".into(), "ຄຳສັ່ງ TODO:".into());
    m.insert(
        "help.todo.done".into(),
        "todo done <N> ## ໝາຍວຽກ N ວ່າສຳເລັດ".into(),
    );
    m.insert(
        "help.todo.undo".into(),
        "todo undo <N> ## ເປີດວຽກ N ໃໝ່".into(),
    );
    m.insert(
        "help.todo.edit".into(),
        "todo edit <N> \"name\" ## ປ່ຽນຊื่อວຽກ N".into(),
    );
    m.insert("help.arc.title".into(), "ຄຳສັ່ງ ARCHIVE:".into());
    m.insert(
        "help.arc.create".into(),
        "arc create <out.zip|tar.gz|tar> <ໄຟລ໌...> ## ສ້າງໄຟລ໌ບີບอัด".into(),
    );
    m.insert(
        "help.arc.extract".into(),
        "arc extract <ໄຟລ໌> [ປາຍທາງ] ## ແຕກໄຟລ໌".into(),
    );
    m.insert(
        "help.arc.list".into(),
        "arc list <ໄຟລ໌> ## ສະແດງເນື້ອໃນ".into(),
    );
    m.insert(
        "help.arc.example1".into(),
        "arc create backup.zip src Cargo.toml ## ສ້າງ zip".into(),
    );
    m.insert(
        "help.arc.example2".into(),
        "arc list backup.zip ## ສະແດງເນື້ອໃນ".into(),
    );
    m.insert(
        "help.arc.example3".into(),
        "arc extract backup.zip restored ## ແຕກໄຟລ໌".into(),
    );
    m.insert(
        "help.scan.depth".into(),
        "scan ext <ນາມສະກຸນ...> -d, --depth <n> ## ຈຳກັດຄວາມເລິກສະແກນ".into(),
    );
    m.insert(
        "help.scan.example1".into(),
        "scan ext rs ## ສະແດງໄຟລ໌ .rs ທັງໝົດ".into(),
    );
    m.insert(
        "help.scan.example2".into(),
        "scan ext rs c -d 2 ## ສະແດງໄຟລ໌ .rs/.c ເຖິງຄວາມເລິກ 2".into(),
    );
    m.insert(
        "help.scan.example3".into(),
        "scan ext rs c -s ## ສະແດງພ້ອມຂະໜາດແລະຜົນລວມ".into(),
    );

    m.insert(
        "help.scan.example4".into(),
        "scan port 192.168.1.1 --top 10 ## ສະແກນ 10 ports ແລກ".into(),
    );

    m.insert(
        "help.scan.example5".into(),
        "scan ip 192.168.1.0/24 ## sweep ຫາ hosts ທີ່ alive".into(),
    );
    m.insert(
        "help.scan.example6".into(),
        "scan files *.rs ## ສະແດງໄຟລ໌ຕາມຊື່".into(),
    );
    m.insert(
        "help.scan.example7".into(),
        "scan dir k* ## ສະແດງໄດເຣັກທໍຣີຕາມຊື່".into(),
    );
    m.insert(
        "help.scan.ext".into(),
        "scan ext <ນາມສະກຸນ...> ## ສະແດງໄຟລ໌ຕາມນາມສະກຸນ (ບໍ່ນັບແຖວ)".into(),
    );
    m.insert(
        "help.scan.files".into(),
        "scan files [ແບບ] ## ສະແດງໄຟລ໌ຕາມຊື່ (* = ທັງໝົດ)".into(),
    );
    m.insert(
        "help.scan.dir".into(),
        "scan dir [ແບບ] ## ສະແດງໄດເຣັກທໍຣີຕາມຊື່ (* = ທັງໝົດ)".into(),
    );

    m.insert(
        "help.scan.port".into(),
        "scan port [host] [--top N | --ports <spec> | --all] ## ສະແກນ TCP port".into(),
    );

    m.insert(
        "help.scan.ip".into(),
        "scan ip [cidr] [--top N | --ports <spec>] ## sweep ຫາ hosts + ARP".into(),
    );
    m.insert(
        "help.scan.size".into(),
        "scan ext <ນາມສະກຸນ...> -s, --size ## ສະແດງຂະໜາດແລະຜົນລວມ".into(),
    );
    m.insert(
        "help.scan.care".into(),
        "scan ext <ນາມສະກຸນ...> --care ## ລວມທຸກໄຟລ໌ (ໂໝດ care)".into(),
    );
    m.insert("help.scan.title".into(), "ຄຳສັ່ງ SCAN:".into());
    m.insert("help.nssh.title".into(), "ຄຳສັ່ງ NSSH:".into());
    m.insert(
        "help.nssh.add".into(),
        "nssh add <ຊື່> --host <ໂຮສ> ## ບັນທຶກໂປຣໄຟລ໌ SSH".into(),
    );
    m.insert(
        "help.nssh.ssh".into(),
        "nssh ssh [ຊື່] [--cmd \"ຄຳສັ່ງ\"] ## ເຊວຫຼືຄຳສັ່ງດຽວ".into(),
    );
    m.insert(
        "help.nssh.cp".into(),
        "nssh cp <ໂລຄອລ> <ຊື່>:<ຣີໂໝດ> ## ອັບໂຫຼດ/ດາວໂຫຼດ SFTP".into(),
    );
    m.insert(
        "help.nssh.scan".into(),
        "nssh scan <ໂຮສ> [--top N] ## ສະແກນພອດ TCP".into(),
    );
    m.insert(
        "help.nssh.manage".into(),
        "nssh ls|info|rm|edit ## ຈັດການໂປຣໄຟລ໌".into(),
    );
    m.insert(
        "help.nssh.keygen".into(),
        "nssh keygen ## ສ້າງຄີຜ່ານ ssh-keygen".into(),
    );
    m.insert(
        "help.nssh.example1".into(),
        "nssh add prod --host 203.0.113.10 --user root ## ບັນທຶກໂປຣໄຟລ໌".into(),
    );
    m.insert(
        "help.nssh.example2".into(),
        "nssh ssh prod --cmd \"uptime\" ## ຣັນຄຳສັ່ງຣີໂໝດ".into(),
    );
    m.insert(
        "help.nssh.example3".into(),
        "nssh cp ./app.tar.gz prod:/tmp/ ## ອັບໂຫຼດໄຟລ໌".into(),
    );
    m.insert("help.nsheet.title".into(), "ຄຳສັ່ງ NSHEET:".into());
    m.insert(
        "help.nsheet.open".into(),
        "nsheet open <ໄຟລ໌> [ຕົວເລືອກ] ## ເບິ່ງຕາຕະລາງແບບເຕັມຈໍ (CSV/TSV/JSON)".into(),
    );
    m.insert(
        "help.nsheet.view".into(),
        "nsheet view <ໄຟລ໌> [ຕົວເລືອກ] ## ພິມຕາຕະລາງ (ເລີ່ມຕົ້ນ 50 ແຖວ)".into(),
    );
    m.insert(
        "help.nsheet.sort".into(),
        "nsheet view <ໄຟລ໌> --sort <ຖັນ> [--desc] ## ຮຽງຕາມຖັນ".into(),
    );
    m.insert(
        "help.nsheet.filter".into(),
        "nsheet view <ໄຟລ໌> --filter <ເງື່ອນໄຂ> ## ກອງແຖວ (col~ຂໍ້ຄວາມ, col=ຄ່າ, col>ເລກ)".into(),
    );
    m.insert(
        "help.nsheet.export".into(),
        "nsheet view <ໄຟລ໌> --export <json|md> [-o <ໄຟລ໌>] ## ສົ່ງອອກ".into(),
    );
    m.insert(
        "help.nsheet.tui".into(),
        "ປຸ່ມເຕັມຈໍ: ລูกສອນ/hjkl ເລື່ອນ · / ຄົ້ນຫາ · n ຖັດໄປ · s ຮຽງ · f ກອງ · e ສົ່ງອອກ · Enter ເບິ່ງ · q ອອກ"
            .into(),
    );
    m.insert(
        "help.nsheet.example1".into(),
        "nsheet open users.csv ## ເປີດເບິ່ງຕາຕະລາງ".into(),
    );
    m.insert(
        "help.nsheet.example2".into(),
        "nsheet view data.json --filter \"age>30\" --sort age ## ກອງແລະຮຽງ".into(),
    );
    m.insert(
        "help.nsheet.example3".into(),
        "nsheet view a.csv --export md -o out.md ## ສົ່ງອອກ Markdown".into(),
    );
    m.insert("help.nms.title".into(), "ຄຳສັ່ງ NMS (NTC Music):".into());
    m.insert(
        "help.nms.play".into(),
        "nms play <ໄຟລ໌> ## ເຄື່ອງຫຼິ້ນສຽງເຕັມຈໍ (space ຢຸດ · ←/→ ເລື່ອນ · ↑/↓ ລະດັບສຽງ · a ຫຼິ້ນຄືນ)".into(),
    );
    m.insert(
        "help.nms.scan".into(),
        "nms scan [-d <n>] [-s] [-l <dur>] [--care] ## ສະແດງໄຟລ໌ສຽງ + ວິດີໂອ (dur: 25s, 1mn, 1h)"
            .into(),
    );
    m.insert(
        "help.nms.long".into(),
        "nms scan -l <dur> ## ສະແດງສະເພາະມີເດຍທີ່ຍາວກວ່າ <dur> (ເຊັ່ນ 25s, 1mn, 1h)".into(),
    );
    m.insert(
        "help.nms.keys".into(),
        "ໜ້າຈໍເຄື່ອງຫຼິ້ນ: ຄື້ນສຽງຈິງ + ມາດຕະວັດລະດັບສູງ/ຕ່ຳ · ວິດີໂອຫຼິ້ນສະເພາະສຽງ · ຈົບແລ້ວຈະຢຸດນິ້ງ ກົດ a ເພື່ອຫຼິ້ນຄືນ".into(),
    );
    m.insert(
        "help.nms.example1".into(),
        "nms play song.mp3 ## ຫຼິ້ນໄຟລ໌ສຽງ".into(),
    );
    m.insert(
        "help.nms.example2".into(),
        "nms play movie.mp4 ## ຫຼິ້ນສຽງຈາກວິດີໂອ".into(),
    );
    m.insert(
        "help.nms.example3".into(),
        "nms scan -d 2 -s ## ສະແດງໄຟລ໌ມີເດຍພ້ອມຂະໜາດ".into(),
    );
    m.insert(
        "help.nms.example4".into(),
        "nms scan -l 1mn ## ສະແດງມີເດຍທີ່ຍາວກວ່າ 1 ນາທີ".into(),
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
        "cmd.report.invalid_sort".into(),
        "Invalid sort key: '{}' (size|char|ext)".into(),
    );
    m.insert(
        "cmd.report.invalid_count".into(),
        "Invalid line count: '{}' (must be > 0)".into(),
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
        "help.config.example1".into(),
        "config output ~/Desktop ## ຕັ້ງ path ຜົນລັບເປັນ Desktop".into(),
    );
    m.insert(
        "help.config.example2".into(),
        "config depth 3 ## ຕັ້ງຄວາມເລິກສູງສຸດເປັນ 3".into(),
    );
    m.insert(
        "help.config.example3".into(),
        "config color ON ## ເປີດການສະແດງສີ".into(),
    );
    m.insert(
        "help.config.example4".into(),
        "config threads 8 ## ຕັ້ງຈຳນວນ threads ເປັນ 8".into(),
    );
    m.insert(
        "help.config.example5".into(),
        "showcg ## ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.editor.example1".into(),
        "ne main.rs ## ເປີດ main.rs ໃນ editor".into(),
    );
    m.insert(
        "help.editor.example2".into(),
        "ne src/main.rs:42 ## ເປີດ main.rs ທີ່ແຖວ 42".into(),
    );
    m.insert(
        "help.editor.example3".into(),
        "ne --init a.py b.html -I a.py ## ສ້າງ template ແລະເປີດ a.py".into(),
    );
    m.insert(
        "help.editor.example4".into(),
        "ne --hyper myproject --lang c ## ສ້າງ C hyper project".into(),
    );
    m.insert(
        "help.theme.example1".into(),
        "theme list ## ລາຍຊື່ themes ທີ່ມີທັງໝົດ".into(),
    );
    m.insert(
        "help.theme.example2".into(),
        "theme dark ## ສະຫຼັບໄປ theme 'dark'".into(),
    );
    m.insert(
        "help.theme.example3".into(),
        "theme current ## ສະແດງຂໍ້ມູນ theme ປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.theme.example4".into(),
        "theme add mytheme ## ສ້າງ theme ໃໝ່".into(),
    );
    m.insert(
        "help.teleport.example1".into(),
        "tp add web ## ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນເປັນ 'web'".into(),
    );
    m.insert(
        "help.teleport.example2".into(),
        "tp add docs ~/Documents ## ບັນທຶກ path ສະເພາະ".into(),
    );
    m.insert(
        "help.teleport.example3".into(),
        "tp jump web ## Teleport ໄປທີ່ 'web'".into(),
    );
    m.insert(
        "help.teleport.example4".into(),
        "tp list ## ລາຍຊື່ savepoints ທັງໝົດ".into(),
    );
    m.insert(
        "help.teleport.example5".into(),
        "@web ## ທາງລັດ teleport ດ່ວນ".into(),
    );
    m.insert(
        "help.ran.example1".into(),
        "ran ## ເຮັດວຽກ standalone target ເລີ່ມຕົ້ນ".into(),
    );
    m.insert(
        "help.ran.example2".into(),
        "ran build ## ເຮັດວຽກ target 'build'".into(),
    );
    m.insert(
        "help.ran.example3".into(),
        "ran list ## ລາຍຊື່ targets ທັງໝົດ".into(),
    );
    m.insert(
        "help.alias.example1".into(),
        "ral add build \"cargo build\" ## ສ້າງ alias ງ່າຍໆ".into(),
    );
    m.insert(
        "help.alias.example2".into(),
        "ral add greet(x) \"echo Hello $x\" ## alias ແບບ parameterised".into(),
    );
    m.insert(
        "help.alias.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## alias ພ້ອມຄ່າເລີ່ມຕົ້ນ".into(),
    );
    m.insert(
        "help.alias.example4".into(),
        "ral list ## ສະແດງ aliases ທັງໝົດ".into(),
    );
    m.insert(
        "help.alias.example5".into(),
        "ral export --all myaliases ## ສົ່ງອອກ aliases".into(),
    );
    m.insert(
        "help.alias.example6".into(),
        "build ## ເຮັດວຽກ alias ໂດຍກົງ".into(),
    );
    m.insert(
        "help.alias.example7".into(),
        "ral hintn add port [8080,5050] ## ລາຍການທີ່ຕັ້ງຊື່ໄວ້".into(),
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
    m.insert(
        "help.size.example1".into(),
        "size ## ສະແດງຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.size.example2".into(),
        "size ../src ## ສະແດງຂະໜາດຂອງໄດເຣັກທໍຣີອື່ນ".into(),
    );
    m.insert(
        "help.size.example3".into(),
        "size Cargo.toml ## ສະແດງຂະໜາດຂອງໄຟລ໌".into(),
    );
    m.insert(
        "help.size.example4".into(),
        "size @web --care -t ## ໂໝດ care ພ້ອມເວລາທີ່ໃຊ້ສຳລັບ savepoint".into(),
    );
    m.insert(
        "help.report.example1".into(),
        "report txt ## ສ້າງບົດລາຍງານ TXT".into(),
    );
    m.insert(
        "help.report.example2".into(),
        "report html --cp ## ສຳເນົາບົດລາຍງານ HTML ໃສ່ clipboard".into(),
    );
    m.insert(
        "help.report.example3".into(),
        "report pdf ## ສ້າງບົດລາຍງານ PDF".into(),
    );
    m.insert(
        "help.report.example4".into(),
        "txtc file.txt ## ສຳເນົາເນື້ອໃນໄຟລ໌ໃສ່ clipboard".into(),
    );
    m.insert(
        "help.backup.example1".into(),
        "backup create ## ສ້າງ backup ໃໝ່".into(),
    );
    m.insert(
        "help.backup.example2".into(),
        "backup list ## ລາຍຊື່ backups ທັ່ງໝົດ".into(),
    );
    m.insert(
        "help.backup.example3".into(),
        "backup restore 1 ## ກູ້ຄືນ backup #1".into(),
    );
    m.insert(
        "help.backup.example4".into(),
        "backup diff 1 ## ປຽບທຽບກັບ backup #1".into(),
    );
    m.insert(
        "help.search.example1".into(),
        "gs fn main ## ຄົ້ນຫາ 'fn main' ໃນໄຟລ໌".into(),
    );
    m.insert(
        "help.search.example2".into(),
        "gs -i config ## ຜົນການຄົ້ນຫາ interactive".into(),
    );
    m.insert(
        "help.search.example3".into(),
        "fs *.rs ## ຊອກຫາໄຟລ໌ .rs".into(),
    );
    m.insert(
        "help.search.example4".into(),
        "ds src ## ຊອກຫາໄດເຣັກທໍຣີທີ່ຊື່ 'src'".into(),
    );
    m.insert(
        "help.search.example5".into(),
        "locate README ## ຄົ້ນຫາໄຟລ໌ & ໄດເຣັກທໍຣີທີ່ມີ 'README'".into(),
    );
    m.insert(
        "help.search.example6".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + ສະແດງແບບ grep".into(),
    );
    m.insert(
        "help.search.example7".into(),
        "gs TODO < app.log ## ຄົ້ນຫາເນື້ອໃນທີ່ pipe/redirect ເຂົ້າມາ".into(),
    );
    m.insert(
        "help.search.example8".into(),
        "gs error\\|warning ## ຄົ້ນຫາແບບ OR: ກົງກັບຄຳໃດຄຳໜຶ່ງ (a\\|b)".into(),
    );
    m.insert(
        "misc.qur.stdin_needs_select".into(),
        "qur: ຂໍ້ມູນທີ່ pipe ເຂົ້າມາຕ້ອງໃຊ້ -s \"<pattern>\" (ເຊັ່ນ cmd | qur -s \"foo\")".into(),
    );
    m.insert(
        "misc.head.usage".into(),
        "ວິທີໃຊ້: head [N] [file] (N ຄ່າເລີ່ມຕົ້ນ 10; ຖ້າບໍ່ລະບຸໄຟລ໌ຈະອ່ານຈາກ pipe)".into(),
    );
    m.insert(
        "misc.tail.usage".into(),
        "ວິທີໃຊ້: tail [N] [file] (N ຄ່າເລີ່ມຕົ້ນ 10; ຖ້າບໍ່ລະບຸໄຟລ໌ຈະອ່ານຈາກ pipe)".into(),
    );
    m.insert(
        "misc.betw.usage".into(),
        "ວິທີໃຊ້: betw A:B [file] (A:B ເລີ່ມທີ່ 1 ລວມປາຍ; ຖ້າບໍ່ລະບຸໄຟລ໌ຈະອ່ານຈາກ pipe)".into(),
    );
    m.insert(
        "misc.window.bad_arg".into(),
        "{}: ອາກິວເມັນທີ່ບໍ່ຄາດຄິດ '{}'".into(),
    );
    m.insert(
        "misc.window.no_input".into(),
        "{}: ບໍ່ມີ input (ໃຫ້ pipe ເຂົ້າມາ ຫຼື ລະບຸໄຟລ໌)".into(),
    );
    m.insert(
        "misc.sort.usage".into(),
        "ວິທີໃຊ້: sort [-r] [-n] [-u] [-f] [file]  (ອ່ານ input ທີ່ pipe ເຂົ້າມາເມື່ອບໍ່ລະບຸໄຟລ໌)".into(),
    );
    m.insert("misc.sort.bad_arg".into(), "sort: ຕົວເລືອກທີ່ບໍ່ຮູ້ຈັກ '{}'".into());
    m.insert(
        "misc.rev.usage".into(),
        "ວິທີໃຊ້: rev [file]  (ຍ້ອນຕົວອັກສອນຂອງແຕ່ລະແຖວ; ອ່ານ input ທີ່ pipe ເຂົ້າມາເມື່ອບໍ່ລະບຸໄຟລ໌)".into(),
    );
    m.insert(
        "misc.rev.bad_arg".into(),
        "rev: ອາກິວເມັນທີ່ບໍ່ຄາດຄິດ '{}' (ວິທີໃຊ້: rev [file])".into(),
    );
    m.insert(
        "help.igcare.example1".into(),
        "ignore dir node_modules ## ບໍ່ສົນໃຈ node_modules".into(),
    );
    m.insert(
        "help.igcare.example2".into(),
        "ignore ext .log ## ບໍ່ສົນໃຈໄຟລ໌ .log".into(),
    );
    m.insert(
        "help.igcare.example3".into(),
        "care dir src ## ຍົກເລີກບໍ່ສົນໃຈໄດເຣັກທໍຣີ src".into(),
    );
    m.insert(
        "help.igcare.example4".into(),
        "igcared ## ສະແດງການຕັ້ງຄ່າ ignore/care ທັງໝົດ".into(),
    );
    m.insert(
        "help.network.example1".into(),
        "sys ## ສະແດງ system monitor".into(),
    );
    m.insert(
        "help.network.example2".into(),
        "http GET https://api.example.com ## ຄຳຮ້ອງຂໍ GET ງ່າຍໆ".into(),
    );
    m.insert(
        "help.network.example3".into(),
        "dl https://example.com/file.zip --progress ## ດາວໂຫຼດພ້ອມ progress".into(),
    );
    m.insert(
        "help.network.example4".into(),
        "ping google.com ## Ping google.com".into(),
    );
    m.insert(
        "help.network.example5".into(),
        "dns example.com -t MX ## DNS MX record lookup".into(),
    );
    m.insert(
        "help.network.example6".into(),
        "serve files --port 8080 ## ເລີ່ມ file-drive server ເທິງ port 8080".into(),
    );
    m.insert(
        "help.clipboard.example1".into(),
        "clip ## ສະແດງ clipboard history".into(),
    );
    m.insert(
        "help.clipboard.example2".into(),
        "clip select 1 ## ສຳເນົາ entry #1 ກັບໃສ່ clipboard".into(),
    );
    m.insert(
        "help.clipboard.example3".into(),
        "pathcp ## ສຳເນົາ path ໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.qsc.example1".into(),
        "config qsc \"#c 0\" ## ຕັ້ງ hotkey Ctrl+0".into(),
    );
    m.insert(
        "help.qsc.example2".into(),
        "qsc listen ## ເລີ່ມ QSC daemon".into(),
    );
    m.insert(
        "help.qsc.example3".into(),
        "qsc status ## ກວດສະຖານະ daemon".into(),
    );
    m.insert("help.todo.example1".into(), "todo ## ສະແດງວຽກທັງໝົດ".into());
    m.insert(
        "help.todo.example2".into(),
        "todo add \"Fix bug\" ## ເພີ່ມວຽກໃໝ່".into(),
    );
    m.insert("help.todo.example3".into(), "todo rm 1 ## ລຶບວຽກ #1".into());
    m.insert(
        "help.other.example1".into(),
        "init --ran ## ສ້າງ NTCRANFILE.toml".into(),
    );
    m.insert("help.other.example2".into(), "clear ## ລ້າງໜ້າຈໍ".into());
    m.insert(
        "help.other.example3".into(),
        "version ## ສະແດງຂໍ້ມູນເວີຊັນ".into(),
    );
    m.insert(
        "help.other.example4".into(),
        "tutorial ## ເລີ່ມ tutorial interactive".into(),
    );
    m.insert(
        "help.fileops.example1".into(),
        "mkf index.html ## ສ້າງໄຟລ໌ໃໝ່".into(),
    );
    m.insert(
        "help.fileops.example2".into(),
        "mkd src/components ## ສ້າງໄດເຣັກທໍຣີ".into(),
    );
    m.insert(
        "help.fileops.example3".into(),
        "cp file.txt backup.txt ## ສຳເນົາໄຟລ໌".into(),
    );
    m.insert(
        "help.tl.example1".into(),
        "tl file main.rs ## ນັບແຖວໃນ main.rs".into(),
    );
    m.insert(
        "help.tl.example2".into(),
        "tl ext .rs ## ນັບແຖວໃນໄຟລ໌ .rs ທັງໝົດ".into(),
    );
    m.insert(
        "help.tl.example3".into(),
        "tl dir . ## ນັບແຖວໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.tl.example4".into(),
        "tl ext .rs -s -r ## ລາຍຊື່ໄຟລ໌ .rs ຈັດຕາມແຖວ, ໃຫຍ່ຫານ້ອຍ".into(),
    );
    m.insert(
        "help.ntpkg.example1".into(),
        "ntpkg install node 18 ## ຕິດຕັ້ງ Node.js 18".into(),
    );
    m.insert(
        "help.ntpkg.example2".into(),
        "ntpkg list ## ສະແດງ runtimes ທີ່ຕິດຕັ້ງ".into(),
    );
    m.insert(
        "help.ntpkg.example3".into(),
        "ntpkg use node 18 ## ເປີດໃຊ້ Node.js 18".into(),
    );
    m.insert(
        "help.ntpkg.example4".into(),
        "ntpkg which node ## ສະແດງ path ຂອງ Node.js".into(),
    );
    m.insert(
        "help.ntpkg.example5".into(),
        "ntpkg search python ## ຄົ້ນຫາ packages Python".into(),
    );
    m.insert(
        "help.ntpkg.example6".into(),
        "ntpkg info node ## ສະແດງຂໍ້ມູນ package Node.js".into(),
    );
    m.insert(
        "help.ntpkg.example7".into(),
        "ntpkg available ## ລາຍຊື່ພາສາທີ່ດາວໂຫຼດໄດ້".into(),
    );
    m.insert(
        "help.ntpkg.example8".into(),
        "ntpx node npm init ## ເຮັດວຽກ npm ຜ່ານ ntpx".into(),
    );
    m.insert(
        "help.ntpkg.example9".into(),
        "ntpkg help ## ສະແດງການໃຊ້ງານ ntpkg".into(),
    );
    m.insert(
        "help.qr.example1".into(),
        "qr \"Hello World\" ## ສະແດງ QR code ໃນ terminal".into(),
    );
    m.insert(
        "help.qr.example2".into(),
        "qr encode \"Hello\" -o hello.png ## ບັນທຶກ QR ເປັນ PNG".into(),
    );
    m.insert(
        "help.qr.example3".into(),
        "qr decode qr.png ## ຖອດລະຫັດ QR ຈາກຮູບ".into(),
    );
    m.insert(
        "help.qr.example4".into(),
        "qr encode image.png -o imageqrcode.png ## ເຂົ້າລະຫັດຮູບເພື່ອໃຫ້ scan ເຫັນມັນ".into(),
    );
    m.insert(
        "help.pathman.example1".into(),
        "usrpath show ## ສະແດງ user PATH entries".into(),
    );
    m.insert(
        "help.pathman.example2".into(),
        "usrpath add C:\\tools ## ເພີ່ມໃສ່ user PATH".into(),
    );
    m.insert(
        "help.pathman.example3".into(),
        "usrpath rm 2 ## ລຶບ entry #2 ຈາກ user PATH".into(),
    );
    m.insert(
        "help.pathman.example4".into(),
        "syspath show ## ສະແດງ system PATH entries".into(),
    );
    m.insert(
        "help.pathman.example5".into(),
        "syspath add C:\\bin ## ເພີ່ມໃສ່ system PATH".into(),
    );
    m.insert(
        "help.pathman.example6".into(),
        "syspath move 3 1 ## ຍ້າຍ entry ໄປຕຳແໜ່ງ 1".into(),
    );
    m.insert(
        "help.autoup.example1".into(),
        "autoup list ## ລາຍຊື່ startup entries".into(),
    );
    m.insert(
        "help.autoup.example2".into(),
        "autoup add ntc \"C:\\ntc\\ntc.exe\" ## ເພີ່ມໃສ່ startup".into(),
    );
    m.insert(
        "help.autoup.example3".into(),
        "autoup rm ntc ## ລຶບ startup entry".into(),
    );
    m.insert(
        "help.autoup.example4".into(),
        "autoup disable ntc ## ປິດໃຊ້ງານ startup entry".into(),
    );
    m.insert(
        "help.mode.example1".into(),
        "mode admin ## ເລີ່ມໃໝ່ເປັນ Administrator".into(),
    );
    m.insert(
        "help.explorer.example1".into(),
        "explorer ## ເປີດ file explorer ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## ເປີດ explorer ໃນ ~/projects".into(),
    );
    m.insert(
        "help.math.example1".into(),
        "math 3+4*5 ## ຄຳນວນພົດ arithmetic".into(),
    );
    m.insert(
        "help.math.example2".into(),
        "math sin(PI/2) ## ຄິດໄລ່ດ້ວຍຟັງຊັນ".into(),
    );
    m.insert(
        "help.math.example3".into(),
        "math timer 30 ## ເລີ່ມນັບຖອຍຫຼັງ 30 ວິນາທີ".into(),
    );
    m.insert(
        "help.math.example4".into(),
        "math fun add myfn \"x+1\" ## ເພີ່ມ custom function".into(),
    );
    m.insert(
        "help.math.example5".into(),
        "math fun ls ## ລາຍຊື່ custom functions".into(),
    );
    m.insert(
        "help.math.example6".into(),
        "math calc.ntc.math ## ເຮັດວຽກໄຟລ໌ math".into(),
    );
    m.insert(
        "help.python.example1".into(),
        "py_activate ## ເປີດໃຊ້ Python venv ອັດຕະໂນມັດ".into(),
    );
    m.insert(
        "help.python.example2".into(),
        "py_activate .venv ## ເປີດໃຊ້ venv ສະເພາະ".into(),
    );
    m.insert(
        "help.python.example3".into(),
        "py_deactivate ## ປິດໃຊ້ງານ venv".into(),
    );
    m.insert(
        "help.encdec.example1".into(),
        "encode b64 \"hello\" ## Base64 encode ສະຕຣິງ".into(),
    );
    m.insert(
        "help.encdec.example2".into(),
        "encode hex file.bin ## Hex encode ໄຟລ໌".into(),
    );
    m.insert(
        "help.encdec.example3".into(),
        "decode b64 aGVsbG8= ## Base64 decode".into(),
    );
    m.insert(
        "help.encdec.example4".into(),
        "cvcode b64 \"aGVsbG8=\" to hex ## ແປງລະຫວ່າງຮູບແບບ".into(),
    );
    m.insert(
        "help.fmt.example1".into(),
        "fmt json config.json to yaml ## ແປງ JSON ເປັນ YAML".into(),
    );
    m.insert(
        "help.fmt.example2".into(),
        "fmt toml Cargo.toml to json -o output.json ## ແປງ TOML ເປັນ JSON".into(),
    );
    m.insert(
        "help.image.example1".into(),
        "image screenshot.png to json ## ແປງຮູບເປັນ JSON ສຳລັບ AI".into(),
    );
    m.insert(
        "help.image.example2".into(),
        "image photo.jpg --ocr ## ສະກັດຂໍ້ຄວາມຈາກຮູບຜ່ານ OCR".into(),
    );
    m.insert(
        "helpers.building_tree".into(),
        "ກຳລັງສ້າງ directory tree...".into(),
    );
    m.insert(
        "helpers.calculating_sizes".into(),
        "ກຳລັງຄິດໄລ່ຂະໜາດໄດເຣັກທໍຣີ...".into(),
    );
    m.insert("helpers.export_cancelled".into(), "ການສົ່ງອອກຖືກຍົກເລີກ.".into());
    m.insert(
        "helpers.exported_all".into(),
        "ສົ່ງອອກ alias ທັງໝົດ {} ໄປຍັງ: {}".into(),
    );
    m.insert(
        "helpers.exported_categories".into(),
        "ສົ່ງອອກ {} ປະເພດໄປຍັງ: {}".into(),
    );
    m.insert(
        "helpers.exported_count".into(),
        "ສົ່ງອອກ alias {} ໄປຍັງ: {}".into(),
    );
    m.insert(
        "helpers.exported_igcare_all".into(),
        "ສົ່ງອອກການຕັ້ງຄ່າ ignore/care ທັງໝົດໄປຍັງ: {}".into(),
    );
    m.insert("helpers.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());
    m.insert(
        "helpers.imported_igcare".into(),
        "ນຳເຂົ້າການຕັ້ງຄ່າ ignore/care ຈາກ: {}".into(),
    );
    m.insert(
        "helpers.imported_ral".into(),
        "ນຳເຂົ້າ run aliases ຈາກ: {}".into(),
    );
    m.insert(
        "helpers.invalid_back_count".into(),
        "ຈຳນວນ back ບໍ່ຖືກຕ້ອງ: {}".into(),
    );
}
