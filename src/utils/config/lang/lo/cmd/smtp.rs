use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.smtp.build_fail".into(), "ສ້າງຂໍ້ຄວາມ email ບໍ່ສຳເລັດ".into());

    m.insert(
        "cmd.smtp.create_one_hint".into(),
        "  ສ້າງອັນໜຶ່ງ: {} -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert(
        "cmd.smtp.create_syntax".into(),
        "smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert("cmd.smtp.create_usage".into(), "ຕ້ອງການ server, username, ແລະ password. ວິທີໃຊ້: smtp -i <server> -p <port> -u <user> -pwd <password>".into());

    m.insert("cmd.smtp.created_label".into(), "ສ້າງແລ້ວ:".into());

    m.insert(
        "cmd.smtp.decode_failed".into(),
        "ຖອດລະຫັດ password ທີ່ເກັບໄວ້ບໍ່ສຳເລັດ".into(),
    );

    m.insert(
        "cmd.smtp.body_file_missing".into(),
        "ບໍ່ພົບໄຟລ໌ body: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_html_from_file".into(),
        "body ແບບ HTML ຈາກໄຟລ໌: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_md_from_file".into(),
        "body ແບບ Markdown ຈາກໄຟລ໌: {} (ແປງເປັນ HTML ແລ້ວ)".into(),
    );

    m.insert(
        "cmd.smtp.body_read_fail".into(),
        "ອ່ານໄຟລ໌ body {} ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.smtp.body_stdin_empty".into(),
        "ບໍ່ໄດ້ຮັບ body ຈາກ stdin".into(),
    );

    m.insert(
        "cmd.smtp.body_text_from_file".into(),
        "body ແບບຂໍ້ຄວາມທຳມະດາ ຈາກໄຟລ໌: {}".into(),
    );

    m.insert(
        "cmd.smtp.help_body_file".into(),
        "  -b letter.html | letter.md      ອ່ານ body ຈາກໄຟລ໌ (Markdown ຖືກແປງເປັນ HTML)".into(),
    );

    m.insert(
        "cmd.smtp.help_body_format".into(),
        "  --body-format auto|text|html|md ກຳນົດວິທີຕີຄວາມ -b".into(),
    );

    m.insert(
        "cmd.smtp.help_body_inline".into(),
        "  -b '<p style=text-align:center>ສະບາຍດີ</p>'   ກວດພົບ HTML ໃນບັນທັດໂດຍອັດຕະໂນມັດ".into(),
    );

    m.insert(
        "cmd.smtp.help_body_stdin".into(),
        "  -b -                           ອ່ານ body ຈາກ stdin".into(),
    );

    m.insert("cmd.smtp.help_body_title".into(), "ເນື້ອຫາ email:".into());

    m.insert(
        "cmd.smtp.invalid_body_format".into(),
        "ບໍ່ຮູ້ຈັກຮູບແບບ body '{}' (ໃຊ້ auto, text, html ຫຼື md)".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_file".into(),
        "ຂາດ path ຫຼັງ --body-file".into(),
    );

    m.insert(
        "cmd.smtp.missing_body_format".into(),
        "ຂາດຄ່າຫຼັງ --body-format".into(),
    );

    m.insert(
        "cmd.smtp.help_attachments_desc".into(),
        "  -f ຮັບ path ໄຟລ໌ ແລະ ໄດເຣັກທໍຣີທີ່ຄັນດ້ວຍເຄື່ອງໝາຍໂກ່ມ (files ຖືກຂະຫຍາຍແບບ recursive)".into(),
    );

    m.insert("cmd.smtp.help_attachments_example".into(), "  ຕົວຢ່າງ: smtp --id a1b2 --to user@test.com -s \"Report\" -b \"See attached\" -f \"./report.pdf,./images/\"".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "ໄຟລ໌ແນບ:".into());

    m.insert(
        "cmd.smtp.help_create_example".into(),
        "  ຕົວຢ່າງ: smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into(),
    );

    m.insert(
        "cmd.smtp.help_create_syntax".into(),
        "  smtp -i <server> -p <port> -u <user> -pwd <password>".into(),
    );

    m.insert("cmd.smtp.help_create_title".into(), "ສ້າງ profile:".into());

    m.insert(
        "cmd.smtp.help_info_syntax".into(),
        "  smtp --id-info <id>".into(),
    );

    m.insert(
        "cmd.smtp.help_info_title".into(),
        "ສະແດງຂໍ້ມູນ profile:".into(),
    );

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "ລາຍຊື່ profiles:".into());

    m.insert(
        "cmd.smtp.help_port_25".into(),
        "  25   Plain (ຖ້າ server ຮອງຮັບ)".into(),
    );

    m.insert("cmd.smtp.help_port_465".into(), "  465  TLS (SMTPS)".into());

    m.insert(
        "cmd.smtp.help_port_587".into(),
        "  587  STARTTLS (ເລີ່ມຕົ້ນ)".into(),
    );

    m.insert("cmd.smtp.help_ports_title".into(), "Ports:".into());

    m.insert(
        "cmd.smtp.help_rm_syntax".into(),
        "  smtp --id-rm <id>".into(),
    );

    m.insert("cmd.smtp.help_rm_title".into(), "ລຶບ profile:".into());

    m.insert("cmd.smtp.help_send_syntax".into(), "  smtp --id <id> --to <email> [--cc <email>] [--bcc <email>] -s <subject> -b <body> [-f <files>]".into());

    m.insert("cmd.smtp.help_send_title".into(), "ສົ່ງ email:".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP Email Client".into());

    m.insert("cmd.smtp.id_label".into(), "ID:".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "BCC ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "CC ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("cmd.smtp.invalid_port".into(), "ເລກ port ບໍ່ຖືກຕ້ອງ".into());

    m.insert(
        "cmd.smtp.invalid_pwd_encoding".into(),
        "ການເຂົ້າລະຫັດ password ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert("cmd.smtp.invalid_recipient".into(), "ຜູ້ຮັບບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "cmd.smtp.invalid_sender".into(),
        "ທີ່ຢູ່ email ຜູ້ສົ່ງບໍ່ຖືກຕ້ອງ".into(),
    );

    m.insert("cmd.smtp.missing_body".into(), "ຂາດ body ຫຼັງ -b".into());

    m.insert("cmd.smtp.missing_id".into(), "ຂາດ ID ຫຼັງ --id".into());

    m.insert(
        "cmd.smtp.missing_id_info".into(),
        "ຂາດ ID ຫຼັງ --id-info".into(),
    );

    m.insert("cmd.smtp.missing_id_rm".into(), "ຂາດ ID ຫຼັງ --id-rm".into());

    m.insert(
        "cmd.smtp.missing_password".into(),
        "ຂາດ password ຫຼັງ -pwd".into(),
    );

    m.insert("cmd.smtp.missing_path".into(), "ຂາດ path ຫຼັງ -f".into());

    m.insert("cmd.smtp.missing_port".into(), "ຂາດ port ຫຼັງ -p".into());

    m.insert(
        "cmd.smtp.missing_recipient_bcc".into(),
        "ຂາດຜູ້ຮັບຫຼັງ --bcc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_cc".into(),
        "ຂາດຜູ້ຮັບຫຼັງ --cc".into(),
    );

    m.insert(
        "cmd.smtp.missing_recipient_to".into(),
        "ຂາດຜູ້ຮັບຫຼັງ --to".into(),
    );

    m.insert("cmd.smtp.missing_server".into(), "ຂາດ server ຫຼັງ -i".into());

    m.insert(
        "cmd.smtp.missing_subject".into(),
        "ຂາດ subject ຫຼັງ -s".into(),
    );

    m.insert("cmd.smtp.missing_user".into(), "ຂາດ username ຫຼັງ -u".into());

    m.insert(
        "cmd.smtp.need_recipient".into(),
        "ຕ້ອງການຜູ້ຮັບ. ໃຊ້ --to <email>".into(),
    );

    m.insert(
        "cmd.smtp.need_subject".into(),
        "ຕ້ອງການ subject. ໃຊ້ -s <subject>".into(),
    );

    m.insert(
        "cmd.smtp.no_files_in_dir".into(),
        "ບໍ່ພົບໄຟລ໌ໃນໄດເຣັກທໍຣີ: {}".into(),
    );

    m.insert(
        "cmd.smtp.no_profiles".into(),
        "  {}  ບໍ່ພົບ SMTP profiles.".into(),
    );

    m.insert("cmd.smtp.password_label".into(), "Password:".into());

    m.insert("cmd.smtp.path_not_found".into(), "ບໍ່ພົບ path: {}".into());

    m.insert("cmd.smtp.port_label".into(), "Port:".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP Profile:".into());

    m.insert(
        "cmd.smtp.profile_not_found".into(),
        "ບໍ່ພົບ profile: {}".into(),
    );

    m.insert(
        "cmd.smtp.profile_saved".into(),
        "ບັນທຶກ SMTP profile ແລ້ວ: {}".into(),
    );

    m.insert("cmd.smtp.profiles_title".into(), "SMTP Profiles:".into());

    m.insert(
        "cmd.smtp.read_attach_fail".into(),
        "ອ່ານໄຟລ໌ແນບບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.smtp.removed_profile".into(),
        "ລຶບ SMTP profile ແລ້ວ: {}".into(),
    );

    m.insert("cmd.smtp.send_fail".into(), "ສົ່ງ email ບໍ່ສຳເລັດ: {}".into());

    m.insert("cmd.smtp.send_ok".into(), "ສົ່ງ email ສຳເລັດ{}".into());

    m.insert("cmd.smtp.sending".into(), "ກຳລັງສົ່ງ email ໄປຍັງ {}...".into());

    m.insert("cmd.smtp.server_label".into(), "Server:".into());

    m.insert(
        "cmd.smtp.skipping_exec".into(),
        "ຂ້າມໄຟລ໌ executable: {}".into(),
    );

    m.insert(
        "cmd.smtp.skipping_not_file".into(),
        "ຂ້າມ (ບໍ່ແມ່ນໄຟລ໌ ຫຼື ໄດເຣັກທໍຣີ): {}".into(),
    );

    m.insert(
        "cmd.smtp.transport_fail".into(),
        "ກຳນົດຄ່າ SMTP transport ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("cmd.smtp.unknown_option".into(), "ບໍ່ຮູ້ຈັກ option: {}".into());

    m.insert("cmd.smtp.user_label".into(), "User:".into());

    m.insert("cmd.smtp.with_attachments".into(), "  ({} ໄຟລ໌ແນບ{})".into());
}
