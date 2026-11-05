use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.explorer.hint1".into(),
        "h=help, b=parent dir, f=find (hits show their paths)".into(),
    );

    m.insert(
        "help.explorer.hint2".into(),
        "g/:=command (file|dir|line), /=live filter, e=editor".into(),
    );

    m.insert(
        "help.explorer.hint3".into(),
        "r=rename, d=delete, n=new file, m=new dir, i=hidden toggle".into(),
    );

    m.insert(
        "help.explorer.hint4".into(),
        "s=sort mode, S=sort direction (dirs too), F5=refresh".into(),
    );

    m.insert(
        "help.explorer.hint5".into(),
        "find: type/Backspace=query, Tab=hjkl/arrows, Enter=open, Esc=back".into(),
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
