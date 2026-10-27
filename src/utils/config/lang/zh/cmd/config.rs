use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.config.deinit.confirm".into(),
        "删除 {}？（y/N）：".into(),
    );

    m.insert(
        "cmd.config.deinit.local_not_found".into(),
        "未找到 ntconfig.toml，跳过。".into(),
    );

    m.insert(
        "cmd.config.deinit.local_remove_failed".into(),
        "删除 ntconfig.toml 失败：{}".into(),
    );

    m.insert(
        "cmd.config.deinit.local_removed".into(),
        "ntconfig.toml 已删除。".into(),
    );

    m.insert(
        "cmd.config.deinit.not_found".into(),
        "未找到 {}，跳过。".into(),
    );

    m.insert(
        "cmd.config.deinit.remove_failed".into(),
        "删除 {} 失败：{}".into(),
    );

    m.insert("cmd.config.deinit.removed".into(), "已删除 {}".into());

    m.insert("cmd.config.deinit.skipped".into(), "已跳过。".into());

    m.insert(
        "cmd.config.deinit.usage".into(),
        "用法：deinit [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.deinit.opencode_not_found".into(),
        "未找到 .opencode 脚手架，跳过。".into(),
    );

    m.insert(
        "cmd.config.example_depth".into(),
        " config depth 4  # 设置最大深度为4".into(),
    );

    m.insert(
        "cmd.config.example_history".into(),
        " config history off  # 禁用历史记录".into(),
    );

    m.insert(
        "cmd.config.example_linenumbers".into(),
        " config linenumbers ON  # 启用行号显示".into(),
    );

    m.insert(
        "cmd.config.example_output".into(),
        " config output C:\\reports  # 设置输出路径".into(),
    );

    m.insert(
        "cmd.config.example_watch".into(),
        " config watch ON  # 启用文件监视器".into(),
    );

    m.insert("cmd.config.examples_header".into(), "示例：".into());

    m.insert(
        "cmd.config.init.create_failed".into(),
        "创建 {} 失败：{}".into(),
    );

    m.insert("cmd.config.init.created".into(), "已创建 {}".into());

    m.insert(
        "cmd.config.init.exists_warn".into(),
        "{} 已存在，跳过。".into(),
    );

    m.insert(
        "cmd.config.init.local_exists".into(),
        "ntconfig.toml 已存在，跳过。".into(),
    );

    m.insert(
        "cmd.config.init.usage".into(),
        "用法：init [--all | --ran | --local | --opencode] [--force]".into(),
    );

    m.insert(
        "cmd.config.init.opencode_ready".into(),
        "OpenCode 脚手架已就绪：{}".into(),
    );

    m.insert("cmd.config.lang.active".into(), "{}（活动中）".into());

    m.insert("cmd.config.lang.available".into(), "可用语言：".into());

    m.insert(
        "cmd.config.lang.coming_soon".into(),
        "{}（已设置，即将推出）".into(),
    );

    m.insert("cmd.config.lang.set".into(), "语言已设置为：{}".into());

    m.insert(
        "cmd.config.lang.set_coming_soon".into(),
        "语言已设置为：{}（即将推出）".into(),
    );

    m.insert(
        "cmd.config.lang.unsupported".into(),
        "不支持的语言 '{}'。可用：{}".into(),
    );

    m.insert(
        "cmd.config.local.create_failed".into(),
        "创建 ntconfig.toml 失败：{}".into(),
    );

    m.insert(
        "cmd.config.local.created_template".into(),
        "已在 {} 创建 ntconfig.toml 模板".into(),
    );

    m.insert(
        "cmd.config.local.exists_warning".into(),
        "\u{26a0}\u{fe0f} ntconfig.toml 已存在！".into(),
    );

    m.insert(
        "cmd.config.local.init_cancelled".into(),
        "初始化已取消。".into(),
    );

    m.insert(
        "cmd.config.local.overwrite_prompt".into(),
        "是否覆盖？(y/N)：".into(),
    );

    m.insert(
        "cmd.config.local.template_edit_hint".into(),
        " 编辑文件并删除想启用的行前的 '#'".into(),
    );

    m.insert(
        "cmd.config.local.template_hint".into(),
        "\u{1f4dd} 这是一个已注释的模板 — 尚无活动设置".into(),
    );

    m.insert(
        "cmd.config.local.tip_global".into(),
        "\u{1f4a1} 提示：全局设置（传送点、输出路径、深度）保持全局".into(),
    );

    m.insert(
        "cmd.config.local_deinit.cancelled".into(),
        "反初始化已取消。".into(),
    );

    m.insert(
        "cmd.config.local_deinit.confirm".into(),
        "删除 ntconfig.toml？（y/N）：".into(),
    );

    m.insert(
        "cmd.config.local_deinit.done".into(),
        "ntconfig.toml 已删除。全局配置现在是唯一的活动配置。".into(),
    );

    m.insert(
        "cmd.config.local_deinit.global_safe".into(),
        "全局配置不会受影响。".into(),
    );

    m.insert(
        "cmd.config.local_deinit.lose_settings".into(),
        "本地忽略/关注和运行别名设置将丢失。".into(),
    );

    m.insert(
        "cmd.config.local_deinit.not_found".into(),
        "当前目录中未找到 ntconfig.toml。".into(),
    );

    m.insert(
        "cmd.config.local_deinit.remove_failed".into(),
        "删除 ntconfig.toml 失败：{}".into(),
    );

    m.insert(
        "cmd.config.local_deinit.title".into(),
        "\u{26A0}\u{FE0F} 本地配置删除".into(),
    );

    m.insert(
        "cmd.config.local_deinit.will_delete".into(),
        "此操作将删除：{}".into(),
    );

    m.insert(
        "cmd.config.opencg.created".into(),
        "已在以下位置创建配置文件：{}".into(),
    );

    m.insert(
        "cmd.config.opencg.opening".into(),
        "正在打开配置：{}".into(),
    );

    m.insert("cmd.config.qsc.disabled".into(), "QSC热键已禁用。".into());

    m.insert(
        "cmd.config.qsc.hotkey_set".into(),
        "QSC热键已设置为：{}".into(),
    );

    m.insert("cmd.config.qsc.hotkey_status".into(), "QSC热键：{}".into());

    m.insert(
        "cmd.config.qsc.invalid_format".into(),
        "无效的QSC格式：'{}'。使用 '#c #s 0' 样式。".into(),
    );

    m.insert("cmd.config.qsc.not_configured".into(), "未配置".into());

    m.insert(
        "cmd.config.qsc.usage1".into(),
        "用法：config qsc \"#c 0\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage2".into(),
        " config qsc \"#c #s a\"".into(),
    );

    m.insert(
        "cmd.config.qsc.usage3".into(),
        " config qsc \"#c #s #a F1\"".into(),
    );

    m.insert("cmd.config.qsc.usage4".into(), " config qsc off".into());

    m.insert(
        "cmd.config.resetcg.backup_created".into(),
        "已创建备份：{}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_failed".into(),
        "创建备份失败：{}".into(),
    );

    m.insert(
        "cmd.config.resetcg.backup_info".into(),
        "备份已保存。如有需要，使用 'restorecg' 恢复。".into(),
    );

    m.insert("cmd.config.resetcg.cancelled".into(), "重置已取消。".into());

    m.insert(
        "cmd.config.resetcg.config_file_label".into(),
        "配置文件：".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_aliases".into(),
        " \u{2022} 运行别名 \u{2192} 已清除".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_custom_ignores".into(),
        " \u{2022} 自定义忽略 \u{2192} 已移除".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_depth".into(),
        " \u{2022} 最大深度 \u{2192} 2".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_history".into(),
        " \u{2022} 历史记录 \u{2192} 关闭".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_ignored_dirs".into(),
        " \u{2022} 忽略目录 \u{2192} target, build, venv, node_modules, installer, logs, .git"
            .into(),
    );

    m.insert(
        "cmd.config.resetcg.default_linenumbers".into(),
        " \u{2022} 行号 \u{2192} 关闭".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_output".into(),
        " \u{2022} 输出路径 \u{2192} 桌面".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_teleports".into(),
        " \u{2022} 传送点 \u{2192} 已清除".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_threads".into(),
        " \u{2022} 线程数 \u{2192} 4".into(),
    );

    m.insert(
        "cmd.config.resetcg.default_watcher".into(),
        " \u{2022} 文件监视 \u{2192} 关闭".into(),
    );

    m.insert(
        "cmd.config.resetcg.not_found".into(),
        "未找到配置文件。无需重置。".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_backup".into(),
        " \u{2022} 输入 'backup' - 重置前创建备份".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_no".into(),
        " \u{2022} 输入 'no' - 取消".into(),
    );

    m.insert(
        "cmd.config.resetcg.opt_yes".into(),
        " \u{2022} 输入 'yes' - 重置配置（不备份）".into(),
    );

    m.insert("cmd.config.resetcg.options_label".into(), "选项：".into());

    m.insert(
        "cmd.config.resetcg.reset_done".into(),
        "配置已重置为默认值！".into(),
    );

    m.insert(
        "cmd.config.resetcg.serialize_failed".into(),
        "序列化默认配置失败".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_desc".into(),
        "此操作将把配置重置为默认值：".into(),
    );

    m.insert(
        "cmd.config.resetcg.warning_title".into(),
        "\u{26A0}\u{FE0F} 配置重置警告".into(),
    );

    m.insert(
        "cmd.config.resetcg.write_failed".into(),
        "写入配置失败：{}".into(),
    );

    m.insert("cmd.config.resetcg.your_choice".into(), "您的选择：".into());

    m.insert(
        "cmd.config.restorecg.backed_up".into(),
        "当前配置已备份到：{}".into(),
    );

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. 取消".into());

    m.insert(
        "cmd.config.restorecg.cancelled".into(),
        "恢复已取消。".into(),
    );

    m.insert("cmd.config.restorecg.failed".into(), "恢复失败：{}".into());

    m.insert("cmd.config.restorecg.invalid".into(), "无效的选择。".into());

    m.insert(
        "cmd.config.restorecg.none".into(),
        "未找到备份文件。".into(),
    );

    m.insert(
        "cmd.config.restorecg.reloaded".into(),
        "配置已重新加载！运行 'showcg' 查看恢复的设置。".into(),
    );

    m.insert(
        "cmd.config.restorecg.restored".into(),
        "已从以下位置恢复配置：{}".into(),
    );

    m.insert(
        "cmd.config.restorecg.title".into(),
        "\u{1F4CB} 可用备份：".into(),
    );

    m.insert("cmd.config.mgmt_header".into(), "管理：".into());

    m.insert("cmd.config.settings_header".into(), "设置：".into());

    m.insert(
        "cmd.config.showcg.lang_coming_soon".into(),
        "{}（即将推出）".into(),
    );

    m.insert("cmd.config.showcg.lang_label".into(), "语言：".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "QSC热键：".into());

    m.insert(
        "cmd.config.title".into(),
        "config \u{2014} 查看或更改配置设置".into(),
    );

    m.insert(
        "cmd.config.watch.alias_not_found".into(),
        "未找到别名 '{}'。使用 'ral list' 查看别名。".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_cleared".into(),
        "监视器触发别名已清除。".into(),
    );

    m.insert("cmd.config.watch.trigger_label".into(), "触发别名：".into());

    m.insert(
        "cmd.config.watch.trigger_off_usage".into(),
        "config watch trigger off 禁用自动运行".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_set".into(),
        "监视器触发已设置为别名：'{}'".into(),
    );

    m.insert(
        "cmd.config.watch.trigger_usage".into(),
        "config watch trigger <别名> 更改时自动运行别名".into(),
    );

    m.insert(
        "cmd.config.watch.limit_usage".into(),
        "config watch limit <数量> 更改框中显示的行数".into(),
    );

    m.insert("cmd.config.watch.usage".into(), "使用：config watch 开 | config watch 关 | config watch trigger <别名> | config watch trigger off | config watch limit <数量>".into());

    m.insert(
        "cmd.config.watch.usage_line".into(),
        "用法：config watch on|off".into(),
    );

    m.insert(
        "cmd.config.serve.title".into(),
        "Serve 配置（各服务默认值）".into(),
    );

    m.insert("cmd.config.serve.status".into(), "  {}: {}:{}".into());

    m.insert("cmd.config.serve.usage".into(), "用法：serve config [files|http-server|chess] [--port-default <端口>] [--bind-default <地址>]（别名：config serve …）".into());

    m.insert(
        "cmd.config.serve.example_show".into(),
        "  serve config                  显示所有服务".into(),
    );

    m.insert(
        "cmd.config.serve.example_show_one".into(),
        "  serve config files            显示文件服务默认值".into(),
    );

    m.insert(
        "cmd.config.serve.example_set".into(),
        "  serve config files --port-default 9000   设置文件服务默认端口".into(),
    );

    m.insert(
        "cmd.config.serve.saved".into(),
        "Serve {} 默认值已保存：{}:{}".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_service".into(),
        "未知服务 '{}'（请用 files|http-server|chess）".into(),
    );

    m.insert(
        "cmd.config.serve.invalid_port".into(),
        "无效端口 '{}'（请用 1-65535）".into(),
    );

    m.insert(
        "cmd.config.serve.empty_bind".into(),
        "--bind-default 缺少值".into(),
    );

    m.insert("cmd.config.serve.unknown_option".into(), "未知：{}".into());

    m.insert(
        "cmd.config.local.init_all_removed".into(),
        "'local init --all' 已被移除 —— 本地配置不再复制全局设置。请使用 'local init' 生成模板。"
            .into(),
    );

    m.insert(
        "cmd.config.local.precedence_hint".into(),
        "本地条目优先；全局条目继续生效，全局配置不会被修改。".into(),
    );
}
