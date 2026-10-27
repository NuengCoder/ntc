use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.editor.ne".into(),
        "ne, ntceditor <file> ## Open an existing file in built-in text editor".into(),
    );

    m.insert(
        "help.editor.ne_hyper".into(),
        "ne --hyper <name> --lang <l> [-I] ## Create C/C++ hyper project (-I opens project dir)"
            .into(),
    );

    m.insert(
        "help.editor.ne_init".into(),
        "ne --init <file> [<file> ...] [-I <file>] ## Create template file(s); -I opens one".into(),
    );

    m.insert(
        "help.editor.ne_line".into(),
        "ne, ntceditor <file>:<n> ## Open file at line <n>".into(),
    );

    m.insert(
        "help.editor.ne_super".into(),
        "ne --super <file> [<file> ...] [-I <file>] ## Create super template file(s); -I opens one"
            .into(),
    );

    m.insert(
        "help.editor.ne_welcome".into(),
        "ne, ntceditor ## Open welcome screen in current directory".into(),
    );

    m.insert("help.editor.title".into(), "EDITOR COMMANDS:".into());

    m.insert(
        "help.editor.example1".into(),
        "ne main.rs ## Open main.rs in editor".into(),
    );

    m.insert(
        "help.editor.example2".into(),
        "ne src/main.rs:42 ## Open main.rs at line 42".into(),
    );

    m.insert(
        "help.editor.example3".into(),
        "ne --init a.py b.html -I a.py ## Create templates and open a.py".into(),
    );

    m.insert(
        "help.editor.example4".into(),
        "ne --hyper myproject --lang c ## Create C hyper project".into(),
    );
}
