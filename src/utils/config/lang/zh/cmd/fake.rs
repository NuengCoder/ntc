use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.fake.usage_title".into(),
        "fake - 生成假数据/随机数据".into(),
    );

    m.insert(
        "cmd.fake.usage_gen".into(),
        "  fake gen <类型> [-n N] [-f 格式] [-s 种子] [-o 文件] [-g 分组] [-l 区域] [-u [列]]"
            .into(),
    );

    m.insert(
        "cmd.fake.usage_types".into(),
        "  fake types [-s] [模式]         列出所有类型（-s 模糊搜索）".into(),
    );

    m.insert(
        "cmd.fake.usage_locales".into(),
        "  fake locales                   列出支持的 --locale 代码".into(),
    );

    m.insert(
        "cmd.fake.usage_group".into(),
        "  fake group <子命令>            管理已保存的列分组".into(),
    );

    m.insert(
        "cmd.fake.usage_help".into(),
        "  fake help                      显示此帮助".into(),
    );

    m.insert("cmd.fake.options_title".into(), "选项（fake gen）：".into());

    m.insert(
        "cmd.fake.opt_count".into(),
        "  -n, --count <N>       生成的行数（默认 1）".into(),
    );

    m.insert(
        "cmd.fake.opt_format".into(),
        "  -f, --format <格式>   json | txt | sql | csv | xlsx（默认 txt）".into(),
    );

    m.insert(
        "cmd.fake.opt_seed".into(),
        "  -s, --seed <N>        随机种子，可复现输出".into(),
    );

    m.insert(
        "cmd.fake.opt_output".into(),
        "  -o, --output <文件>   写入文件（扩展名决定格式）".into(),
    );

    m.insert(
        "cmd.fake.opt_group".into(),
        "  -g, --group <名称>    从已保存的分组生成，替代 <类型>".into(),
    );

    m.insert(
        "cmd.fake.opt_locale".into(),
        "  -l, --locale <代码>   区域数据：en_US en_GB zh_CN th_TH lo_LA".into(),
    );

    m.insert(
        "cmd.fake.opt_unique".into(),
        "  -u, --unique [列]     使整行（或指定列）唯一".into(),
    );

    m.insert("cmd.fake.examples_header".into(), "示例：".into());

    m.insert(
        "cmd.fake.example1".into(),
        "  fake gen name,email,city -n 3".into(),
    );

    m.insert(
        "cmd.fake.example2".into(),
        "  fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "cmd.fake.example3".into(),
        "  fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.example4".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.unknown_sub".into(),
        "未知的 fake 子命令：'{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.wrote".into(),
        "已写入 {} 行到 {}（{}）".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_ext".into(),
        "未知的输出扩展名 '{}'（请用 -f 指定格式）".into(),
    );

    m.insert(
        "cmd.fake.gen.xlsx_needs_output".into(),
        "xlsx 输出需要 -o <文件.xlsx>（二进制工作簿无法打印到终端）".into(),
    );

    m.insert(
        "cmd.fake.gen.count_usage".into(),
        "fake gen：-n/--count 需要一个数字".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_count".into(),
        "fake gen：无效的行数 '{}'（必须为正整数）".into(),
    );

    m.insert(
        "cmd.fake.gen.format_usage".into(),
        "fake gen：-f/--format 需要一个值".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_format".into(),
        "fake gen：未知格式 '{}'（json|txt|sql|csv|xlsx）".into(),
    );

    m.insert(
        "cmd.fake.gen.seed_usage".into(),
        "fake gen：-s/--seed 需要一个数字".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_seed".into(),
        "fake gen：无效的种子 '{}'（必须为整数）".into(),
    );

    m.insert(
        "cmd.fake.gen.output_usage".into(),
        "fake gen：-o/--output 需要一个文件路径".into(),
    );

    m.insert(
        "cmd.fake.gen.group_usage".into(),
        "fake gen：-g/--group 需要一个分组名称".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_flag".into(),
        "fake gen：未知选项 '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.group_and_types".into(),
        "fake gen：<类型> 与 -g/--group 不能同时使用".into(),
    );

    m.insert(
        "cmd.fake.gen.need_input".into(),
        "fake gen：请给出一个或多个类型，或使用 -g/--group".into(),
    );

    m.insert(
        "cmd.fake.gen.unknown_type".into(),
        "fake gen：未知类型 '{}'（运行 'fake types' 查看全部）".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_with_group".into(),
        "fake gen：--unique 不能与 -g/--group 同时使用（请在分组中给列加上尾随 '!'）".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_unknown_col".into(),
        "fake gen：--unique 指定了未知列 '{}'".into(),
    );

    m.insert(
        "cmd.fake.gen.unique_exhausted".into(),
        "fake gen：无法为 '{}' 生成唯一值（取值空间太小）".into(),
    );

    m.insert(
        "cmd.fake.gen.locale_usage".into(),
        "fake gen：-l/--locale 需要一个区域代码（en_US en_GB zh_CN th_TH lo_LA）".into(),
    );

    m.insert(
        "cmd.fake.gen.bad_locale".into(),
        "fake gen：未知区域 '{}'（en_US en_GB zh_CN th_TH lo_LA）".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_cycle".into(),
        "fake gen：通过 '{}' 形成循环分组引用".into(),
    );

    m.insert(
        "cmd.fake.gen.ref_unknown_col".into(),
        "fake gen：在分组 '{}' 中找不到被引用的列 '{}'".into(),
    );

    m.insert(
        "cmd.fake.types.title".into(),
        "可用的 fake 类型（{}）：".into(),
    );

    m.insert("cmd.fake.types.none".into(), "没有类型匹配 '{}'。".into());

    m.insert(
        "cmd.fake.types.matches".into(),
        "匹配 '{}' 的类型（{}）：".into(),
    );

    m.insert("cmd.fake.locales.title".into(), "支持的区域（{}）：".into());

    m.insert(
        "cmd.fake.locales.note".into(),
        "用法：fake gen <类型> --locale <代码>".into(),
    );

    m.insert(
        "cmd.fake.group.usage_title".into(),
        "fake group - 管理已保存的列分组：".into(),
    );

    m.insert(
        "cmd.fake.group.help_add".into(),
        "  fake group add <名称> [别名 as 类型, ...]    创建分组".into(),
    );

    m.insert(
        "cmd.fake.group.help_edit".into(),
        "  fake group edit <名称> [别名 as 类型, ...]   替换分组的列".into(),
    );

    m.insert(
        "cmd.fake.group.help_rm".into(),
        "  fake group rm <名称>                          删除分组".into(),
    );

    m.insert(
        "cmd.fake.group.help_rnm".into(),
        "  fake group rnm <旧名> <新名>                  重命名分组".into(),
    );

    m.insert(
        "cmd.fake.group.help_ls".into(),
        "  fake group ls [模式]                          列出分组（通配符，如 u*）".into(),
    );

    m.insert(
        "cmd.fake.group.help_info".into(),
        "  fake group info <名称>                        显示分组的列".into(),
    );

    m.insert(
        "cmd.fake.group.help_clear".into(),
        "  fake group clear [-f]                         删除所有分组".into(),
    );

    m.insert(
        "cmd.fake.group.help_export".into(),
        "  fake group export <名称>|--all [-o 文件]      导出分组为 .ntc_fake".into(),
    );

    m.insert(
        "cmd.fake.group.help_import".into(),
        "  fake group import <文件> [--force] [--prefix p]  从文件导入分组".into(),
    );

    m.insert(
        "cmd.fake.group.example1".into(),
        "  fake group add users [username as name, email, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.example2".into(),
        "  fake gen -g users -f sql -n 10".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_sub".into(),
        "未知的 'fake group' 子命令：'{}'".into(),
    );

    m.insert(
        "cmd.fake.group.no_columns".into(),
        "至少给出一个列，例如 [username as name, year as age]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_column".into(),
        "无法解析列 '{}'（应为 '别名 as 类型'）".into(),
    );

    m.insert(
        "cmd.fake.group.unknown_type".into(),
        "未知类型 '{}'（运行 'fake types' 查看全部）".into(),
    );

    m.insert(
        "cmd.fake.group.add_usage".into(),
        "用法：fake group add <名称> [别名 as 类型, ...]".into(),
    );

    m.insert(
        "cmd.fake.group.bad_name".into(),
        "无效的分组名 '{}'（请使用字母、数字、'_' 或 '-'）".into(),
    );

    m.insert(
        "cmd.fake.group.not_found".into(),
        "没有名为 '{}' 的 fake 分组".into(),
    );

    m.insert(
        "cmd.fake.group.exists".into(),
        "fake 分组 '{}' 已存在".into(),
    );

    m.insert("cmd.fake.group.scope_local".into(), "本地".into());

    m.insert("cmd.fake.group.scope_global".into(), "全局".into());

    m.insert(
        "cmd.fake.group.added".into(),
        "已添加分组 '{}'（{} 列）到{}配置".into(),
    );

    m.insert(
        "cmd.fake.group.updated".into(),
        "已更新分组 '{}'（{} 列）于{}配置".into(),
    );

    m.insert(
        "cmd.fake.group.rm_usage".into(),
        "用法：fake group rm <名称>".into(),
    );

    m.insert("cmd.fake.group.removed".into(), "已删除分组 '{}'".into());

    m.insert(
        "cmd.fake.group.rnm_usage".into(),
        "用法：fake group rnm <旧名> <新名>".into(),
    );

    m.insert(
        "cmd.fake.group.renamed".into(),
        "已将分组 '{}' 重命名为 '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.ls_empty".into(),
        "尚未保存任何 fake 分组。".into(),
    );

    m.insert(
        "cmd.fake.group.ls_none".into(),
        "没有分组匹配 '{}'。".into(),
    );

    m.insert(
        "cmd.fake.group.info_usage".into(),
        "用法：fake group info <名称>".into(),
    );

    m.insert("cmd.fake.group.info_title".into(), "分组 '{}'：".into());

    m.insert("cmd.fake.group.info_count".into(), "{} 列".into());

    m.insert(
        "cmd.fake.group.clear_empty".into(),
        "没有可清除的 fake 分组。".into(),
    );

    m.insert(
        "cmd.fake.group.clear_prompt".into(),
        "删除 {} 个 fake 分组？[y/N] ".into(),
    );

    m.insert("cmd.fake.group.clear_cancelled".into(), "已取消。".into());

    m.insert(
        "cmd.fake.group.cleared".into(),
        "已清除 {} 个 fake 分组。".into(),
    );

    m.insert(
        "cmd.fake.group.bad_ref".into(),
        "无效引用 '{}'（应为 ref:<分组>.<列>）".into(),
    );

    m.insert(
        "cmd.fake.group.ref_col_missing".into(),
        "在分组 '{}' 中找不到被引用的列 '{}'".into(),
    );

    m.insert(
        "cmd.fake.group.export_usage".into(),
        "用法：fake group export <名称>|--all [-o 文件]".into(),
    );

    m.insert(
        "cmd.fake.group.import_usage".into(),
        "用法：fake group import <文件> [--force] [--prefix p]".into(),
    );

    m.insert(
        "cmd.fake.group.exported".into(),
        "已导出 {} 个分组到 {}".into(),
    );

    m.insert(
        "cmd.fake.group.import_ok".into(),
        "已导入 {} 个分组。".into(),
    );

    m.insert(
        "cmd.fake.group.import_exists".into(),
        "以下分组已存在：{}（使用 --force 覆盖）".into(),
    );

    m.insert(
        "cmd.fake.group.import_bad_file".into(),
        "无法导入分组：{}".into(),
    );
}
