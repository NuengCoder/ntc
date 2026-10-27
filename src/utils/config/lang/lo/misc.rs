use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.qur.usage".into(),
        "ວິທີໃຊ້: qur <glob> [-s \"pattern\"] [-rp \"replacement\"] [-dp] [-d <depth>] [-l <limit>]"
            .into(),
    );

    m.insert(
        "misc.qur.example1".into(),
        " qur *.* # ສະແດງໄຟລ໌ທີ່ກົງກັບ glob (ແບບເອີ້ນຊ້ຳ)".into(),
    );

    m.insert(
        "misc.qur.example2".into(),
        " qur *.rs -s \"TODO\" # ສະຫຼຸບຕໍ່ໄຟລ໌".into(),
    );

    m.insert(
        "misc.qur.example3".into(),
        " qur *.* -s \"old\" -dp # ສະແດງລາຍລະອຽດທີ່ກົງກັນ (ຄື gs)".into(),
    );

    m.insert(
        "misc.qur.example4".into(),
        " qur *.md -s \"old\" -rp \"new\" -d 3 # ເບິ່ງຕົວຢ່າງ, ຢືນຢັນ, ແທນທີ່".into(),
    );

    m.insert("misc.qur.searching".into(), "ກຳລັງຄົ້ນຫາໄຟລ໌...".into());

    m.insert("misc.qur.no_files".into(), "qur: ບໍ່ມີໄຟລ໌ທີ່ກົງກັບ '{}'".into());

    m.insert("misc.qur.no_matches".into(), "qur: ບໍ່ພົບ '{}'".into());

    m.insert("misc.qur.files_header".into(), "ໄຟລ໌ທີ່ກົງກັບຄຳຄົ້ນ ({})：".into());

    m.insert(
        "misc.qur.summary_header".into(),
        "ຜົນທີ່ກົງກັບ \"{}\" ({} ແຖວ ໃນ {} ໄຟລ໌):".into(),
    );

    m.insert(
        "misc.qur.summary_hint".into(),
        "ເພີ່ມ -dp ເພື່ອເບິ່ງແຖວລາຍລະອຽດ (ຄື gs) ຫຼື -rp \"new\" ເພື່ອແທນທີ່".into(),
    );

    m.insert(
        "misc.qur.replace_preview".into(),
        "ແທນທີ່ \"{}\" ດ້ວຍ \"{}\" — {} ຕຳແໜ່ງ ໃນ {} ໄຟລ໌:".into(),
    );

    m.insert(
        "misc.qur.confirm".into(),
        "ແທນທີ່ {} ຕຳແໜ່ງ ໃນ {} ໄຟລ໌? [y/N]:".into(),
    );

    m.insert("misc.qur.cancelled".into(), "ຍົກເລີກການແທນທີ່ແລ້ວ".into());

    m.insert(
        "misc.qur.replaced".into(),
        "ແທນທີ່ {} ຕຳແໜ່ງ ໃນ {} ໄຟລ໌ແລ້ວ".into(),
    );

    m.insert(
        "misc.qur.skipped".into(),
        "ຂ້າມ {} ໄຟລ໌ (ອ່ານ/ຂຽນບໍ່ສຳເລັດ)".into(),
    );

    m.insert(
        "misc.qur.many_results".into(),
        "qur: ພົບ {} ລາຍການ ສະແດງພຽງ 20,000 ລາຍການທຳອິດ ກະລຸນາຈຳກັດການຄົ້ນຫາ".into(),
    );

    m.insert(
        "misc.qur.replace_needs_select".into(),
        "qur: -rp ຕ້ອງໃຊ້ -s \"<pattern>\" (ເຊັ່ນ qur *.* -s \"old\" -rp \"new\")".into(),
    );

    m.insert(
        "misc.qur.display_needs_select".into(),
        "qur: -dp ຕ້ອງໃຊ້ -s \"<pattern>\" (ເຊັ່ນ qur *.* -s \"foo\" -dp)".into(),
    );

    m.insert(
        "misc.qur.empty_select".into(),
        "qur: -s ຕ້ອງມີ pattern (ເຊັ່ນ -s \"foo\")".into(),
    );

    m.insert(
        "misc.qur.empty_replace".into(),
        "qur: -rp ຕ້ອງມີຂໍ້ຄວາມແທນທີ່ (ເຊັ່ນ -rp \"bar\")".into(),
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
        "misc.trim.usage".into(),
        "ວິທີໃຊ້: trim <pattern> [ຂໍ້ຄວາມ]   (ຫຼື: <ຄຳສັ່ງ> | trim <pattern> [...])".into(),
    );

    m.insert(
        "misc.trim.empty_pattern".into(),
        "trim: pattern ຕ້ອງບໍ່ຫວ່າງ".into(),
    );

    m.insert(
        "misc.clear.type_help".into(),
        "ພິມ 'help' ສຳລັບຄຳສັ່ງທີ່ມີ, 'exit' ເພື່ອອອກ.".into(),
    );

    m.insert(
        "misc.clear.welcome".into(),
        "ຍິນດີຕ້ອນຮັບສູ່ ntc {} - Navigate, Toolkit, Center".into(),
    );

    m.insert(
        "misc.ds.example1".into(),
        " ds src # ຄົ້ນຫາດ້ວຍ config depth".into(),
    );

    m.insert(
        "misc.ds.example2".into(),
        " ds src -d 3 # ຄົ້ນຫາສູງເຖິງ 3 ລະດັບ".into(),
    );

    m.insert(
        "misc.ds.example3".into(),
        " ds test # ກົງບາງສ່ວນ: ຊອກເຫັນ test_utils/ ອື່ນໆ.".into(),
    );

    m.insert(
        "misc.ds.usage".into(),
        "Usage: ds <pattern> [-d <depth>]".into(),
    );

    m.insert(
        "misc.fs.example1".into(),
        " fs main.c # ຄົ້ນຫາດ້ວຍ config depth".into(),
    );

    m.insert(
        "misc.fs.example2".into(),
        " fs main.c -d 5 # ຄົ້ນຫາສູງເຖິງ 5 ລະດັບ".into(),
    );

    m.insert(
        "misc.fs.example3".into(),
        " fs main # ກົງບາງສ່ວນ: ຊອກເຫັນ main_helper.c ອື່ນໆ.".into(),
    );

    m.insert(
        "misc.fs.usage".into(),
        "Usage: fs <pattern> [-d <depth>]".into(),
    );

    m.insert(
        "misc.fsc.copied".into(),
        "ສຳເນົາ '{}' ໃສ່ clipboard ແລ້ວ!".into(),
    );

    m.insert(
        "misc.fsc.copy_prompt".into(),
        "ສຳເນົາໃສ່ clipboard ບໍ? (y/N): ".into(),
    );

    m.insert("misc.fsc.done_opt".into(), "0. ສຳເລັດ".into());

    m.insert("misc.fsc.done_str".into(), "ສຳເລັດ.".into());

    m.insert(
        "misc.fsc.example1".into(),
        " fsc main.rs # ຄົ້ນຫາຈາກນັ້ນເລືອກໄຟລ໌ເພື່ອສະແດງ".into(),
    );

    m.insert(
        "misc.fsc.example2".into(),
        " fsc main.rs -d 5 # ຄົ້ນຫາສູງເຖິງ 5 ລະດັບ".into(),
    );

    m.insert(
        "misc.fsc.found".into(),
        "\u{1F50D} ພົບໄຟລ໌ {} ສຳລັບ \"{}\":".into(),
    );

    m.insert("misc.fsc.invalid_input".into(), "ຂໍ້ມູນເຂົ້າບໍ່ຖືກຕ້ອງ.".into());

    m.insert("misc.fsc.invalid_selection".into(), "ການເລືອກບໍ່ຖືກຕ້ອງ.".into());

    m.insert("misc.fsc.not_file".into(), "path ທີ່ເລືອກບໍ່ແມ່ນໄຟລ໌.".into());

    m.insert("misc.fsc.select_prompt".into(), "ເລືອກໄຟລ໌ເພື່ອສະແດງ: ".into());

    m.insert("misc.fsc.skipped".into(), "ຂ້າມ (ບໍ່ຮອງຮັບຮູບແບບ): {}".into());

    m.insert(
        "misc.fsc.usage".into(),
        "Usage: fsc <pattern> [-d <depth>]".into(),
    );

    m.insert(
        "misc.gs.example1".into(),
        " gs main # ຄົ້ນຫາເນື້ອໃນໄຟລ໌ດ້ວຍ config depth".into(),
    );

    m.insert(
        "misc.gs.example2".into(),
        " gs main -d 5 # ຄົ້ນຫາສູງເຖິງ 5 ລະດັບ".into(),
    );

    m.insert(
        "misc.gs.example3".into(),
        " gs -i main # interactive mode (ຊອກຫາຜົນ, Enter ເພື່ອເປີດ)".into(),
    );

    m.insert(
        "misc.gs.example4".into(),
        " gs -i main -d 3 # interactive mode ດ້ວຍຂອບເຂດ depth".into(),
    );

    m.insert(
        "misc.gs.example5".into(),
        " gs main -d 5 -l 10 # ສະແດງພຽງ 10 ລາຍການທຳອິດ".into(),
    );

    m.insert(
        "misc.gs.example6".into(),
        " gs main -p \"*.rs\" # ຄົ້ນສະເພາະໄຟລ໌ທີ່ກົງ glob".into(),
    );

    m.insert(
        "misc.gs.example7".into(),
        " gs main -s size # ຮຽງຕາມຂະໜາດໄຟລ໌ (line|path|size, -s ເປົ່າ = line)".into(),
    );

    m.insert(
        "misc.gs.example8".into(),
        " gs error\\|warning # ຄົ້ນຫາແບບ OR: ກົງກັບຄຳໃດຄຳໜຶ່ງ (a\\|b)".into(),
    );

    m.insert(
        "misc.gs.no_pipe_path".into(),
        "gs: -p ໃຊ້ກັບ piped input ບໍ່ໄດ້ (pipe ສົ່ງຂໍ້ຄວາມ ບໍ່ແມ່ນໄຟລ໌)".into(),
    );

    m.insert(
        "misc.gs.pipe_sort_only_line".into(),
        "gs: piped input ໃຊ້ໄດ້ແຄ່ -s line (path/size ຕ້ອງໃຊ້ໄຟລ໌)".into(),
    );

    m.insert("misc.gs.searching".into(), "ກຳລັງຄົ້ນຫາເນື້ອໃນ...".into());

    m.insert(
        "misc.gs.many_results".into(),
        "gs: ພົບ {} ລາຍການ, ສະແດງ 20,000 ທຳອິດ. ຈຳກັດການຄົ້ນຫາໃຫ້ແຄບລົງ.".into(),
    );

    m.insert(
        "misc.gs.usage".into(),
        "Usage: gs <pattern> [-d <depth>] [-l <limit>] [-i] [-p <glob>]... [-s [line|path|size]]"
            .into(),
    );

    m.insert("misc.wi.not_dir".into(), "wi: ບໍ່ແມ່ນໄດເຣັກທໍຣີ: '{}'".into());

    m.insert("misc.wi.start_error".into(), "wi: ເຝົ້າລະວັງບໍ່ໄດ້: {}".into());

    m.insert("misc.wi.usage".into(), "Usage: wi [path]".into());

    m.insert(
        "misc.locate.example1".into(),
        " locate main.c # ຄົ້ນຫາໄຟລ໌ ແລະ dirs (config depth)".into(),
    );

    m.insert(
        "misc.locate.example2".into(),
        " locate main.c -d 5 # ຄົ້ນຫາສູງເຖິງ 5 ລະດັບ".into(),
    );

    m.insert(
        "misc.locate.example3".into(),
        " locate main # ກົງບາງສ່ວນ: ຊອກເຫັນ main.c, main_helper/ ອື່ນໆ.".into(),
    );

    m.insert(
        "misc.locate.header".into(),
        "\u{1F50D} {} ສຳລັບ \"{}\" ({} ຜົນ):".into(),
    );

    m.insert(
        "misc.locate.label_didyoumean".into(),
        "ທ່ານໝາຍຄວາມວ່າບໍ?".into(),
    );

    m.insert("misc.locate.label_exact".into(), "ກົງທີ່ແນ່ນອນ".into());

    m.insert("misc.locate.label_partial".into(), "ກົງບາງສ່ວນ".into());

    m.insert(
        "misc.locate.no_results".into(),
        "\n\u{1F50D} ບໍ່ມີຜົນສຳລັບ \"{}\"\n".into(),
    );

    m.insert(
        "misc.locate.usage".into(),
        "Usage: locate <pattern> [-d <depth>]".into(),
    );

    m.insert("misc.mkd.created".into(), "ສ້າງໄດເຣັກທໍຣີ: {}".into());

    m.insert("misc.mkd.error".into(), "mkd: {}: {}".into());

    m.insert("misc.mkd.missing".into(), "mkd: ຂາດ operand".into());

    m.insert("misc.mkf.created".into(), "ສ້າງໄຟລ໌: {}".into());

    m.insert("misc.mkf.error".into(), "mkf: {}: {}".into());

    m.insert("misc.mkf.exists".into(), "ມີຢູ່ແລ້ວ: {}".into());

    m.insert("misc.mkf.missing".into(), "mkf: ຂາດ operand".into());

    m.insert(
        "misc.ne.create_failed".into(),
        "ບໍ່ສາມາດສ້າງ template: {}".into(),
    );

    m.insert(
        "misc.ne.created_hyper".into(),
        "ສ້າງ hyper project: {}".into(),
    );

    m.insert(
        "misc.ne.created_super".into(),
        "ສ້າງ super template: {}".into(),
    );

    m.insert("misc.ne.created_template".into(), "ສ້າງ template: {}".into());

    m.insert("misc.ne.editor_error".into(), "Editor error: {}".into());

    m.insert(
        "misc.ne.file_missing".into(),
        "ບໍ່ພົບໄຟລ໌: {} ໃຊ້ --init <file> ຫຼື --super <file> ເພື່ອສ້າງ".into(),
    );

    m.insert(
        "misc.ne.hyper_error".into(),
        "Hyper project error: {}".into(),
    );

    m.insert("misc.ne.missing_file".into(), "ne: ຂາດ operand ໄຟລ໌".into());

    m.insert("misc.ne.one_file".into(), "ສ້າງໄດ້ພຽງໜຶ່ງໄຟລ໌ໃນເວລາດຽວ".into());

    m.insert(
        "misc.ne.open_not_created".into(),
        "ໄຟລ໌ -I ຕ້ອງເປັນໜຶ່ງໃນໄຟລ໌ທີ່ກຳລັງສ້າງ: {}".into(),
    );

    m.insert(
        "misc.ne.opened_welcome".into(),
        "ເປີດໜ້າຈໍ welcome ໃນ {}".into(),
    );

    m.insert(
        "misc.ne.usage_open".into(),
        "ການໃຊ້: -I <file> ຕ້ອງເປັນ argument ສຸດທ້າຍ (ໄຟລ໌ດຽວເທົ່ານັ້ນ)".into(),
    );

    m.insert(
        "misc.ne.usage_hyper".into(),
        "Usage: ne --hyper <name> --lang <lang> (name ຕ້ອງບໍ່ມີ extension)".into(),
    );

    m.insert(
        "misc.pathcp.copied".into(),
        "ສຳເນົາ path ໃສ່ clipboard: {}".into(),
    );

    m.insert("misc.pip.found".into(), "\u{2713} ພົບ pip ທີ່: {}".into());

    m.insert(
        "misc.pip.not_found".into(),
        "ບໍ່ພົບ pip ໃນໄດເຣັກທໍຣີ venv/Scripts".into(),
    );

    m.insert("misc.rmd.error".into(), "rmd: {}: {}".into());

    m.insert("misc.rmd.missing".into(), "rmd: ຂາດ operand".into());

    m.insert("misc.rmd.not_dir".into(), "rmd: '{}' ບໍ່ແມ່ນໄດເຣັກທໍຣີ".into());

    m.insert("misc.rmd.not_exist".into(), "rmd: '{}' ບໍ່ມີຢູ່".into());

    m.insert("misc.rmd.removed".into(), "ລຶບໄດເຣັກທໍຣີ: {}".into());

    m.insert("misc.rmd.no_match".into(), "rmd: ບໍ່ມີໄດເຣັກທໍຣີກົງກັບ '{}'".into());

    m.insert(
        "misc.rmd.pattern_matches".into(),
        "ມີ {} ໄດເຣັກທໍຣີກົງກັບ '{}':".into(),
    );

    m.insert("misc.rmd.confirm_delete".into(), "ລຶບ {} ໄດເຣັກທໍຣີບໍ?".into());

    m.insert("misc.rmd.cancelled".into(), "ຍົກເລີກແລ້ວ".into());

    m.insert(
        "misc.rmd.deadlock_vault_admin_only".into(),
        "rmd: '{}' ມີ deadlock vault ລຶບໄດ້ໂດຍ root/ຜູ່ດູ້ແຕ່ດາຍເທັກ".into(),
    );

    m.insert("misc.rmf.error".into(), "rmf: {}: {}".into());

    m.insert(
        "misc.rmf.is_dir".into(),
        "rmf: '{}' ເປັນໄດເຣັກທໍຣີ (ໃຊ້ rmd ແທນ)".into(),
    );

    m.insert("misc.rmf.missing".into(), "rmf: ຂາດ operand".into());

    m.insert("misc.rmf.not_exist".into(), "rmf: '{}' ບໍ່ມີຢູ່".into());

    m.insert("misc.rmf.no_match".into(), "rmf: ບໍ່ມີໄຟລ໌ກົງກັບ '{}'".into());

    m.insert(
        "misc.rmf.pattern_matches".into(),
        "ມີ {} ໄຟລ໌ກົງກັບ '{}':".into(),
    );

    m.insert("misc.rmf.confirm_delete".into(), "ລຶບ {} ໄຟລ໌ບໍ?".into());

    m.insert("misc.rmf.cancelled".into(), "ຍົກເລີກແລ້ວ".into());

    m.insert("misc.rmf.removed".into(), "ລຶບແລ້ວ: {}".into());

    m.insert(
        "misc.rmf.deadlock_vault_admin_only".into(),
        "rmf: '{}' ເປັນ deadlock vault ລືບໄດ້ໂດຍ root/ຜູ່ດູ້ແຕ່ດາຍເທັກ".into(),
    );

    m.insert("misc.rnmd.error".into(), "rnmd: {}".into());

    m.insert("misc.rnmd.exists".into(), "rnmd: '{}' ມີຢູ່ແລ້ວ".into());

    m.insert("misc.rnmd.missing".into(), "rnmd: ຂາດ operand".into());

    m.insert("misc.rnmd.not_dir".into(), "rnmd: '{}' ບໍ່ແມ່ນໄດເຣັກທໍຣີ".into());

    m.insert("misc.rnmd.not_exist".into(), "rnmd: '{}' ບໍ່ມີຢູ່".into());

    m.insert(
        "misc.rnmd.renamed".into(),
        "ປ່ຽນຊື່ໄດເຣັກທໍຣີ '{}' -> '{}'".into(),
    );

    m.insert(
        "misc.rnmd.usage".into(),
        " Usage: rnmd <old_name> <new_name>".into(),
    );

    m.insert("misc.rnmf.error".into(), "rnmf: {}".into());

    m.insert("misc.rnmf.exists".into(), "rnmf: '{}' ມີຢູ່ແລ້ວ".into());

    m.insert(
        "misc.rnmf.is_dir".into(),
        "rnmf: '{}' ເປັນໄດເຣັກທໍຣີ (ໃຊ້ rnmd ແທນ)".into(),
    );

    m.insert("misc.rnmf.missing".into(), "rnmf: ຂາດ operand".into());

    m.insert("misc.rnmf.not_exist".into(), "rnmf: '{}' ບໍ່ມີຢູ່".into());

    m.insert("misc.rnmf.renamed".into(), "ປ່ຽນຊື່ '{}' -> '{}'".into());

    m.insert(
        "misc.rnmf.usage".into(),
        " Usage: rnmf <old_name> <new_name>".into(),
    );

    m.insert("misc.mv.error".into(), "mv: {}".into());

    m.insert(
        "misc.mv.exists".into(),
        "mv: ເປົ້າໝາຍມີຢູ່ແລ້ວ: '{}' (ໃຊ້ -f ເພື່ອຂຽນທັບ)".into(),
    );

    m.insert("misc.mv.missing".into(), "mv: ຂາດ operand".into());

    m.insert("misc.mv.moved".into(), "ຍ້າຍ '{}' -> '{}' ແລ້ວ".into());

    m.insert("misc.mv.not_exist".into(), "mv: ບໍ່ພົບຕົ້ນທາງ: '{}'".into());

    m.insert("misc.mv.no_match".into(), "mv: ບໍ່ມີໄຟລ໌ທີ່ກົງກັບ '{}'".into());

    m.insert(
        "misc.mv.dst_dir".into(),
        "mv: ປາຍທາງຕ້ອງເປັນໂຟນເດີ: '{}'".into(),
    );

    m.insert("misc.mv.wild_done".into(), "ຍ້າຍ {} ໄຟລ໌ໄປຍັງ '{}' ແລ້ວ".into());

    m.insert(
        "misc.mv.usage".into(),
        "Usage: mv <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert("misc.cp.error".into(), "cp: {}".into());

    m.insert(
        "misc.cp.exists".into(),
        "cp: ເປົ້າໝາຍມີຢູ່ແລ້ວ: '{}' (ໃຊ້ -f ເພື່ອຂຽນທັບ)".into(),
    );

    m.insert("misc.cp.missing".into(), "cp: ຂາດ operand".into());

    m.insert("misc.cp.copied".into(), "ສຳເນົາ '{}' -> '{}' ແລ້ວ".into());

    m.insert("misc.cp.not_exist".into(), "cp: ບໍ່ພົບຕົ້ນທາງ: '{}'".into());

    m.insert("misc.cp.no_match".into(), "cp: ບໍ່ມີໄຟລ໌ທີ່ກົງກັບ '{}'".into());

    m.insert(
        "misc.cp.dst_dir".into(),
        "cp: ປາຍທາງຕ້ອງເປັນໂຟນເດີ: '{}'".into(),
    );

    m.insert(
        "misc.cp.wild_done".into(),
        "ສຳເນົາ {} ໄຟລ໌ໄປຍັງ '{}' ແລ້ວ".into(),
    );

    m.insert(
        "misc.cp.same".into(),
        "cp: ຕົ້ນທາງແລະປາຍທາງແມ່ນໄຟລ໌ດຽວກັນ: '{}'".into(),
    );

    m.insert(
        "misc.cp.usage".into(),
        "Usage: cp <src|@tp|pattern> <dst|@tp> [-f]".into(),
    );

    m.insert(
        "misc.say.example1".into(),
        " say \"hello world\" --loop 5 -n # 5 ແຖວ".into(),
    );

    m.insert(
        "misc.say.example2".into(),
        " say \"hello world\" --loop 3 -t # 1 ແຖວ, 3 ຄັ້ງ".into(),
    );

    m.insert(
        "misc.say.loop_exceed".into(),
        "ຄ່າ --loop ເກີນສູງສຸດ ({}), ກຳລັງຈຳກັດ".into(),
    );

    m.insert(
        "misc.say.loop_invalid".into(),
        "ຄ່າ --loop '{}' ບໍ່ຖືກຕ້ອງ, ໃຊ້ 1".into(),
    );

    m.insert(
        "misc.say.loop_requires".into(),
        "--loop ຕ້ອງເປັນຈຳນວນ, ໃຊ້ 1".into(),
    );

    m.insert(
        "misc.say.loop_warning".into(),
        "ຄ່າ --loop ບໍ່ສາມາດນ້ອຍກວ່າ 1, ໃຊ້ 1".into(),
    );

    m.insert(
        "misc.say.usage".into(),
        "Usage: say <text> [--loop N] [-n|-t]".into(),
    );

    m.insert(
        "misc.show_env.activate_hint".into(),
        "ເຮັດວຽກ 'python_activate' ເພື່ອເປີດໃຊ້ virtual environment".into(),
    );

    m.insert(
        "misc.show_env.active".into(),
        "Virtual environment ກຳລັງ".into(),
    );

    m.insert("misc.show_env.and_more".into(), "... ແລະ {} ອີກ".into());

    m.insert("misc.show_env.cwd_label".into(), "ໄດເຣັກທໍຣີປັດຈຸບັນ:".into());

    m.insert(
        "misc.show_env.deactivate_hint".into(),
        "ເຮັດວຽກ 'deactivate' ເພື່ອອອກຈາກ virtual environment".into(),
    );

    m.insert(
        "misc.show_env.not_active".into(),
        "Virtual environment ບໍ່".into(),
    );

    m.insert("misc.show_env.not_set".into(), "(ບໍ່ໄດ້ຕັ້ງ)".into());

    m.insert("misc.show_env.os_label".into(), "OS:".into());

    m.insert(
        "misc.show_env.other_header".into(),
        "\u{1F527} ຕົວແປ Other Python/Venv:".into(),
    );

    m.insert(
        "misc.show_env.path_full".into(),
        "\u{1F4C2} PATH (full):".into(),
    );

    m.insert(
        "misc.show_env.path_header".into(),
        "\u{1F4C2} PATH (ສອງສາມລາຍການທຳອິດ):".into(),
    );

    m.insert("misc.show_env.shell_label".into(), "Shell:".into());

    m.insert(
        "misc.show_env.summary_header".into(),
        "\u{1F4CA} ສະຫຼຸບ:".into(),
    );

    m.insert(
        "misc.show_env.sys_header".into(),
        "\u{1F4BB} ຂໍ້ມູນລະບົບ:".into(),
    );

    m.insert(
        "misc.show_env.venv_header".into(),
        "\u{1F40D} Python Virtual Environment:".into(),
    );

    m.insert("misc.size.bytes_label".into(), "Bytes:".into());

    m.insert(
        "misc.size.calculating".into(),
        "ກຳລັງຄິດໄລ່ຂະໜາດໄດເຣັກທໍຣີ...".into(),
    );

    m.insert("misc.size.human_label".into(), "Human:".into());

    m.insert("misc.size.path_label".into(), "ເສັ້ນທາງ:".into());

    m.insert("misc.size.time_label".into(), "Time:".into());

    m.insert("misc.size.title".into(), "ຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into());

    m.insert(
        "misc.tp.example_info".into(),
        "ຕົວຢ່າງ: tp info my_project".into(),
    );

    m.insert("misc.tp.info_canonical".into(), "Canonical".into());

    m.insert("misc.tp.info_exists".into(), "Exists".into());

    m.insert("misc.tp.info_global".into(), "global".into());

    m.insert("misc.tp.info_index".into(), "Index".into());

    m.insert("misc.tp.info_name".into(), "Name".into());

    m.insert("misc.tp.info_path".into(), "Path".into());

    m.insert("misc.tp.info_source".into(), "Source".into());

    m.insert(
        "misc.tp.info_title".into(),
        "\u{1F4CC} ຂໍ້ມູນ Teleport Savepoint".into(),
    );

    m.insert(
        "misc.tp.invalid_format".into(),
        "ຮູບແບບບໍ່ຖືກຕ້ອງ. ໃຊ້: tp rnm <old> to <new>".into(),
    );

    m.insert("misc.tp.not_found".into(), "ບໍ່ພົບ Savepoint '{}'".into());

    m.insert(
        "misc.tp.type_help".into(),
        "ພິມ 'tp help' ສຳລັບການໃຊ້ງານ.".into(),
    );

    m.insert(
        "misc.tp.unknown_subcmd".into(),
        "tp subcommand ບໍ່ຮູ້ຈັກ: {}".into(),
    );

    m.insert(
        "misc.tp.usage_add".into(),
        "Usage: tp add <name> [path]".into(),
    );

    m.insert("misc.tp.usage_info".into(), "Usage: tp info <name>".into());

    m.insert(
        "misc.tp.usage_jump".into(),
        "Usage: tp jump <name|number>".into(),
    );

    m.insert(
        "misc.tp.usage_rm".into(),
        "Usage: tp rm <name|number>[, <name|number>, ...]".into(),
    );

    m.insert(
        "misc.tp.usage_rnm".into(),
        "Usage: tp rnm <old_name> to <new_name>".into(),
    );

    m.insert("misc.tpb.unknown".into(), "tpb option ບໍ່ຮູ້ຈັກ: {}".into());

    m.insert("misc.tpb.usage".into(), "Usage:".into());

    m.insert(
        "misc.tpb.usage_basic".into(),
        " tpb - Teleport ກັບໄປຕຳແໜ່ງກ່ອນໜ້າ".into(),
    );

    m.insert(
        "misc.tpb.usage_clear".into(),
        " tpb clear - ລຶບ teleport history".into(),
    );

    m.insert(
        "misc.tpb.usage_history".into(),
        " tpb history - ສະແດງ teleport history".into(),
    );

    m.insert(
        "misc.ui.removed".into(),
        "The modern UI ຖືກລຶບແລ້ວ. ມີແຕ່ classic shell ເທົ່ານັ້ນ.".into(),
    );

    m.insert("misc.venva.activated".into(), "ເປີດໃຊ້ແລ້ວ: {}".into());

    m.insert(
        "misc.venva.activating".into(),
        "ກຳລັງເປີດໃຊ້ virtual environment: {}".into(),
    );

    m.insert(
        "misc.venva.active_info".into(),
        "Virtual environment ເປີດໃຊ້ງານແລ້ວ. ຄຳສັ່ງ pip ແລະ python ຈະໃຊ້ environment ນີ້.".into(),
    );

    m.insert(
        "misc.venva.already_in".into(),
        "ຢູ່ໃນ virtual environment ແລ້ວ. ໃຊ້ 'deactivate' ເພື່ອອອກກ່ອນ.".into(),
    );

    m.insert("misc.venva.failed".into(), "ເປີດໃຊ້ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "misc.venva.no_script".into(),
        "ບໍ່ພົບ activate script ໃນ: {}".into(),
    );

    m.insert(
        "misc.venva.not_found".into(),
        "ບໍ່ພົບ virtual environment. Usage: python_activate <venv_name>".into(),
    );

    m.insert(
        "misc.venvd.deactivated".into(),
        "ປິດໃຊ້ງານ virtual environment ແລ້ວ".into(),
    );

    m.insert(
        "misc.venvd.not_in_venv".into(),
        "ບໍ່ໄດ້ຢູ່ໃນ virtual environment".into(),
    );
}
