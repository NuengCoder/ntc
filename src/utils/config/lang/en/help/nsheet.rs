use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nsheet.title".into(), "NSHEET COMMANDS:".into());

    m.insert(
        "help.nsheet.open".into(),
        "nsheet open <file> [options] ## Fullscreen sheet viewer (CSV/TSV/JSON)".into(),
    );

    m.insert(
        "help.nsheet.view".into(),
        "nsheet view <file> [options] ## Print sheet as a table (default 50 rows)".into(),
    );

    m.insert(
        "help.nsheet.sort".into(),
        "nsheet view <file> --sort <col> [--desc] ## Sort by column".into(),
    );

    m.insert(
        "help.nsheet.filter".into(),
        "nsheet view <file> --filter <expr> ## Filter rows (col~text, col=n, col>n)".into(),
    );

    m.insert(
        "help.nsheet.export".into(),
        "nsheet view <file> --export <json|md> [-o <file>] ## Export rows".into(),
    );

    m.insert("help.nsheet.tui".into(), "TUI keys: arrows/hjkl move · / search · n next · s sort · f filter · e export · Enter view · q quit".into());

    m.insert(
        "help.nsheet.example1".into(),
        "nsheet open users.csv ## Browse the sheet".into(),
    );

    m.insert(
        "help.nsheet.example2".into(),
        "nsheet view data.json --filter \"age>30\" --sort age ## Filter + sort".into(),
    );

    m.insert(
        "help.nsheet.example3".into(),
        "nsheet view a.csv --export md -o out.md ## Export Markdown".into(),
    );
}
