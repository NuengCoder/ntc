use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "alias.ral.add_cmd".into(),
        " ral add <name> <command> Create a new run alias".into(),
    );

    m.insert(
        "alias.ral.alias_count".into(),
        "You have {} alias(es) defined:".into(),
    );

    m.insert("alias.ral.and_more".into(), "... and {} more".into());

    m.insert("alias.ral.array_cmd".into(), " ral add <name>(x[],y) <cmd> Create alias with array params (use $x[].ext or ${{x}}[].ext)".into());

    m.insert("alias.ral.cls_cancelled".into(), "Clear cancelled.".into());

    m.insert(
        "alias.ral.cls_cmd".into(),
        " ral cls Clear ALL aliases (asks confirmation)".into(),
    );

    m.insert(
        "alias.ral.cls_prompt".into(),
        "Are you sure? Type 'yes' to confirm: ".into(),
    );

    m.insert(
        "alias.ral.cls_warning".into(),
        "\u{26A0}\u{FE0F} WARNING: This will delete ALL run aliases!".into(),
    );

    m.insert("alias.ral.config_path".into(), " Config: {}".into());

    m.insert(
        "alias.ral.create_failed".into(),
        "Failed to create file: {}".into(),
    );

    m.insert(
        "alias.ral.default_cmd".into(),
        " ral add <name>({{x=a,e=txt}}) <cmd> Create alias with default values".into(),
    );

    m.insert(
        "alias.ral.edit_cmd".into(),
        " ral edit <name> <command> Update an existing alias".into(),
    );

    m.insert(
        "alias.ral.empty".into(),
        "No run aliases defined. Use 'ral add <name> <command>'".into(),
    );

    m.insert("alias.ral.examples_header".into(), "Examples:".into());

    m.insert(
        "alias.ral.export_all_cmd".into(),
        " ral export --all <name> Export all aliases to <name>.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_import_header".into(),
        "Export/Import:".into(),
    );

    m.insert(
        "alias.ral.export_name_empty".into(),
        "Export name cannot be empty.".into(),
    );

    m.insert(
        "alias.ral.export_select_cmd".into(),
        " ral export --select <name> Select aliases to export to <name>.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.file_exists".into(),
        "File already exists: {}".into(),
    );

    m.insert(
        "alias.ral.import_cmd".into(),
        " ral import <file> Import aliases from a .ntc.ral file".into(),
    );

    m.insert("alias.ral.info_cmd".into(), " ral info <name> [--edit] Show alias details (--edit copies the edit command to clipboard)".into());

    m.insert("alias.ral.info_command".into(), "Command".into());

    m.insert("alias.ral.info_name".into(), "Name".into());

    m.insert("alias.ral.info_none".into(), "none".into());

    m.insert("alias.ral.info_params".into(), "Parameters".into());

    m.insert("alias.ral.info_hints".into(), "Completion hints".into());

    m.insert(
        "alias.ral.hints_saved".into(),
        "Completion hints: {}".into(),
    );

    m.insert(
        "alias.ral.hint_error".into(),
        "Bad completion hints ({}): alias not saved.".into(),
    );

    m.insert("alias.ral.hint_usage".into(), "Hints look like: #x --hint-dir  #y --hint-tp --hint-dir  (--hint-tp|--hint-ext [list]|--hint-ext-name [list]|--hint-spf|--hint-file|--hint-file-name|--hint-dir|--hint-fd|--hint-all)  Selects look like: #x --select-dir  #x --select-ext rs,txt  (--select-dir|--select-ext [list]|--select-ext-name [list]|--select-file|--select-file-name|--select-fd|--select-tp)  Inline values: #x --hint [8080,5000]  Named lists: #x --hint-n port (see ral hintn)".into());

    m.insert(
        "alias.ral.hint_cmd".into(),
        " ral add <name> <command> #x --hint-dir  Complete param x as a directory".into(),
    );

    m.insert(
        "alias.ral.select_cmd".into(),
        " ral add <name> <command> #x --select-dir  Prompt to pick param x when missing".into(),
    );

    m.insert(
        "alias.ral.hintn_cmd".into(),
        " ral hintn add|rm|rnm|info|ls  Manage named value lists for #x --hint-n <list>".into(),
    );

    m.insert("alias.ral.usage_hintn".into(), "Usage: ral hintn add <name> [values] | ral hintn rm <name>[, ...] [--force] | ral hintn rnm <old> to <new> | ral hintn info <name> | ral hintn ls".into());

    m.insert("alias.ral.usage_hintn_add".into(), "Usage: ral hintn add <name> [value, ...]  (quote values with spaces: [\"what is this\", ok])".into());

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
        "array join: $x[,] over [a,b] gives \"a , b\" (sep + suffix per element: $x[,].c)".into(),
    );

    m.insert("alias.ral.info_source".into(), "Source".into());

    m.insert("alias.ral.info_source_global".into(), "global".into());

    m.insert(
        "alias.ral.info_source_local".into(),
        "local (ntconfig.toml)".into(),
    );

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} Alias Info".into());

    m.insert(
        "alias.ral.info_edit_copied".into(),
        "Edit command copied to clipboard:".into(),
    );

    m.insert(
        "alias.ral.info_edit_hint".into(),
        "Paste it, tweak the command, then press Enter.".into(),
    );

    m.insert(
        "alias.ral.invalid_name".into(),
        "Invalid alias name: '{}'".into(),
    );

    m.insert("alias.ral.invalid_suffix".into(), "{} (INVALID)".into());

    m.insert(
        "alias.ral.list_cmd".into(),
        " ral list Show all aliases".into(),
    );

    m.insert(
        "alias.ral.make_cmd".into(),
        " ral make [name] Create alias template in CWD".into(),
    );

    m.insert(
        "alias.ral.name_rule_at".into(),
        " - Start with @ or #".into(),
    );

    m.insert(
        "alias.ral.name_rule_reserved".into(),
        " - Be a reserved command (go, view, txt, etc.)".into(),
    );

    m.insert("alias.ral.name_rules".into(), "Alias names cannot:".into());

    m.insert(
        "alias.ral.no_aliases".into(),
        "No run aliases to clear.".into(),
    );

    m.insert("alias.ral.not_found".into(), "Alias '{}' not found".into());

    m.insert("alias.ral.now_run".into(), "Now you can run: {}".into());

    m.insert(
        "alias.ral.now_run_arg".into(),
        "Now you can run: {}({})".into(),
    );

    m.insert(
        "alias.ral.now_run_defaults".into(),
        "Now you can run: {}(<arg>) (defaults: {})".into(),
    );

    m.insert(
        "alias.ral.param_cmd".into(),
        " ral add <name>(x) <command> Create parameterised alias (use $x or ${{x}} in command)"
            .into(),
    );

    m.insert(
        "alias.ral.rest_cmd".into(),
        " ral add <name>(x) <command> using $*{x}  Capture one arg (spaces kept, quotes stripped)"
            .into(),
    );

    m.insert(
        "alias.ral.renamed".into(),
        "Renamed alias '{}' to '{}'".into(),
    );

    m.insert(
        "alias.ral.rm_cmd".into(),
        " ral rm <name>[, <name>, ...] Remove alias(es)".into(),
    );

    m.insert(
        "alias.ral.rnm_cmd".into(),
        " ral rnm <old> to <new> Rename an alias".into(),
    );

    m.insert(
        "alias.ral.template_created".into(),
        "Created ral template: {}".into(),
    );

    m.insert(
        "alias.ral.template_info".into(),
        "This is a local template file. Use 'ral import' to add aliases to your config.".into(),
    );

    m.insert("alias.ral.title".into(), "Run Alias (ral) Commands:".into());

    m.insert(
        "alias.ral.type_ral_help".into(),
        "Type 'ral' for help.".into(),
    );

    m.insert(
        "alias.ral.unknown_flag".into(),
        "Unknown export flag: {}. Use --all or --select".into(),
    );

    m.insert(
        "alias.ral.unknown_subcmd".into(),
        "Unknown ral subcommand: {}".into(),
    );

    m.insert(
        "alias.ral.usage_add".into(),
        "Usage: ral add <name> <command>".into(),
    );

    m.insert(
        "alias.ral.usage_edit".into(),
        "Usage: ral edit <name> <new_command>".into(),
    );

    m.insert(
        "alias.ral.usage_export".into(),
        "Usage: ral export --all <name> | ral export --select <name>".into(),
    );

    m.insert(
        "alias.ral.usage_hint".into(),
        "Usage: <alias> or <alias>(<arg>) for parameterised aliases".into(),
    );

    m.insert(
        "alias.ral.usage_import".into(),
        "Usage: ral import <file>".into(),
    );

    m.insert(
        "alias.ral.usage_info".into(),
        "Usage: ral info <alias> [--edit]".into(),
    );

    m.insert(
        "alias.ral.usage_rm".into(),
        "Usage: ral rm <name>[, <name>, ...]".into(),
    );

    m.insert(
        "alias.ral.usage_rnm".into(),
        "Usage: ral rnm <old_name> to <new_name>".into(),
    );

    m.insert("alias.ral.usage_with_run".into(), "Usage with run:".into());

    m.insert(
        "alias.ral.run_example1".into(),
        "  run btr              # Executes: cargo build --release".into(),
    );

    m.insert(
        "alias.ral.run_example2".into(),
        "  run py               # Executes: python test.py".into(),
    );

    m.insert(
        "alias.ral.run_example3".into(),
        "  run_file(hello)      # Executes: python hello.py".into(),
    );

    m.insert("alias.ral.run_example4".into(), "  runc [add , minus , mul] math  # Executes: cls && gcc -o math.exe add.c minus.c mul.c && ...".into());

    m.insert(
        "alias.ral.export_example1".into(),
        "  ral export --all myaliases       # Creates myaliases.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_example2".into(),
        "  ral export --select myaliases    # Interactive pick & export".into(),
    );

    m.insert(
        "alias.ral.export_example3".into(),
        "  ral import myaliases.ntc.ral     # Import into current config".into(),
    );

    m.insert("alias.run.examples_header".into(), "Examples:".into());

    m.insert(
        "alias.run.example_alias".into(),
        "  run btr                     # Run alias".into(),
    );

    m.insert(
        "alias.run.example_alias_args".into(),
        "  run py test.py              # Run alias with args".into(),
    );

    m.insert(
        "alias.run.example_param".into(),
        "  run_file(hello)             # Run parameterised alias".into(),
    );

    m.insert(
        "alias.run.example_real".into(),
        "  run python --version        # Run real command".into(),
    );

    m.insert(
        "alias.run.usage".into(),
        "Usage: run <command|alias> [args...]".into(),
    );

    m.insert("alias.ral.section_global".into(), "[global]".into());

    m.insert("alias.ral.section_local".into(), "[local]".into());

    m.insert(
        "alias.ral.no_local".into(),
        "(no local config in this directory)".into(),
    );

    m.insert("alias.ral.none".into(), "(none)".into());

    m.insert("alias.ral.shadowed".into(), "(shadowed by local)".into());

    m.insert(
        "alias.ral.shadowed_global".into(),
        "Global entry (shadowed by local):".into(),
    );

    m.insert(
        "alias.ral.global_active_again".into(),
        "The global alias '{}' is still defined and is active again now.".into(),
    );

    m.insert("alias.ral.global_only_hint".into(), "Alias '{}' is only defined in the global config. Use 'ral rm -g {}' to remove it from the global config.".into());

    m.insert(
        "alias.ral.local_priority_note".into(),
        "Note: the local alias '{}' in ntconfig.toml takes priority over this global alias.".into(),
    );

    m.insert("alias.ral.scope_local".into(), "local config".into());

    m.insert("alias.ral.scope_global".into(), "global config".into());

    m.insert("alias.ral.cls_target".into(), "Target: {}".into());
}
