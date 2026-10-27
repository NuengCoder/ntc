use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.nsheet.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.nsheet.help_open".into(),
        "nsheet open <file> [options]   Fullscreen viewer (CSV/TSV/JSON/JSONL)".into(),
    );

    m.insert(
        "cmd.nsheet.help_view".into(),
        "nsheet view <file> [options]   Print table (default 50 rows, --all for all)".into(),
    );

    m.insert("cmd.nsheet.options_title".into(), "Options:".into());

    m.insert(
        "cmd.nsheet.help_delimiter".into(),
        "  -d, --delimiter <c|auto>  Field separator (default: auto-detect , ; tab |)".into(),
    );

    m.insert(
        "cmd.nsheet.help_header".into(),
        "  --header / --no-header      Force header row on/off (default: auto-detect)".into(),
    );

    m.insert(
        "cmd.nsheet.help_limit".into(),
        "  -l, --limit <n> / --all      Rows to print in view mode".into(),
    );

    m.insert(
        "cmd.nsheet.help_sort".into(),
        "  --sort <col> [--desc]         Sort by column name or 1-based index".into(),
    );

    m.insert(
        "cmd.nsheet.help_filter".into(),
        "  --filter <expr>              e.g. age>30, city~york, name=bob (!=,>=,<=,&&,||)".into(),
    );

    m.insert(
        "cmd.nsheet.help_export".into(),
        "  --export <json|md>           Export instead of printing".into(),
    );

    m.insert(
        "cmd.nsheet.help_output".into(),
        "  -o, --output <file>          Export destination (default: stdout)".into(),
    );

    m.insert(
        "cmd.nsheet.help_help".into(),
        "  --help, -h                   Show this help".into(),
    );

    m.insert("cmd.nsheet.examples_title".into(), "Examples:".into());

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
        "Missing file. Usage: nsheet open <file> | nsheet view <file>".into(),
    );

    m.insert(
        "cmd.nsheet.file_not_found".into(),
        "File '{}' not found".into(),
    );

    m.insert(
        "cmd.nsheet.load_fail".into(),
        "Cannot load sheet: {}".into(),
    );

    m.insert(
        "cmd.nsheet.empty_sheet".into(),
        "Empty sheet (no columns found)".into(),
    );

    m.insert("cmd.nsheet.no_rows".into(), "No rows match.".into());

    m.insert("cmd.nsheet.unknown_option".into(), "Unknown: {}".into());

    m.insert(
        "cmd.nsheet.unknown_column".into(),
        "Unknown column: '{}'".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_limit".into(),
        "Invalid limit: '{}'".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_delimiter".into(),
        "Invalid delimiter: '{}' (use a single char or auto)".into(),
    );

    m.insert(
        "cmd.nsheet.invalid_export".into(),
        "Invalid export format: '{}' (use json or md)".into(),
    );

    m.insert(
        "cmd.nsheet.delimiter_usage".into(),
        "Usage: --delimiter <char|auto>".into(),
    );

    m.insert("cmd.nsheet.limit_usage".into(), "Usage: --limit <n>".into());

    m.insert(
        "cmd.nsheet.sort_usage".into(),
        "Usage: --sort <column>".into(),
    );

    m.insert(
        "cmd.nsheet.filter_usage".into(),
        "Usage: --filter <expr> (e.g. age>30)".into(),
    );

    m.insert(
        "cmd.nsheet.export_usage".into(),
        "Usage: --export <json|md>".into(),
    );

    m.insert(
        "cmd.nsheet.output_usage".into(),
        "Usage: --output <file>".into(),
    );

    m.insert("cmd.nsheet.filter_fail".into(), "Bad filter: {}".into());

    m.insert("cmd.nsheet.export_fail".into(), "Export failed: {}".into());

    m.insert("cmd.nsheet.exported".into(), "Exported: {}".into());

    m.insert(
        "cmd.nsheet.rows_shown".into(),
        "Showing {} of {} rows ({} cols)".into(),
    );

    m.insert(
        "cmd.nsheet.rows_truncated".into(),
        "Truncated to {} rows (use --all or -l <n>)".into(),
    );

    m.insert("cmd.nsheet.tui_hint".into(), "arrows/hjkl move · / search · n next · s sort · f filter · c clear · e export · Enter view · q quit".into());

    m.insert(
        "cmd.nsheet.tui_search_prompt".into(),
        "search (empty clears)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_sort_prompt".into(),
        "sort by col (! suffix = desc, empty clears)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_filter_prompt".into(),
        "filter (empty clears)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_export_prompt".into(),
        "export to (.json/.md path)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_cell_title".into(),
        "Cell (any key to close)".into(),
    );

    m.insert(
        "cmd.nsheet.tui_popup_hint".into(),
        "any key to close".into(),
    );

    m.insert("cmd.nsheet.tui_no_matches".into(), "No matches".into());

    m.insert(
        "cmd.nsheet.tui_large".into(),
        "Large sheet ({} rows) — navigation may be slow".into(),
    );
}
