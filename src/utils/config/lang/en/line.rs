use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.line.usage".into(),
        "Usage: line [-A <prefix>] [-B <separator>] [file]  (numbers each line; reads piped input when no file)"
            .into(),
    );

    m.insert(
        "misc.line.bad_arg".into(),
        "line: unexpected argument '{}' (usage: line [-A <prefix>] [-B <separator>] [file])".into(),
    );
}
