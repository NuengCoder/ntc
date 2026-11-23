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
    m.insert(
        "cmd.serve.url_web".into(),
        "ໜ້າເວັບ:     {}  (ເບິ່ງ + ລາກໄຟລ໌ອັບໂຫຼດ)".into(),
    );

    m.insert(
        "cmd.serve.url_upload".into(),
        "ອັບໂຫຼດ:     {}  (POST multipart, ຊ່ອງ 'file')".into(),
    );

    m.insert(
        "cmd.serve.url_download".into(),
        "ດາວໂຫຼດ:     {}  (ຕໍ່ດ້ວຍເສັ້ນທາງໄຟລ໌)".into(),
    );

    m.insert(
        "cmd.serve.url_info".into(),
        "ຂໍ້ມູນ:      {}  (ນີ້ຄືຫຍັງ + ຕົວຢ່າງ curl/wget/ntc)".into(),
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
        "cmd.serve.services_row_git".into(),
        "  git           Git GUI: status, diff, stage, commit, branch, tag, stash, push/pull"
            .into(),
    );

    m.insert(
        "cmd.serve.services_row_arc".into(),
        "  arc           Archive GUI: ສ້າງ, ເບິ່ງ/ແຕກໄຟລ໌ບີບອັດ".into(),
    );

    m.insert(
        "cmd.serve.services_row_theme".into(),
        "  theme         Theme GUI: ແກ້ໄຂ, ເບິ່ງຕົວຢ່າງ, ບັນທຶກ ແລະ ນຳໃຊ້ທີມ".into(),
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
    m.insert(
        "cmd.serve.help_online".into(),
        "  --online              ເປີດ lobby ແລະ ຫ້ອງອອນລາຍ (ແຂກ, ຈັບຄູ່ໄວ, ຫ້ອງ coach)".into(),
    );

    m.insert(
        "cmd.serve.example_chess_online".into(),
        "  serve chess --online -b 0.0.0.0 --share  ໝາກຮຸກອອນລາຍຜ່ານ LAN ພ້ອມ QR".into(),
    );

    m.insert(
        "cmd.serve.git_serving".into(),
        "ເຊີບເວີ Git ເຮັດວຽກຢູ່ {} ({})".into(),
    );

    m.insert("cmd.serve.git_title".into(), "Git (ອິນເຕີເຟສບຣາວເຊີ)".into());

    m.insert(
        "cmd.serve.git_usage".into(),
        "serve git [ໄດເຣກະທໍລີ] [--port 8000] [--bind ທີ່ຢູ່]".into(),
    );

    m.insert(
        "cmd.serve.git_help_port".into(),
        "  --port, -p <ພອດ>     ພອດທີ່ຮັບຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.git_help_bind".into(),
        "  --bind, -b <ທີ່ຢູ່>  ທີ່ຢູ່ທີ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.git_example".into(),
        "  serve git --port 9000  ໃຫ້ບໍລິການໂຄງການປັດຈຸບັນທີ່ພອດ 9000".into(),
    );

    m.insert("cmd.serve.git_no_repo".into(), "ບໍ່ແມ່ນໂຄງການ Git: {}".into());

    m.insert("cmd.serve.git_no_git".into(), "ບໍ່ພົບ git ຫຼືບໍ່ຢູ່ໃນ PATH".into());

    m.insert(
        "cmd.serve.git_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.arc_serving".into(),
        "ເຊີບເວີໄຟລ໌ບີບອັດເຮັດວຽກຢູ່ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.arc_title".into(),
        "ໄຟລ໌ບີບອັດ (ອິນເຕີເຟສບຣາວເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.arc_usage".into(),
        "serve arc [ໄດເຣກະທໍລີ] [--port 8000] [--bind ທີ່ຢູ່]".into(),
    );

    m.insert(
        "cmd.serve.arc_help_port".into(),
        "  --port, -p <ພອດ>     ພອດທີ່ຮັບຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_help_bind".into(),
        "  --bind, -b <ທີ່ຢູ່>  ທີ່ຢູ່ທີ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_example".into(),
        "  serve arc --port 9000  ໃຫ້ບໍລິການໄດເຣກະທໍລີປັດຈຸບັນທີ່ພອດ 9000".into(),
    );

    m.insert("cmd.serve.arc_no_dir".into(), "ບໍ່ພົບໄດເຣກະທໍລີ '{}'".into());

    m.insert(
        "cmd.serve.arc_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert("cmd.serve.theme_title".into(), "ທີມ (ອິນເຕີເຟສເບົາເຊີ)".into());

    m.insert(
        "cmd.serve.theme_usage".into(),
        "serve theme [--port 8000] [--bind ທີ່ຢູ່]".into(),
    );

    m.insert(
        "cmd.serve.theme_help_port".into(),
        "  --port, -p <ພອດ>     ພອດທີ່ຮັບຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_help_bind".into(),
        "  --bind, -b <ທີ່ຢູ່>  ທີ່ຢູ່ທີ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_example".into(),
        "  serve theme --port 9000  ແກ້ໄຂທີມທີ່ພອດ 9000".into(),
    );

    m.insert("cmd.serve.theme_serving".into(), "ເຊີບເວີທີມຢູ່ {} ({})".into());

    m.insert(
        "cmd.serve.theme_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.services_row_ndb".into(),
        "  ndb           ເບິ່ງ SQLite: ເລືອກຖານຂໍ້ມູນໃນໂຟນເດີ, ເບິ່ງໂຄງສ້າງ, ເລືອກ --write".into(),
    );

    m.insert("cmd.serve.ndb_title".into(), "ເບິ່ງຖານຂໍ້ມູນ".into());

    m.insert(
        "cmd.serve.ndb_usage".into(),
        "serve ndb [--port 8000] [--bind ທີ່ຢູ່] [--auth user:pass] [--write]".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_port".into(),
        "  --port, -p <ພອດ>     ພອດທີ່ຮັບຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_bind".into(),
        "  --bind, -b <ທີ່ຢູ່>  ທີ່ຢູ່ທີ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_write".into(),
        "  --write               ອະນຸຍາດ INSERT/UPDATE/DELETE/REPLACE; ຈະສ້າງສຳເນົາ <ຊື່>.bak.<ເວລາ> ກ່ອນການຂຽນຄັ້ງທຳອິດ".into(),
    );

    m.insert(
        "cmd.serve.ndb_serving".into(),
        "ໃຫ້ບໍລິການຖານຂໍ້ມູນໃນ {} ທີ່ {}".into(),
    );

    m.insert(
        "cmd.serve.ndb_write_on".into(),
        "ເປີດການຂຽນແລ້ວ: ທີລະຫນຶ່ງຄຳສັ່ງ INSERT, UPDATE, DELETE ຫຼື REPLACE; ສຳເນົາ <ຊື່>.bak.<ເວລາ> ຈະຢູ່ຂ້າງຖານຂໍ້ມູນກ່ອນການຂຽນຄັ້ງທຳອິດ".into(),
    );

    m.insert(
        "cmd.serve.ndb_readonly".into(),
        "ອ່ານຢ່າງດຽວ: ອະນຸຍາດ SELECT, PRAGMA, EXPLAIN, WITH, VALUES ແລະ TABLE; ໃຊ້ --write ເພື່ອເປີດການຂຽນລະດັບແຖວ".into(),
    );

    m.insert(
        "cmd.serve.ndb_local_warning".into(),
        "Loopback ເທົ່ານັ້ນ: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0 ພ້ອມ --auth ເພື່ອແບ່ງປັນ).".into(),
    );

    m.insert(
        "cmd.serve.ndb_no_db".into(),
        "ບໍ່ພົບຖານຂໍ້ມູນ SQLite ທີ່ອ່ານໄດ້ໃນ {} - ຄັດລອກໄຟລ໌ .db, .sqlite ຫຼື .sqlite3 ໄປໄວ້ທີ່ນັ້ນ".into(),
    );

    m.insert(
        "cmd.serve.ndb_flags_only".into(),
        "'{}' ບໍ່ແມ່ນຕົວເລືອກ: `serve ndb` ຮັບແຕ່ແຟລັກ - ເລືອກຖານຂໍ້ມູນໃນຫນ້າເວັບ".into(),
    );

    m.insert(
        "cmd.serve.ndb_requires_auth".into(),
        "ປະຕິເສດການເປີດຖານຂໍ້ມູນຢູ່ {} ໂດຍບໍ່ມີ --auth <user:pass>".into(),
    );

    m.insert(
        "cmd.serve.ndb_example".into(),
        "  serve ndb --port 9000  ສະແດງຖານຂໍ້ມູນໃນໂຟນເດີນີ້ທີ່ພອດ 9000".into(),
    );

    m.insert(
        "cmd.serve.services_row_fake".into(),
        "  fake          UI ຂໍ້ມູນຈຳລອງ: ເລືອກຊະນິດ ຫຼື ກຸ່ມ ແລ້ວເບິ່ງຕົວຢ່າງແຖວ".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        UI ການຕັ້ງຄ່າ: ແກ້ທຸກຄ່າ ແລະ ຄ່າເລີ່ມຕົ້ນຂອງ serve ໃນເບົາເຊີ".into(),
    );

    m.insert(
        "cmd.serve.services_example_fake".into(),
        "  serve fake --port 9000".into(),
    );

    m.insert(
        "cmd.serve.fake_title".into(),
        "ຂໍ້ມູນຈຳລອງ (ອິນເຕີເຟສເບົາເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.fake_usage".into(),
        "serve fake [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.fake_help_port".into(),
        "  --port, -p <port>     ພອດທີ່ຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_help_bind".into(),
        "  --bind, -b <addr>     ທີ່ຢູ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_example".into(),
        "  serve fake --port 9000  ເປີດ UI ຂໍ້ມູນຈຳລອງທີ່ພອດ 9000".into(),
    );

    m.insert("cmd.serve.fake_serving".into(), "ເຊີບເວີຂໍ້ມູນຈຳລອງຢູ່ {}".into());

    m.insert(
        "cmd.serve.fake_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.config_title".into(),
        "ການຕັ້ງຄ່າ NTC (ອິນເຕີເຟສເບົາເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.config_usage".into(),
        "serve config [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.config_help_port".into(),
        "  --port, -p <port>     ພອດທີ່ຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.config_help_bind".into(),
        "  --bind, -b <addr>     ທີ່ຢູ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.config_example".into(),
        "  serve config --port 9000  ເປີດ UI ການຕັ້ງຄ່າທີ່ພອດ 9000".into(),
    );

    m.insert("cmd.serve.config_serving".into(), "ເຊີບເວີການຕັ້ງຄ່າຢູ່ {}".into());

    m.insert(
        "cmd.serve.config_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.services_row_backup".into(),
        "  backup        UI ສຳຮອງຂໍ້ມູນ: ສ້າງ, diff, ກູ້ຄືນ, ຍ້ອນກັບ, ລຶບໂຟນເດີນີ້".into(),
    );

    m.insert(
        "cmd.serve.services_example_backup".into(),
        "  serve backup --port 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_title".into(),
        "ສຳຮອງຂໍ້ມູນ (ອິນເຕີເຟສເບົາເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.backup_usage".into(),
        "serve backup [path] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.backup_help_port".into(),
        "  --port, -p <port>     ພອດທີ່ຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_help_bind".into(),
        "  --bind, -b <addr>     ທີ່ຢູ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_example".into(),
        "  serve backup --port 9000  ຈັດການສຳຮອງຂໍ້ມູນຂອງໂຟນເດີນີ້ທີ່ພອດ 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_serving".into(),
        "ເຊີບເວີສຳຮອງຂໍ້ມູນຢູ່ {} ({})".into(),
    );

    m.insert(
        "cmd.serve.backup_local_warning".into(),
        "ຜູກກັບ loopback: ອຸປະກອນໃນ LAN ເຂົ້າເຖິງບໍ່ໄດ້ (ໃຊ້ --bind 0.0.0.0)".into(),
    );

    m.insert("cmd.serve.backup_no_dir".into(), "ບໍ່ພົບໂຟນເດີ: {}".into());

    m.insert(
        "cmd.serve.services_row_mdview".into(),
        "  mdview        ໂປຣແກຣມແກ້ໄຂ Markdown ພ້ອມຕົວຢ່າງສົດ (ຂຽນໄຟລ໌ .md)".into(),
    );

    m.insert(
        "cmd.serve.mdview_title".into(),
        "ໂປຣແກຣມແກ້ໄຂ Markdown (ອິນເຕີເຟສເບົາເຊີ)".into(),
    );

    m.insert(
        "cmd.serve.mdview_usage".into(),
        "serve mdview [ໂຟນເດີ] [--port 8000] [--bind ທີ່ຢູ່]".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_port".into(),
        "  --port, -p <ພອດ>     ພອດທີ່ຮັບຟັງ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_bind".into(),
        "  --bind, -b <ທີ່ຢູ່>     ທີ່ຢູ່ທີ່ຜູກ (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_example".into(),
        "  serve mdview docs --port 9000  ແກ້ໄຂ markdown ໃນ docs/ ຢູ່ພອດ 9000".into(),
    );

    m.insert(
        "cmd.serve.mdview_serving".into(),
        "ໂປຣແກຣມແກ້ໄຂ Markdown ສຳລັບ {} ຢູ່ {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_no_dir".into(),
        "ບໍ່ແມ່ນໂຟນເດີ: {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_local_warning".into(),
        "ຜູກກັບ loopback: ມີພຽງເຄື່ອງນີ້ທີ່ແກ້ໄຂໄຟລ໌ໄດ້ (ໃຊ້ --bind 0.0.0.0 ເພື່ອແບ່ງປັນ)".into(),
    );
}
