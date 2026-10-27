use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.deadlock.title".into(),
        "DEADLOCK - encrypted password vault:".into(),
    );

    m.insert(
        "help.deadlock.launch".into(),
        "  deadlock                          Open the vault (asks for the master password)".into(),
    );

    m.insert(
        "help.deadlock.ls".into(),
        "  ls                                List every pass-env with its real password".into(),
    );

    m.insert(
        "help.deadlock.add".into(),
        "  add <pass-env> <real-password>    Store or update a secret (password may be quoted)"
            .into(),
    );

    m.insert(
        "help.deadlock.rmpw".into(),
        "  rmpw <pass-env>                   Delete a stored secret".into(),
    );

    m.insert(
        "help.deadlock.master".into(),
        "  master-pass                       Change the master password".into(),
    );

    m.insert("help.deadlock.forget".into(), "  forget                            Remove the master password from the OS credential store".into());

    m.insert(
        "help.deadlock.destroy".into(),
        "  destroy                           Delete the vault and every stored secret".into(),
    );

    m.insert(
        "help.deadlock.exit".into(),
        "  exit                              Close the vault and return to the ntc shell".into(),
    );

    m.insert("help.deadlock.note".into(), "Secrets are encrypted with AES-256-GCM under an Argon2id key derived from the master password. The default master password is 12345678 and deadlock asks for a new one on first login. The vault file is restricted to your user account and is protected against deletion by the OS (Windows deny-DELETE ACL / macOS uchg; Linux needs root). The master is registered in the OS credential store so nssh can resolve pass-env names without opening this shell.".into());

    m.insert("help.deadlock.example1".into(), "  deadlock".into());

    m.insert(
        "help.deadlock.example2".into(),
        "  deadlock > add prod_pass CatLoveFish360@house".into(),
    );
}
