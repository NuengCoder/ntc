use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.network.dl".into(),
        "dl <url> ## 下载文件（--resume, --stop, --history, --cancel）".into(),
    );

    m.insert(
        "help.network.dns".into(),
        "dns <主机> ## DNS查询（-t A, AAAA, MX, NS, CNAME, TXT, PTR, SOA, ALL）".into(),
    );

    m.insert(
        "help.network.http".into(),
        "http <方法> <url> ## HTTP API客户端（-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t）"
            .into(),
    );

    m.insert(
        "help.network.ping".into(),
        "ping <主机> ## ICMP ping（-c 次数）".into(),
    );

    m.insert(
        "help.network.ipcfg".into(),
        "ipcfg [名称] ## 显示网卡信息（类似 ipconfig/ifconfig）".into(),
    );

    m.insert(
        "help.network.serve".into(),
        "serve files|http-server|config ## 服务：文件网盘、静态目录服务器、各服务默认值".into(),
    );

    m.insert(
        "help.network.smtp".into(),
        "smtp ## SMTP邮件客户端（--id, -i, -u, -pwd, -s, -b, -f, --to）".into(),
    );

    m.insert(
        "help.network.spec".into(),
        "spec ## 显示电脑/系统规格（CPU、显卡、内存、存储、系统、序列号）".into(),
    );

    m.insert(
        "help.network.sys".into(),
        "sys, top ## 实时系统监视器（CPU、内存、磁盘、进程）".into(),
    );

    m.insert("help.network.title".into(), "系统与网络命令：".into());

    m.insert(
        "help.network.up".into(),
        "up <文件> <url> ## 上传文件（默认 multipart POST；--put, --raw, -a, --bearer, -F, -H, -d）".into(),
    );

    m.insert(
        "help.network.example1".into(),
        "sys ## 显示系统监视器".into(),
    );

    m.insert(
        "help.network.example2".into(),
        "http GET https://api.example.com ## 发送 GET 请求".into(),
    );

    m.insert(
        "help.network.example3".into(),
        "dl https://example.com/file.zip ## 下载并显示进度".into(),
    );

    m.insert(
        "help.network.example4".into(),
        "ping google.com ## Ping google.com".into(),
    );

    m.insert(
        "help.network.example5".into(),
        "dns example.com -t MX ## 查询 DNS MX 记录".into(),
    );

    m.insert(
        "help.network.example6".into(),
        "serve files --port 8080 ## 在端口 8080 启动文件网盘服务器".into(),
    );
    m.insert(
        "help.network.trace".into(),
        "trace <主机> [-m N] [-w 毫秒] ## 逐跳路由追踪（tracert/traceroute，回退 TTL 探测）".into(),
    );

    m.insert(
        "help.network.ping_flags".into(),
        "ping <主机> [-c N] [--forever] [-w 毫秒] [-s 字节] [-t TTL] [-4|-6] ## 带参数的 ICMP ping"
            .into(),
    );
}
