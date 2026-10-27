use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nssh.title".into(), "SSH 命令：".into());

    m.insert(
        "help.nssh.add".into(),
        "nssh add <名称> --host <主机> ## 保存 SSH 配置".into(),
    );

    m.insert(
        "help.nssh.ssh".into(),
        "nssh ssh [名称] [--cmd \"命令\"] ## 交互式 shell 或单条命令".into(),
    );

    m.insert(
        "help.nssh.cp".into(),
        "nssh cp <本地> <名称>:<远端> ## SFTP 上传/下载".into(),
    );

    m.insert(
        "help.nssh.scan".into(),
        "nssh scan <主机> [--top 数量] ## TCP 端口扫描".into(),
    );

    m.insert(
        "help.nssh.manage".into(),
        "nssh ls|info|rm|edit ## 管理配置".into(),
    );

    m.insert(
        "help.nssh.keygen".into(),
        "nssh keygen ## 通过 ssh-keygen 生成密钥".into(),
    );

    m.insert(
        "help.nssh.example1".into(),
        "nssh add prod --host 203.0.113.10 --user root ## 保存配置".into(),
    );

    m.insert(
        "help.nssh.example2".into(),
        "nssh ssh prod --cmd \"uptime\" ## 运行远端命令".into(),
    );

    m.insert(
        "help.nssh.example3".into(),
        "nssh cp ./app.tar.gz prod:/tmp/ ## 上传文件".into(),
    );
}
