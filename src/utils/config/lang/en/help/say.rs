use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.say.title".into(), "SAY COMMAND:".into());

    m.insert(
        "help.say.usage".into(),
        "say <text> [--loop N] [-n|-t] ## Print text (no file redirect)".into(),
    );

    m.insert(
        "help.say.example1".into(),
        "say \"hello\" ## Print text".into(),
    );
}
