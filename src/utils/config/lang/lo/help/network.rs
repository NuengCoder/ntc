use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.network.dl".into(),
        "dl <url> ## ດາວໂຫຼດໄຟລ໌ (--progress, --resume, --stop, --cancel)".into(),
    );

    m.insert(
        "help.network.dns".into(),
        "dns <host> ## DNS lookup (-t ປະເພດ record: A, AAAA, ALL, MX, NS, ອື່ນໆ)".into(),
    );

    m.insert(
        "help.network.http".into(),
        "http <method> <url> ## HTTP API client (-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t)"
            .into(),
    );

    m.insert(
        "help.network.ping".into(),
        "ping <host> ## ICMP ping (-c count)".into(),
    );

    m.insert(
        "help.network.ipcfg".into(),
        "ipcfg [ຊື່] ## ສະແດງ adapters ແບບ ipconfig/ifconfig".into(),
    );

    m.insert(
        "help.network.serve".into(),
        "serve files|http-server|config ## ບໍລິການ: file-drive, ເຊີເວີ CWD, ຄ່າເລີ່ມຕົ້ນລາຍບໍລິການ".into(),
    );

    m.insert(
        "help.network.smtp".into(),
        "smtp ## SMTP email client (--id, -i, -u, -pwd, -s, -b, -f, --to)".into(),
    );

    m.insert(
        "help.network.spec".into(),
        "spec ## ສະແດງສະເປັກຄອມພິວເຕີ/ລະບົບ (CPU, GPU, RAM, ບ່ອນເກັບຂໍ້ມູນ, OS, ເລກຊີຣຽວ)".into(),
    );

    m.insert(
        "help.network.sys".into(),
        "sys, top ## ຕິດຕາມລະບົບແບບສົດ (CPU, memory, disk, processes)".into(),
    );

    m.insert("help.network.title".into(), "ຄຳສັ່ງ SYSTEM & NETWORK:".into());

    m.insert(
        "help.network.up".into(),
        "up <file> <url> ## ອັບໂຫຼດໄຟລ໌ (--post, -H, -d)".into(),
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
}
