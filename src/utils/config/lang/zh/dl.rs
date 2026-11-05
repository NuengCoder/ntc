use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "dl.cancel_hint".into(),
        "使用 'dl --cancel <编号|url>' 删除记录。".into(),
    );

    m.insert(
        "dl.completed".into(),
        "已下载 {}（{}，耗时 {}，速度 {}）".into(),
    );

    m.insert("dl.downloading".into(), "正在下载 {}".into());

    m.insert("dl.failed_append".into(), "打开 {} 以追加失败".into());

    m.insert("dl.failed_create".into(), "创建 {} 失败".into());

    m.insert("dl.failed_download".into(), "下载 {} 失败".into());

    m.insert("dl.failed_read".into(), "读取 {} 失败".into());

    m.insert(
        "dl.file_still_exists".into(),
        "文件 '{}' 仍然存在。如需，请手动删除。".into(),
    );

    m.insert("dl.help.examples".into(), "示例：".into());

    m.insert(
        "dl.help.opt_cancel".into(),
        " --cancel <编号|url> 删除下载记录".into(),
    );

    m.insert("dl.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert(
        "dl.help.opt_o".into(),
        " -o, --output <文件> 保存为文件名".into(),
    );

    m.insert(
        "dl.help.opt_history".into(),
        " --history 显示下载历史".into(),
    );

    m.insert(
        "dl.help.opt_resume".into(),
        " --resume 恢复未完成的下载（从列表中选择）".into(),
    );

    m.insert(
        "dl.help.opt_stop".into(),
        " --stop 停止正在进行的下载（从列表中选择）".into(),
    );

    m.insert("dl.help.options".into(), "选项：".into());

    m.insert("dl.help.title".into(), "下载（dl）".into());

    m.insert("dl.help.usage".into(), "用法：".into());

    m.insert("dl.help.usage_line".into(), " dl <url> [选项]".into());

    m.insert("dl.history_title".into(), "下载历史：".into());

    m.insert("dl.http_error".into(), "HTTP {}：{}".into());

    m.insert(
        "dl.invalid_header".into(),
        "无效的标头格式，应为 \"Key: Value\"，实际为：{}".into(),
    );

    m.insert("dl.invalid_index".into(), "无效的索引".into());

    m.insert("dl.missing_cancel".into(), "--cancel 后缺少URL/名称".into());

    m.insert("dl.missing_data".into(), "-d 后缺少数据".into());

    m.insert("dl.missing_header".into(), "-H 后缺少标头值".into());

    m.insert("dl.missing_o".into(), "-o 后缺少文件名".into());

    m.insert("dl.no_records".into(), "尚无下载记录。".into());

    m.insert("dl.not_found".into(), "未找到下载记录：{}".into());

    m.insert("dl.not_found_404".into(), "未找到URL（404）：{}".into());

    m.insert("dl.removed_record".into(), "已删除下载记录：{}".into());

    m.insert(
        "dl.up_url_required".into(),
        "需要URL。用法：up <文件> <url>".into(),
    );

    m.insert("dl.uploading".into(), "正在上传 {}（{}）".into());

    m.insert("dl.url_required".into(), "需要URL。用法：dl <url>".into());

    m.insert(
        "dl.dns_host_required".into(),
        "缺少主机。用法：dns <主机>".into(),
    );

    m.insert(
        "dl.dns_looking_up".into(),
        "正在通过系统工具查询 {} {} 记录...".into(),
    );

    m.insert(
        "dl.dns_lookup_failed".into(),
        "DNS 查询失败（退出码：{}）".into(),
    );

    m.insert("dl.dns_missing_type".into(), "缺少 -t 后的记录类型".into());

    m.insert("dl.dns_no_records".into(), "  未找到 {} 条 {} 记录".into());

    m.insert(
        "dl.dns_tool_failed".into(),
        "运行 DNS 工具失败：{}。请安装 dig 或 nslookup 以支持 {} 记录。".into(),
    );

    m.insert("dl.invalid_request".into(), "无效的请求：{}".into());

    m.insert("dl.ping_exit_code".into(), "ping 退出码：{}".into());

    m.insert("dl.ping_failed".into(), "运行 ping 失败：{}".into());

    m.insert(
        "dl.ping_host_required".into(),
        "缺少主机。用法：ping <主机>".into(),
    );

    m.insert("dl.ping_invalid_count".into(), "无效的数量".into());

    m.insert("dl.ping_missing_count".into(), "缺少 -c 后的数量".into());

    m.insert("dl.request_failed".into(), "请求 {} 失败".into());

    m.insert("dl.upload_failed".into(), "上传到 {} 失败".into());

    m.insert("dl.active_title".into(), "正在运行的下载：".into());

    m.insert("dl.cancelled".into(), "已取消。".into());

    m.insert("dl.incomplete_title".into(), "未完成的下载：".into());

    m.insert("dl.invalid_choice".into(), "选择无效。".into());

    m.insert("dl.no_active".into(), "没有正在运行的下载。".into());

    m.insert(
        "dl.nothing_incomplete".into(),
        "没有可恢复的未完成下载。".into(),
    );

    m.insert("dl.resuming".into(), "正在从 {} 恢复 {}".into());

    m.insert(
        "dl.select_resume".into(),
        "选择要恢复的下载 [1-N]（留空或 q 取消）：".into(),
    );

    m.insert(
        "dl.select_stop".into(),
        "选择要停止的下载 [1-N]（留空或 q 取消）：".into(),
    );

    m.insert(
        "dl.stopped_ok".into(),
        "已停止 {}，可用 'dl --resume' 继续。".into(),
    );

    m.insert(
        "dl.stopped_stale".into(),
        "已将 {} 标记为已停止（它已不在运行）。".into(),
    );

    m.insert(
        "dl.dns_unsupported_type".into(),
        "不支持的记录类型：{}（支持：{}）".into(),
    );

    m.insert("dl.dns_rcode".into(), "服务器返回 {} ({})".into());

    m.insert("dl.dns_timeout".into(), "DNS 服务器无响应（超时）".into());

    m.insert("dl.dns_malformed".into(), "DNS 响应格式错误".into());

    m.insert("dl.dns_io".into(), "DNS 网络错误：{}".into());

    m.insert(
        "dl.dns_system_fallback".into(),
        "回退到系统 DNS 工具……".into(),
    );
}
