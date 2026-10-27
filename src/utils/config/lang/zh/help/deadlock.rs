use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.deadlock.title".into(),
        "DEADLOCK - 加密密码保险库:".into(),
    );

    m.insert(
        "help.deadlock.launch".into(),
        "  deadlock                          打开保险库（会要求输入主密码）".into(),
    );

    m.insert(
        "help.deadlock.ls".into(),
        "  ls                                列出每个 pass-env 及其真实密码".into(),
    );

    m.insert(
        "help.deadlock.add".into(),
        "  add <pass-env> <real-password>    保存或更新密钥（密码可加引号）".into(),
    );

    m.insert(
        "help.deadlock.rmpw".into(),
        "  rmpw <pass-env>                   删除已保存的密钥".into(),
    );

    m.insert(
        "help.deadlock.master".into(),
        "  master-pass                       更改主密码".into(),
    );

    m.insert(
        "help.deadlock.forget".into(),
        "  forget                            从操作系统凭据存储中移除主密码".into(),
    );

    m.insert(
        "help.deadlock.destroy".into(),
        "  destroy                           删除保险库和所有已保存的密钥".into(),
    );

    m.insert(
        "help.deadlock.exit".into(),
        "  exit                              关闭保险库并返回 ntc 外壳".into(),
    );

    m.insert("help.deadlock.note".into(), "密钥使用 AES-256-GCM 加密，密钥由主密码经 Argon2id 派生。默认主密码是 12345678，首次登录时 deadlock 会要求设置新密码。保险库文件只允许你的账户访问，并受操作系统保护以防删除（Windows deny-DELETE ACL / macOS uchg；Linux 需要 root）。主密码会登记在操作系统凭据存储中，因此 nssh 无需打开此外壳即可解析 pass-env 名称。".into());

    m.insert("help.deadlock.example1".into(), "  deadlock".into());

    m.insert(
        "help.deadlock.example2".into(),
        "  deadlock > add prod_pass CatLoveFish360@house".into(),
    );
}
