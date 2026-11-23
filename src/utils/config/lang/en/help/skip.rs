use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.skip.title".into(), "SKIP COMMAND:".into());

    m.insert(
        "help.skip.usage1".into(),
        "skip [N] [file] ## Print every line after the first N (default 10)".into(),
    );

    m.insert(
        "help.skip.usage2".into(),
        "skip -x <lines> [file] ## Print every line except the listed ones (1-based)".into(),
    );

    m.insert(
        "help.skip.note".into(),
        "The -x list separates numbers with commas or spaces (2,5,8 or 2 5 8); input comes from a file or a pipe".into(),
    );

    m.insert(
        "help.skip.example1".into(),
        "skip 2 app.log ## app.log without its first 2 lines".into(),
    );

    m.insert(
        "help.skip.example2".into(),
        "gs TODO | skip -x 2,5,8 ## Drop lines 2, 5 and 8 of the search output".into(),
    );
}
