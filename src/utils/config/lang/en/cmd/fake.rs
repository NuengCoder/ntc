use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.fake.usage_title".into(),
        "fake - generate fake/random data".into(),
    );

    m.insert(
        "cmd.fake.usage_gen".into(),
        "  fake gen <types> [-n N] [-f fmt] [-s seed] [-o file] [-g group] [-l locale] [-u [cols]]"
            .into(),
    );

    m.insert(
        "cmd.fake.usage_types".into(),
        "  fake types [-s] [pattern]      List every type (fuzzy search with -s)".into(),
    );

    m.insert(
        "cmd.fake.usage_locales".into(),
        "  fake locales                   List the supported --locale codes".into(),
    );

    m.insert(
        "cmd.fake.usage_group".into(),
        "  fake group <sub>               Manage saved column groups".into(),
    );

    m.insert(
        "cmd.fake.usage_help".into(),
        "  fake help                      Show this help".into(),
    );

    m.insert(
        "cmd.fake.options_title".into(),
        "Options (fake gen):".into(),
    );

    m.insert(
        "cmd.fake.opt_count".into(),
        "  -n, --count <N>       Rows to generate (default 1)".into(),
    );

    m.insert(
        "cmd.fake.opt_format".into(),
        "  -f, --format <fmt>    json | txt | sql | csv | xlsx (default txt)".into(),
    );

    m.insert(
        "cmd.fake.opt_seed".into(),
        "  -s, --seed <N>        Seed for reproducible output".into(),
    );

    m.insert(
        "cmd.fake.opt_output".into(),
        "  -o, --output <file>   Write to a file (extension picks the format)".into(),
    );

    m.insert(
        "cmd.fake.opt_group".into(),
        "  -g, --group <name>    Generate from a saved group instead of <types>".into(),
    );

    m.insert(
        "cmd.fake.opt_locale".into(),
        "  -l, --locale <code>   Locale pools: en_US en_GB zh_CN th_TH lo_LA".into(),
    );

    m.insert(
        "cmd.fake.opt_unique".into(),
        "  -u, --unique [cols]   Make the whole row (or the named cols) unique".into(),
    );

    m.insert("cmd.fake.examples_header".into(), "Examples:".into());

    m.insert(
        "cmd.fake.example1".into(),
        "  fake gen name,email,city -n 3".into(),
    );

    m.insert(
        "cmd.fake.example2".into(),
        "  fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "cmd.fake.example3".into(),
        "  fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.example4".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.unknown_sub".into(),
        "Unknown fake subcommand: '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.wrote".into(),
        "Wrote {} row(s) to {} ({})".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_ext".into(),
        "Unknown output extension '{}' (use -f to set the format)".into(),
    );

    m.insert(
        "cmd.fake.gen.xlsx_needs_output".into(),
        "xlsx output needs -o <file.xlsx> (a binary workbook cannot be printed to the terminal)"
            .into(),
    );

    m.insert(
        "cmd.fake.gen.count_usage".into(),
        "fake gen: -n/--count needs a number".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_count".into(),
        "fake gen: invalid count '{}' (must be a positive integer)".into(),
    );

    m.insert(
        "cmd.fake.gen.format_usage".into(),
        "fake gen: -f/--format needs a value".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_format".into(),
        "fake gen: unknown format '{}' (json|txt|sql|csv|xlsx)".into(),
    );

    m.insert(
        "cmd.fake.gen.seed_usage".into(),
        "fake gen: -s/--seed needs a number".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_seed".into(),
        "fake gen: invalid seed '{}' (must be an integer)".into(),
    );

    m.insert(
        "cmd.fake.gen.output_usage".into(),
        "fake gen: -o/--output needs a file path".into(),
    );

    m.insert(
        "cmd.fake.gen.group_usage".into(),
        "fake gen: -g/--group needs a group name".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_flag".into(),
        "fake gen: unknown option '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.group_and_types".into(),
        "fake gen: <types> and -g/--group cannot be used together".into(),
    );

    m.insert(
        "cmd.fake.gen.need_input".into(),
        "fake gen: give one or more types, or use -g/--group".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_type".into(),
        "fake gen: unknown type '{}' (run 'fake types' to list them)".into(),
    );

    m.insert("cmd.fake.gen.unique_with_group".into(), "fake gen: --unique cannot be used with -g/--group (mark columns with a trailing '!' in the group instead)".into());

    m.insert(
        "cmd.fake.gen.unique_unknown_col".into(),
        "fake gen: --unique names unknown column '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_exhausted".into(),
        "fake gen: could not generate a unique value for '{}' (the value space is too small)"
            .into(),
    );

    m.insert(
        "cmd.fake.gen.locale_usage".into(),
        "fake gen: -l/--locale needs a locale code (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_locale".into(),
        "fake gen: unknown locale '{}' (en_US en_GB zh_CN th_TH lo_LA)".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_cycle".into(),
        "fake gen: circular group reference through '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_unknown_col".into(),
        "fake gen: referenced column '{}' not found in group '{}'".into(),
    );

    m.insert(
        "cmd.fake.types.title".into(),
        "Available fake types ({}):".into(),
    );

    m.insert("cmd.fake.types.none".into(), "No type matches '{}'.".into());

    m.insert(
        "cmd.fake.types.matches".into(),
        "Types matching '{}' ({}):".into(),
    );

    m.insert(
        "cmd.fake.locales.title".into(),
        "Supported locales ({}):".into(),
    );

    m.insert(
        "cmd.fake.locales.note".into(),
        "Use with: fake gen <types> --locale <code>".into(),
    );

    m.insert(
        "cmd.fake.group.usage_title".into(),
        "fake group - manage saved column groups:".into(),
    );

    m.insert(
        "cmd.fake.group.help_add".into(),
        "  fake group add <name> [alias as type, ...]    Create a group".into(),
    );

    m.insert(
        "cmd.fake.group.help_edit".into(),
        "  fake group edit <name> [alias as type, ...]   Replace a group's columns".into(),
    );

    m.insert(
        "cmd.fake.group.help_rm".into(),
        "  fake group rm <name>                          Delete a group".into(),
    );

    m.insert(
        "cmd.fake.group.help_rnm".into(),
        "  fake group rnm <old> <new>                    Rename a group".into(),
    );

    m.insert(
        "cmd.fake.group.help_ls".into(),
        "  fake group ls [pattern]                       List groups (glob, e.g. u*)".into(),
    );

    m.insert(
        "cmd.fake.group.help_info".into(),
        "  fake group info <name>                        Show a group's columns".into(),
    );

    m.insert(
        "cmd.fake.group.help_clear".into(),
        "  fake group clear [-f]                         Delete every group".into(),
    );

    m.insert(
        "cmd.fake.group.help_export".into(),
        "  fake group export <name>|--all [-o file]       Share groups as .ntc_fake".into(),
    );

    m.insert(
        "cmd.fake.group.help_import".into(),
        "  fake group import <file> [--force] [--prefix p]  Load groups from a file".into(),
    );

    m.insert(
        "cmd.fake.group.example1".into(),
        "  fake group add users [username as name, email, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.example2".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_sub".into(),
        "Unknown 'fake group' subcommand: '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.no_columns".into(),
        "give at least one column, e.g. [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_column".into(),
        "could not parse column '{}' (expected 'alias as type')".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_type".into(),
        "unknown type '{}' (run 'fake types' to list them)".into(),
    );

    m.insert(
        "cmd.fake.group.add_usage".into(),
        "Usage: fake group add <name> [alias as type, ...]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_name".into(),
        "invalid group name '{}' (use letters, digits, '_' or '-')".into(),
    );

    m.insert(
        "cmd.fake.group.not_found".into(),
        "no fake group named '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.exists".into(),
        "fake group '{}' already exists".into(),
    );

    m.insert("cmd.fake.group.scope_local".into(), "local".into());

    m.insert("cmd.fake.group.scope_global".into(), "global".into());

    m.insert(
        "cmd.fake.group.added".into(),
        "Added group '{}' with {} column(s) to {} config".into(),
    );

    m.insert(
        "cmd.fake.group.updated".into(),
        "Updated group '{}' with {} column(s) in {} config".into(),
    );

    m.insert(
        "cmd.fake.group.rm_usage".into(),
        "Usage: fake group rm <name>".into(),
    );

    m.insert("cmd.fake.group.removed".into(), "Removed group '{}'".into());

    m.insert(
        "cmd.fake.group.rnm_usage".into(),
        "Usage: fake group rnm <old> <new>".into(),
    );

    m.insert(
        "cmd.fake.group.renamed".into(),
        "Renamed group '{}' to '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.ls_empty".into(),
        "No fake groups saved yet.".into(),
    );

    m.insert(
        "cmd.fake.group.ls_none".into(),
        "No fake group matches '{}'.".into(),
    );

    m.insert(
        "cmd.fake.group.info_usage".into(),
        "Usage: fake group info <name>".into(),
    );

    m.insert("cmd.fake.group.info_title".into(), "Group '{}':".into());

    m.insert("cmd.fake.group.info_count".into(), "{} column(s)".into());

    m.insert(
        "cmd.fake.group.clear_empty".into(),
        "No fake groups to clear.".into(),
    );

    m.insert(
        "cmd.fake.group.clear_prompt".into(),
        "Remove {} fake group(s)? [y/N] ".into(),
    );

    m.insert("cmd.fake.group.clear_cancelled".into(), "Cancelled.".into());

    m.insert(
        "cmd.fake.group.cleared".into(),
        "Cleared {} fake group(s).".into(),
    );

    m.insert(
        "cmd.fake.group.bad_ref".into(),
        "invalid reference '{}' (expected ref:<group>.<column>)".into(),
    );

    m.insert(
        "cmd.fake.group.ref_col_missing".into(),
        "referenced column '{}' not found in group '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.export_usage".into(),
        "Usage: fake group export <name>|--all [-o file]".into(),
    );

    m.insert(
        "cmd.fake.group.import_usage".into(),
        "Usage: fake group import <file> [--force] [--prefix p]".into(),
    );

    m.insert(
        "cmd.fake.group.exported".into(),
        "Exported {} group(s) to {}".into(),
    );

    m.insert(
        "cmd.fake.group.import_ok".into(),
        "Imported {} group(s).".into(),
    );

    m.insert(
        "cmd.fake.group.import_exists".into(),
        "group(s) already exist: {} (use --force to overwrite)".into(),
    );

    m.insert(
        "cmd.fake.group.import_bad_file".into(),
        "could not import groups: {}".into(),
    );
}
