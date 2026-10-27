use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nssh.title".into(), "NSSH COMMANDS:".into());

    m.insert(
        "help.nssh.add".into(),
        "nssh add <name> --host <h> ## Save an SSH profile".into(),
    );

    m.insert(
        "help.nssh.ssh".into(),
        "nssh ssh [name] [--cmd \"...\"] ## Shell or one remote command".into(),
    );

    m.insert(
        "help.nssh.cp".into(),
        "nssh cp <local> <name>:<remote> ## SFTP upload/download".into(),
    );

    m.insert(
        "help.nssh.scan".into(),
        "nssh scan <host> [--top N] ## TCP port scan".into(),
    );

    m.insert(
        "help.nssh.manage".into(),
        "nssh ls|info|rm|edit ## Manage profiles".into(),
    );

    m.insert(
        "help.nssh.keygen".into(),
        "nssh keygen ## Generate keys via ssh-keygen".into(),
    );

    m.insert(
        "help.nssh.example1".into(),
        "nssh add prod --host 203.0.113.10 --user root ## Save profile".into(),
    );

    m.insert(
        "help.nssh.example2".into(),
        "nssh ssh prod --cmd \"uptime\" ## Run remote command".into(),
    );

    m.insert(
        "help.nssh.example3".into(),
        "nssh cp ./app.tar.gz prod:/tmp/ ## Upload file".into(),
    );
}
