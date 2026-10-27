use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("msg.ignore.dir_added".into(), "正在忽略目录：{}".into());

    m.insert("msg.ignore.dir_removed".into(), "不再忽略目录：{}".into());

    m.insert("msg.ignore.ext_added".into(), "正在忽略 .{} 文件".into());

    m.insert("msg.ignore.ext_removed".into(), "正在关注 .{} 文件".into());

    m.insert("msg.ignore.file_added".into(), "正在忽略文件：{}".into());

    m.insert("msg.ignore.file_removed".into(), "正在关注文件：{}".into());

    m.insert("msg.ignored.dirs".into(), "已忽略的目录：{}".into());

    m.insert("msg.ignored.extensions".into(), "已忽略的扩展名：{}".into());

    m.insert(
        "msg.ignored.extra_extensions".into(),
        "额外支持的扩展名：{}".into(),
    );

    m.insert(
        "msg.ignored.extra_files".into(),
        "额外支持的文件：{}".into(),
    );

    m.insert("msg.ignored.files".into(), "已忽略的文件：{}".into());

    m.insert(
        "msg.image.converted".into(),
        "图像已转换并保存到：{}".into(),
    );

    m.insert("msg.image.not_found".into(), "未找到图像：{}".into());

    m.insert("msg.init.template_created".into(), "已创建模板：{}".into());

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert(
        "msg.list.title".into(),
        "ntc {} - 可用的命令行功能：\n".into(),
    );

    m.insert(
        "msg.report.binary_clipboard_warning".into(),
        "{} 报告无法复制到剪贴板（二进制格式）".into(),
    );

    m.insert(
        "msg.report.copied_to_clipboard".into(),
        "{} 报告已复制到剪贴板！".into(),
    );

    m.insert("msg.report.file_saved".into(), "文件已保存到：{}".into());

    m.insert("msg.report.path_not_found".into(), "未找到路径：{}".into());

    m.insert(
        "msg.report.skipped_unsupported".into(),
        "已跳过（不支持的格式）：{}".into(),
    );

    m.insert(
        "msg.setA.error".into(),
        "setA的值无效：{}。使用 开 或 关。".into(),
    );

    m.insert("msg.setA.off".into(), "自动建议：关".into());

    m.insert("msg.setA.on".into(), "自动建议：开".into());

    m.insert("msg.setA.status".into(), "自动建议（幽灵文本）：{}".into());

    m.insert(
        "msg.setC.error".into(),
        "setC的值无效：{}。使用 开 或 关。".into(),
    );

    m.insert("msg.setC.status".into(), "颜色输出：{}".into());

    m.insert("msg.setC.success".into(), "颜色：{}".into());

    m.insert(
        "msg.setD.error_out_of_range".into(),
        "无效的深度值：{}。必须在1到20之间。".into(),
    );

    m.insert(
        "msg.setD.error_parse".into(),
        "无效的深度值：{}。必须是1到20之间的正整数。".into(),
    );

    m.insert("msg.setD.status".into(), "当前最大深度：{}".into());

    m.insert("msg.setD.success".into(), "最大深度已设置为：{}".into());

    m.insert("msg.setH.off".into(), "历史记录：关".into());

    m.insert("msg.setH.on".into(), "历史记录：开".into());

    m.insert("msg.setH.path".into(), "历史记录路径：{}".into());

    m.insert("msg.setH.path_default".into(), "历史记录路径：默认".into());

    m.insert(
        "msg.setH.path_reset".into(),
        "历史记录路径已重置为默认".into(),
    );

    m.insert(
        "msg.setH.path_set".into(),
        "历史记录路径已设置为：{}".into(),
    );

    m.insert("msg.setH.status".into(), "历史记录：{}".into());

    m.insert(
        "msg.setL.error".into(),
        "setL的值无效：{}。使用 开 或 关。".into(),
    );

    m.insert("msg.setL.status".into(), "行号：{}".into());

    m.insert("msg.setL.success".into(), "行号：{}".into());

    m.insert("msg.setO.status".into(), "当前输出路径：{}".into());

    m.insert("msg.setO.success".into(), "输出路径已设置为：{}".into());

    m.insert(
        "msg.setT.error_out_of_range".into(),
        "无效的线程数：{}。必须在1到64之间。".into(),
    );

    m.insert(
        "msg.setT.error_parse".into(),
        "无效的线程数：{}。必须是1到64之间的正整数。".into(),
    );

    m.insert("msg.setT.status".into(), "当前线程数：{}".into());

    m.insert("msg.setT.success".into(), "线程数已设置为：{}".into());

    m.insert("msg.showcg.box_bottom".into(), "\u{2514}{}\u{2518}".into());

    m.insert("msg.showcg.box_divider".into(), "\u{251C}{}\u{2524}".into());

    m.insert("msg.showcg.box_top".into(), "\u{250C}{}\u{2510}".into());

    m.insert("msg.showcg.label_color".into(), "颜色：".into());

    m.insert("msg.showcg.label_history".into(), "历史记录：".into());

    m.insert("msg.showcg.label_line_numbers".into(), "行号：".into());

    m.insert("msg.showcg.label_max_depth".into(), "最大深度：".into());

    m.insert("msg.showcg.label_output_path".into(), "输出路径：".into());

    m.insert("msg.showcg.label_threads".into(), "线程数：".into());

    m.insert("msg.showcg.label_watcher".into(), "监视器：".into());

    m.insert("msg.showcg.label_watch_limit".into(), "监视器行数：".into());

    m.insert("msg.showcg.label_dir_view".into(), "目录视图：".into());

    m.insert("msg.showcg.label_update_check".into(), "更新检查：".into());

    m.insert(
        "msg.showcg.label_serve_files".into(),
        "Serve 文件服务：".into(),
    );

    m.insert(
        "msg.showcg.label_serve_http".into(),
        "Serve 静态服务：".into(),
    );

    m.insert(
        "msg.showcg.label_serve_chess".into(),
        "Serve 象棋服务：".into(),
    );

    m.insert("msg.showcg.title".into(), "当前配置".into());

    m.insert("msg.size.bytes_label".into(), "字节：".into());

    m.insert("msg.size.human_label".into(), "可读格式：".into());

    m.insert("msg.size.path_label".into(), "路径：{}".into());

    m.insert("msg.size.time_label".into(), "耗时：".into());

    m.insert("msg.size.title".into(), "当前目录大小".into());

    m.insert("msg.teleport.error".into(), "传送到 '{}' 失败：{}".into());

    m.insert(
        "msg.teleport.hint_list".into(),
        "使用 'ntc --tp-list' 查看所有保存点。".into(),
    );

    m.insert("msg.teleport.not_found".into(), "未找到传送点：'{}'".into());

    m.insert(
        "msg.teleport.success".into(),
        "\u{2713} 已传送到 '{}' -> {}".into(),
    );

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert(
        "msg.watch.invalid".into(),
        "使用 --watch 开 或 --watch 关".into(),
    );

    m.insert(
        "msg.watch.off".into(),
        "文件监视器：关（重启ntc以停用）".into(),
    );

    m.insert(
        "msg.watch.on".into(),
        "文件监视器：开（重启ntc以激活）".into(),
    );

    m.insert("msg.watch.status".into(), "文件监视器：{}".into());

    m.insert("msg.watch.limit_status".into(), "监视器显示行数：{}".into());

    m.insert(
        "msg.watch.limit_success".into(),
        "监视器显示行数已设为 {}".into(),
    );

    m.insert(
        "msg.watch.limit_error".into(),
        "无效的监视器行数：'{}'。请输入正数。".into(),
    );

    m.insert("msg.where.current_dir".into(), "当前目录：{}".into());

    m.insert("msg.where.executable".into(), "ntc 可执行文件：{}".into());

    m.insert("msg.setGit.status".into(), "Git 集成：{}".into());

    m.insert("msg.setGit.on".into(), "已启用 Git 集成。".into());

    m.insert("msg.setGit.off".into(), "已禁用 Git 集成。".into());

    m.insert("msg.setGit.error".into(), "用法：config git ON|OFF".into());

    m.insert("msg.setIV.status".into(), "启动介绍：{}".into());

    m.insert(
        "msg.setIV.on".into(),
        "已启用启动介绍（logo、ntpkg 与 watcher 提示）。".into(),
    );

    m.insert(
        "msg.setIV.off".into(),
        "已关闭启动介绍（不再显示 logo、ntpkg 与 watcher 提示）。".into(),
    );

    m.insert(
        "msg.setIV.error".into(),
        "用法：config intro_view ON|OFF".into(),
    );

    m.insert("msg.showcg.label_intro_view".into(), "启动介绍：".into());

    m.insert("msg.setDV.status".into(), "目录视图：{}".into());

    m.insert("msg.setDV.on".into(), "已启用目录视图。".into());

    m.insert("msg.setDV.off".into(), "已禁用目录视图。".into());

    m.insert(
        "msg.setDV.error".into(),
        "用法：config dir_view ON|OFF".into(),
    );

    m.insert("msg.setUC.status".into(), "更新检查：{}".into());

    m.insert("msg.setUC.on".into(), "已启用每日新版本提醒。".into());

    m.insert("msg.setUC.off".into(), "已禁用更新检查。".into());

    m.insert(
        "msg.setUC.error".into(),
        "用法：config update_check ON|OFF".into(),
    );

    m.insert(
        "msg.update.available".into(),
        "有新的 NTC {}（你用的是 {}）— 在 {} 中运行 'ntc --upgrade' 或查看 {}".into(),
    );
}
