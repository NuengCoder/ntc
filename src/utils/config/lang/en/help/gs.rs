use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gs.title".into(), "GS COMMAND:".into());

    m.insert(
        "help.gs.usage".into(),
        "gs <pattern> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-A <n>] [-B <n>] [-I] [-x] [-h|-g|-w] [-z] ## Search file contents (simple grep)".into(),
    );

    m.insert(
        "help.gs.interactive".into(),
        "gs -I [pattern] [-d <n>] ## Interactive grep (10/page, f=pattern, e/Enter=open, 0-9=row, ?=keys, x/h/g/w/z=toggles)"
            .into(),
    );

    m.insert(
        "help.gs.pipe".into(),
        "cmd | gs <pattern> ## Search piped text (qur -s <pattern> too)".into(),
    );

    m.insert(
        "help.gs.context".into(),
        "gs <pattern> -A <n> -B <n> ## Show n lines after / before each match (10 matches unless -l <n>)"
            .into(),
    );

    m.insert(
        "help.gs.filters".into(),
        "gs -x <literal> -h <starts> -g <ends> -w <whole> -z <case> ## -x searches the text literally (a\\|b is 'a|b', no OR); -h/-g/-w match the trimmed line and can't be combined; -z keeps case"
            .into(),
    );

    m.insert(
        "help.gs.example1".into(),
        "gs fn main ## Search for 'fn main' in files".into(),
    );

    m.insert(
        "help.gs.example2".into(),
        "gs -I config ## Interactive search results".into(),
    );

    m.insert(
        "help.gs.example3".into(),
        "gs TODO < app.log ## Search piped/redirected text".into(),
    );

    m.insert(
        "help.gs.example4".into(),
        "gs error\\|warning ## OR search: match either term (a\\|b)".into(),
    );

    m.insert(
        "help.gs.example5".into(),
        "gs main -A 3 -B 2 ## Match plus 3 lines after and 2 before".into(),
    );

    m.insert(
        "help.gs.example6".into(),
        "gs -w main ## Only lines that are exactly 'main'".into(),
    );

    m.insert(
        "help.gs.example7".into(),
        "gs -z Main -h ## Case-sensitive: lines starting with 'Main'".into(),
    );
}
