use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.scan.depth_usage".into(),
        "用法：scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.scan.ext_usage".into(),
        "用法：scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.help_ext".into(), "  scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]  按扩展名列出文件（不统计行数）".into());

    m.insert("cmd.scan.help_files".into(), "  scan files [匹配] [-d|--depth <n>] [-s|--size] [--care]  按名称匹配列出文件（* 匹配全部）".into());

    m.insert(
        "cmd.scan.help_dir".into(),
        "  scan dir [匹配] [-d|--depth <n>] [-s|--size] [--care]  按名称匹配列出目录（* 匹配全部）"
            .into(),
    );

    m.insert("cmd.scan.help_port".into(), "  scan port [主机] [--top N | --ports <范围> | --all] [--timeout MS]  TCP 端口扫描（默认本机，前 20 个端口）".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [网段] [--top N | --ports <范围>] [--timeout MS]  存活主机扫描（默认自动本地 /24）".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "存活：{} / 已扫 {}".into());

    m.insert("cmd.scan.ip_none".into(), "{} 中没有存活主机".into());

    m.insert(
        "cmd.scan.ip_start".into(),
        "正在扫描 {}（每主机 {} 个端口）...".into(),
    );

    m.insert("cmd.scan.ip_title".into(), "{} 中的存活主机：".into());

    m.insert(
        "cmd.scan.missing_ports".into(),
        "'--ports' 后缺少值。".into(),
    );

    m.insert(
        "cmd.scan.missing_timeout".into(),
        "'--timeout' 后缺少值。".into(),
    );

    m.insert("cmd.scan.missing_top".into(), "'--top' 后缺少值。".into());

    m.insert("cmd.scan.invalid_depth".into(), "无效的深度：{}".into());

    m.insert(
        "cmd.scan.invalid_timeout".into(),
        "无效的 --timeout 值：'{}'".into(),
    );

    m.insert(
        "cmd.scan.invalid_top".into(),
        "无效的 --top 值：'{}'".into(),
    );

    m.insert("cmd.scan.list_title".into(), "扫描：{}".into());

    m.insert("cmd.scan.no_files".into(), "未找到 {} 文件".into());

    m.insert("cmd.scan.port_done".into(), "开放：{} / 已扫 {}".into());

    m.insert(
        "cmd.scan.files_usage".into(),
        "用法：scan files [匹配] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert(
        "cmd.scan.dir_usage".into(),
        "用法：scan dir [匹配] [-d|--depth <n>] [-s|--size] [--care]".into(),
    );

    m.insert("cmd.scan.no_dirs".into(), "未找到 {} 目录".into());

    m.insert("cmd.scan.total_dirs".into(), "总计：{} 个目录".into());

    m.insert(
        "cmd.scan.total_dirs_size".into(),
        "总计：{} 个目录（{}）".into(),
    );

    m.insert("cmd.scan.port_none".into(), "{} 上没有开放端口".into());

    m.insert(
        "cmd.scan.port_start".into(),
        "正在扫描 {}（{} 个端口）...".into(),
    );

    m.insert("cmd.scan.port_title".into(), "{} 上开放的端口：".into());

    m.insert(
        "cmd.scan.progress".into(),
        "正在扫描 {} … {}/{} ({}%) · {} 个开放".into(),
    );

    m.insert("cmd.scan.total_files".into(), "共计：{} 个文件".into());

    m.insert(
        "cmd.scan.total_files_size".into(),
        "共计：{} 个文件（{}）".into(),
    );

    m.insert(
        "cmd.scan.unknown_option".into(),
        "未知的 scan 选项：{}".into(),
    );

    m.insert(
        "cmd.scan.unknown_sub".into(),
        "未知的 scan 子命令：'{}'".into(),
    );

    m.insert("cmd.scan.unexpected_arg".into(), "多余的参数：'{}'".into());

    m.insert("cmd.scan.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.scan.help_waep".into(),
        "  scan waep <主机|URL>    枚举网页路径（接口发现）".into(),
    );

    m.insert(
        "cmd.scan.waep_usage".into(),
        "用法：scan waep <主机|主机:端口|URL[/路径]> [选项]（见 scan help）".into(),
    );

    m.insert("cmd.scan.waep_bad_target".into(), "无效目标：{}".into());

    m.insert(
        "cmd.scan.waep_bad_method".into(),
        "无效方法：{}（用 GET 或 HEAD）".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_status".into(),
        "无效状态码列表：{}".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_depth".into(),
        "无效深度：{}（1-4）".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_concurrency".into(),
        "无效并发数：{}（1-64）".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_timeout".into(),
        "无效超时：{}（>=50 毫秒）".into(),
    );

    m.insert(
        "cmd.scan.waep_bad_header".into(),
        "无效请求头：{}（格式 'Name: value'）".into(),
    );

    m.insert(
        "cmd.scan.waep_start".into(),
        "正在探测 {}（{} 条路径）...".into(),
    );

    m.insert("cmd.scan.waep_none".into(), "{} 上没有路径响应".into());

    m.insert("cmd.scan.waep_summary".into(), "命中 {} 个".into());

    m.insert("cmd.scan.waep_elapsed".into(), "耗时：{} 毫秒".into());

    m.insert("cmd.scan.waep_saved".into(), "已保存：{}".into());

    m.insert("cmd.scan.waep_save_failed".into(), "无法保存 {}：{}".into());

    m.insert(
        "cmd.scan.waep_wordlist_failed".into(),
        "无法读取字典 {}：{}".into(),
    );

    m.insert(
        "cmd.scan.waep_empty_wordlist".into(),
        "字典 {} 没有可用条目".into(),
    );

    m.insert("cmd.scan.waep_flags_title".into(), "选项：".into());

    m.insert(
        "cmd.scan.waep_flags".into(),
        "-w, --wordlist <文件>      使用自定义字典（每行一个路径）\n  -x, --ext <扩展名,...>     同时探测 word.ext\n  -m, --method GET|HEAD      HTTP 方法（默认 GET）\n  -s, --status <代码,...>    视为命中的状态码，逗号分隔\n      --all                  显示所有状态（包括 404）\n  -r, --recursive            深入已发现的路径\n  -d, --depth <1-4>          递归深度（默认 1）\n  -c, --concurrency <1-64>   并发探测数（默认 16）\n      --timeout <毫秒>       单次探测超时（默认 4000）\n  -H, --header \"名称: 值\"   额外请求头（可重复）\n  -A, --user-agent <ua>      User-Agent 请求头\n  -o, --output <文件>        将结果保存到文件\n  -q, --quiet                只输出摘要\n  -h, --help                 显示此帮助".into(),
    );

    m.insert("cmd.scan.waep_options_title".into(), "说明：".into());

    m.insert(
        "cmd.scan.waep_options".into(),
        "target        host、host:port、url 或 url/base（默认 scheme http）\n  paths         相对 base 显示：.../api 显示 /files 而不是 /api/files\n  状态 405      视为命中（路径存在，但不支持该方法）\n  recursion     只深入看起来是目录（无点号）的命中".into(),
    );

    m.insert(
        "cmd.scan.waep_unreachable".into(),
        "{} 无响应 - 请检查主机、端口和 scheme（DNS/连接/TLS 失败）".into(),
    );

    m.insert(
        "cmd.scan.waep_some_failed".into(),
        "{} / {} 次探测没有响应（超时或连接被拒绝）".into(),
    );
}
