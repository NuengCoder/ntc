use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.search.ds".into(),
        "ds <pattern> [-d <n>] ## Search directories by name".into(),
    );

    m.insert(
        "help.search.fs".into(),
        "fs <pattern> [-d <n>] ## Search files by name (fuzzy fallback)".into(),
    );

    m.insert(
        "help.search.fsc".into(),
        "fsc <pattern> [-d <n>] ## Search files, pick one to display & optionally copy".into(),
    );

    m.insert("help.search.gs".into(), "gs <pattern> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-i] ## Search file contents (simple grep)".into());

    m.insert(
        "help.search.gs_interactive".into(),
        "gs -i <pattern> [-d <n>] ## Interactive grep (10/page, Enter to open)".into(),
    );

    m.insert(
        "help.search.locate".into(),
        "locate <pattern> [-d <n>] ## Search both files and directories".into(),
    );

    m.insert(
        "help.search.pipe".into(),
        "cmd | gs <pattern> ## Search piped text (qur -s <pattern> too)".into(),
    );

    m.insert("help.search.qur".into(), "qur <glob> [-s \"pat\"] [-rp \"new\"] [-dp] [-d <n>] ## Query files, select content, replace".into());

    m.insert("help.search.title".into(), "SEARCH COMMANDS:".into());

    m.insert(
        "help.search.example1".into(),
        "gs fn main ## Search for 'fn main' in files".into(),
    );

    m.insert(
        "help.search.example2".into(),
        "gs -i config ## Interactive search results".into(),
    );

    m.insert(
        "help.search.example3".into(),
        "fs *.rs ## Find .rs files".into(),
    );

    m.insert(
        "help.search.example4".into(),
        "ds src ## Find directories named 'src'".into(),
    );

    m.insert(
        "help.search.example5".into(),
        "locate README ## Search files & dirs for 'README'".into(),
    );

    m.insert(
        "help.search.example6".into(),
        "qur *.rs -s \"TODO\" -dp ## Query glob + grep-like display".into(),
    );

    m.insert(
        "help.search.example7".into(),
        "gs TODO < app.log ## Search piped/redirected text".into(),
    );

    m.insert(
        "help.search.example8".into(),
        "gs error\\|warning ## OR search: match either term (a\\|b)".into(),
    );
}
