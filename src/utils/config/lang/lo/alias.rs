use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "alias.ral.add_cmd".into(),
        " ral add <name> <command> ສ້າງ run alias ໃໝ່".into(),
    );

    m.insert(
        "alias.ral.alias_count".into(),
        "ທ່ານມີ {} alias ຖືກກຳນົດ:".into(),
    );

    m.insert("alias.ral.and_more".into(), "... ແລະອີກ {} ອັນ".into());

    m.insert(
        "alias.ral.array_cmd".into(),
        " ral add <name>(x[],y) <cmd> ສ້າງ alias ທີ່ມີພາຣາມິເຕີແບບ array (ໃຊ້ $x[].ext ຫຼື ${{x}}[].ext)"
            .into(),
    );

    m.insert("alias.ral.cls_cancelled".into(), "ຍົກເລີກການລຶບ.".into());

    m.insert(
        "alias.ral.cls_cmd".into(),
        " ral cls ລຶບ alias ທັງໝົດ (ຖາມຢືນຢັນ)".into(),
    );

    m.insert(
        "alias.ral.cls_prompt".into(),
        "ແນ່ໃຈບໍ? ພິມ 'yes' ເພື່ອຢືນຢັນ: ".into(),
    );

    m.insert(
        "alias.ral.cls_warning".into(),
        "\u{26A0}\u{FE0F} ຄຳເຕືອນ: ອັນນີ້ຈະລຶບ run alias ທັງໝົດ!".into(),
    );

    m.insert("alias.ral.config_path".into(), " ການຕັ້ງຄ່າ: {}".into());

    m.insert("alias.ral.create_failed".into(), "ສ້າງໄຟລ໌ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "alias.ral.default_cmd".into(),
        " ral add <name>({{x=a,e=txt}}) <cmd> ສ້າງ alias ທີ່ມີຄ່າເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "alias.ral.edit_cmd".into(),
        " ral edit <name> <command> ອັບເດດ alias ທີ່ມີຢູ່".into(),
    );

    m.insert(
        "alias.ral.empty".into(),
        "ບໍ່ມີ run alias ຖືກກຳນົດ. ໃຊ້ 'ral add <name> <command>'".into(),
    );

    m.insert("alias.ral.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "alias.ral.export_all_cmd".into(),
        " ral export --all <name> ສົ່ງອອກ alias ທັງໝົດໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_import_header".into(),
        "ສົ່ງອອກ/ນຳເຂົ້າ:".into(),
    );

    m.insert(
        "alias.ral.export_name_empty".into(),
        "ຊື່ສົ່ງອອກບໍ່ສາມາດຫວ່າງເປົ່າໄດ້.".into(),
    );

    m.insert(
        "alias.ral.export_select_cmd".into(),
        " ral export --select <name> ເລືອກ alias ເພື່ອສົ່ງອອກໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert("alias.ral.file_exists".into(), "ໄຟລ໌ມີຢູ່ແລ້ວ: {}".into());

    m.insert(
        "alias.ral.import_cmd".into(),
        " ral import <file> ນຳເຂົ້າ alias ຈາກໄຟລ໌ .ntc.ral".into(),
    );

    m.insert(
        "alias.ral.info_cmd".into(),
        " ral info <name> [--edit] ສະແດງລາຍລະອຽດ alias (--edit ສຳເນົາຄຳສັ່ງແກ້ໄຂໃສ່ clipboard)".into(),
    );

    m.insert("alias.ral.info_command".into(), "ຄຳສັ່ງ".into());

    m.insert("alias.ral.info_name".into(), "ຊື່".into());

    m.insert("alias.ral.info_none".into(), "ບໍ່ມີ".into());

    m.insert("alias.ral.info_params".into(), "ພາຣາມິເຕີ".into());

    m.insert("alias.ral.info_hints".into(), "ຄຳใบ້ການเติม".into());

    m.insert("alias.ral.hints_saved".into(), "ຄຳใบ້ການเติม: {}".into());

    m.insert(
        "alias.ral.hint_error".into(),
        "ຄຳใบ້ຜິດ ({}): ບໍ່ໄດ້ບັນທຶກ alias".into(),
    );

    m.insert("alias.ral.hint_usage".into(), "ຄຳใบ້ໜ້າຕາແບບ: #x --hint-dir  #y --hint-tp --hint-dir  ເລືອກແບບ: #x --select-dir  #x --select-ext rs,txt  ຄ່າໂດຍກົງ: #x --hint [8080,5000]  ລາຍການທີ່ຕັ້ງຊື່: #x --hint-n port (ເບິ່ງ ral hintn)  ແຟລັກໃໝ່: --hint-ext-name [list] | --hint-file-name | --select-ext-name [list] | --select-file-name".into());

    m.insert(
        "alias.ral.hint_cmd".into(),
        " ral add <name> <command> #x --hint-dir complete param x as directory".into(),
    );

    m.insert(
        "alias.ral.select_cmd".into(),
        " ral add <name> <command> #x --select-dir pick x interactively when missing".into(),
    );

    m.insert(
        "alias.ral.hintn_cmd".into(),
        " ral hintn add|rm|rnm|info|ls manage named value lists for #x --hint-n <list>".into(),
    );

    m.insert("alias.ral.usage_hintn".into(), "Usage: ral hintn add <name> [values] | ral hintn rm <name>[, ...] [--force] | ral hintn rnm <old> to <new> | ral hintn info <name> | ral hintn ls".into());

    m.insert("alias.ral.usage_hintn_add".into(), "Usage: ral hintn add <name> [value, ...] (quote values with spaces: [\"what is this\", ok])".into());

    m.insert(
        "alias.ral.usage_hintn_rm".into(),
        "Usage: ral hintn rm <name>[, ...] [--force]".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_rnm".into(),
        "Usage: ral hintn rnm <old> to <new>".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_info".into(),
        "Usage: ral hintn info <name>".into(),
    );

    m.insert(
        "alias.ral.hintn_added".into(),
        "Hint list '{}' saved ({} values): {}".into(),
    );

    m.insert(
        "alias.ral.hintn_removed".into(),
        "Hint list '{}' removed.".into(),
    );

    m.insert(
        "alias.ral.hintn_not_found".into(),
        "Hint list '{}' not found.".into(),
    );

    m.insert(
        "alias.ral.hintn_renamed".into(),
        "Hint list '{}' renamed to '{}'.".into(),
    );

    m.insert(
        "alias.ral.hintn_empty".into(),
        "No hint lists defined. Use 'ral hintn add <name> [values]'.".into(),
    );

    m.insert(
        "alias.ral.hintn_referenced".into(),
        "Hint list '{}' is used by: {}. Use --force to remove anyway.".into(),
    );

    m.insert(
        "alias.ral.hintn_info_title".into(),
        "📌 Hint List Info".into(),
    );

    m.insert("alias.ral.hintn_info_name".into(), "Name".into());

    m.insert("alias.ral.hintn_info_values".into(), "Values".into());

    m.insert("alias.ral.hintn_info_used_by".into(), "Used by".into());

    m.insert("alias.ral.hintn_value_count".into(), "{} values".into());

    m.insert(
        "alias.ral.unknown_hint_list".into(),
        "Unknown hint list: '{}'.".into(),
    );

    m.insert(
        "alias.ral.hintn_create_hint".into(),
        "Create it first: ral hintn add {} [values]".into(),
    );

    m.insert(
        "alias.ral.arr_sep_error".into(),
        "error: {} have {} char in the array {} try $x[,] or $x[c]".into(),
    );

    m.insert(
        "alias.ral.example_hint_ext".into(),
        "ral add opimg \"image display ${x}\" #x --hint-ext png,jpeg,jpg,ico,bmp".into(),
    );

    m.insert(
        "alias.ral.example_rest".into(),
        "ral add hi \"echo $*{x}\"    # hi \"wow nice\" -> echo wow nice".into(),
    );

    m.insert(
        "alias.ral.example_hint_array".into(),
        "ral add runc \"gcc -o ${o}.exe $x[] && .\\${o}.exe && rmf ${o}.exe\" #x[] --hint-ext c"
            .into(),
    );

    m.insert(
        "alias.ral.example_select_dir".into(),
        "ral add tldsel \"tl dir ${x:-.}\" #x --select-dir".into(),
    );

    m.insert(
        "alias.ral.example_hint_values".into(),
        "ral add srv \"ssh $p\" #p --hint [8080,5050,5000]".into(),
    );

    m.insert(
        "alias.ral.example_hintn".into(),
        "ral hintn add port [8080,5050,5000] + ral add srvn \"ssh $p\" #p --hint-n port".into(),
    );

    m.insert(
        "alias.ral.example_hint_sep".into(),
        "array join: $x[,] over [a,b] gives \"a , b\"".into(),
    );

    m.insert("alias.ral.info_source".into(), "ແຫຼ່ງທີ່ມາ".into());

    m.insert("alias.ral.info_source_global".into(), "global".into());

    m.insert(
        "alias.ral.info_source_local".into(),
        "local (ntconfig.toml)".into(),
    );

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} ຂໍ້ມູນ Alias".into());

    m.insert(
        "alias.ral.info_edit_copied".into(),
        "ສຳເນົາຄຳສັ່ງແກ້ໄຂໃສ່ clipboard ແລ້ວ:".into(),
    );

    m.insert(
        "alias.ral.info_edit_hint".into(),
        "ວາງມັນ, ປັບຄຳສັ່ງ, ແລ້ວກົດ Enter.".into(),
    );

    m.insert(
        "alias.ral.invalid_name".into(),
        "ຊື່ alias ບໍ່ຖືກຕ້ອງ: '{}'".into(),
    );

    m.insert("alias.ral.invalid_suffix".into(), "{} (ບໍ່ຖືກຕ້ອງ)".into());

    m.insert(
        "alias.ral.list_cmd".into(),
        " ral list ສະແດງ alias ທັງໝົດ".into(),
    );

    m.insert(
        "alias.ral.make_cmd".into(),
        " ral make [name] ສ້າງ template alias ໃນ CWD".into(),
    );

    m.insert("alias.ral.name_rule_at".into(), " - ເລີ່ມດ້ວຍ @ ຫຼື #".into());

    m.insert(
        "alias.ral.name_rule_reserved".into(),
        " - ເປັນຄຳສັ່ງສະຫງວນ (go, view, txt, ແລະອື່ນໆ)".into(),
    );

    m.insert("alias.ral.name_rules".into(), "ຊື່ alias ບໍ່ສາມາດ:".into());

    m.insert("alias.ral.no_aliases".into(), "ບໍ່ມີ run alias ໃຫ້ລຶບ.".into());

    m.insert("alias.ral.not_found".into(), "ບໍ່ພົບ alias '{}'".into());

    m.insert("alias.ral.now_run".into(), "ຕອນນີ້ທ່ານສາມາດເຮັດໄດ້: {}".into());

    m.insert(
        "alias.ral.now_run_arg".into(),
        "ຕອນນີ້ທ່ານສາມາດເຮັດໄດ້: {}({})".into(),
    );

    m.insert(
        "alias.ral.now_run_defaults".into(),
        "ຕອນນີ້ທ່ານສາມາດເຮັດໄດ້: {}(<arg>) (ຄ່າເລີ່ມຕົ້ນ: {})".into(),
    );

    m.insert(
        "alias.ral.param_cmd".into(),
        " ral add <name>(x) <command> ສ້າງ alias ແບບມີພາຣາມິເຕີ (ໃຊ້ $x ຫຼື ${{x}} ໃນຄຳສັ່ງ)".into(),
    );

    m.insert(
        "alias.ral.rest_cmd".into(),
        " ral add <name>(x) <command> ໃຊ້ $*{x}  ຈັບໜຶ່ງອາກິວເມັນ (ເກັບຊ່ອງວ່າງ, ຕັດເຄື່ອງໝາຍຄຳເວົ້າ)".into(),
    );

    m.insert(
        "alias.ral.renamed".into(),
        "ປ່ຽນຊື່ alias '{}' ເປັນ '{}'".into(),
    );

    m.insert(
        "alias.ral.rm_cmd".into(),
        " ral rm <name>[, <name>, ...] ລຶບ alias".into(),
    );

    m.insert(
        "alias.ral.rnm_cmd".into(),
        " ral rnm <old> to <new> ປ່ຽນຊື່ alias".into(),
    );

    m.insert(
        "alias.ral.template_created".into(),
        "ສ້າງ ral template ແລ້ວ: {}".into(),
    );

    m.insert(
        "alias.ral.template_info".into(),
        "ນີ້ແມ່ນໄຟລ໌ template ທ້ອງຖິ່ນ. ໃຊ້ 'ral import' ເພື່ອເພີ່ມ alias ໃສ່ການຕັ້ງຄ່າຂອງທ່ານ.".into(),
    );

    m.insert("alias.ral.title".into(), "ຄຳສັ່ງ Run Alias (ral):".into());

    m.insert(
        "alias.ral.type_ral_help".into(),
        "ພິມ 'ral' ເພື່ອເບິ່ງຄວາມຊ່ວຍເຫຼືອ.".into(),
    );

    m.insert(
        "alias.ral.unknown_flag".into(),
        "ແທັກສົ່ງອອກບໍ່ຮູ້ຈັກ: {}. ໃຊ້ --all ຫຼື --select".into(),
    );

    m.insert(
        "alias.ral.unknown_subcmd".into(),
        "ບໍ່ຮູ້ຈັກ ral subcommand: {}".into(),
    );

    m.insert(
        "alias.ral.usage_add".into(),
        "ວິທີໃຊ້: ral add <name> <command>".into(),
    );

    m.insert(
        "alias.ral.usage_edit".into(),
        "ວິທີໃຊ້: ral edit <name> <new_command>".into(),
    );

    m.insert(
        "alias.ral.usage_export".into(),
        "ວິທີໃຊ້: ral export --all <name> | ral export --select <name>".into(),
    );

    m.insert(
        "alias.ral.usage_hint".into(),
        "ວິທີໃຊ້: <alias> ຫຼື <alias>(<arg>) ສຳລັບ alias ແບບມີພາຣາມິເຕີ".into(),
    );

    m.insert(
        "alias.ral.usage_import".into(),
        "ວິທີໃຊ້: ral import <file>".into(),
    );

    m.insert(
        "alias.ral.usage_info".into(),
        "ວິທີໃຊ້: ral info <alias> [--edit]".into(),
    );

    m.insert(
        "alias.ral.usage_rm".into(),
        "ວິທີໃຊ້: ral rm <name>[, <name>, ...]".into(),
    );

    m.insert(
        "alias.ral.usage_rnm".into(),
        "ວິທີໃຊ້: ral rnm <old_name> to <new_name>".into(),
    );

    m.insert("alias.ral.usage_with_run".into(), "ວິທີໃຊ້ກັບ run:".into());

    m.insert(
        "alias.ral.run_example1".into(),
        "  run btr            # ເຮັດວຽກ: cargo build --release".into(),
    );

    m.insert(
        "alias.ral.run_example2".into(),
        "  run py             # ເຮັດວຽກ: python test.py".into(),
    );

    m.insert(
        "alias.ral.run_example3".into(),
        "  run_file(hello)    # ເຮັດວຽກ: python hello.py".into(),
    );

    m.insert("alias.ral.run_example4".into(), "  runc [add , minus , mul] math  # ເຮັດວຽກ: cls && gcc -o math.exe add.c minus.c mul.c && ...".into());

    m.insert(
        "alias.ral.export_example1".into(),
        "  ral export --all myaliases      # ສ້າງ myaliases.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_example2".into(),
        "  ral export --select myaliases   # ເລືອກ ແລະ ສົ່ງອອກແບບ interactive".into(),
    );

    m.insert(
        "alias.ral.export_example3".into(),
        "  ral import myaliases.ntc.ral    # ນຳເຂົ້າໄປໃນການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert("alias.run.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "alias.run.example_alias".into(),
        "  run btr                   # ເຮັດວຽກ alias".into(),
    );

    m.insert(
        "alias.run.example_alias_args".into(),
        "  run py test.py            # ເຮັດວຽກ alias ພ້ອມ args".into(),
    );

    m.insert(
        "alias.run.example_param".into(),
        "  run_file(hello)           # ເຮັດວຽກ alias ແບບມີພາຣາມິເຕີ".into(),
    );

    m.insert(
        "alias.run.example_real".into(),
        "  run python --version      # ເຮັດວຽກຄຳສັ່ງຈິງ".into(),
    );

    m.insert(
        "alias.run.usage".into(),
        "ວິທີໃຊ້: run <command|alias> [args...]".into(),
    );

    m.insert("alias.ral.section_global".into(), "[global]".into());

    m.insert("alias.ral.section_local".into(), "[local]".into());

    m.insert(
        "alias.ral.no_local".into(),
        "(ບໍ່ມີ config local ໃນໂຟນ໌ດີ້ນີ້)".into(),
    );

    m.insert("alias.ral.none".into(), "(ບໍ່ມີ)".into());

    m.insert("alias.ral.shadowed".into(), "(ຖືກ local ບ່ງມາບັງ)".into());

    m.insert(
        "alias.ral.shadowed_global".into(),
        "ລາຍກาร Global (ຖືກ local ບ່ງມາບັງ):".into(),
    );

    m.insert(
        "alias.ral.global_active_again".into(),
        "alias global '{}' ຍັງມີອູ່ ແລະໃຊ້ງານໄດ້ແລ້ວ".into(),
    );

    m.insert(
        "alias.ral.global_only_hint".into(),
        "alias '{}' ຖືກກຳນົດໄວ້ໃນ config global ເທົ່ານັ້ນ ໃຊ້ 'ral rm -g {}' ເພື່ອລຶກຈາກ config global".into(),
    );

    m.insert(
        "alias.ral.local_priority_note".into(),
        "ຫມາຍເຫັນ: alias local '{}' ໃນ ntconfig.toml ມີຄວາມສຳຄັນກວ່າ alias global ນີ້".into(),
    );

    m.insert("alias.ral.scope_local".into(), "config local".into());

    m.insert("alias.ral.scope_global".into(), "config global".into());

    m.insert("alias.ral.cls_target".into(), "ເປົ້າຫມາຍ: {}".into());
}
