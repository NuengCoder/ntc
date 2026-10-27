use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "alias.ral.add_cmd".into(),
        " ral add <名称> <命令> 创建新的运行别名".into(),
    );

    m.insert(
        "alias.ral.alias_count".into(),
        "您定义了 {} 个别名：".into(),
    );

    m.insert("alias.ral.and_more".into(), "... 及另外 {} 个".into());

    m.insert(
        "alias.ral.array_cmd".into(),
        " ral add <名称>(x[],y) <命令> 创建带数组参数的别名（使用 $x[].ext 或 ${{x}}[].ext）"
            .into(),
    );

    m.insert("alias.ral.cls_cancelled".into(), "清除已取消。".into());

    m.insert(
        "alias.ral.cls_cmd".into(),
        " ral cls 清除所有别名（需确认）".into(),
    );

    m.insert(
        "alias.ral.cls_prompt".into(),
        "确定吗？输入 'yes' 确认：".into(),
    );

    m.insert(
        "alias.ral.cls_warning".into(),
        "\u{26A0}\u{FE0F} 警告：这将删除所有运行别名！".into(),
    );

    m.insert("alias.ral.config_path".into(), " 配置：{}".into());

    m.insert("alias.ral.create_failed".into(), "创建文件失败：{}".into());

    m.insert(
        "alias.ral.default_cmd".into(),
        " ral add <名称>({{x=a,e=txt}}) <命令> 创建带默认值的别名".into(),
    );

    m.insert(
        "alias.ral.edit_cmd".into(),
        " ral edit <名称> <命令> 更新现有别名".into(),
    );

    m.insert(
        "alias.ral.empty".into(),
        "未定义运行别名。使用 'ral add <名称> <命令>'".into(),
    );

    m.insert("alias.ral.examples_header".into(), "示例：".into());

    m.insert(
        "alias.ral.export_all_cmd".into(),
        " ral export --all <名称> 将所有别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_import_header".into(),
        "导出/导入：".into(),
    );

    m.insert(
        "alias.ral.export_name_empty".into(),
        "导出名称不能为空。".into(),
    );

    m.insert(
        "alias.ral.export_select_cmd".into(),
        " ral export --select <名称> 选择别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert("alias.ral.file_exists".into(), "文件已存在：{}".into());

    m.insert(
        "alias.ral.import_cmd".into(),
        " ral import <文件> 从 .ntc.ral 文件导入别名".into(),
    );

    m.insert(
        "alias.ral.info_cmd".into(),
        " ral info <名称> [--edit] 显示别名详情（--edit 将编辑命令复制到剪贴板）".into(),
    );

    m.insert("alias.ral.info_command".into(), "命令".into());

    m.insert("alias.ral.info_name".into(), "名称".into());

    m.insert("alias.ral.info_none".into(), "无".into());

    m.insert("alias.ral.info_params".into(), "参数".into());

    m.insert("alias.ral.info_hints".into(), "补全提示".into());

    m.insert("alias.ral.hints_saved".into(), "补全提示：{}".into());

    m.insert(
        "alias.ral.hint_error".into(),
        "补全提示有误（{}）：别名未保存。".into(),
    );

    m.insert("alias.ral.hint_usage".into(), "提示形如：#x --hint-dir  #y --hint-tp --hint-dir（--hint-tp|--hint-ext [列表]|--hint-ext-name [列表]|--hint-spf|--hint-file|--hint-file-name|--hint-dir|--hint-fd|--hint-all） 选择形如：#x --select-dir  #x --select-ext rs,txt（--select-dir|--select-ext [列表]|--select-ext-name [列表]|--select-file|--select-file-name|--select-fd|--select-tp） 内联值：#x --hint [8080,5000]  命名列表：#x --hint-n port（见 ral hintn）".into());

    m.insert(
        "alias.ral.hint_cmd".into(),
        " ral add <名称> <命令> #x --hint-dir  将参数 x 按目录补全".into(),
    );

    m.insert(
        "alias.ral.select_cmd".into(),
        " ral add <名称> <命令> #x --select-dir  缺参数时交互选择 x".into(),
    );

    m.insert(
        "alias.ral.hintn_cmd".into(),
        " ral hintn add|rm|rnm|info|ls  管理命名值列表，供 #x --hint-n <列表> 使用".into(),
    );

    m.insert("alias.ral.usage_hintn".into(), "用法：ral hintn add <名称> [值] | ral hintn rm <名称>[, ...] [--force] | ral hintn rnm <旧> to <新> | ral hintn info <名称> | ral hintn ls".into());

    m.insert(
        "alias.ral.usage_hintn_add".into(),
        "用法：ral hintn add <名称> [值, ...]（含空格的值加引号：[\"what is this\", ok]）".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_rm".into(),
        "用法：ral hintn rm <名称>[, ...] [--force]".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_rnm".into(),
        "用法：ral hintn rnm <旧名称> to <新名称>".into(),
    );

    m.insert(
        "alias.ral.usage_hintn_info".into(),
        "用法：ral hintn info <名称>".into(),
    );

    m.insert(
        "alias.ral.hintn_added".into(),
        "提示列表 '{}' 已保存（{} 个值）：{}".into(),
    );

    m.insert(
        "alias.ral.hintn_removed".into(),
        "提示列表 '{}' 已删除。".into(),
    );

    m.insert(
        "alias.ral.hintn_not_found".into(),
        "找不到提示列表 '{}'。".into(),
    );

    m.insert(
        "alias.ral.hintn_renamed".into(),
        "提示列表 '{}' 已重命名为 '{}'。".into(),
    );

    m.insert(
        "alias.ral.hintn_empty".into(),
        "暂无提示列表。使用 'ral hintn add <名称> [值]' 创建。".into(),
    );

    m.insert(
        "alias.ral.hintn_referenced".into(),
        "提示列表 '{}' 正被使用：{}。加 --force 仍要删除。".into(),
    );

    m.insert(
        "alias.ral.hintn_info_title".into(),
        "📌 提示列表信息".into(),
    );

    m.insert("alias.ral.hintn_info_name".into(), "名称".into());

    m.insert("alias.ral.hintn_info_values".into(), "值".into());

    m.insert("alias.ral.hintn_info_used_by".into(), "被引用".into());

    m.insert("alias.ral.hintn_value_count".into(), "{} 个值".into());

    m.insert(
        "alias.ral.unknown_hint_list".into(),
        "未知的提示列表：'{}'。".into(),
    );

    m.insert(
        "alias.ral.hintn_create_hint".into(),
        "请先创建：ral hintn add {} [值]".into(),
    );

    m.insert(
        "alias.ral.arr_sep_error".into(),
        "错误：{} 有 {} 个字符在数组 {} 中，请用 $x[,] 或 $x[c]".into(),
    );

    m.insert(
        "alias.ral.example_hint_ext".into(),
        "ral add opimg \"image display ${x}\" #x --hint-ext png,jpeg,jpg,ico,bmp".into(),
    );

    m.insert(
        "alias.ral.example_rest".into(),
        "ral add hi \"echo $*{x}\"    # hi \"wow nice\" -> echo wow nice".into(),
    );

    m.insert(
        "alias.ral.example_hint_array".into(),
        "ral add runc \"gcc -o ${o}.exe $x[] && .\\${o}.exe && rmf ${o}.exe\" #x[] --hint-ext c"
            .into(),
    );

    m.insert(
        "alias.ral.example_select_dir".into(),
        "ral add tldsel \"tl dir ${x:-.}\" #x --select-dir".into(),
    );

    m.insert(
        "alias.ral.example_hint_values".into(),
        "ral add srv \"ssh $p\" #p --hint [8080,5050,5000]".into(),
    );

    m.insert(
        "alias.ral.example_hintn".into(),
        "ral hintn add port [8080,5050,5000] + ral add srvn \"ssh $p\" #p --hint-n port".into(),
    );

    m.insert(
        "alias.ral.example_hint_sep".into(),
        "数组连接：[a,b] 上的 $x[,] 得出 \"a , b\"（$x[,].c 则每项加后缀）".into(),
    );

    m.insert("alias.ral.info_source".into(), "来源".into());

    m.insert("alias.ral.info_source_global".into(), "全局".into());

    m.insert(
        "alias.ral.info_source_local".into(),
        "本地（ntconfig.toml）".into(),
    );

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} 别名信息".into());

    m.insert(
        "alias.ral.info_edit_copied".into(),
        "编辑命令已复制到剪贴板：".into(),
    );

    m.insert(
        "alias.ral.info_edit_hint".into(),
        "粘贴它，调整命令，然后按回车。".into(),
    );

    m.insert(
        "alias.ral.invalid_name".into(),
        "无效的别名名称：'{}'".into(),
    );

    m.insert("alias.ral.invalid_suffix".into(), "{}（无效）".into());

    m.insert("alias.ral.list_cmd".into(), " ral list 显示所有别名".into());

    m.insert(
        "alias.ral.make_cmd".into(),
        " ral make [名称] 在当前目录创建别名模板".into(),
    );

    m.insert("alias.ral.name_rule_at".into(), " - 以 @ 或 # 开头".into());

    m.insert(
        "alias.ral.name_rule_reserved".into(),
        " - 是保留命令（go, view, txt 等）".into(),
    );

    m.insert("alias.ral.name_rules".into(), "别名名称不能：".into());

    m.insert(
        "alias.ral.no_aliases".into(),
        "没有要清除的运行别名。".into(),
    );

    m.insert("alias.ral.not_found".into(), "未找到别名 '{}'".into());

    m.insert("alias.ral.now_run".into(), "现在您可以运行：{}".into());

    m.insert(
        "alias.ral.now_run_arg".into(),
        "现在您可以运行：{}({})".into(),
    );

    m.insert(
        "alias.ral.now_run_defaults".into(),
        "现在您可以运行：{}(<参数>) （默认值：{}）".into(),
    );

    m.insert(
        "alias.ral.param_cmd".into(),
        " ral add <名称>(x) <命令> 创建参数化别名（在命令中使用 $x 或 ${{x}}）".into(),
    );

    m.insert(
        "alias.ral.rest_cmd".into(),
        " ral add <名称>(x) <命令> 使用 $*{x}  捕获一个参数（保留空格，去除引号）".into(),
    );

    m.insert(
        "alias.ral.renamed".into(),
        "别名 '{}' 已重命名为 '{}'".into(),
    );

    m.insert(
        "alias.ral.rm_cmd".into(),
        " ral rm <名称>[, <名称>, ...] 删除别名".into(),
    );

    m.insert(
        "alias.ral.rnm_cmd".into(),
        " ral rnm <旧名称> to <新名称> 重命名别名".into(),
    );

    m.insert(
        "alias.ral.template_created".into(),
        "已创建 ral 模板：{}".into(),
    );

    m.insert(
        "alias.ral.template_info".into(),
        "这是一个本地模板文件。使用 'ral import' 将别名添加到配置。".into(),
    );

    m.insert("alias.ral.title".into(), "运行别名（ral）命令：".into());

    m.insert(
        "alias.ral.type_ral_help".into(),
        "输入 'ral' 获取帮助。".into(),
    );

    m.insert(
        "alias.ral.unknown_flag".into(),
        "未知的导出标志：{}。使用 --all 或 --select".into(),
    );

    m.insert(
        "alias.ral.unknown_subcmd".into(),
        "未知的 ral 子命令：{}".into(),
    );

    m.insert(
        "alias.ral.usage_add".into(),
        "用法：ral add <名称> <命令>".into(),
    );

    m.insert(
        "alias.ral.usage_edit".into(),
        "用法：ral edit <名称> <新命令>".into(),
    );

    m.insert(
        "alias.ral.usage_export".into(),
        "用法：ral export --all <名称> | ral export --select <名称>".into(),
    );

    m.insert(
        "alias.ral.usage_hint".into(),
        "用法：<别名> 或 <别名>(<参数>) 用于参数化别名".into(),
    );

    m.insert(
        "alias.ral.usage_import".into(),
        "用法：ral import <文件>".into(),
    );

    m.insert(
        "alias.ral.usage_info".into(),
        "用法：ral info <别名> [--edit]".into(),
    );

    m.insert(
        "alias.ral.usage_rm".into(),
        "用法：ral rm <名称>[, <名称>, ...]".into(),
    );

    m.insert(
        "alias.ral.usage_rnm".into(),
        "用法：ral rnm <旧名称> to <新名称>".into(),
    );

    m.insert(
        "alias.ral.usage_with_run".into(),
        "使用 run 的用法：".into(),
    );

    m.insert(
        "alias.ral.run_example1".into(),
        "  run btr              # 执行：cargo build --release".into(),
    );

    m.insert(
        "alias.ral.run_example2".into(),
        "  run py               # 执行：python test.py".into(),
    );

    m.insert(
        "alias.ral.run_example3".into(),
        "  run_file(hello)      # 执行：python hello.py".into(),
    );

    m.insert("alias.ral.run_example4".into(), "  runc [add , minus , mul] math  # 执行：cls && gcc -o math.exe add.c minus.c mul.c && ...".into());

    m.insert(
        "alias.ral.export_example1".into(),
        "  ral export --all myaliases       # 创建 myaliases.ntc.ral".into(),
    );

    m.insert(
        "alias.ral.export_example2".into(),
        "  ral export --select myaliases    # 交互选择并导出".into(),
    );

    m.insert(
        "alias.ral.export_example3".into(),
        "  ral import myaliases.ntc.ral     # 导入到当前配置".into(),
    );

    m.insert("alias.run.examples_header".into(), "示例：".into());

    m.insert(
        "alias.run.example_alias".into(),
        "  run btr                     # 运行别名".into(),
    );

    m.insert(
        "alias.run.example_alias_args".into(),
        "  run py test.py              # 带参数运行别名".into(),
    );

    m.insert(
        "alias.run.example_param".into(),
        "  run_file(hello)             # 运行带参数的别名".into(),
    );

    m.insert(
        "alias.run.example_real".into(),
        "  run python --version        # 运行真实命令".into(),
    );

    m.insert(
        "alias.run.usage".into(),
        "用法：run <命令|别名> [参数...]".into(),
    );

    m.insert("alias.ral.section_global".into(), "[全局]".into());

    m.insert("alias.ral.section_local".into(), "[本地]".into());

    m.insert(
        "alias.ral.no_local".into(),
        "（当前目录没有本地配置）".into(),
    );

    m.insert("alias.ral.none".into(), "（无）".into());

    m.insert("alias.ral.shadowed".into(), "（被本地覆盖）".into());

    m.insert(
        "alias.ral.shadowed_global".into(),
        "全局条目（被本地覆盖）：".into(),
    );

    m.insert(
        "alias.ral.global_active_again".into(),
        "全局别名 '{}' 仍然存在，现在重新生效。".into(),
    );

    m.insert(
        "alias.ral.global_only_hint".into(),
        "别名 '{}' 只定义在全局配置中。使用 'ral rm -g {}' 从全局配置中删除。".into(),
    );

    m.insert(
        "alias.ral.local_priority_note".into(),
        "注意：ntconfig.toml 中的本地别名 '{}' 优先于这个全局别名。".into(),
    );

    m.insert("alias.ral.scope_local".into(), "本地配置".into());

    m.insert("alias.ral.scope_global".into(), "全局配置".into());

    m.insert("alias.ral.cls_target".into(), "目标：{}".into());
}
