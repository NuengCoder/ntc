use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "autoup.added".into(),
        "已添加启动条目：{} \u{2192} {}".into(),
    );

    m.insert(
        "autoup.already_disabled".into(),
        "条目 '{}' 已禁用。".into(),
    );

    m.insert("autoup.already_enabled".into(), "条目 '{}' 已启用。".into());

    m.insert(
        "autoup.already_exists".into(),
        "条目 '{}' 已存在于用户启动中。".into(),
    );

    m.insert("autoup.cmd_empty".into(), "命令值不能为空。".into());

    m.insert("autoup.disabled".into(), "已禁用启动条目：{}".into());

    m.insert(
        "autoup.disabled_system".into(),
        "已禁用系统启动条目：{}".into(),
    );

    m.insert("autoup.enabled".into(), "已启用启动条目：{}".into());

    m.insert(
        "autoup.enabled_system".into(),
        "已启用系统启动条目：{}".into(),
    );

    m.insert(
        "autoup.help_add".into(),
        " autoup add <名称> \"<命令>\" 添加启动条目（HKCU）".into(),
    );

    m.insert(
        "autoup.help_disable".into(),
        " autoup disable <名称> 禁用启动条目".into(),
    );

    m.insert(
        "autoup.help_enable".into(),
        " autoup enable <名称> 重新启用禁用的条目".into(),
    );

    m.insert("autoup.help_help".into(), " autoup help 显示此帮助".into());

    m.insert(
        "autoup.help_list".into(),
        " autoup list 列出所有来源的启动条目".into(),
    );

    m.insert(
        "autoup.help_rm".into(),
        " autoup rm <名称> 删除启动条目".into(),
    );

    m.insert(
        "autoup.hklm_hint".into(),
        "'{}' 是系统条目（HKLM）。先运行 'mode admin'，然后重试。".into(),
    );

    m.insert(
        "autoup.hklm_hint_short".into(),
        "'{}' 是系统条目。先运行 'mode admin'，然后重试。".into(),
    );

    m.insert("autoup.list_empty".into(), "（未找到启动条目）".into());

    m.insert("autoup.list_title".into(), "Windows 启动条目".into());

    m.insert(
        "autoup.list_total".into(),
        "总计：{} [A]:{} [D]:{} （CU:{} LM:{} SF:{}）".into(),
    );

    m.insert("autoup.not_found".into(), "未找到启动条目：{}".into());

    m.insert(
        "autoup.reg_delete_fail".into(),
        "删除注册表条目失败。".into(),
    );

    m.insert(
        "autoup.reg_disable_fail".into(),
        "设置禁用状态失败。".into(),
    );

    m.insert(
        "autoup.reg_exec_fail".into(),
        "运行 reg.exe 失败：{}".into(),
    );

    m.insert("autoup.reg_query_fail".into(), "查询注册表失败：{}".into());

    m.insert(
        "autoup.reg_write_fail".into(),
        "写入注册表条目失败。请尝试以管理员身份运行。".into(),
    );

    m.insert("autoup.removed".into(), "已删除启动条目：{}".into());

    m.insert(
        "autoup.removed_system".into(),
        "已删除系统启动条目：{}".into(),
    );

    m.insert(
        "autoup.sf_disable_hint".into(),
        "'{}' 在启动文件夹中。将快捷方式移出以禁用它。".into(),
    );

    m.insert(
        "autoup.sf_rm_hint".into(),
        "'{}' 在启动文件夹中。请手动删除快捷方式文件。".into(),
    );

    m.insert(
        "autoup.source_cu".into(),
        " [CU] 当前用户 （HKCU\\...\\Run）".into(),
    );

    m.insert(
        "autoup.source_lm".into(),
        " [LM] 本地机器 （HKLM\\...\\Run）\u{2014} 需要管理员权限（mode admin）".into(),
    );

    m.insert(
        "autoup.source_sf".into(),
        " [SF] 启动文件夹（%APPDATA%\\...\\Startup）\u{2014} 只读".into(),
    );

    m.insert("autoup.source_tags".into(), "来源标签：".into());

    m.insert(
        "autoup.usage_add".into(),
        "用法：autoup add <名称> \"<命令>\"".into(),
    );

    m.insert(
        "autoup.usage_disable".into(),
        "用法：autoup disable <名称>".into(),
    );

    m.insert(
        "autoup.usage_enable".into(),
        "用法：autoup enable <名称>".into(),
    );

    m.insert("autoup.usage_examples".into(), "示例：".into());

    m.insert("autoup.usage_rm".into(), "用法：autoup rm <名称>".into());

    m.insert("autoup.usage_title".into(), "用法：".into());

    m.insert(
        "autoup.use_rm_first".into(),
        "如果要替换，请先使用 'autoup rm'。".into(),
    );

    m.insert(
        "autoup.will_run".into(),
        "该条目将在下次登录时运行。".into(),
    );

    m.insert(
        "autoup.windows_only".into(),
        "autoup 是仅限 Windows 的功能。".into(),
    );
}
