use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.line.title".into(), "LINE COMMAND:".into());

    m.insert(
        "help.line.usage".into(),
        "line [-A <prefix>] [-B <separator>] [file] ## Number each line (default '1: line')".into(),
    );

    m.insert(
        "help.line.example1".into(),
        "gs main | line -A 'line' -B ':' ## Prefix every line with line1: ...".into(),
    );
}
