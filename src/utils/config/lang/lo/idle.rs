use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "idle.tp".into(),
        "ລອງ: tp add <ຊື່ໂຟນເດີ>  ແລ້ວ: @<ຊື່ໂຟນເດີ>   (ບັນທຶກແລະກັບມາບ່ອນນີ້)".into(),
    );

    m.insert(
        "idle.explorer".into(),
        "ລອງ: nfe   (ເບິ່ງໂຟນເດີດ້ວຍ file explorer; f ຄົ້ນຫາໄຟລ໌)".into(),
    );

    m.insert(
        "idle.ne".into(),
        "ລອງ: ne --init <ຟາຍ>  (ສ້າງໄຟລ໌ຈາກແມ່ແບບ)".into(),
    );

    m.insert(
        "idle.view".into(),
        "ລອງ: view -d 2   (ສະແດງໂຄງສ້າງສອງຊັ້ນ)".into(),
    );

    m.insert(
        "idle.search".into(),
        "ລອງ: gs <ຂໍ້ຄວາມ>   (ຄົ້ນຂໍ້ຄວາມນັ້ນໃນໄຟລ໌ຂອງໂຟນເດີນີ້)".into(),
    );

    m.insert(
        "idle.todo".into(),
        "ລອງ: todo add \"ຊື່ວຽກ\"   (ໂຟນເດີນີ້ມີລາຍການວຽກແລ້ວ)".into(),
    );

    m.insert(
        "idle.git".into(),
        "ລອງ: git status   (ນີ້ຄື git repository)".into(),
    );

    m.insert(
        "idle.serve".into(),
        "ລອງ: serve files --lock   (ແບ່ງປັນໂຟນເດີນີ້ແບບອ່ານຢ່າງດຽວຜ່ານ HTTP)".into(),
    );

    m.insert(
        "idle.tutorial".into(),
        "ລອງ: tutorial   (ຝຶກໃຊ້ຈິງໃນ sandbox ທີ່ລຶກທິ້ງໄດ້)".into(),
    );

    m.insert(
        "idle.ral".into(),
        "ລອງ: ral add <ຊື່> \"<ຄຳສັ່ງ>\"   (ພິມ <ຊື່> ເພື່ອຣັນອີກຄັ້ງ)".into(),
    );

    m.insert(
        "idle.ran".into(),
        "ລອງ: ran list   (ໂຟນເດີນີ້ມີ NTCRANFILE.toml ກຳນົດວຽກໄວ້)".into(),
    );

    m.insert(
        "idle.pipe".into(),
        "ລອງ: gs <ຂໍ້ຄວາມ> | sort -u | head 10   (ສົ່ງຜົນໄປຍັງຄຳສັ່ງຕໍ່ໄປ)".into(),
    );

    m.insert(
        "idle.math".into(),
        "ລອງ: math 3+4*5   (ຄິດໄລ່ມີຟັງຊັນ ບໍ່ຕ້ອງໃຊ້ໂປຣແກຣມອື່ນ)".into(),
    );

    m.insert(
        "idle.qr".into(),
        "ລອງ: qr <ຂໍ້ຄວາມ|ລິ້ງ>   (ພິມ QR code ໃຫ້ສະແກນໄດ້)".into(),
    );

    m.insert(
        "idle.theme".into(),
        "ລອງ: theme list   (ແລ້ວ: theme <ຊື່> - ປ່ຽນສີຂອງ shell)".into(),
    );

    m.insert(
        "idle.backup".into(),
        "ລອງ: backup create   (ສຳຮອງໂຟນເດີນີ້ກ່ອນແກ້ທີ່ສ່ຽງ)".into(),
    );

    m.insert(
        "idle.empty".into(),
        "ລອງ: mkf notes.md   (ໂຟນເດີຫວ່າງເປົ່າ - ເລີ່ມຈາກໄຟລ໌ກ່ອນ)".into(),
    );

    m.insert(
        "idle.help".into(),
        "ລອງ: help   (ເບິ່ງຄຳສັ່ງທັງໝົດ ຫຼື: help <ຫົວຂໍ້>)".into(),
    );

    m.insert(
        "idle.tl".into(),
        "ລອງ: tl dir .   (ນັບຈຳນວນແຖວຂອງໄຟລ໌ທີ່ຮອງຮັບໃນໂຟນເດີນີ້)".into(),
    );

    m.insert("idle.size".into(), "ລອງ: size   (ຂະໜາດລວມຂອງໂຟນເດີນີ້)".into());

    m.insert(
        "idle.report".into(),
        "ລອງ: report project md   (ສ້າງລາຍງານໂຄງການເປັນ markdown ໄວ້ນີ້)".into(),
    );

    m.insert(
        "idle.scr".into(),
        "ລອງ: scr init demo   (ເລີ່ມສະຄຣິບ .ntc ທີ່ໃຊ້ຄືນໄດ້)".into(),
    );

    m.insert(
        "idle.time".into(),
        "ລອງ: time bench --runs 3 <ຄຳສັ່ງ>   (ວັດເວລາສະເລ່ຍຂອງຄຳສັ່ງ)".into(),
    );

    m.insert(
        "idle.encode".into(),
        "ລອງ: encode b64 \"hello\"   (base64, hex, url ແລະອື່ນໆ)".into(),
    );

    m.insert(
        "idle.fmt".into(),
        "ລອງ: fmt Cargo.toml to json   (ແປງຮູບແບບໄຟລ໌ config)".into(),
    );

    m.insert(
        "idle.arc".into(),
        "ລອງ: arc list <ໄຟລ໌.zip>   (ເບິ່ງລາຍການໃນ archive ໂດຍບໍ່ແຕກໄຟລ໌)".into(),
    );

    m.insert(
        "idle.scan".into(),
        "ລອງ: scan ext rs   (ສະແດງໄຟລ໌ .rs ທັງໝົດຢູ່ນີ້)".into(),
    );

    m.insert(
        "idle.sys".into(),
        "ລອງ: sys   (ຕິດຕາມ CPU, ໜ່ວຍຄວາມຈຳ ແລະ process ແບບສົດໆ)".into(),
    );

    m.insert(
        "idle.showcg".into(),
        "ລອງ: showcg   (ເບິ່ງການຕັ້ງຄ່າທັງໝົດໃນເວລາດຽວ)".into(),
    );

    m.insert(
        "idle.ntpkg".into(),
        "ລອງ: ntpkg list   (runtime ທີ່ຕິດຕັ້ງຜ່ານ ntc)".into(),
    );

    m.insert(
        "idle.ndb".into(),
        "ລອງ: ndb   (ເປີດເບິ່ງຖານຂໍ້ມູນ SQLite ໃນໂຟນເດີນີ້)".into(),
    );

    m.insert(
        "idle.nsheet".into(),
        "ລອງ: nsheet open <ໄຟລ໌.csv>   (ເບິ່ງສະເປຣດຊີດ)".into(),
    );

    m.insert(
        "idle.fgo".into(),
        "ລອງ: fgo task.txt   (ຄົ້ນໄຟລ໌ຕາມຊື່ແລ້ວໂດດໄປ)".into(),
    );

    m.insert(
        "idle.back".into(),
        "ລອງ: back   (ຂຶ້ນໜຶ່ງໂຟນເດີ; gos, gosc, godrive ກໍເປັນຄຳດຽວ)".into(),
    );

    m.insert(
        "idle.viewf".into(),
        "ລອງ: viewf .   (ເບິ່ງສະເພາະໄຟລ໌) · viewd . ເບິ່ງສະເພາະໂຟນເດີ".into(),
    );

    m.insert(
        "idle.views".into(),
        "ລອງ: views .   (ລາຍການໂຟນເດີພ້ອມ --size ແລະ --sort)".into(),
    );

    m.insert(
        "idle.txtf".into(),
        "ລອງ: txtf task.txt --head 20   (ສະແດງໄຟລ໌) · txtc ສຳເນົາໃສ່ clipboard".into(),
    );

    m.insert(
        "idle.text".into(),
        "ລອງ: head 5 task.txt   (ມີ tail, betw 2:4, sort, rev)".into(),
    );

    m.insert(
        "idle.count".into(),
        "ລອງ: view task.txt | count ntc   (ນັບຄຳທີ່ຕົງໃນຂໍ້ຄວາມທີ່ສົ່ງຕໍ່) · trim ລຶບ".into(),
    );

    m.insert(
        "idle.files".into(),
        "ລອງ: mkf notes.md   (ແລ້ວ mkd, cp, mv, rnmf, rmf, rmd)".into(),
    );

    m.insert(
        "idle.config".into(),
        "ລອງ: config output   (ເບິ່ງຄ່າໜຶ່ງ; showcg ເບິ່ງທັງໝົດ)".into(),
    );

    m.insert(
        "idle.opencg".into(),
        "ລອງ: opencg   (ແກ້ config.toml) · resetcg ແລະ restorecg ຄືນໄດ້".into(),
    );

    m.insert(
        "idle.local".into(),
        "ລອງ: config local init   (ຕັ້ງຄ່າສະເພາະໂຟນເດີນີ້)".into(),
    );

    m.insert(
        "idle.project".into(),
        "ລອງ: init   (ສ້າງ NTCRANFILE.toml ຢູ່ນີ້) · deinit ລຶບອອກ".into(),
    );

    m.insert(
        "idle.tpb".into(),
        "ລອງ: tpb history   (ເສັ້ນທາງທີ່ເຄີຍໂດດໄປ)".into(),
    );

    m.insert(
        "idle.qur".into(),
        "ລອງ: qur *.txt -s \"old\" -dp   (ເລືອກກ່ອນ ແລ້ວ -rp ເພື່ອແທນທີ່)".into(),
    );

    m.insert(
        "idle.fsc".into(),
        "ລອງ: fsc task.txt   (ຫາໄຟລ໌ແລ້ວເປີດ) · fs ຫາໄຟລ໌, ds ຫາໂຟນເດີ".into(),
    );

    m.insert(
        "idle.ignore".into(),
        "ລອງ: ignore dir target   (ຂ້າມໂຟນເດີນີ້ຕອນສະແກນ) · care ຍົກເລີກ".into(),
    );

    m.insert(
        "idle.igcare".into(),
        "ລອງ: igcared   (ເບິ່ງກົດ ignore ແລະ care ທັງໝົດ)".into(),
    );

    m.insert(
        "idle.http".into(),
        "ລອງ: http --help   (HTTP client: method, header, body)".into(),
    );

    m.insert(
        "idle.dl".into(),
        "ລອງ: dl --help   (ດາວໂຫຼດຕໍ່ໄດ້ດ້ວຍ --resume)".into(),
    );

    m.insert(
        "idle.dns".into(),
        "ລອງ: dns example.com -t MX   (ຄົ້ນລະບຽບ DNS)".into(),
    );

    m.insert(
        "idle.up".into(),
        "ລອງ: up --help   (ອັບໂຫຼດໄຟລ໌ຜ່ານ HTTP)".into(),
    );

    m.insert(
        "idle.ping".into(),
        "ລອງ: ping 127.0.0.1   (ກວດວ່າໂຮສຕອບ ຫຼື ບໍ່)".into(),
    );

    m.insert(
        "idle.trace".into(),
        "ລອງ: trace 127.0.0.1   (ເສັ້ນທາງເທື່ອລະ hop)".into(),
    );

    m.insert(
        "idle.smtp".into(),
        "ລອງ: smtp --help   (ສົ່ງອີເມວ; ລະຫັດຜ່ານຢູ່ໃນ deadlock)".into(),
    );

    m.insert(
        "idle.ipcfg".into(),
        "ລອງ: ipcfg   (ອາແດັບເຕີ ແລະ ທີ່ຢູ່ຂອງເຄື່ອງນີ້)".into(),
    );

    m.insert(
        "idle.wi".into(),
        "ລອງ: wi .   (ຕິດຕາມການປ່ຽນແປງຂອງໂຟນເດີນີ້; q ອອກ)".into(),
    );

    m.insert(
        "idle.spec".into(),
        "ລອງ: spec   (ສະຫຼຸບ CPU, RAM, ດິສ ແລະ ລະບົບ)".into(),
    );

    m.insert(
        "idle.tmprt".into(),
        "ລອງ: tmprt city   (ຕັ້ງເມືອງເລີ່ມຕົ້ນ) · tmprt com ເບິ່ງເຊັນເຊີ".into(),
    );

    m.insert(
        "idle.clip".into(),
        "ລອງ: clip   (ຊ່ອງ clipboard) · clip 2 ເອົາຊ່ອງທີ 2".into(),
    );

    m.insert(
        "idle.qsc".into(),
        "ລອງ: qsc status   (daemon ຄີລັດກຳລັງເຮັດວຽກຢູ່ບໍ່)".into(),
    );

    m.insert(
        "idle.pathman".into(),
        "ລອງ: usrpath show   (PATH ຂອງຜູ້ໃຊ້) · syspath show ຂອງລະບົບ".into(),
    );

    m.insert(
        "idle.autoup".into(),
        "ລອງ: autoup list   (ໂປຣແກຣມທີ່ເລີ່ມພ້ອມເຄື່ອງ)".into(),
    );

    m.insert(
        "idle.mode".into(),
        "ລອງ: mode   (ໂໝດຜູ້ດູແລ ຫຼື ປົກກະຕິປັດຈຸບັນ)".into(),
    );

    m.insert(
        "idle.python".into(),
        "ລອງ: showenv   (ຕົວແປສະພາບແວດລ້ອມ) · py_activate ເປີດ venv".into(),
    );

    m.insert(
        "idle.decode".into(),
        "ລອງ: decode b64 \"aGVsbG8=\"   (ຍ້ອນ encode) · cvcode ແປງຮູບແບບ".into(),
    );

    m.insert(
        "idle.image".into(),
        "ລອງ: image --help   (ສະແດງຮູບໃນ terminal)".into(),
    );

    m.insert(
        "idle.nssh".into(),
        "ລອງ: nssh ls   (ໂປຣໄຟລ໌ SSH ທີ່ບັນທຶກໄວ້)".into(),
    );

    m.insert(
        "idle.nms".into(),
        "ລອງ: nms scan .   (ຫາໄຟລ໌ເພງ ແລະ ວິດີໂອຢູ່ນີ້)".into(),
    );

    m.insert(
        "idle.fake".into(),
        "ລອງ: fake gen name,email -n 3   (ຂໍ້ມູນທົດສອບ; fake types ເບິ່ງທັງໝົດ)".into(),
    );

    m.insert(
        "idle.deadlock".into(),
        "ລອງ: deadlock   (ຕູ້ລະຫັດຜ່ານທີ່ເຂົ້າລະຫັດ)".into(),
    );

    m.insert(
        "idle.ntpx".into(),
        "ລອງ: ntpx   (ຣັນ runtime ທີ່ ntpkg ຕິດຕັ້ງ)".into(),
    );

    m.insert(
        "idle.pathcp".into(),
        "ລອງ: pathcp   (ສຳເນົາເສັ້ນທາງປັດຈຸບັນໃສ່ clipboard)".into(),
    );

    m.insert(
        "idle.say".into(),
        "ລອງ: say hello --loop 3   (ພິມຂໍ້ຄວາມ; print ຄືກັນ)".into(),
    );

    m.insert(
        "idle.ui".into(),
        "ລອງ: ui   (ເບິ່ງໂໝດ UI) · version ເບິ່ງເວີຊັນ".into(),
    );
}
