use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
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
    m.insert(
        "help.scan.waep".into(),
        "scan waep <ໂຮສ|URL> ## ຄົ້ນຫາເສັ້ນທາງເວັບ (/api, /admin, /upload, ...)".into(),
    );

    m.insert(
        "help.scan.waep_base".into(),
        "scan waep example.com/api -r -d 2 ## ຄົ້ນຕໍ່ໃຕ້ base path".into(),
    );

    m.insert(
        "help.scan.waep_flags".into(),
        "scan waep <ເປົ້າໝາຍ> [-w <ຟາຍ>] [-x <ນາມສະກຸນ,...>] [-m GET|HEAD] [-s <ລະຫັດ>] [--all] ## wordlist, ນາມສະກຸນ, method, ສະຖານະທີ່ຮັບ".into(),
    );

    m.insert(
        "help.scan.waep_recursive".into(),
        "scan waep <ເປົ້າໝາຍ> [-r] [-d 1-4] [-c 1-64] [--timeout <ms>] [-H 'ຊື່: ຄ່າ'] [-A <ua>] [-o <ຟາຍ>] [-q] ## ການລົງເລິກ, ຄວາມໄວ, header, ຜົນອອກ".into(),
    );

    m.insert(
        "help.scan.example8".into(),
        "scan waep http://127.0.0.1:55555/api ## ເບິ່ງວ່າ API ຂອງ ntc serve ເປີດຫຍັງໄວ້".into(),
    );

    m.insert(
        "help.scan.example9".into(),
        "scan waep example.com -x php,html --all ## ສະແດງທຸກການຕອບສະໜອງ ລວມ .php/.html".into(),
    );

    m.insert(
        "help.scan.example10".into(),
        "scan waep example.com/api -r -d 2 -c 32 ## ລົງສອງຊັ້ນດ້ວຍ 32 probe ພ້ອມກັນ".into(),
    );
}
