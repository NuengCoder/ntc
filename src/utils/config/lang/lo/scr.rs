use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "scr.calling_not_allowed".into(),
        "ບໍ່ອະນຸຍາດໃຫ້ເອີ້ນ 'scr' ພາຍໃນສະຄຣິບ".into(),
    );

    m.insert("scr.cmd_failed".into(), "ຄຳສັ່ງລົ້ມເຫຼວ".into());

    m.insert("scr.completed".into(), "ສະຄຣິບ '{}' ສຳເລັດ ({} ຄຳສັ່ງ)".into());

    m.insert("scr.continue_warn".into(), "ສືບຕໍ່ຫຼັງຂໍ້ຜິດພາດທີ່ບັນທັດ {}".into());

    m.insert("scr.desc_args".into(), " $1..$9, $@ ອາກິວເມັນຕຳແໜ່ງ".into());

    m.insert(
        "scr.desc_force".into(),
        " @ <cmd> ບັງຄັບຄຳສັ່ງລະບົບ (ຂ້າມຄຳສັ່ງ ntc)".into(),
    );

    m.insert(
        "scr.desc_interp".into(),
        " $var / ${var} / ${var:-default} ການແທນຄ່າຕົວແປ".into(),
    );

    m.insert("scr.desc_set".into(), " set NAME = value ກຳນົດຕົວແປ".into());

    m.insert(
        "scr.done_errors".into(),
        "ສະຄຣິບ '{}' ສຳເລັດ ມີ {} ຂໍ້ຜິດພາດ".into(),
    );

    m.insert("scr.error_at".into(), "ຂໍ້ຜິດພາດທີ່ {}:{} \u{2014} {}".into());

    m.insert("scr.executing".into(), "[{}] ກຳລັງແລ່ນ: {}".into());

    m.insert(
        "scr.failed_abort".into(),
        "ສະຄຣິບ '{}' ຢຸດທີ່ບັນທັດ {} ({} ຂໍ້ຜິດພາດ)".into(),
    );

    m.insert(
        "scr.help_continue".into(),
        " scr -c <file.ntc> [args...] ສືບຕໍ່ຫຼັງຂໍ້ຜິດພາດ".into(),
    );

    m.insert("scr.help_help".into(), " scr help ສະແດງຄຳຊ່ວຍເຫຼືອນີ້".into());

    m.insert(
        "scr.help_init".into(),
        " scr init [name] ສ້າງ template ສະຄຣິບ".into(),
    );

    m.insert(
        "scr.help_ls".into(),
        " scr ls ລາຍຊື່ *.ntc ໃນໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "scr.help_run".into(),
        " scr <file.ntc> [args...] ແລ່ນສະຄຣິບອັດຕະໂນມັດ".into(),
    );

    m.insert(
        "scr.help_title".into(),
        "scr \u{2014} ຕົວແລ່ນສະຄຣິບອັດຕະໂນມັດຂອງ ntc".into(),
    );

    m.insert("scr.help_usage".into(), "ການໃຊ້ງານ:".into());

    m.insert("scr.init_created".into(), "ສ້າງ template {} ແລ້ວ".into());

    m.insert("scr.init_exists".into(), "{} ມີຢູ່ແລ້ວ".into());

    m.insert("scr.ls_empty".into(), "ບໍ່ພົບ *.ntc ໃນ {}".into());

    m.insert("scr.ls_title".into(), "ສະຄຣິບອັດຕະໂນມັດ:".into());

    m.insert("scr.no_commands".into(), "{} ບໍ່ມີຄຳສັ່ງໃຫ້ແລ່ນ".into());

    m.insert("scr.not_found".into(), "ບໍ່ພົບສະຄຣິບ: {} ({})".into());

    m.insert("scr.start".into(), "ກຳລັງແລ່ນສະຄຣິບ {}...".into());

    m.insert("scr.syscmd".into(), "ກຳລັງແລ່ນ: {}".into());

    m.insert("scr.syscmd_exit".into(), "ຄຳສັ່ງອອກດ້ວຍລະຫັດ: {}".into());

    m.insert("scr.syscmd_terminated".into(), "ຄຳສັ່ງສິ້ນສຸດ (Ctrl+C)".into());

    // ── ພາສາຂຽນສະຄຣິບ ────────────────────────────────────────────────
    m.insert("scr.help_syntax_header".into(), "ໄວຍາກອນ:".into());

    m.insert(
        "scr.help_direct".into(),
        " <file.ntc> [args...] ແລ່ນສະຄຣິບໂດຍບໍ່ຕ້ອງພິມ scr".into(),
    );

    m.insert(
        "scr.desc_expr".into(),
        " set NAME = <expr> ຄ່າ, ການຄິດໄລ່, ການເອີ້ນຟັງຊັນ".into(),
    );

    m.insert(
        "scr.desc_if".into(),
        " if <ເງື່ອນໄຂ> / else if / else / end ບລັອກເງື່ອນໄຂ".into(),
    );

    m.insert(
        "scr.desc_while".into(),
        " while <ເງື່ອນໄຂ> / end ວົນຊ້ຳເມື່ອເງື່ອນໄຂເປັນຈິງ".into(),
    );

    m.insert(
        "scr.desc_for".into(),
        " for VAR in <ລາຍການ> / end ວົນຊ້ຳຕາມຄຳ".into(),
    );

    m.insert(
        "scr.desc_loop".into(),
        " loop [ຈຳນວນ] / end ເຮັດຊ້ຳຕາມຈຳນວນ (ຄ່າເລີ່ມຕົ້ນ: ຈົນກວ່າ break)".into(),
    );

    m.insert(
        "scr.desc_break".into(),
        " break / continue ອອກຈາກວົນ ຫຼື ຂ້າມໄປຮອບຕໍ່ໄປ".into(),
    );

    m.insert(
        "scr.desc_func".into(),
        " func NAME(ພາລາມິເຕີ) / end ປະກາດຟັງຊັນ".into(),
    );

    m.insert(
        "scr.desc_call".into(),
        " call NAME(ອາກິວເມັນ) ເອີ້ນຟັງຊັນ".into(),
    );

    m.insert(
        "scr.desc_return".into(),
        " return [ຄ່າ] ອອກຈາກຟັງຊັນພ້ອມຄ່າ".into(),
    );

    m.insert(
        "scr.desc_input".into(),
        " input NAME [ຂໍ້ຄວາມ] ອ່ານໜຶ່ງແຖວເຂົ້າຕົວແປ".into(),
    );

    m.insert("scr.desc_exit".into(), " exit ຢຸດສະຄຣິບ".into());

    m.insert("scr.desc_functions_header".into(), "ຟັງຊັນໃນນິພົດ:".into());

    m.insert(
        "scr.desc_fn_nprintf".into(),
        " nprintf(fmt, ...) ຈັດຮູບແບບຄືກັບຄຳສັ່ງ nprintf".into(),
    );

    m.insert(
        "scr.desc_fn_strings".into(),
        " len / upper / lower / trim / str ຟັງຊັນຂໍ້ຄວາມ".into(),
    );

    m.insert(
        "scr.desc_fn_math".into(),
        " int / abs ຟັງຊັນຕົວເລກ".into(),
    );

    m.insert(
        "scr.desc_fn_env".into(),
        " env(NAME) ອ່ານຕົວແປສະພາບແວດລ້ອມ".into(),
    );

    m.insert("scr.input_prompt".into(), "> ".into());

    m.insert(
        "scr.input_eof".into(),
        "ບໍ່ມີຂໍ້ມູນສຳລັບ '{}' — ຢຸດສະຄຣິບ".into(),
    );

    m.insert("scr.parse_error".into(), "{}:{} — {}".into());

    m.insert(
        "scr.parse_failed".into(),
        "ສະຄຣິບ '{}' ມີຂໍ້ຜິດພາດໄວຍາກອນ {} ຈຸດ ຈຶ່ງບໍ່ໄດ້ແລ່ນ".into(),
    );

    m.insert("scr.err_expr".into(), "ນິພົດບໍ່ຖືກຕ້ອງ '{}': {}".into());

    m.insert("scr.err_loop_limit".into(), "ວົນເກີນ {} ຮອບ ຈຶ່ງຢຸດ".into());

    m.insert(
        "scr.err_func_missing".into(),
        "ບໍ່ຮູ້ຈັກຟັງຊັນ '{}'".into(),
    );

    m.insert(
        "scr.err_func_args".into(),
        "'{}' ຕ້ອງການ {} ອາກິວເມັນ ແຕ່ໄດ້ {}".into(),
    );

    m.insert(
        "scr.err_recursion".into(),
        "'{}' ເອີ້ນຊ້ອນເກີນ {} ຊັ້ນ".into(),
    );

    m.insert(
        "scr.err_break_outside_loop".into(),
        "`break` / `continue` ອອກຈາກຟັງຊັນ '{}' ບໍ່ໄດ້".into(),
    );

    m.insert(
        "scr.nested_too_deep".into(),
        "ສະຄຣິບຊ້ອນກັນເກີນ {} ຊັ້ນ ຈຶ່ງຢຸດ".into(),
    );

    m.insert(
        "scr.pipe_refused".into(),
        "'{}' ຕັ້ງ `set-pipe = fail` ຈຶ່ງໃຊ້ກັບ pipe ຫຼື redirect ບໍ່ໄດ້".into(),
    );

    m.insert(
        "scr.desc_pipe".into(),
        " set-pipe = on|off ອະນຸຍາດໃຫ້ສະຄຣິບນີ້ຖືກ pipe ຫຼື redirect ຫຼືບໍ່".into(),
    );

    m.insert(
        "scr.desc_showexc".into(),
        " set-showexc = on|off ສະແດງແຖວ [n] Executing: ຫຼືບໍ່".into(),
    );

}
