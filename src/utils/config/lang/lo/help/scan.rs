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
}
