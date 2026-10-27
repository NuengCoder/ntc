use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.report.content_saved".into(),
        "ບັນທຶກເນື້ອໃນໄຟລ໌ໄປທີ່: {}".into(),
    );

    m.insert(
        "cmd.report.copied_clipboard".into(),
        "ສຳເນົາໄຟລ໌ '{}' ໃສ່ clipboard ແລ້ວ!".into(),
    );

    m.insert("cmd.report.copy_failed".into(), "ສຳເນົາບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "cmd.report.copying_clipboard".into(),
        "ກຳລັງສຳເນົາ '{}' ໃສ່ clipboard...".into(),
    );

    m.insert(
        "cmd.report.dir_only".into(),
        "{} ບົດລາຍງານເຮັດວຽກກັບໄດເຣັກທໍຣີເທົ່ານັ້ນ".into(),
    );

    m.insert(
        "cmd.report.example_json".into(),
        "  report json ..         ລາຍງານກ່ຽວກັບໄດເຣັກທໍຣີ parent".into(),
    );

    m.insert(
        "cmd.report.example_txt".into(),
        "  report txt             ລາຍງານກ່ຽວກັບໄດເຣັກທໍຣີປັດຈຸບັນ".into(),
    );

    m.insert("cmd.report.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert("cmd.report.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: {}".into());

    m.insert(
        "cmd.report.no_files_in_dir".into(),
        "ບໍ່ມີໄຟລ໌ທີ່ຮອງຮັບໃຫ້ສະແດງໃນ: {}".into(),
    );

    m.insert(
        "cmd.report.help_docx".into(),
        "  report docx [path]     ສ້າງບົດລາຍງານ DOCX".into(),
    );

    m.insert(
        "cmd.report.help_html".into(),
        "  report html [path]     ສ້າງບົດລາຍງານ HTML".into(),
    );

    m.insert(
        "cmd.report.help_json".into(),
        "  report json [path]     ສ້າງບົດລາຍງານ JSON".into(),
    );

    m.insert(
        "cmd.report.help_md".into(),
        "  report md [path]       ສ້າງບົດລາຍງານ Markdown".into(),
    );

    m.insert(
        "cmd.report.help_pdf".into(),
        "  report pdf [path]      ສ້າງບົດລາຍງານ PDF".into(),
    );

    m.insert(
        "cmd.report.help_txt".into(),
        "  report txt [path]      ສ້າງບົດລາຍງານ TXT".into(),
    );

    m.insert(
        "cmd.report.help_xlsx".into(),
        "  report xlsx [path]     ສ້າງບົດລາຍງານ XLSX".into(),
    );

    m.insert(
        "cmd.report.invalid_depth".into(),
        "ຄວາມເລິກບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "cmd.report.neovim_hint".into(),
        "ທ່ານສາມາດເປີດນີ້ໃນ Neovim ໄດ້ດ້ວຍ: :edit {}".into(),
    );

    m.insert(
        "cmd.report.skipped_unsupported".into(),
        "ຂ້າມ (ບໍ່ຮອງຮັບຮູບແບບ): {}".into(),
    );

    m.insert("cmd.report.time_label".into(), "ເວລາ:".into());

    m.insert(
        "cmd.report.title".into(),
        "report — ສ້າງບົດລາຍງານໂຄງການ".into(),
    );

    m.insert(
        "cmd.report.ext_usage".into(),
        "ວິທີໃຊ້: view -ext <ນາມສະກຸນ...> (ຕົວຢ່າງ view -ext rs c)".into(),
    );

    m.insert(
        "cmd.report.unknown_option".into(),
        "ບໍ່ຮູ້ຈັກ view option: {}".into(),
    );

    m.insert("cmd.report.view_usage".into(), "ວິທີໃຊ້: view [-s|--size] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <path|@tp>] [-ext|--ext <ນາມສະກຸນ...>]".into());

    m.insert(
        "cmd.report.invalid_range".into(),
        "Invalid range: '{}' (expected n:m, e.g. --betw 10:20)".into(),
    );

    m.insert(
        "cmd.report.invalid_sort".into(),
        "Invalid sort key: '{}' (size|char|ext)".into(),
    );

    m.insert(
        "cmd.report.invalid_count".into(),
        "Invalid line count: '{}' (must be > 0)".into(),
    );

    m.insert(
        "cmd.report.clip_header".into(),
        "ບັນທຶກ clipboard #{}:".into(),
    );

    m.insert("cmd.report.clip_meta_date".into(), "ວັນທີ".into());

    m.insert("cmd.report.clip_meta_lines".into(), "ຈຳນວນແຖວ".into());

    m.insert("cmd.report.clip_meta_chars".into(), "ຈຳນວນຕົວອັກສອນ".into());
}
