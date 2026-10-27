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
}
