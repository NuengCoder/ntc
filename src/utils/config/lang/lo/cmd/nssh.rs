use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nssh.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.nssh.help_add".into(),
        "nssh add <ຊື່> --host <ໂຮສ> [--user ຜູ້ໃຊ້] [--port ພອດ] [--key ຄີ] [--pass-env ໂຕແປ]".into(),
    );

    m.insert(
        "cmd.nssh.help_ssh".into(),
        "nssh ssh [ຊື່] [--cmd \"ຄຳສັ່ງ\"] [--trust] [--ask-pass] [--timeout ວິນາທີ]".into(),
    );

    m.insert(
        "cmd.nssh.help_cp".into(),
        "nssh cp <ໂລຄອລ> <ຊື່>:<ຣີໂໝດ> | <ຊື່>:<ຣີໂໝດ> <ໂລຄອລ>".into(),
    );

    m.insert(
        "cmd.nssh.help_scan".into(),
        "nssh scan <ໂຮສ> [--ports ພອດ | --top ຈຳນວນ] [--timeout ມິນລິວິນາທີ]".into(),
    );

    m.insert(
        "cmd.nssh.help_manage".into(),
        "nssh ls | nssh info <ຊື່> | nssh rm <ຊື່> | nssh edit <ຊື່> [ຕົວເລືອກ]".into(),
    );

    m.insert(
        "cmd.nssh.help_keygen".into(),
        "nssh keygen [ອາກິວເມັນ...]   ສ້າງຄີຜ່ານ ssh-keygen".into(),
    );

    m.insert(
        "cmd.nssh.help_help".into(),
        "nssh help   ສະແດງ help ນີ້ (ເອີ້ນເປົ່າຈະໃຫ້ເລືອກໂປຣໄຟລ໌)".into(),
    );

    m.insert("cmd.nssh.options_title".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "cmd.nssh.help_host".into(),
        "  --host <ໂຮສ>           ຊື່ຫຼື IP ເຊີເວີ (add ຕ້ອງระบุ)".into(),
    );

    m.insert(
        "cmd.nssh.help_user".into(),
        "  --user, -u <ຜູ້ໃຊ້>    ຜູ້ໃຊ້ລັອກອິນ (ເລີ່ມຕົ້ນ: ຜູ້ໃຊ້ປັດຈຸບັນ)".into(),
    );

    m.insert(
        "cmd.nssh.help_port".into(),
        "  --port, -p <ພອດ>      ພອດ SSH (ເລີ່ມຕົ້ນ: 22)".into(),
    );

    m.insert(
        "cmd.nssh.help_key".into(),
        "  --key <ເສັ້ນທາງ>       ໄຟລ໌ຄີສ່ວນຕົວ (ເລີ່ມຕົ້ນ: ~/.ssh/id_ed25519, id_rsa)".into(),
    );

    m.insert(
        "cmd.nssh.help_pass_env".into(),
        "  --pass-env <ໂຕແປ>    ຊື່ pass-env ໃນ deadlock (ບໍ່ເກັບລົງດິສ); ຖ້າບໍ່ມີຈຶ່ງໃຊ້ໂຕແປສະພາບແວດລ້ອມ".into(),
    );

    m.insert(
        "cmd.nssh.help_cmd".into(),
        "  --cmd \"ຄຳສັ່ງ\"         ຣັນຄຳສັ່ງດຽວແທນເຊວໂຕ້ຕອບ".into(),
    );

    m.insert(
        "cmd.nssh.help_trust".into(),
        "  --trust               ຍອມຮັບ+ບັນທຶກຄີໂຮສທີ່ບໍ່ຮູ້ຈັກ (ເລີ່ມຕົ້ນ: ປະຕິເສດ)".into(),
    );

    m.insert(
        "cmd.nssh.help_ask_pass".into(),
        "  --ask-pass            ຖາມລະຫັດຜ່ານ (ເຊື່ອງການພິມ)".into(),
    );

    m.insert(
        "cmd.nssh.help_timeout".into(),
        "  --timeout <ໂຕເລກ>    ໝົດເວລາເຊື່ອມຕໍ່ວິນາທີ (ssh/cp) ຫຼືມິນລິວິນາທີ (scan)".into(),
    );

    m.insert(
        "cmd.nssh.help_top".into(),
        "  --top <ຈຳນວນ>         ສະແກນພອດຍອດນິຍົມ N ອັນດັບທຳອິດ (ເລີ່ມຕົ້ນ: 20)".into(),
    );

    m.insert(
        "cmd.nssh.help_scan_ports".into(),
        "  --ports <ສະເປັກ>     ເຊັ່ນ 22,80,8000-8010 (ช่วงละບໍ່ເກີນ 2000)".into(),
    );

    m.insert(
        "cmd.nssh.help_yes".into(),
        "  --yes, -y             ຂ້າມການຢືນຢັນລบ".into(),
    );

    m.insert("cmd.nssh.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.nssh.example_add".into(),
        "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into(),
    );

    m.insert(
        "cmd.nssh.example_ssh".into(),
        "nssh ssh prod --cmd \"uptime\"".into(),
    );

    m.insert(
        "cmd.nssh.example_cp".into(),
        "nssh cp ./app.tar.gz prod:/tmp/".into(),
    );

    m.insert(
        "cmd.nssh.example_scan".into(),
        "nssh scan example.com --top 20".into(),
    );

    m.insert("cmd.nssh.missing_host".into(), "ຂາດຄ່າຫຼັງ --host".into());

    m.insert("cmd.nssh.missing_user".into(), "ຂາດຄ່າຫຼັງ --user".into());

    m.insert("cmd.nssh.missing_port".into(), "ຂາດຄ່າຫຼັງ --port".into());

    m.insert("cmd.nssh.missing_key".into(), "ຂາດຄ່າຫຼັງ --key".into());

    m.insert(
        "cmd.nssh.missing_pass_env".into(),
        "ຂາດຄ່າຫຼັງ --pass-env".into(),
    );

    m.insert("cmd.nssh.missing_cmd".into(), "ຂາດຄ່າຫຼັງ --cmd".into());

    m.insert(
        "cmd.nssh.missing_timeout".into(),
        "ຂາດຄ່າຫຼັງ --timeout".into(),
    );

    m.insert("cmd.nssh.missing_ports".into(), "ຂາດຄ່າຫຼັງ --ports".into());

    m.insert("cmd.nssh.missing_top".into(), "ຂາດຄ່າຫຼັງ --top".into());

    m.insert("cmd.nssh.invalid_port".into(), "ພອດບໍ່ຖືກຕ້ອງ: '{}'".into());

    m.insert(
        "cmd.nssh.invalid_timeout".into(),
        "ໝົດເວลາບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_top".into(),
        "ຄ່າ --top ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "cmd.nssh.invalid_name".into(),
        "ຊື່ໂປຣໄຟລ໌ບໍ່ຖືກຕ້ອງ '{}' (ໂຕอักษร โຕເລກ _ - . ເທົ່ານັ້ນ)".into(),
    );

    m.insert("cmd.nssh.unknown_option".into(), "ບໍ່ຮູ້ຈັກ: {}".into());

    m.insert(
        "cmd.nssh.add_usage".into(),
        "ວິທີໃຊ້: nssh add <ຊື່> --host <ໂຮສ> [--user ຜູ້ໃຊ້] [--port ພອດ] [--key ຄີ] [--pass-env ໂຕແປ]"
            .into(),
    );

    m.insert(
        "cmd.nssh.profile_exists".into(),
        "ມີໂປຣໄຟລ໌ '{}' ແລ້ວ (ໃຊ້ nssh edit ເພື່ອແກ້)".into(),
    );

    m.insert("cmd.nssh.profile_saved".into(), "ບັນທຶກໂປຣໄຟລ໌ແລ້ວ: {}".into());

    m.insert("cmd.nssh.profile_not_found".into(), "ບໍ່ພົບໂປຣໄຟລ໌ '{}'".into());

    m.insert(
        "cmd.nssh.key_not_found".into(),
        "ບໍ່ມີໄຟລ໌ຄີ '{}' (ບັນທຶກຢູ່ດີ)".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_missing".into(),
        "ບໍ່ພົບ pass-env '{}' ໃນ deadlock vault ແລະ ບໍ່ໄດ້ຕັ້ງໃນໂຕແປສະພາບແວດລ້ອມ (ບັນທຶກຢູ່ດີ)".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_registered".into(),
        "pass-env '{}' ລົງທະບຽນໃນ deadlock vault ແລ້ວ".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_pending".into(),
        "pass-env '{}' ຈະຖືກແກ້ຈາກ deadlock vault ເມື່ອເຊື່ອມຕໍ່".into(),
    );

    m.insert(
        "cmd.nssh.pass_env_from_vault".into(),
        "ກຳລັງໃຊ້ pass-env '{}' ຈາກ deadlock vault".into(),
    );

    m.insert(
        "cmd.nssh.master_unlock_failed".into(),
        "ບໍ່ສາມາດປົດລັອກ deadlock vault ໄດ້ ({}); ຈະຖາມລະຫັດຜ່ານ SSH ແທນ".into(),
    );

    m.insert(
        "cmd.nssh.ls_empty".into(),
        "ຍັງບໍ່ມີໂປຣໄຟລ໌ ເພີ່ມເລີຍ: nssh add <ຊື່> --host <ໂຮສ>".into(),
    );

    m.insert("cmd.nssh.ls_title".into(), "ໂປຣໄຟລ໌ SSH:".into());

    m.insert("cmd.nssh.info_title".into(), "ໂປຣໄຟລ໌ '{}':".into());

    m.insert("cmd.nssh.info_usage".into(), "ວິທີໃຊ້: nssh info <ຊື່>".into());

    m.insert("cmd.nssh.info_host".into(), "ໂຮສ:".into());

    m.insert("cmd.nssh.info_user".into(), "ຜູ້ໃຊ້:".into());

    m.insert("cmd.nssh.info_port".into(), "ພອດ:".into());

    m.insert("cmd.nssh.info_key".into(), "ຄີ:".into());

    m.insert("cmd.nssh.info_pass".into(), "ໂຕແປລະຫັດຜ່ານ:".into());

    m.insert(
        "cmd.nssh.rm_usage".into(),
        "ວິທີໃຊ້: nssh rm <ຊື່> [--yes]".into(),
    );

    m.insert("cmd.nssh.rm_confirm".into(), "ລบໂປຣໄຟລ໌ '{}'?".into());

    m.insert("cmd.nssh.rm_deleted".into(), "ລบໂປຣໄຟລ໌ແລ້ວ: {}".into());

    m.insert("cmd.nssh.rm_cancelled".into(), "ຍົກເລີກແລ້ວ".into());

    m.insert("cmd.nssh.edit_usage".into(), "ວິທີໃຊ້: nssh edit <ຊື່> [--host ໂຮສ] [--user ຜູ້ໃຊ້] [--port ພອດ] [--key ຄີ|--no-key] [--pass-env ໂຕແປ|--no-pass-env]".into());

    m.insert("cmd.nssh.edit_saved".into(), "ອັບເດດໂປຣໄຟລ໌ແລ້ວ: {}".into());

    m.insert("cmd.nssh.pick_profile".into(), "ເລືອກໂປຣໄຟລ໌ SSH".into());

    m.insert("cmd.nssh.ask_pass".into(), "ລະຫັດຜ່ານຂອງ {}@{}: ".into());

    m.insert(
        "cmd.nssh.pass_env_unset".into(),
        "ໂຕແປ '{}' ຫວ່າງຫຼືບໍ່ໄດ້ຕັ້ງ — ຈະຖາມຖ້າໃຊ້ --ask-pass".into(),
    );

    m.insert(
        "cmd.nssh.ssh_connecting".into(),
        "ກຳລັງເຊື່ອມ {}@{}:{}...".into(),
    );

    m.insert("cmd.nssh.cmd_exit".into(), "ຄຳສັ່ງຣີໂໝດຈบດ້ວຍລະຫັດ {}".into());

    m.insert(
        "cmd.nssh.cp_usage".into(),
        "ວິທີໃຊ້: nssh cp <ໂລຄອລ> <ຊື່>:<ຣີໂໝດ> | nssh cp <ຊື່>:<ຣີໂໝດ> <ໂລຄອລ>".into(),
    );

    m.insert(
        "cmd.nssh.cp_no_remote".into(),
        "ຕ້ອງມີຝั่งໜຶ່ງເປັນ <ໂປຣໄຟລ໌>:<ເສັ້ນທາງ>".into(),
    );

    m.insert(
        "cmd.nssh.cp_two_sides".into(),
        "ຣີໂໝດທັງສອງຝั่ง — ສຳເນົາທີละຝั่ง".into(),
    );

    m.insert(
        "cmd.nssh.cp_local_missing".into(),
        "ບໍ່ພົບໄຟລ໌ໂລຄອລ '{}'".into(),
    );

    m.insert("cmd.nssh.cp_read_fail".into(), "ອ່ານໄຟລ໌ໂລຄອລບໍ່ໄດ້: {}".into());

    m.insert(
        "cmd.nssh.cp_too_large".into(),
        "ໄຟລ໌ເກີນຂີດຈຳກັດສົ່ງ 256MB".into(),
    );

    m.insert(
        "cmd.nssh.cp_uploading".into(),
        "ກຳລັງອັບໂຫຼດ {} ໄປ {}:{}...".into(),
    );

    m.insert(
        "cmd.nssh.cp_downloading".into(),
        "ກຳລັງດາວໂຫຼດ {}:{} ໄປ {}...".into(),
    );

    m.insert("cmd.nssh.cp_done".into(), "ສຳເລັດ ({} ໄບຕ໌)".into());

    m.insert(
        "cmd.nssh.scan_usage".into(),
        "ວິທີໃຊ້: nssh scan <ໂຮສ> [--ports ສະເປັກ | --top ຈຳນວນ] [--timeout ມິນລິວິນາທີ]".into(),
    );

    m.insert(
        "cmd.nssh.scan_start".into(),
        "ກຳລັງສະແກນ {} ({} ພອດ)...".into(),
    );

    m.insert("cmd.nssh.scan_title".into(), "ພອດທີ່ເປີດຢູ່ {}:".into());

    m.insert("cmd.nssh.scan_none".into(), "ບໍ່ມີພອດເປີດຢູ່ {}".into());

    m.insert("cmd.nssh.scan_done".into(), "ເປີດ {} ຈາກທີ່ສຳຫຼວດ {}".into());

    m.insert(
        "cmd.nssh.keygen_missing".into(),
        "ບໍ່ພົບ ssh-keygen: {}".into(),
    );

    m.insert(
        "cmd.nssh.keygen_exit".into(),
        "ssh-keygen ຈบດ້ວຍລະຫັດ {}".into(),
    );

    m.insert(
        "cmd.nssh.shell_hint".into(),
        "ເຊວໂຕ້ຕອບ — Enter ~ . ເພື່ອຕັດການເຊື່ອມຕໍ່".into(),
    );

    m.insert("cmd.nssh.shell_back".into(), "ກັບສູ່ ntc shell ແລ້ວ".into());

    m.insert("cmd.nssh.shell_forced".into(), "ຕັດການເຊື່ອມຕໍ່ແລ້ວ (~.)".into());

    m.insert(
        "cmd.nssh.password_prompt".into(),
        "ລະຫັດຜ่านสำหรับ {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.password_incorrect".into(),
        "ລະຫັດຜ่านບໍ່ຖືກຕ້ອງ。ລະຫັດຜ่านสำหรับ {}@{}: ".into(),
    );

    m.insert(
        "cmd.nssh.auth_failed".into(),
        "SSH ການຍืนຢືນຕົວຕົນລົ້ມເຫຼວສຳລັບ {}@{}.".into(),
    );
}
