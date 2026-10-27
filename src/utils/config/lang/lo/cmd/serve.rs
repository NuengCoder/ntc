use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.serve.db_title".into(), "ຖານຂໍ້ມູນ:".into());

    m.insert(
        "cmd.serve.example_default".into(),
        "  serve files            HTTP ຢູ່ port 8080".into(),
    );

    m.insert(
        "cmd.serve.example_port".into(),
        "  serve files 3000       HTTP ຢູ່ port 3000".into(),
    );

    m.insert(
        "cmd.serve.example_spa".into(),
        "  serve files --spa      SPA fallback ສຳລັບ React Router".into(),
    );

    m.insert(
        "cmd.serve.example_tunnel".into(),
        "  serve files --tunnel   HTTP + cloudflared tunnel".into(),
    );

    m.insert("cmd.serve.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert("cmd.serve.files_stored_title".into(), "ເກັບໄຟລ໌ໄວ້ທີ່:".into());

    m.insert(
        "cmd.serve.help_bind".into(),
        "  --bind, -b <addr>     ຜູກ bind address (ເລີ່ມຕົ້ນ: 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.help_help".into(),
        "  --help, -h            ສະແດງ help ນີ້".into(),
    );

    m.insert(
        "cmd.serve.help_port".into(),
        "  --port, -p <port>     Port ທີ່ຟັງ (ເລີ່ມຕົ້ນ: 8080)".into(),
    );

    m.insert(
        "cmd.serve.help_spa".into(),
        "  --spa                 SPA mode (fallback ໄປ index.html)".into(),
    );

    m.insert("cmd.serve.help_title".into(), "HTTP File Server".into());

    m.insert(
        "cmd.serve.help_tunnel".into(),
        "  --tunnel, -t          ເປີດເຜີຍຜ່ານ cloudflared tunnel".into(),
    );

    m.insert(
        "cmd.serve.help_share".into(),
        "  --share [path]        ກວດເນັດ ແລ້ວສະແດງ LAN URL + QR ສຳລັບມືຖື".into(),
    );

    m.insert(
        "cmd.serve.help_qr".into(),
        "  --qr                  ບັງຄັບສະແດງ QR (ລວມ --share)".into(),
    );

    m.insert(
        "cmd.serve.help_no_qr".into(),
        "  --no-qr               ບໍ່ສະແດງ QR".into(),
    );

    m.insert(
        "cmd.serve.example_share".into(),
        "  serve files 8080 --share  ກວດເນັດ ແລ້ວພິມ LAN URL + QR".into(),
    );

    m.insert(
        "cmd.serve.share_checking".into(),
        "ກຳລັງກວດການເຊື່ອມຕໍ່ອິນເຕີເນັດ...".into(),
    );

    m.insert(
        "cmd.serve.share_online".into(),
        "ອິນເຕີເນັດປົກກະຕິ — ໂຫມດແຊຣ໌".into(),
    );

    m.insert(
        "cmd.serve.share_offline".into(),
        "ບໍ່ມີອິນເຕີເນັດ — ສະແດງສະເພາະ localhost".into(),
    );

    m.insert("cmd.serve.share_lan_title".into(), "LAN:".into());

    m.insert("cmd.serve.share_path_title".into(), "ແຊຣ໌:".into());

    m.insert(
        "cmd.serve.share_no_lan".into(),
        "ບໍ່ພົບທີ່ຢູ່ LAN — ສະແດງສະເພາະ localhost".into(),
    );

    m.insert(
        "cmd.serve.share_bound_loopback".into(),
        "bind ຢູ່ loopback: ອຸປະກອນ LAN ເຂົ້າບໍ່ເຖິງ (ໃຊ້ຄ່າເລີ່ມຕົ້ນ 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.share_not_private".into(),
        "ທີ່ຢູ່ {} ບໍ່ແມ່ນຊ່ວງ LAN ສ່ວນຕົວ".into(),
    );

    m.insert(
        "cmd.serve.share_not_private_hint".into(),
        "ການແຊຣ໌ມືຖືຕ້ອງໃຊ້ 10.x.x.x, 172.16-31.x.x ຫຼື 192.168.x.x".into(),
    );

    m.insert(
        "cmd.serve.share_qr_fail".into(),
        "render QR ລົ້ມເຫຼວ: {}".into(),
    );

    m.insert(
        "cmd.serve.help_usage".into(),
        "serve files [port] [options]".into(),
    );

    m.insert(
        "cmd.serve.help_web_dir".into(),
        "  --web-dir, -w <dir>   ສົ່ງ frontend directory ທີ່ກຳນົດເອງ".into(),
    );

    m.insert(
        "cmd.serve.help_auth".into(),
        "  --auth <user:pass>     ຕ້ອງເຂົ້າສູ່ລະບົບສຳລັບ /api/* (ເຊັ່ນ --auth admin:secret)".into(),
    );

    m.insert(
        "cmd.serve.example_auth".into(),
        "  serve files 8080 --auth admin:secret   HTTP ຢູ່ 8080, API ຕ້ອງເຂົ້າສູ່ລະບົບ".into(),
    );

    m.insert(
        "cmd.serve.invalid_auth".into(),
        "--auth ບໍ່ຖືກຕ້ອງ (ໃຊ້ --auth <user:pass>)".into(),
    );

    m.insert("cmd.serve.missing_auth".into(), "ຂາດຄ່າຂອງ --auth".into());

    m.insert(
        "cmd.serve.auth_on".into(),
        "ເປີດການຢືນຢັນຕົວຕົນ API ສຳລັບຜູ້ໃຊ້ '{}' ແລ້ວ".into(),
    );

    m.insert(
        "cmd.serve.help_lock".into(),
        "  --lock                ອ່ານຢ່າງດຽວ: ດາວໂຫລດ/ເບິ່ງໄດ້ ຫ້າມອັບໂຫລດ/ແກ້ໄຂ/ສ້າງ/ລຶບ".into(),
    );

    m.insert(
        "cmd.serve.lock_on".into(),
        "ໂຫມດອ່ານຢ່າງດຽວ (--lock): ບລັອກອັບໂຫລດ ແກ້ໄຂ ສ້າງ ແລະ ລຶບແລ້ວ".into(),
    );

    m.insert("cmd.serve.invalid_port".into(), "port ບໍ່ຖືກຕ້ອງ".into());

    m.insert("cmd.serve.missing_bind".into(), "ຂາດ bind".into());

    m.insert("cmd.serve.missing_dir".into(), "ຂາດ dir".into());

    m.insert("cmd.serve.missing_port".into(), "ຂາດ port".into());

    m.insert("cmd.serve.options_title".into(), "ຕົວເລືອກ:".into());

    m.insert("cmd.serve.stopped".into(), "ຢຸດ server ແລ້ວ.".into());

    m.insert("cmd.serve.unknown_option".into(), "ບໍ່ຮູ້ຈັກ: {}".into());

    m.insert("cmd.serve.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.serve.cloudflared_starting".into(),
        "ກຳລັງເລີ່ມ tunnel ຂອງ cloudflared (PID {})".into(),
    );

    m.insert(
        "cmd.serve.cloudflared_missing".into(),
        "ບໍ່ພົບ cloudflared: {}. ຕິດຕັ້ງ: {}".into(),
    );

    m.insert(
        "cmd.serve.chess_soon".into(),
        "`serve chess` ກຳລັງມາ (ຫຼິ້ນອອນລາຍຫຼາຍຄົນ)".into(),
    );

    m.insert(
        "cmd.serve.services_title".into(),
        "Serve — ເລືອກບໍລິການ".into(),
    );

    m.insert(
        "cmd.serve.services_usage".into(),
        "serve <ບໍລິການ> [ຕົວເລືອກ] (`serve` ເປົ່າບໍ່ຣັນຫຍັງ)".into(),
    );

    m.insert("cmd.serve.services_header".into(), "ບໍລິການ:".into());

    m.insert(
        "cmd.serve.services_row_files".into(),
        "  files         ເວັບ file-drive (ອັບໂຫຼດ/ດາວໂຫຼດໃນບຣາວເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.services_row_http".into(),
        "  http-server   ເຊີເວີໄຟລ໌ສະຖິດຂອງໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "cmd.serve.services_row_chess".into(),
        "  chess         ໝากรุก 2 ຄົນ hotseat ໃນບຣາວເຊີ".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        ເບິ່ງ/ຕັ້ງຄ່າເລີ່ມຕົ້ນລາຍບໍລິການ (ພອດ, bind)".into(),
    );

    m.insert(
        "cmd.serve.services_example_files".into(),
        "  serve files --port 9000".into(),
    );

    m.insert(
        "cmd.serve.services_example_http".into(),
        "  serve http-server ./dist".into(),
    );

    m.insert(
        "cmd.serve.services_example_config".into(),
        "  serve config files --port-default 9000".into(),
    );

    m.insert(
        "cmd.serve.http_title".into(),
        "ເຊີເວີໄຟລ໌ສະຖິດ HTTP (ໄດເຣັກທໍຣີປັດຈຸບັນ)".into(),
    );

    m.insert(
        "cmd.serve.http_usage".into(),
        "serve http-server [dir] [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_http_default".into(),
        "  serve http-server            ສົ່ງ CWD ຢູ່ພອດ 8080".into(),
    );

    m.insert(
        "cmd.serve.example_http_dir".into(),
        "  serve http-server ./dist --port 8001   ສົ່ງ ./dist ຢູ່ພອດ 8001".into(),
    );

    m.insert(
        "cmd.serve.example_http_auth".into(),
        "  serve http-server --auth admin:secret  ຕ້ອງເຂົ້າສູ່ລະບົບ".into(),
    );

    m.insert(
        "cmd.serve.help_auth_http".into(),
        "  --auth <user:pass>     ທຸກເສັ້ນທາງຕ້ອງເຂົ້າສູ່ລະບົບ (ເຊັ່ນ --auth admin:secret)".into(),
    );

    m.insert("cmd.serve.http_serving".into(), "ກຳລັງສົ່ງ".into());

    m.insert("cmd.serve.http_no_dir".into(), "ບໍ່ພົບໄດເຣັກທໍຣີ '{}'".into());

    m.insert(
        "cmd.serve.current_defaults".into(),
        "ຄ່າເລີ່ມຕົ້ນທີ່ຕັ້ງໄວ້ (serve config): ພອດ {} bind {}".into(),
    );

    m.insert(
        "cmd.serve.chess_no_auth".into(),
        "`serve chess` ບໍ່ຮັບ --auth (hotseat ບໍ່ຕ້ອງເຂົ້າສູ່ລະບົບ)".into(),
    );

    m.insert("cmd.serve.chess_serving".into(), "ເຊີເວີໝากรຸກຢູ່".into());

    m.insert(
        "cmd.serve.chess_title".into(),
        "ໝากรุก (2 ຄົນ hotseat)".into(),
    );

    m.insert(
        "cmd.serve.chess_usage".into(),
        "serve chess [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_chess_default".into(),
        "  serve chess              ໝากรຸກຢູ່ພອດ 8080".into(),
    );

    m.insert(
        "cmd.serve.example_chess_port".into(),
        "  serve chess --port 9000  ໝากรຸກຢູ່ພອດ 9000".into(),
    );
}
