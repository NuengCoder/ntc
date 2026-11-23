use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ne.title".into(), "EDITOR COMMANDS:".into());

    m.insert(
        "help.ne.open".into(),
        "ne, ntceditor <file> ## Open an existing file in built-in text editor".into(),
    );

    m.insert(
        "help.ne.line".into(),
        "ne, ntceditor <file>:<n> ## Open file at line <n>".into(),
    );

    m.insert(
        "help.ne.welcome".into(),
        "ne, ntceditor ## Open welcome screen in current directory".into(),
    );

    m.insert(
        "help.ne.init".into(),
        "ne --init <file> [<file> ...] [-I <file>] ## Create template file(s); -I opens one".into(),
    );

    m.insert(
        "help.ne.super".into(),
        "ne --super <file> [<file> ...] [-I <file>] ## Create super template file(s); -I opens one"
            .into(),
    );

    m.insert(
        "help.ne.example1".into(),
        "ne main.rs ## Open main.rs in editor".into(),
    );

    m.insert(
        "help.ne.example2".into(),
        "ne src/main.rs:42 ## Open main.rs at line 42".into(),
    );

    m.insert(
        "help.ne.example3".into(),
        "ne --init a.py b.html -I a.py ## Create templates and open a.py".into(),
    );

    m.insert(
        "help.ne.example4".into(),
        "ne --super run.sh run.ps1 run.bat -I run.sh ## Create script starter files".into(),
    );
}
