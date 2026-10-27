use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.scan.depth".into(),
        "scan ext <扩展名...> -d, --depth <n> ## 限制扫描深度".into(),
    );

    m.insert(
        "help.scan.example1".into(),
        "scan ext rs ## 列出所有 .rs 文件".into(),
    );

    m.insert(
        "help.scan.example2".into(),
        "scan ext rs c -d 2 ## 列出深度 2 内的 .rs/.c 文件".into(),
    );

    m.insert(
        "help.scan.example3".into(),
        "scan ext rs c -s ## 列出并显示大小与总计".into(),
    );

    m.insert(
        "help.scan.example4".into(),
        "scan port 192.168.1.1 --top 10 ## 扫描前 10 个端口".into(),
    );

    m.insert(
        "help.scan.example5".into(),
        "scan ip 192.168.1.0/24 ## 扫描存活主机".into(),
    );

    m.insert(
        "help.scan.example6".into(),
        "scan files *.rs ## 按名称匹配列出文件".into(),
    );

    m.insert(
        "help.scan.example7".into(),
        "scan dir k* ## 按名称匹配列出目录".into(),
    );

    m.insert(
        "help.scan.ext".into(),
        "scan ext <扩展名...> ## 按扩展名列出文件（不统计行数）".into(),
    );

    m.insert(
        "help.scan.files".into(),
        "scan files [匹配] ## 按名称匹配列出文件（* 表示全部）".into(),
    );

    m.insert(
        "help.scan.dir".into(),
        "scan dir [匹配] ## 按名称匹配列出目录（* 表示全部）".into(),
    );

    m.insert(
        "help.scan.port".into(),
        "scan port [主机] [--top N | --ports <范围> | --all] ## TCP 端口扫描".into(),
    );

    m.insert(
        "help.scan.ip".into(),
        "scan ip [网段] [--top N | --ports <范围>] ## 存活主机扫描（含 ARP）".into(),
    );

    m.insert(
        "help.scan.size".into(),
        "scan ext <扩展名...> -s, --size ## 显示大小与总计".into(),
    );

    m.insert(
        "help.scan.care".into(),
        "scan ext <扩展名...> --care ## 包含所有文件（care 模式）".into(),
    );

    m.insert("help.scan.title".into(), "扫描命令：".into());
}
