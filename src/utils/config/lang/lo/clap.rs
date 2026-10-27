use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "clap.arg.cared.help".into(),
        "ຢຸດການບໍ່ສົນໃຈໄດເຣັກທໍຣີອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert(
        "clap.arg.caref.help".into(),
        "ໃສ່ໃຈກັບນາມສະກຸນໄຟລ໌ອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert(
        "clap.arg.caren.help".into(),
        "ໃສ່ໃຈກັບໄຟລ໌ສະເພາະອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert("clap.arg.clear.help".into(), "ລ້າງໜ້າຈໍ terminal".into());

    m.insert(
        "clap.arg.copy.help".into(),
        "ສຳເນົາບົດລາຍງານໄປໃສ່ clipboard ແທນການບັນທຶກເປັນໄຟລ໌".into(),
    );

    m.insert(
        "clap.arg.edit.help".into(),
        "ເປີດໄຟລ໌ໃນ text editor ໃນຕົວ".into(),
    );

    m.insert(
        "clap.arg.explorer.help".into(),
        "ເປີດ NTC File Explorer TUI (path ເລີ່ມຕົ້ນທາງເລືອກ)".into(),
    );

    m.insert(
        "clap.arg.format_cli.help".into(),
        "ຮູບແບບຜົນລັບ: txt, html, json, md".into(),
    );

    m.insert(
        "clap.arg.generate_completions.help".into(),
        "ສ້າງ shell completion script (bash|zsh|fish|powershell)".into(),
    );

    m.insert(
        "clap.arg.help_arg.help".into(),
        "ສະແດງຄວາມຊ່ວຍເຫຼືອລາຍລະອຽດ".into(),
    );

    m.insert(
        "clap.arg.hyper.help".into(),
        "ສ້າງ hyper project scaffold (DSA library)".into(),
    );

    m.insert(
        "clap.arg.igcare_export_all.help".into(),
        "ສົ່ງອອກການຕັ້ງຄ່າ ignore/care ທັງໝົດໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_export_select.help".into(),
        "ເລືອກໝວດໝູ່ເພື່ອສົ່ງອອກໄປຍັງ <name>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_import.help".into(),
        "ນຳເຂົ້າການຕັ້ງຄ່າ ignore/care ຈາກໄຟລ໌ .ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.ignore.help".into(),
        "ບໍ່ສົນໃຈຊື່ໄດເຣັກທໍຣີອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert("clap.arg.ignored.help".into(), "ສະແດງລາຍການທີ່ຖືກບໍ່ສົນໃຈ".into());

    m.insert(
        "clap.arg.ignoref.help".into(),
        "ບໍ່ສົນໃຈນາມສະກຸນໄຟລ໌ອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert(
        "clap.arg.ignoren.help".into(),
        "ບໍ່ສົນໃຈໄຟລ໌ສະເພາະອັນໜຶ່ງຂຶ້ນໄປ".into(),
    );

    m.insert(
        "clap.arg.image.help".into(),
        "ແປງຮູບພາບເປັນເອກະສານ (JSON, MD, HTML, TXT)".into(),
    );

    m.insert(
        "clap.arg.image_to.help".into(),
        "ຮູບແບບຜົນລັບສຳລັບການແປງຮູບພາບ: json, md, html, txt".into(),
    );

    m.insert(
        "clap.arg.init.help".into(),
        "ສ້າງໄຟລ໌ starter ຈາກ template ແລະເປີດໃນ editor".into(),
    );

    m.insert(
        "clap.arg.input.help".into(),
        "ໄຟລ໌ ຫຼື path ໄດເຣັກທໍຣີທີ່ນຳເຂົ້າ".into(),
    );

    m.insert(
        "clap.arg.lang_cli.help".into(),
        "ພາສາສຳລັບ hyper project".into(),
    );

    m.insert(
        "clap.arg.list.help".into(),
        "ລາຍຊື່ຟັງຊັນ command-line ທັງໝົດ".into(),
    );

    m.insert(
        "clap.arg.lsp.help".into(),
        "ເລີ່ມ ntc-math LSP server ຜ່ານ stdio".into(),
    );

    m.insert(
        "clap.arg.lsp_log.help".into(),
        "path ເພື່ອຂຽນ LSP debug log".into(),
    );

    m.insert("clap.arg.math.help".into(), "ຄິດໄລ່ນິພົດຄະນິດສາດ".into());

    m.insert("clap.arg.output.help".into(), "ຊື່ໄຟລ໌ສຳລັບບົດລາຍງານ".into());

    m.insert(
        "clap.arg.ral_export_all.help".into(),
        "ສົ່ງອອກ run aliases ທັງໝົດໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_export_select.help".into(),
        "ເລືອກ run aliases ເພື່ອສົ່ງອອກໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_import.help".into(),
        "ນຳເຂົ້າ run aliases ຈາກໄຟລ໌ .ntc.ral".into(),
    );

    m.insert("clap.arg.say.help".into(), "ພິມຂໍ້ຄວາມອອກ stdout".into());

    m.insert(
        "clap.arg.setA.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງ autosuggest (ghost text) (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setC.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງສີຜົນລັບ (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setD.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງຄວາມເລິກ recursion ສູງສຸດ".into(),
    );

    m.insert(
        "clap.arg.setH.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງ history (path/ON/OFF/default)".into(),
    );

    m.insert(
        "clap.arg.setL.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງການສະແດງເບີແຖວ (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.setO.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງໄດເຣັກທໍຣີຜົນລັບເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "clap.arg.setT.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງຈຳນວນ threads".into(),
    );

    m.insert(
        "clap.arg.showcg.help".into(),
        "ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert(
        "clap.arg.size_cli.help".into(),
        "ສະແດງຂະໜາດໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "clap.arg.time.help".into(),
        "ສະແດງເວລາທີ່ໃຊ້ (ໃຊ້ກັບ --size ຫຼື --view)".into(),
    );

    m.insert(
        "clap.arg.tp_add.help".into(),
        "ບັນທຶກໄດເຣັກທໍຣີປັດຈຸບັນເປັນ teleport point".into(),
    );

    m.insert(
        "clap.arg.tp_list.help".into(),
        "ລາຍຊື່ teleport points ທັງໝົດ".into(),
    );

    m.insert("clap.arg.tp_rm.help".into(), "ລຶບ teleport point".into());

    m.insert(
        "clap.arg.upgrade.help".into(),
        "ກວດຫາການອັບເດດ ແລະ ອັບເກຣດ ntc ເປັນເວີຊັນລ່າສຸດ".into(),
    );

    m.insert("clap.arg.version.help".into(), "ສະແດງຂໍ້ມູນເວີຊັນ".into());

    m.insert(
        "clap.arg.view_cli.help".into(),
        "ເບິ່ງໂຄງສ້າງຕົ້ນໄມ້ໄດເຣັກທໍຣີປັດຈຸບັນຢ່າງໄວ".into(),
    );

    m.insert(
        "clap.arg.watch_cli.help".into(),
        "ສະແດງ ຫຼື ຕັ້ງ file watcher (ON/OFF)".into(),
    );

    m.insert(
        "clap.arg.where_cli.help".into(),
        "ສະແດງບ່ອນຕັ້ງ executable ຂອງ ntc".into(),
    );

    m.insert("clap.value_name.depth".into(), "DEPTH".into());

    m.insert("clap.value_name.expr".into(), "EXPR".into());

    m.insert("clap.value_name.ext".into(), "EXT".into());

    m.insert("clap.value_name.file".into(), "FILE".into());

    m.insert("clap.value_name.format_cli".into(), "FORMAT".into());

    m.insert("clap.value_name.image".into(), "IMAGE".into());

    m.insert("clap.value_name.lang".into(), "LANG".into());

    m.insert("clap.value_name.name".into(), "NAME".into());

    m.insert("clap.value_name.path".into(), "PATH".into());

    m.insert("clap.value_name.project".into(), "PROJECT".into());

    m.insert("clap.value_name.shell".into(), "SHELL".into());

    m.insert("clap.value_name.state".into(), "STATE".into());

    m.insert("clap.value_name.text".into(), "TEXT".into());

    m.insert("clap.value_name.threads".into(), "THREADS".into());

    m.insert("clap.value_name.value".into(), "VALUE".into());
}
