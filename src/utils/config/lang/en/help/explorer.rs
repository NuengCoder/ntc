use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=help, l=go to line, b=parent dir, f=find".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "/=filter, e=editor, r=rename, d=delete".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "n=new file, m=new dir, i=hidden toggle".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s/sort toggle, S=sort reverse, F5=refresh".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "q/Esc=quit, mouse=click/scroll".into(),
    );

    m.insert(
        "help.explorer.launch".into(),
        "explorer, nfe [PATH] ## Launch the full-screen terminal file explorer".into(),
    );

    m.insert("help.explorer.title".into(), "FILE EXPLORER (NFE):".into());

    m.insert(
        "help.explorer.example1".into(),
        "explorer ## Launch file explorer in current dir".into(),
    );

    m.insert(
        "help.explorer.example2".into(),
        "nfe ~/projects ## Open explorer in ~/projects".into(),
    );
}
