use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ran.already_exists".into(), "{} 已存在于 {} 中".into());

    m.insert(
        "ran.calling_not_allowed".into(),
        "不允许在 NTCRANFILE 目标中调用 'ran'".into(),
    );

    m.insert("ran.completed".into(), "命令已成功完成。".into());

    m.insert("ran.created".into(), "已创建 {}".into());

    m.insert(
        "ran.desc_aliases".into(),
        " 命令通过 ntc 的运行别名系统展开，因此别名也可以工作。".into(),
    );

    m.insert(
        "ran.desc_aliases2".into(),
        " 添加 'run ' 前缀以绕过别名展开，直接作为系统命令运行。".into(),
    );

    m.insert(
        "ran.desc_deps".into(),
        " 目标可以依赖其他目标。依赖项按顺序先运行。".into(),
    );

    m.insert(
        "ran.desc_note".into(),
        " 不允许在 NTCRANFILE 目标命令中调用 'ran'。".into(),
    );

    m.insert(
        "ran.desc_reserved".into(),
        " init, deinit, help, list, ls, check, fmt \u{2014} 这些不能用作目标名称。".into(),
    );

    m.insert(
        "ran.desc_vars".into(),
        " 在 [vars] 下定义变量，并在命令中使用 $(VAR_NAME) 引用。".into(),
    );

    m.insert(
        "ran.desc_vars2".into(),
        " 它们像 Make 变量一样工作，但使用 $(...) 语法。".into(),
    );

    m.insert("ran.error_prefix".into(), "ran：{}".into());

    m.insert("ran.exec_failed".into(), "执行命令失败：{}".into());

    m.insert("ran.executing".into(), "正在执行：{}".into());

    m.insert("ran.exit_code".into(), "命令退出代码：{}".into());

    m.insert("ran.get_cwd_failed".into(), "获取当前目录失败：{}".into());

    m.insert(
        "ran.help_deinit_fmt".into(),
        " ran deinit 从当前目录删除 {}".into(),
    );

    m.insert("ran.help_help".into(), " ran help 显示此帮助".into());

    m.insert(
        "ran.help_init_fmt".into(),
        " ran init 在当前目录创建 {}".into(),
    );

    m.insert("ran.help_list".into(), " ran list 列出所有可用目标".into());

    m.insert(
        "ran.help_standalone".into(),
        " ran 运行 'standalone' 目标（默认）".into(),
    );

    m.insert("ran.help_target".into(), " ran <目标> 运行特定目标".into());

    m.insert(
        "ran.help_title".into(),
        "ran \u{2014} ntc 任务运行器（类似 Make 但 ntc 原生）".into(),
    );

    m.insert("ran.help_usage".into(), "用法：".into());

    m.insert("ran.not_found".into(), "在 {} 中未找到 {}".into());

    m.insert("ran.remove_failed".into(), "删除 {} 失败：{}".into());

    m.insert("ran.removed".into(), "已删除 {}".into());

    m.insert("ran.section_aliases".into(), "运行别名：".into());

    m.insert("ran.section_deps".into(), "依赖：".into());

    m.insert("ran.section_note".into(), "注意：".into());

    m.insert("ran.section_reserved".into(), "保留目标名称：".into());

    m.insert("ran.section_vars".into(), "变量：".into());

    m.insert("ran.terminated".into(), "命令已终止（Ctrl+C）".into());

    m.insert("ran.check_title".into(), "ran check 正在解析 {}".into());

    m.insert("ran.check_ok".into(), "{} 没有问题".into());

    m.insert("ran.check_problems".into(), "在 {} 中发现 {} 个问题".into());

    m.insert("ran.check_error".into(), "错误".into());

    m.insert("ran.check_hint".into(), "提示".into());

    m.insert(
        "ran.help_check_fmt".into(),
        " ran check 解析 {} 并报告每个问题所在行".into(),
    );

    m.insert(
        "ran.help_fmt".into(),
        " ran fmt <toml|json|yaml> 转换 ranfile 格式（-o <文件>，-y 跳过询问）".into(),
    );

    m.insert(
        "ran.fmt.usage".into(),
        "用法：ran fmt <toml|json|yaml> [-o <文件>] [-y]".into(),
    );

    m.insert(
        "ran.fmt.unknown".into(),
        "未知格式 '{}'。可用：toml、json、yaml。".into(),
    );

    m.insert(
        "ran.fmt.current".into(),
        "ranfile 格式偏好：{}（ran init 会创建 {}）".into(),
    );

    m.insert(
        "ran.fmt.no_file".into(),
        "此处没有 ranfile。偏好已设为 {}；运行 `ran init` 创建 {}。".into(),
    );

    m.insert("ran.fmt.already".into(), "{} 已经是 {} 格式。".into());

    m.insert(
        "ran.fmt.confirm".into(),
        "将 {} 转换为 {} \u{2192} {}？[y|n] ".into(),
    );

    m.insert(
        "ran.fmt.skipped".into(),
        "未写入任何文件。`ran init` 将创建 {}。".into(),
    );

    m.insert(
        "ran.fmt.exists".into(),
        "{} 已存在 \u{2014} 请先删除或使用 -o <文件>。".into(),
    );

    m.insert("ran.fmt.written".into(), "已写入 {}".into());

    m.insert(
        "ran.fmt.old_kept".into(),
        "{} 仍然存在；`ran` 现在优先使用 {}（删除旧文件可避免两个 ranfile）。".into(),
    );

    m.insert(
        "ran.fmt.no_terminal".into(),
        "非终端环境：请加 -y 确认转换。".into(),
    );
}
