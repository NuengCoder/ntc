use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nsheet.usage_title".into(), "ວິທີໃຊ້:".into());

    m.insert(
        "cmd.nsheet.help_open".into(),
        "nsheet open <ໄຟລ໌> [ຕົວເລືອກ]   ເບິ່ງແບບເຕັມຈໍ (CSV/TSV/JSON/JSONL)".into(),
    );

    m.insert(
        "cmd.nsheet.help_view".into(),
        "nsheet view <ໄຟລ໌> [ຕົວເລືອກ]   ພິມຕາຕະລາງ (ເລີ່ມຕົ້ນ 50 ແຖວ, --all ທັງໝົດ)".into(),
    );

    m.insert("cmd.nsheet.options_title".into(), "ຕົວເລືອກ:".into());

    m.insert(
        "cmd.nsheet.help_delimiter".into(),
        "  -d, --delimiter <ອັກຂระ|auto>  ຕົວຄັ່ນຟິວ (ເລີ່ມຕົ້ນ: ກວດອັດຕະໂນມັດ , ; tab |)".into(),
    );

    m.insert(
        "cmd.nsheet.help_header".into(),
        "  --header / --no-header        ບັງຄັບມີ/ບໍ່ມີແຖວຫົວ (ເລີ່ມຕົ້ນ: ກວດອັດຕະໂນມັດ)".into(),
    );

    m.insert(
        "cmd.nsheet.help_limit".into(),
        "  -l, --limit <n> / --all        ຈຳນວນແຖວທີ່ພິມໃນໂຫມດ view".into(),
    );

    m.insert(
        "cmd.nsheet.help_sort".into(),
        "  --sort <ຖັນ> [--desc]           ຮຽງຕາມຊື່ຖັນຫຼືລຳດັບที่ (ເລີ່ມที่ 1)".into(),
    );

    m.insert(
        "cmd.nsheet.help_filter".into(),
        "  --filter <ເງື່ອນໄຂ>           ເຊັ່ນ age>30, city~york, name=bob (!=,>=,<=,&&,||)".into(),
    );

    m.insert(
        "cmd.nsheet.help_export".into(),
        "  --export <json|md>            ສົ່ງອອກແທນການພິມ".into(),
    );

    m.insert(
        "cmd.nsheet.help_output".into(),
        "  -o, --output <ໄຟລ໌>           ປາຍທາງສົ່ງອອກ (ເລີ່ມຕົ້ນ: ໜ້າຈໍ)".into(),
    );

    m.insert(
        "cmd.nsheet.help_help".into(),
        "  --help, -h                    ສະແດງ help ນີ້".into(),
    );

    m.insert("cmd.nsheet.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.nsheet.example_open".into(),
        "nsheet open users.csv".into(),
    );

    m.insert(
        "cmd.nsheet.example_view".into(),
        "nsheet view users.csv -l 20 --sort age --desc".into(),
    );

    m.insert(
        "cmd.nsheet.example_filter".into(),
        "nsheet view data.json --filter \"age>30 && city~york\"".into(),
    );

    m.insert(
        "cmd.nsheet.example_export".into(),
        "nsheet view a.csv --export json -o out.json".into(),
    );

    m.insert(
        "cmd.nsheet.missing_file".into(),
        "ຂາດໄຟລ໌ ວິທີໃຊ້: nsheet open <ໄຟລ໌> | nsheet view <ໄຟລ໌>".into(),
    );

    m.insert("cmd.nsheet.file_not_found".into(), "ບໍ່ພົບໄຟລ໌ '{}'".into());

    m.insert("cmd.nsheet.load_fail".into(), "ໂຫຼດຕາຕະລາງບໍ່ໄດ້: {}".into());

    m.insert(
        "cmd.nsheet.empty_sheet".into(),
        "ຕາຕະລາງຫວ່າງ (ບໍ່ພົບຖັນ)".into(),
    );

    m.insert("cmd.nsheet.no_rows".into(), "ບໍ່ມີແຖວທີ່ກົງກັນ".into());

    m.insert("cmd.nsheet.unknown_option".into(), "ບໍ່ຮູ້ຈັກ: {}".into());

    m.insert("cmd.nsheet.unknown_column".into(), "ບໍ່ຮູ້ຈັກຖັນ: '{}'".into());

    m.insert(
        "cmd.nsheet.invalid_limit".into(),
        "ຈຳນວນບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_delimiter".into(),
        "ຕົວຄັ່ນບໍ່ຖືກຕ້ອງ: '{}' (ໃຊ້ອັກຂระດຽວຫຼື auto)".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_export".into(),
        "ຮູບແບບສົ່ງອອກບໍ່ຖືກຕ້ອງ: '{}' (ໃຊ້ json ຫຼື md)".into(),
    );

    m.insert(
        "cmd.nsheet.delimiter_usage".into(),
        "ວິທີໃຊ້: --delimiter <ອັກຂระ|auto>".into(),
    );

    m.insert(
        "cmd.nsheet.limit_usage".into(),
        "ວິທີໃຊ້: --limit <ຈຳນວນ>".into(),
    );

    m.insert("cmd.nsheet.sort_usage".into(), "ວິທີໃຊ້: --sort <ຖັນ>".into());

    m.insert(
        "cmd.nsheet.filter_usage".into(),
        "ວິທີໃຊ້: --filter <ເງື່ອນໄຂ> (ເຊັ່ນ age>30)".into(),
    );

    m.insert(
        "cmd.nsheet.export_usage".into(),
        "ວິທີໃຊ້: --export <json|md>".into(),
    );

    m.insert(
        "cmd.nsheet.output_usage".into(),
        "ວິທີໃຊ້: --output <ໄຟລ໌>".into(),
    );

    m.insert("cmd.nsheet.filter_fail".into(), "ເງື່ອນໄຂບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert("cmd.nsheet.export_fail".into(), "ສົ່ງອອກລົ້ມເຫຼວ: {}".into());

    m.insert("cmd.nsheet.exported".into(), "ສົ່ງອອກແລ້ວ: {}".into());

    m.insert(
        "cmd.nsheet.rows_shown".into(),
        "ສະແດງ {} ຈາກ {} ແຖວ ({} ຖັນ)".into(),
    );

    m.insert(
        "cmd.nsheet.rows_truncated".into(),
        "ຕັດເຫຼືອ {} ແຖວ (ໃຊ້ --all ຫຼື -l <ຈຳນວນ>)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_hint".into(),
        "ລูกສອນ/hjkl ເລື່ອນ · / ຄົ້ນຫາ · n ຖັດໄປ · s ຮຽງ · f ກອງ · c ລ້າງ · e ສົ່ງອອກ · Enter ເບິ່ງ · q ອອກ"
            .into(),
    );

    m.insert(
        "cmd.nsheet.tui_search_prompt".into(),
        "ຄົ້ນຫາ (ຫວ່າງ = ລ้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_sort_prompt".into(),
        "ຮຽງຕາມຖັນ (ທ້າຍ ! = ຫຼາຍไปหน้อย, ຫວ່າງ = ລ้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_filter_prompt".into(),
        "ກອງ (ຫວ່າງ = ລ้าง)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_export_prompt".into(),
        "ສົ່ງອອກໄປ (ເສັ້ນທາງ .json/.md)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_cell_title".into(),
        "ເຊລ (ກົດປຸ່ມໃດກໍໄດ້ເພື່ອປິດ)".into(),
    );

    m.insert("cmd.nsheet.tui_popup_hint".into(), "ກົດປຸ່ມໃດກໍໄດ້ເພື່ອປິດ".into());

    m.insert("cmd.nsheet.tui_no_matches".into(), "ບໍ່ພົບທີ່ກົງກັນ".into());

    m.insert(
        "cmd.nsheet.tui_large".into(),
        "ຕາຕະລາງໃຫຍ່ ({} ແຖວ) — ອາດເລື່ອນຊ້າ".into(),
    );
}
