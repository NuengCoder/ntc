use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.betw.title".into(), "BETW COMMAND:".into());

    m.insert(
        "help.betw.usage".into(),
        "betw A:B [file] ## Print lines A to B (1-based inclusive)".into(),
    );

    m.insert(
        "help.betw.example1".into(),
        "betw 10:20 app.log ## Print lines 10-20 of app.log".into(),
    );
}
