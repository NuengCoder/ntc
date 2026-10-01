use std::collections::HashMap;



pub fn build_map() -> HashMap<String, String> {

    let mut m = HashMap::new();

    // ========== ndb（数据库外壳） ==========
    m.insert("ndb.banner".into(), "ndb {} - NTC 数据库外壳".into());
    m.insert("ndb.cwd_line".into(), "当前目录：{}".into());
    m.insert("ndb.no_db_files".into(), "当前目录中没有找到 .db 文件。".into());
    m.insert("ndb.no_db_files_hint".into(), "请在此处添加数据库文件，或 cd 进入包含数据库的目录。".into());
    m.insert("ndb.db_files_header".into(), "可用数据库（{}）：".into());
    m.insert("ndb.tables_count".into(), "{} 个表".into());
    m.insert("ndb.not_a_db".into(), "不是数据库".into());
    m.insert("ndb.pick_db".into(), "选择要打开的数据库".into());
    m.insert("ndb.pick_table".into(), "选择要显示的表".into());
    m.insert("ndb.pick_prompt".into(), "输入编号（1-{}）或 0：".into());
    m.insert("ndb.open_failed".into(), "打开 '{}' 失败：{}".into());
    m.insert("ndb.opened".into(), "已打开数据库：{}（只读）".into());
    m.insert("ndb.hint_next".into(), "可用：tables、viewf、tablef <表>、schema <表>、sql <查询>、exit。".into());
    m.insert("ndb.no_tables".into(), "此数据库没有表。".into());
    m.insert("ndb.query_failed".into(), "查询失败：{}".into());
    m.insert("ndb.unknown_cmd".into(), "未知的 ndb 命令：'{}'".into());
    m.insert("ndb.hint_help".into(), "输入 'help' 查看所有 ndb 命令。".into());
    m.insert("ndb.interface_usage".into(), "用法：interface <文件.db>（或用 'view' 列出数据库）".into());
    m.insert("ndb.sql_needs_db".into(), "请先打开数据库：interface <文件.db>".into());
    m.insert("ndb.sql_usage".into(), "用法：sql <查询>   例如 sql SELECT * FROM users".into());
    m.insert("ndb.sql_blocked".into(), "ndb 是只读的：仅允许 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into());
    m.insert("ndb.sql_error".into(), "SQL 错误：{}".into());
    m.insert("ndb.table_not_found".into(), "未找到表：{}".into());
    m.insert("ndb.schema_header".into(), "'{}' 的结构：".into());
    m.insert("ndb.schema_usage".into(), "用法：schema <表>".into());
    m.insert("ndb.showing_table".into(), "表：{}（只读）".into());
    m.insert("ndb.editor_fail".into(), "创建行编辑器失败：{}".into());
    m.insert("ndb.start_hint".into(), "输入 'help' 查看命令，输入 'exit' 返回 ntc 外壳。".into());
    m.insert("ndb.closed_db".into(), "数据库已关闭。".into());
    m.insert("ndb.bye".into(), "已退出 ndb 外壳。".into());
    m.insert("ndb.no_args".into(), "用法：ndb（打开 ndb 数据库外壳）".into());
    m.insert("ndb.null".into(), "NULL".into());
    m.insert("ndb.kind_view".into(), "视图".into());
    m.insert("ndb.kind_table".into(), "表".into());
    m.insert("ndb.columns_count".into(), "{} 列".into());
    m.insert("ndb.rows_count".into(), "{} 行".into());
    m.insert("ndb.rows_empty".into(), "0 行".into());
    m.insert("ndb.rows_shown".into(), "已显示 {} 行".into());
    m.insert("ndb.rows_truncated".into(), "仅显示前 {} 行 - 使用 --all 或 -l <n> 查看更多。".into());
    m.insert("ndb.no_columns".into(), "该语句没有返回列。".into());
    m.insert("ndb.col_name".into(), "名称".into());
    m.insert("ndb.col_kind".into(), "类型".into());
    m.insert("ndb.col_columns".into(), "列数".into());
    m.insert("ndb.col_rows".into(), "行数".into());
    m.insert("ndb.col_type".into(), "数据类型".into());
    m.insert("ndb.col_constraints".into(), "约束".into());

    // ========== ndb 外壳内帮助 ==========
    m.insert("ndb.help_root_title".into(), "NDB - 数据库外壳（未打开数据库）：".into());
    m.insert("ndb.help_root_help".into(), "help | h | ?              显示此命令列表".into());
    m.insert("ndb.help_root_view".into(), "view | ls                 列出当前目录中的 .db 文件".into());
    m.insert("ndb.help_root_interface".into(), "interface <文件.db>      打开数据库（提示符变为 ndb [文件.db]>）".into());
    m.insert("ndb.help_root_sql".into(), "sql                       打开数据库后可用（先 interface）".into());
    m.insert("ndb.help_root_clear".into(), "clear | cls               清屏并重新显示数据库".into());
    m.insert("ndb.help_root_exit".into(), "exit | q                  返回 ntc 外壳".into());
    m.insert("ndb.help_db_title".into(), "NDB - 数据库外壳（只读）：".into());
    m.insert("ndb.help_db_help".into(), "help | h | ?              显示此命令列表".into());
    m.insert("ndb.help_db_viewf".into(), "viewf | v                 显示结构树（表 + 列）".into());
    m.insert("ndb.help_db_tables".into(), "tables [筛选]             列出表和视图名称（可选筛选）".into());
    m.insert("ndb.help_db_tablef".into(), "tablef <表>               显示表的行（类似 txtf）".into());
    m.insert("ndb.help_db_schema".into(), "schema <表>               显示 CREATE SQL、列、索引和键".into());
    m.insert("ndb.help_db_sql".into(), "sql <查询>                执行只读查询（-l N、--all）".into());
    m.insert("ndb.help_db_write".into(), "write YES <语句>          执行一条写入（INSERT/UPDATE/DELETE，需先 edit on）".into());
    m.insert("ndb.help_db_edit".into(), "edit on|off              允许或禁止写入当前数据库".into());
    m.insert("ndb.help_db_backup".into(), "backup                    在旁边复制一份 .db 文件".into());
    m.insert("ndb.help_db_more".into(), "more | next             显示上次查询的下一页".into());
    m.insert("ndb.help_db_indexes".into(), "indexes [表]             列出索引（全部或指定表）".into());
    m.insert("ndb.help_db_info".into(), "info                    显示文件、引擎和对象统计".into());
    m.insert("ndb.help_db_interface".into(), "interface <文件.db>      切换到另一个数据库".into());
    m.insert("ndb.info_title".into(), "数据库信息：{}".into());
    m.insert("ndb.col_property".into(), "属性".into());
    m.insert("ndb.col_value".into(), "值".into());
    m.insert("ndb.info_file".into(), "文件".into());
    m.insert("ndb.info_size".into(), "大小".into());
    m.insert("ndb.info_sqlite".into(), "SQLite".into());
    m.insert("ndb.info_page".into(), "页大小".into());
    m.insert("ndb.info_pages".into(), "页数".into());
    m.insert("ndb.info_tables".into(), "表".into());
    m.insert("ndb.info_views".into(), "视图".into());
    m.insert("ndb.info_indexes".into(), "索引".into());
    m.insert("ndb.info_triggers".into(), "触发器".into());
    m.insert("ndb.indexes_title".into(), "索引（{}）：".into());
    m.insert("ndb.no_indexes".into(), "没有找到索引。".into());
    m.insert("ndb.col_table".into(), "表".into());
    m.insert("ndb.col_unique".into(), "唯一".into());
    m.insert("ndb.col_from".into(), "来源".into());
    m.insert("ndb.col_to".into(), "引用".into());
    m.insert("ndb.fkeys_title".into(), "外键：".into());
    m.insert("ndb.triggers_title".into(), "触发器：".into());
    m.insert("ndb.query_time".into(), "查询用时 {} 毫秒".into());
    m.insert("ndb.more_empty".into(), "没有更多行。".into());
    m.insert("ndb.no_query_yet".into(), "先执行 'sql <查询>' 或 'tablef <表>'，再用 'more'。".into());
    m.insert("ndb.multiline_hint".into(), "输入以 ';' 结尾的 SQL（Ctrl-C 中止）：".into());
    m.insert("ndb.yes".into(), "是".into());
    m.insert("ndb.no".into(), "否".into());
    m.insert("ndb.help_db_clear".into(), "clear | cls               清屏并重新显示表".into());
    m.insert("ndb.help_db_exit".into(), "exit | q                  关闭数据库，返回 ndb [目录]>".into());
    m.insert("ndb.help_dbal".into(), "dbal                        管理已保存的查询（数据库别名）".into());
    m.insert("ndb.dbal_title".into(), "DBAL - 数据库别名（已保存的只读查询）：".into());
    m.insert("ndb.dbal_add".into(), "dbal add <名称> \"<SELECT ...>\"   保存查询".into());
    m.insert("ndb.dbal_edit".into(), "dbal edit <名称> \"<SELECT ...>\"  更新已保存的查询".into());
    m.insert("ndb.dbal_rnm".into(), "dbal rnm <旧名> to <新名>        重命名别名".into());
    m.insert("ndb.dbal_rm".into(), "dbal rm <名称>                删除别名".into());
    m.insert("ndb.dbal_info".into(), "dbal info <名称>              显示别名内容".into());
    m.insert("ndb.dbal_list".into(), "dbal list                   列出已保存的查询".into());
    m.insert("ndb.dbal_run".into(), "dbal run <名称> [参数] [-l N|--all]  运行已保存的查询（$参数）".into());
    m.insert("ndb.dbal_cls".into(), "dbal cls --force             删除所有别名".into());
    m.insert("ndb.dbal_exec".into(), "<名称>                        直接运行已保存的查询".into());
    m.insert("ndb.dbal_note".into(), "别名仅保存只读查询；写入会被拒绝。直接输入别名即可运行。参数值按字面替换——字符串请自行加引号：dbal run q \"'2026-01-01'\"。".into());
    m.insert("ndb.dbal_params".into(), "参数：{}".into());
    m.insert("ndb.dbal_missing_args".into(), "缺少参数值：{}".into());
    m.insert("ndb.dbal_single_only".into(), "dbal 别名只保存一条语句——用 && 运行多条。".into());
    m.insert("ndb.dbal_add_usage".into(), "用法：dbal add <名称> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_edit_usage".into(), "用法：dbal edit <名称> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_rnm_usage".into(), "用法：dbal rnm <旧名> to <新名>".into());
    m.insert("ndb.dbal_rm_usage".into(), "用法：dbal rm <名称>".into());
    m.insert("ndb.dbal_info_usage".into(), "用法：dbal info <名称>".into());
    m.insert("ndb.dbal_run_usage".into(), "用法：dbal run <名称> [-l N | --all]".into());
    m.insert("ndb.dbal_cls_usage".into(), "用法：dbal cls --force（删除所有数据库别名）".into());
    m.insert("ndb.dbal_added".into(), "已添加别名 '{}'（{}）".into());
    m.insert("ndb.dbal_updated".into(), "已更新别名 '{}'".into());
    m.insert("ndb.dbal_removed".into(), "已删除别名 '{}'".into());
    m.insert("ndb.dbal_renamed".into(), "已将 '{}' 重命名为 '{}'".into());
    m.insert("ndb.dbal_cleared".into(), "已清除 {} 个数据库别名（{}）".into());
    m.insert("ndb.dbal_not_found".into(), "未找到别名 '{}'".into());
    m.insert("ndb.dbal_invalid_name".into(), "无效的别名 '{}'".into());
    m.insert("ndb.dbal_name_rules".into(), "名称不能是 ndb 命令，也不能包含空格或引号。".into());
    m.insert("ndb.dbal_blocked".into(), "dbal 仅保存只读查询：只允许 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into());
    m.insert("ndb.dbal_unknown_sub".into(), "未知的 dbal 命令：'{}'".into());
    m.insert("ndb.dbal_no_aliases".into(), "还没有数据库别名。添加一个：dbal add <名称> \"<SELECT ...>\"".into());
    m.insert("ndb.dbal_scope_local".into(), "本地".into());
    m.insert("ndb.dbal_scope_global".into(), "全局".into());
    m.insert("ndb.col_sql".into(), "SQL".into());
    m.insert("ndb.help_note".into(), "所有数据库都以只读方式打开；ndb 绝不会修改你的文件。".into());
    m.insert("ndb.help_readonly".into(), "只读：仅执行 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into());
    m.insert("ndb.help_limit".into(), "行数限制：默认 50（-l <n> 修改，--all 显示全部）。".into());
    m.insert("ndb.edit_on".into(), "已为 {} 开启编辑模式（允许写入）".into());
    m.insert("ndb.edit_warn".into(), "写入需要显式确认：write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into());
    m.insert("ndb.edit_failed".into(), "开启编辑模式失败：{}".into());
    m.insert("ndb.edit_off".into(), "编辑模式已关闭（只读）".into());
    m.insert("ndb.edit_status".into(), "编辑模式：{}".into());
    m.insert("ndb.edit_state_on".into(), "开".into());
    m.insert("ndb.edit_state_off".into(), "关".into());
    m.insert("ndb.edit_usage".into(), "用法：edit on|off".into());
    m.insert("ndb.write_needs_edit".into(), "写入已禁用：请先运行 `edit on`".into());
    m.insert("ndb.write_usage".into(), "用法：write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into());
    m.insert("ndb.write_blocked".into(), "`write` 只允许 INSERT、UPDATE、DELETE 和 REPLACE".into());
    m.insert("ndb.write_confirm".into(), "将要执行：{}".into());
    m.insert("ndb.write_confirm_hint".into(), "用 YES 重新运行以确认：write YES <语句>".into());
    m.insert("ndb.write_done".into(), "写入成功：影响 {} 行".into());
    m.insert("ndb.write_failed".into(), "写入失败：{}".into());
    m.insert("ndb.backup_done".into(), "备份已保存：{}".into());
    m.insert("ndb.backup_failed".into(), "备份失败：{}".into());

    // ========== help.ndb（ntc 外壳帮助区段） ==========
    m.insert("help.ndb.title".into(), "NDB（数据库）命令：".into());
    m.insert("help.ndb.launch".into(), "ndb ## 在当前目录打开 ndb 数据库外壳（.db 浏览器）".into());
    m.insert("help.ndb.root_help".into(), "help ## 显示所有 ndb 命令（在 ndb 内）".into());
    m.insert("help.ndb.root_view".into(), "view | ls ## 列出可用的 .db 文件（在 ndb 内）".into());
    m.insert("help.ndb.root_interface".into(), "interface <文件.db> ## 打开数据库：提示符变为 ndb [文件.db]>".into());
    m.insert("help.ndb.root_sql".into(), "sql <查询> ## 打开数据库后执行只读查询".into());
    m.insert("help.ndb.root_exit".into(), "exit ## 关闭数据库，然后离开 ndb".into());
    m.insert("help.ndb.db_viewf".into(), "viewf | v ## 显示表和列的结构树".into());
    m.insert("help.ndb.db_tables".into(), "tables | t ## 列出表和视图名称".into());
    m.insert("help.ndb.db_tablef".into(), "tablef <表> ## 显示表的行（类似 txtf）".into());
    m.insert("help.ndb.db_schema".into(), "schema <表> ## 显示 CREATE TABLE SQL 和列详情".into());
    m.insert("help.ndb.db_sql".into(), "sql <查询> ## 执行只读查询（-l N 行、--all）".into());
    m.insert("help.ndb.db_write".into(), "write YES <语句> ## 执行一条写入，需先 edit on（在 ndb 内）".into());
    m.insert("help.ndb.db_edit".into(), "edit on|off ## 允许或禁止写入（在 ndb 内）".into());
    m.insert("help.ndb.db_backup".into(), "backup ## 在旁边复制一份 .db 文件（在 ndb 内）".into());
    m.insert("help.ndb.more".into(), "more ## 显示上次查询的下一页（在 ndb 内）".into());
    m.insert("help.ndb.indexes".into(), "indexes [表] ## 列出索引（在 ndb 内）".into());
    m.insert("help.ndb.info".into(), "info ## 显示数据库文件、引擎和对象统计（在 ndb 内）".into());
    m.insert("help.ndb.dbal".into(), "dbal ## 管理已保存的只读查询：add|run|list|rm（在 ndb 内）".into());
    m.insert("help.ndb.db_exit".into(), "exit ## 关闭数据库并返回 ndb [目录]>".into());
    m.insert("help.ndb.readonly".into(), "只读 ## 写入被阻止".into());
    m.insert("help.ndb.example1".into(), "ndb ## 然后：interface users.db".into());
    m.insert("help.ndb.example2".into(), "ndb ## 然后：tablef users --all".into());

    m.insert("alias.ral.add_cmd".into(), " ral add <名称> <命令> 创建新的运行别名".into());

    m.insert("alias.ral.alias_count".into(), "您定义了 {} 个别名：".into());

    m.insert("alias.ral.and_more".into(), "... 及另外 {} 个".into());

    m.insert("alias.ral.array_cmd".into(), " ral add <名称>(x[],y) <命令> 创建带数组参数的别名（使用 $x[].ext 或 ${{x}}[].ext）".into());

    m.insert("alias.ral.cls_cancelled".into(), "清除已取消。".into());

    m.insert("alias.ral.cls_cmd".into(), " ral cls 清除所有别名（需确认）".into());

    m.insert("alias.ral.cls_prompt".into(), "确定吗？输入 'yes' 确认：".into());

    m.insert("alias.ral.cls_warning".into(), "\u{26A0}\u{FE0F} 警告：这将删除所有运行别名！".into());

    m.insert("alias.ral.config_path".into(), " 配置：{}".into());

    m.insert("alias.ral.create_failed".into(), "创建文件失败：{}".into());

    m.insert("alias.ral.default_cmd".into(), " ral add <名称>({{x=a,e=txt}}) <命令> 创建带默认值的别名".into());

    m.insert("alias.ral.edit_cmd".into(), " ral edit <名称> <命令> 更新现有别名".into());

    m.insert("alias.ral.empty".into(), "未定义运行别名。使用 'ral add <名称> <命令>'".into());

    m.insert("alias.ral.examples_header".into(), "示例：".into());

    m.insert("alias.ral.export_all_cmd".into(), " ral export --all <名称> 将所有别名导出到 <名称>.ntc.ral".into());

    m.insert("alias.ral.export_import_header".into(), "导出/导入：".into());

    m.insert("alias.ral.export_name_empty".into(), "导出名称不能为空。".into());

    m.insert("alias.ral.export_select_cmd".into(), " ral export --select <名称> 选择别名导出到 <名称>.ntc.ral".into());

    m.insert("alias.ral.file_exists".into(), "文件已存在：{}".into());

    m.insert("alias.ral.import_cmd".into(), " ral import <文件> 从 .ntc.ral 文件导入别名".into());

    m.insert("alias.ral.info_cmd".into(), " ral info <名称> [--edit] 显示别名详情（--edit 将编辑命令复制到剪贴板）".into());

    m.insert("alias.ral.info_command".into(), "命令".into());

    m.insert("alias.ral.info_name".into(), "名称".into());

    m.insert("alias.ral.info_none".into(), "无".into());

    m.insert("alias.ral.info_params".into(), "参数".into());

    m.insert("alias.ral.info_source".into(), "来源".into());

    m.insert("alias.ral.info_source_global".into(), "全局".into());

    m.insert("alias.ral.info_source_local".into(), "本地（ntconfig.toml）".into());

    m.insert("alias.ral.info_title".into(), "\u{1F4CC} 别名信息".into());

    m.insert("alias.ral.info_edit_copied".into(), "编辑命令已复制到剪贴板：".into());

    m.insert("alias.ral.info_edit_hint".into(), "粘贴它，调整命令，然后按回车。".into());

    m.insert("alias.ral.invalid_name".into(), "无效的别名名称：'{}'".into());

    m.insert("alias.ral.invalid_suffix".into(), "{}（无效）".into());

    m.insert("alias.ral.list_cmd".into(), " ral list 显示所有别名".into());

    m.insert("alias.ral.list_title_global".into(), "\u{1F4CC} 运行别名（全局）".into());

    m.insert("alias.ral.list_title_local".into(), "\u{1F4CC} 运行别名（来自本地 ntconfig.toml）".into());

    m.insert("alias.ral.local_config_only".into(), " （仅本地配置 - 全局别名安全）".into());

    m.insert("alias.ral.make_cmd".into(), " ral make [名称] 在当前目录创建别名模板".into());

    m.insert("alias.ral.name_rule_at".into(), " - 以 @ 或 # 开头".into());

    m.insert("alias.ral.name_rule_reserved".into(), " - 是保留命令（go, view, txt 等）".into());

    m.insert("alias.ral.name_rules".into(), "别名名称不能：".into());

    m.insert("alias.ral.no_aliases".into(), "没有要清除的运行别名。".into());

    m.insert("alias.ral.not_found".into(), "未找到别名 '{}'".into());

    m.insert("alias.ral.now_run".into(), "现在您可以运行：{}".into());

    m.insert("alias.ral.now_run_arg".into(), "现在您可以运行：{}({})".into());

    m.insert("alias.ral.now_run_defaults".into(), "现在您可以运行：{}(<参数>) （默认值：{}）".into());

    m.insert("alias.ral.param_cmd".into(), " ral add <名称>(x) <命令> 创建参数化别名（在命令中使用 $x 或 ${{x}}）".into());

    m.insert("alias.ral.renamed".into(), "别名 '{}' 已重命名为 '{}'".into());

    m.insert("alias.ral.rm_cmd".into(), " ral rm <名称>[, <名称>, ...] 删除别名".into());

    m.insert("alias.ral.rnm_cmd".into(), " ral rnm <旧名称> to <新名称> 重命名别名".into());

    m.insert("alias.ral.template_created".into(), "已创建 ral 模板：{}".into());

    m.insert("alias.ral.template_info".into(), "这是一个本地模板文件。使用 'ral import' 将别名添加到配置。".into());

    m.insert("alias.ral.tip_local".into(), "\u{1F4A1} 提示：这些别名是项目特定的（保存在 ntconfig.toml 中）".into());

    m.insert("alias.ral.title".into(), "运行别名（ral）命令：".into());

    m.insert("alias.ral.type_ral_help".into(), "输入 'ral' 获取帮助。".into());

    m.insert("alias.ral.unknown_flag".into(), "未知的导出标志：{}。使用 --all 或 --select".into());

    m.insert("alias.ral.unknown_subcmd".into(), "未知的 ral 子命令：{}".into());

    m.insert("alias.ral.usage_add".into(), "用法：ral add <名称> <命令>".into());

    m.insert("alias.ral.usage_edit".into(), "用法：ral edit <名称> <新命令>".into());

    m.insert("alias.ral.usage_export".into(), "用法：ral export --all <名称> | ral export --select <名称>".into());

    m.insert("alias.ral.usage_hint".into(), "用法：<别名> 或 <别名>(<参数>) 用于参数化别名".into());

    m.insert("alias.ral.usage_import".into(), "用法：ral import <文件>".into());

    m.insert("alias.ral.usage_info".into(), "用法：ral info <别名> [--edit]".into());

    m.insert("alias.ral.usage_rm".into(), "用法：ral rm <名称>[, <名称>, ...]".into());

    m.insert("alias.ral.usage_rnm".into(), "用法：ral rnm <旧名称> to <新名称>".into());

    m.insert("alias.ral.usage_with_run".into(), "使用 run 的用法：".into());

    m.insert("alias.ral.run_example1".into(), "  run btr              # 执行：cargo build --release".into());

    m.insert("alias.ral.run_example2".into(), "  run py               # 执行：python test.py".into());

    m.insert("alias.ral.run_example3".into(), "  run_file(hello)      # 执行：python hello.py".into());

    m.insert("alias.ral.run_example4".into(), "  runc [add,minus,mul] math  # 执行：cls && gcc -o math.exe add.c minus.c mul.c && ...".into());

    m.insert("alias.ral.export_example1".into(), "  ral export --all myaliases       # 创建 myaliases.ntc.ral".into());

    m.insert("alias.ral.export_example2".into(), "  ral export --select myaliases    # 交互选择并导出".into());

    m.insert("alias.ral.export_example3".into(), "  ral import myaliases.ntc.ral     # 导入到当前配置".into());

    m.insert("alias.run.examples_header".into(), "示例：".into());

    m.insert("alias.run.example_alias".into(), "  run btr                     # 运行别名".into());

    m.insert("alias.run.example_alias_args".into(), "  run py test.py              # 带参数运行别名".into());

    m.insert("alias.run.example_param".into(), "  run_file(hello)             # 运行带参数的别名".into());

    m.insert("alias.run.example_real".into(), "  run python --version        # 运行真实命令".into());

    m.insert("alias.run.usage".into(), "用法：run <命令|别名> [参数...]".into());

    m.insert("autoup.added".into(), "已添加启动条目：{} \u{2192} {}".into());

    m.insert("autoup.already_disabled".into(), "条目 '{}' 已禁用。".into());

    m.insert("autoup.already_enabled".into(), "条目 '{}' 已启用。".into());

    m.insert("autoup.already_exists".into(), "条目 '{}' 已存在于用户启动中。".into());

    m.insert("autoup.cmd_empty".into(), "命令值不能为空。".into());

    m.insert("autoup.disabled".into(), "已禁用启动条目：{}".into());

    m.insert("autoup.disabled_system".into(), "已禁用系统启动条目：{}".into());

    m.insert("autoup.enabled".into(), "已启用启动条目：{}".into());

    m.insert("autoup.enabled_system".into(), "已启用系统启动条目：{}".into());

    m.insert("autoup.help_add".into(), " autoup add <名称> \"<命令>\" 添加启动条目（HKCU）".into());

    m.insert("autoup.help_disable".into(), " autoup disable <名称> 禁用启动条目".into());

    m.insert("autoup.help_enable".into(), " autoup enable <名称> 重新启用禁用的条目".into());

    m.insert("autoup.help_help".into(), " autoup help 显示此帮助".into());

    m.insert("autoup.help_list".into(), " autoup list 列出所有来源的启动条目".into());

    m.insert("autoup.help_rm".into(), " autoup rm <名称> 删除启动条目".into());

    m.insert("autoup.hklm_hint".into(), "'{}' 是系统条目（HKLM）。先运行 'mode admin'，然后重试。".into());

    m.insert("autoup.hklm_hint_short".into(), "'{}' 是系统条目。先运行 'mode admin'，然后重试。".into());

    m.insert("autoup.list_empty".into(), "（未找到启动条目）".into());

    m.insert("autoup.list_title".into(), "Windows 启动条目".into());

    m.insert("autoup.list_total".into(), "总计：{} [A]:{} [D]:{} （CU:{} LM:{} SF:{}）".into());

    m.insert("autoup.not_found".into(), "未找到启动条目：{}".into());

    m.insert("autoup.reg_delete_fail".into(), "删除注册表条目失败。".into());

    m.insert("autoup.reg_disable_fail".into(), "设置禁用状态失败。".into());

    m.insert("autoup.reg_exec_fail".into(), "运行 reg.exe 失败：{}".into());

    m.insert("autoup.reg_query_fail".into(), "查询注册表失败：{}".into());

    m.insert("autoup.reg_write_fail".into(), "写入注册表条目失败。请尝试以管理员身份运行。".into());

    m.insert("autoup.removed".into(), "已删除启动条目：{}".into());

    m.insert("autoup.removed_system".into(), "已删除系统启动条目：{}".into());

    m.insert("autoup.sf_disable_hint".into(), "'{}' 在启动文件夹中。将快捷方式移出以禁用它。".into());

    m.insert("autoup.sf_rm_hint".into(), "'{}' 在启动文件夹中。请手动删除快捷方式文件。".into());

    m.insert("autoup.source_cu".into(), " [CU] 当前用户 （HKCU\\...\\Run）".into());

    m.insert("autoup.source_lm".into(), " [LM] 本地机器 （HKLM\\...\\Run）\u{2014} 需要管理员权限（mode admin）".into());

    m.insert("autoup.source_sf".into(), " [SF] 启动文件夹（%APPDATA%\\...\\Startup）\u{2014} 只读".into());

    m.insert("autoup.source_tags".into(), "来源标签：".into());

    m.insert("autoup.usage_add".into(), "用法：autoup add <名称> \"<命令>\"".into());

    m.insert("autoup.usage_disable".into(), "用法：autoup disable <名称>".into());

    m.insert("autoup.usage_enable".into(), "用法：autoup enable <名称>".into());

    m.insert("autoup.usage_examples".into(), "示例：".into());

    m.insert("autoup.usage_rm".into(), "用法：autoup rm <名称>".into());

    m.insert("autoup.usage_title".into(), "用法：".into());

    m.insert("autoup.use_rm_first".into(), "如果要替换，请先使用 'autoup rm'。".into());

    m.insert("autoup.will_run".into(), "该条目将在下次登录时运行。".into());

    m.insert("autoup.windows_only".into(), "autoup 是仅限 Windows 的功能。".into());

    m.insert("backup.and_more".into(), " ... 还有 {} 个更多".into());

    m.insert("backup.available_title".into(), " 可用备份（最新优先）".into());

    m.insert("backup.cancel_opt".into(), " 0. 取消".into());

    m.insert("backup.cancelled".into(), "清除已取消。".into());

    m.insert("backup.clear_history_prompt".into(), "\u{26A0} 清除撤销历史？此操作无法撤销。（y/N）：".into());

    m.insert("backup.cleared".into(), "所有备份已删除。".into());

    m.insert("backup.cleared_count".into(), "已为此项目清除 {} 个备份。".into());

    m.insert("backup.confirm".into(), "输入 'yes' 确认：".into());

    m.insert("backup.create_hint".into(), "使用 'backup create' 创建您的第一个备份。".into());

    m.insert("backup.created".into(), "备份已创建：#{} \u{2014} {} \u{2014} {} 个文件".into());

    m.insert("backup.created_detail".into(), "备份 #{} 已创建：{}（{} 个文件，{}）".into());

    m.insert("backup.creating".into(), "正在创建备份...".into());

    m.insert("backup.delete_extra_failed".into(), "删除额外文件 {} 失败：{}".into());

    m.insert("backup.deleted_since".into(), "{} 自备份以来已被删除。".into());

    m.insert("backup.entry_file_count".into(), " 备份 #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into());

    m.insert("backup.exists".into(), "备份存在：{} 个备份在 {}".into());

    m.insert("backup.failed_to_backup".into(), "备份 {} 失败：{}".into());

    m.insert("backup.file_deleted".into(), "{} 在备份后被删除。".into());

    m.insert("backup.file_modified".into(), "{} 自上次保存以来已更改。".into());

    m.insert("backup.help_clear".into(), " backup clear 删除所有备份（需确认）".into());

    m.insert("backup.help_clear_force".into(), " backup clear --force 删除所有备份（无需确认）".into());

    m.insert("backup.help_create".into(), " backup create 创建新备份".into());

    m.insert("backup.help_diff".into(), " backup diff [num] 比较当前项目与备份的差异".into());

    m.insert("backup.help_examples".into(), "示例：".into());

    m.insert("backup.help_list".into(), " backup list 列出所有备份".into());

    m.insert("backup.help_recreate".into(), " backup recreate 清除所有 + 撤销历史 + 创建全新".into());

    m.insert("backup.help_recreate_force".into(), " backup recreate --force 同上，跳过所有确认".into());

    m.insert("backup.help_restore".into(), " backup restore [num] 恢复备份（无编号则交互选择）".into());

    m.insert("backup.help_restore_choose".into(), " backup restore --choose 从交互菜单选择备份".into());

    m.insert("backup.help_restore_choose_deleted".into(), " backup restore --choose --deleted 选择 + 恢复删除".into());

    m.insert("backup.help_restore_deleted".into(), " backup restore --deleted 仅恢复自备份以来已删除的文件".into());

    m.insert("backup.help_title".into(), "backup \u{2014} 管理项目备份".into());

    m.insert("backup.help_undo".into(), " backup undo 撤销上次恢复".into());

    m.insert("backup.help_undo_cls".into(), " backup undo --cls 清除撤销历史".into());

    m.insert("backup.help_undo_force".into(), " backup undo --force 清除撤销历史（无需确认）".into());

    m.insert("backup.invalid_arg".into(), "无效的参数：{}".into());

    m.insert("backup.invalid_diff_arg".into(), "无效的参数：{}。用法：backup diff <编号>".into());

    m.insert("backup.invalid_selection".into(), "无效的选择：{}".into());

    m.insert("backup.list_entry".into(), " #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into());

    m.insert("backup.list_location".into(), "位置：{}".into());

    m.insert("backup.list_title".into(), "此项目的备份：找到 {} 个".into());

    m.insert("backup.menu_item".into(), "备份 #{}".into());

    m.insert("backup.modified_since".into(), "{} 自备份以来已被修改。".into());

    m.insert("backup.no_deleted_files".into(), "没有要恢复的已删除文件 \u{2014} 项目已与备份快照匹配。".into());

    m.insert("backup.none".into(), "未找到此项目的备份。".into());

    m.insert("backup.not_found_num".into(), "备份 #{} 未找到。".into());

    m.insert("backup.pick_detail".into(), "{} \u{2014} {} \u{2014} {} 个文件".into());

    m.insert("backup.pick_prompt".into(), "输入数字（1-{}）或 0：".into());

    m.insert("backup.pick_title".into(), "选择要恢复的备份".into());

    m.insert("backup.previous_state_saved".into(), "已保存先前状态。使用 'unpd' 撤销此恢复。".into());

    m.insert("backup.recreate_proceed".into(), "继续？（y/N）：".into());

    m.insert("backup.recreate_prompt".into(), "未找到现有备份。仍然创建全新备份？（y/N）：".into());

    m.insert("backup.recreate_step1".into(), " 1. 删除所有现有备份".into());

    m.insert("backup.recreate_step2".into(), " 2. 清除撤销历史".into());

    m.insert("backup.recreate_step3".into(), " 3. 创建当前状态的全新备份".into());

    m.insert("backup.recreate_will_title".into(), "\u{26A0}\u{FE0F} 重建将：".into());

    m.insert("backup.recreated".into(), "项目已重建：旧备份已清除，新备份已创建。".into());

    m.insert("backup.restore_add_count".into(), "{} 个文件将被恢复。".into());

    m.insert("backup.restore_cancelled".into(), "恢复已取消。".into());

    m.insert("backup.restore_cancelled_2".into(), "恢复已取消。".into());

    m.insert("backup.restore_confirm".into(), "继续恢复？（y/N）：".into());

    m.insert("backup.restore_create_title".into(), "\u{26A0} 以下文件将被创建（{}）：".into());

    m.insert("backup.restore_delete_title".into(), "\u{26A0} 以下文件将被删除（不在备份中）（{}）：".into());

    m.insert("backup.restore_deleted_confirm".into(), "继续恢复已删除的文件？（y/N）：".into());

    m.insert("backup.restore_deleted_title".into(), " 将恢复 {} 个文件（备份中存在但现已消失）：".into());

    m.insert("backup.restore_failed".into(), "恢复 {} 失败：{}".into());

    m.insert("backup.restore_overwrite_count".into(), "{} 个文件将被覆盖。".into());

    m.insert("backup.restore_overwrite_title".into(), "\u{26A0} 以下文件将被覆盖（{}）：".into());

    m.insert("backup.restore_preview_title".into(), "恢复预览".into());

    m.insert("backup.restored_backup".into(), "恢复完成：备份 #{} \u{2014} 已恢复 {} 个文件".into());

    m.insert("backup.restored_date".into(), "恢复执行于：{}".into());

    m.insert("backup.restored_deleted".into(), "已从备份 #{} 恢复已删除的文件：{} 个文件已恢复".into());

    m.insert("backup.restored_result".into(), "已恢复备份 #{}（{}）".into());

    m.insert("backup.select_entry".into(), " {}. 备份 #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into());

    m.insert("backup.select_prompt".into(), "选择要恢复的备份（1-{}）：".into());

    m.insert("backup.skip_large".into(), "已跳过 {}（{} MB 超过 50 MB 限制）".into());

    m.insert("backup.skip_metadata".into(), "跳过 {}（元数据错误：{}）".into());

    m.insert("backup.skip_rel_path".into(), "跳过 {}（无法解析相对路径）".into());

    m.insert("backup.skip_symlink".into(), "跳过 {}（符号链接 \u{2014} 不跟随）".into());

    m.insert("backup.skipped_count".into(), "已跳过 {} 个文件 \u{2014} 查看 ignore.txt 了解详情".into());

    m.insert("backup.skipped_error".into(), "已跳过 {}（{}）".into());

    m.insert("backup.skipped_large".into(), "已跳过 {}（{} \u{2014} 太大）".into());

    m.insert("backup.skipped_unsupported".into(), "已跳过 {}（不支持的文件类型）".into());

    m.insert("backup.snapshot_failed".into(), "无法为 {} 创建快照以撤销（{}），继续".into());

    m.insert("backup.some_restore_failed".into(), "某些文件无法恢复。撤销存储已保留。".into());

    m.insert("backup.stored_in".into(), "备份存储在：{}".into());

    m.insert("backup.undo_cleared".into(), "撤销历史已清除（释放了 {}）".into());

    m.insert("backup.undo_complete".into(), "撤销完成：已从上一次恢复前恢复了 {} 个文件".into());

    m.insert("backup.undo_complete_detail".into(), "已撤销备份 #{} 的恢复（{} 已恢复，{} 已删除，{} 已重新恢复）".into());

    m.insert("backup.undo_delete_failed".into(), "无法删除已恢复的文件 {}：{}".into());

    m.insert("backup.undo_missing_original".into(), "无法恢复 {}（撤销存储中缺少原始文件）".into());

    m.insert("backup.undo_missing_snapshot".into(), "无法重新恢复 {}（撤销存储中缺少快照）".into());

    m.insert("backup.undo_no_history".into(), "未找到撤销历史。".into());

    m.insert("backup.undo_none".into(), "没有要撤销的恢复操作。".into());

    m.insert("backup.undo_rerestore_failed".into(), "无法重新恢复已删除的文件 {}：{}".into());

    m.insert("backup.unknown_clear_option".into(), "未知的备份清除选项：{}".into());

    m.insert("backup.unknown_create_option".into(), "未知的备份创建选项：{}".into());

    m.insert("backup.unknown_recreate_option".into(), "未知的备份重建选项：{}".into());

    m.insert("backup.unknown_undo_option".into(), "未知的备份撤销选项：{}".into());

    m.insert("backup.usage_clear".into(), "用法：backup clear [--force]".into());

    m.insert("backup.usage_recreate".into(), "用法：backup recreate [--force]".into());

    m.insert("backup.usage_restore".into(), "用法：backup restore [--choose] [--deleted] [num]".into());

    m.insert("backup.warning_count".into(), "您有 {} 个备份：".into());

    m.insert("backup.warning_title".into(), "\u{26A0}\u{FE0F} 警告：这将删除此项目的所有备份！".into());

    m.insert("backup.would_overwrite".into(), "{} 将被覆盖（使用 --force 跳过）".into());

    m.insert("backup.would_overwrite_deleted".into(), "正在恢复已删除的文件：{}".into());

    m.insert("backup_diff.cancel_opt".into(), "0. 取消".into());

    m.insert("backup_diff.cancelled".into(), "比较已取消。".into());

    m.insert("backup_diff.comparing".into(), "正在将当前项目与备份 #{} 进行比较...".into());

    m.insert("backup_diff.invalid_input".into(), "无效的输入：{}".into());

    m.insert("backup_diff.invalid_selection".into(), "无效的选择：{}".into());

    m.insert("backup_diff.none".into(), "未找到此项目的备份。使用 'bkup' 创建一个。".into());

    m.insert("backup_diff.not_found".into(), "备份 #{} 未找到。".into());

    m.insert("backup_diff.open_manually".into(), "手动打开文件：{}".into());

    m.insert("backup_diff.saved_to".into(), "差异报告已保存到：{}".into());

    m.insert("backup_diff.select_prompt".into(), "选择备份（1-{}）：".into());

    m.insert("backup_diff.select_title".into(), "\u{1F4CA} 选择要与当前项目比较的备份".into());

    m.insert("clap.arg.cared.help".into(), "停止忽略一个或多个目录".into());

    m.insert("clap.arg.caref.help".into(), "关注一个或多个文件扩展名".into());

    m.insert("clap.arg.caren.help".into(), "关注一个或多个特定文件".into());

    m.insert("clap.arg.clear.help".into(), "清除终端屏幕".into());

    m.insert("clap.arg.copy.help".into(), "将报告复制到剪贴板而非保存到文件".into());

    m.insert("clap.arg.edit.help".into(), "在内置文本编辑器中打开文件".into());

    m.insert("clap.arg.explorer.help".into(), "启动NTC文件浏览器TUI（可指定起始路径）".into());

    m.insert("clap.arg.format_cli.help".into(), "输出格式：txt, html, json, md".into());

    m.insert("clap.arg.generate_completions.help".into(), "生成Shell补全脚本（bash|zsh|fish|powershell）".into());

    m.insert("clap.arg.help_arg.help".into(), "显示详细帮助".into());

    m.insert("clap.arg.hyper.help".into(), "创建hyper项目脚手架（DSA库）".into());

    m.insert("clap.arg.igcare_export_all.help".into(), "将所有忽略/关注设置导出到 <名称>.ntc.igcare".into());

    m.insert("clap.arg.igcare_export_select.help".into(), "选择要导出的类别到 <名称>.ntc.igcare".into());

    m.insert("clap.arg.igcare_import.help".into(), "从 .ntc.igcare 文件导入忽略/关注设置".into());

    m.insert("clap.arg.ignore.help".into(), "忽略一个或多个目录名称".into());

    m.insert("clap.arg.ignored.help".into(), "显示已忽略的项目".into());

    m.insert("clap.arg.ignoref.help".into(), "忽略一个或多个文件扩展名".into());

    m.insert("clap.arg.ignoren.help".into(), "忽略一个或多个特定文件".into());

    m.insert("clap.arg.image.help".into(), "将图像转换为文档（JSON, MD, HTML, TXT）".into());

    m.insert("clap.arg.image_to.help".into(), "图像转换的输出格式：json, md, html, txt".into());

    m.insert("clap.arg.init.help".into(), "从模板创建入门文件并在编辑器中打开".into());

    m.insert("clap.arg.input.help".into(), "输入文件或目录路径".into());

    m.insert("clap.arg.lang_cli.help".into(), "hyper项目的语言".into());

    m.insert("clap.arg.list.help".into(), "列出所有命令行功能".into());

    m.insert("clap.arg.lsp.help".into(), "通过stdio启动ntc-math LSP服务器".into());

    m.insert("clap.arg.lsp_log.help".into(), "写入LSP调试日志的路径".into());

    m.insert("clap.arg.math.help".into(), "计算数学表达式".into());

    m.insert("clap.arg.output.help".into(), "报告的输出文件名".into());

    m.insert("clap.arg.ral_export_all.help".into(), "将所有运行别名导出到 <名称>.ntc.ral".into());

    m.insert("clap.arg.ral_export_select.help".into(), "选择要导出的运行别名到 <名称>.ntc.ral".into());

    m.insert("clap.arg.ral_import.help".into(), "从 .ntc.ral 文件导入运行别名".into());

    m.insert("clap.arg.say.help".into(), "将文本输出到stdout".into());

    m.insert("clap.arg.setA.help".into(), "显示或设置自动建议（幽灵文本）（开/关）".into());

    m.insert("clap.arg.setC.help".into(), "显示或设置颜色输出（开/关）".into());

    m.insert("clap.arg.setD.help".into(), "显示或设置最大递归深度".into());

    m.insert("clap.arg.setH.help".into(), "显示或设置历史记录（路径/开/关/默认）".into());

    m.insert("clap.arg.setL.help".into(), "显示或设置行号显示（开/关）".into());

    m.insert("clap.arg.setO.help".into(), "显示或设置默认输出目录".into());

    m.insert("clap.arg.setT.help".into(), "显示或设置线程数".into());

    m.insert("clap.arg.showcg.help".into(), "显示当前配置概览".into());

    m.insert("clap.arg.size_cli.help".into(), "显示当前目录大小".into());

    m.insert("clap.arg.time.help".into(), "显示耗时（与 --size 或 --view 配合使用）".into());

    m.insert("clap.arg.tp_add.help".into(), "将当前目录保存为传送点".into());

    m.insert("clap.arg.tp_list.help".into(), "列出所有传送点".into());

    m.insert("clap.arg.tp_rm.help".into(), "删除传送点".into());

    m.insert("clap.arg.upgrade.help".into(), "检查更新并将ntc升级到最新版本".into());

    m.insert("clap.arg.version.help".into(), "显示版本信息".into());

    m.insert("clap.arg.view_cli.help".into(), "快速查看当前目录树".into());

    m.insert("clap.arg.watch_cli.help".into(), "显示或设置文件监视器（开/关）".into());

    m.insert("clap.arg.where_cli.help".into(), "显示ntc可执行文件位置".into());

    m.insert("clap.value_name.depth".into(), "深度".into());

    m.insert("clap.value_name.expr".into(), "表达式".into());

    m.insert("clap.value_name.ext".into(), "扩展名".into());

    m.insert("clap.value_name.file".into(), "文件".into());

    m.insert("clap.value_name.format_cli".into(), "格式".into());

    m.insert("clap.value_name.image".into(), "图像".into());

    m.insert("clap.value_name.lang".into(), "语言".into());

    m.insert("clap.value_name.name".into(), "名称".into());

    m.insert("clap.value_name.path".into(), "路径".into());

    m.insert("clap.value_name.project".into(), "项目".into());

    m.insert("clap.value_name.shell".into(), "Shell".into());

    m.insert("clap.value_name.state".into(), "状态".into());

    m.insert("clap.value_name.text".into(), "文本".into());

    m.insert("clap.value_name.threads".into(), "线程数".into());

    m.insert("clap.value_name.value".into(), "值".into());

    m.insert("cli.completions.shell.bash".into(), "bash".into());

    m.insert("cli.completions.shell.fish".into(), "fish".into());

    m.insert("cli.completions.shell.powershell".into(), "powershell".into());

    m.insert("cli.completions.shell.zsh".into(), "zsh".into());

    m.insert("cli.example.cat_file".into(), " ntc -i file.txt 显示文件内容".into());

    m.insert("cli.example.image".into(), " ntc -image screenshot.png to json 将图像转换为JSON以供AI使用\n".into());

    m.insert("cli.example.interactive".into(), " ntc 启动交互式Shell".into());

    m.insert("cli.example.math_arithmetic".into(), " ntc --math \"3+4*5\" 计算数学表达式".into());

    m.insert("cli.example.math_func".into(), " ntc --math \"sin(PI/2)\" 使用函数和常量的数学计算".into());

    m.insert("cli.example.report_cp".into(), " ntc -i src --cp 将目录树复制到剪贴板".into());

    m.insert("cli.example.report_docx".into(), " ntc -i src -f docx 生成DOCX报告".into());

    m.insert("cli.example.report_json_cp".into(), " ntc -i src -f json --cp 将JSON报告复制到剪贴板".into());

    m.insert("cli.example.report_pdf".into(), " ntc -i src -f pdf 从src生成PDF报告".into());

    m.insert("cli.example.report_txt".into(), " ntc -i src 生成src目录的TXT报告".into());

    m.insert("cli.example.report_xlsx".into(), " ntc -i src -f xlsx 生成XLSX报告（含依赖分析）".into());

    m.insert("cli.example.say".into(), " ntc -say \"Hello World\" 打印Hello World".into());

    m.insert("cli.example.setL".into(), " ntc --setL 开 启用行号".into());

    m.insert("cli.example.showcg".into(), " ntc --showcg 显示配置".into());

    m.insert("cli.example.teleport_at".into(), " ntc @web 启动并传送至 'web' 保存点".into());

    m.insert("cli.example.watch".into(), " ntc --watch 开 启用文件监视器".into());

    m.insert("cli.flag.cared".into(), " --cared <名称> 停止忽略目录名称".into());

    m.insert("cli.flag.caref".into(), " --caref <扩展名> 关注文件扩展名".into());

    m.insert("cli.flag.caren".into(), " --caren <文件> 关注特定文件\n".into());

    m.insert("cli.flag.clear".into(), " --clear 清除终端屏幕".into());

    m.insert("cli.flag.copy".into(), " --cp 将报告复制到剪贴板而非保存到文件".into());

    m.insert("cli.flag.edit".into(), " -e, --edit <文件> 在内置文本编辑器中打开文件".into());

    m.insert("cli.flag.explorer".into(), " -x, --explorer [路径] 启动NTC文件浏览器TUI".into());

    m.insert("cli.flag.format".into(), " -f, --format <格式> 输出格式：txt, html, json, md, pdf, docx, xlsx（默认：txt）".into());

    m.insert("cli.flag.generate_completions".into(), " --generate-completions <Shell> 生成自动补全（bash|zsh|fish|powershell）\n".into());

    m.insert("cli.flag.help_flag".into(), " --help 显示此帮助信息".into());

    m.insert("cli.flag.hyper".into(), " --hyper <名称> --lang c 创建C/C++ hyper项目脚手架\n".into());

    m.insert("cli.flag.igcare_export_all".into(), " --igcare-export-all <名称> 导出所有忽略/关注设置".into());

    m.insert("cli.flag.igcare_export_select".into(), " --igcare-export-select <名称> 选择要导出的类别".into());

    m.insert("cli.flag.igcare_import".into(), " --igcare-import <文件> 从 .ntc.igcare 文件导入设置\n".into());

    m.insert("cli.flag.ignore".into(), " --ignore <名称> 忽略目录名称".into());

    m.insert("cli.flag.ignored".into(), " --ignored 显示当前忽略的项目".into());

    m.insert("cli.flag.ignoref".into(), " --ignoref <扩展名> 忽略文件扩展名".into());

    m.insert("cli.flag.ignoren".into(), " --ignoren <文件> 忽略特定文件（如 Cargo.lock）".into());

    m.insert("cli.flag.image".into(), " -image <路径> to <格式> 将图像转换为AI可用的文档（json|md|html|txt）".into());

    m.insert("cli.flag.init".into(), " --init <文件> 创建入门模板并在编辑器中打开".into());

    m.insert("cli.flag.input".into(), " -i, --input <路径> 处理文件或目录".into());

    m.insert("cli.flag.list".into(), " --list, --fun 列出所有命令行功能".into());

    m.insert("cli.flag.math".into(), " --math <表达式> 计算数学表达式（如 --math \"3+4*5\"）".into());

    m.insert("cli.flag.output".into(), " -o, --output <文件> 将输出保存到指定文件".into());

    m.insert("cli.flag.ral_export_all".into(), " --ral-export-all <名称> 将所有运行别名导出到 <名称>.ntc.ral".into());

    m.insert("cli.flag.ral_export_select".into(), " --ral-export-select <名称> 选择要导出的别名".into());

    m.insert("cli.flag.ral_import".into(), " --ral-import <文件> 从 .ntc.ral 文件导入别名".into());

    m.insert("cli.flag.say".into(), " -say, -print <文本> 将文本输出到stdout".into());

    m.insert("cli.flag.setA".into(), " --setA [on|off] 显示/设置自动建议（幽灵文本）".into());

    m.insert("cli.flag.setC".into(), " --setC [on|off] 显示或切换颜色输出".into());

    m.insert("cli.flag.setD".into(), " --setD [深度] 显示或设置最大递归深度（最小：1，最大：20）".into());

    m.insert("cli.flag.setH".into(), " --setH [路径|on|off] 显示/设置历史路径或启用/禁用".into());

    m.insert("cli.flag.setL".into(), " --setL [on|off] 显示或切换文件显示的行号".into());

    m.insert("cli.flag.setO".into(), " --setO [路径] 显示或设置输出目录（默认：桌面）".into());

    m.insert("cli.flag.setT".into(), " --setT [线程数] 显示或设置线程数（默认：4）".into());

    m.insert("cli.flag.showcg".into(), " --showcg 显示当前配置概览".into());

    m.insert("cli.flag.size".into(), " --size 显示当前目录大小".into());

    m.insert("cli.flag.tp_add".into(), " --tp-add <名称> 将当前目录保存为传送点".into());

    m.insert("cli.flag.tp_list".into(), " --tp-list 列出所有传送点".into());

    m.insert("cli.flag.tp_rm".into(), " --tp-rm <名称> 删除传送点\n".into());

    m.insert("cli.flag.version".into(), " --version 显示版本信息".into());

    m.insert("cli.flag.view".into(), " --view 快速查看当前目录树".into());

    m.insert("cli.flag.view_size".into(), " --view --size 快速查看含目录大小\n".into());

    m.insert("cli.flag.watch_flag".into(), " --watch [on|off] 显示/设置文件监视器状态".into());

    m.insert("cli.flag.where".into(), " --where 显示ntc可执行文件和配置位置\n".into());

    m.insert("cli.help.description".into(), "导航、项目工具包中心的结合体。\n".into());

    m.insert("cli.help.title".into(), "ntc <版本> \u{2014} 导航, 工具包, 中心".into());

    m.insert("cli.helpers.basic_usage".into(), " ntc [选项]\n ntc -i <路径> [-o <输出>]\n ntc -i <路径> --cp 复制报告到剪贴板\n".into());

    m.insert("cli.helpers.cli_examples".into(), " ntc 启动交互式 shell\n ntc @web 启动并传送到 'web' 保存点\n ntc --math \"3+4*5\" 计算数学表达式\n ntc --math \"sin(PI/2)\" 带函数和常量的数学计算\n ntc -i src 生成 src 目录的 TXT 报告\n ntc -i src --cp 复制目录树到剪贴板\n ntc -i src -f json --cp 复制 JSON 报告到剪贴板\n ntc -i src -f pdf 从 src 生成 PDF 报告\n ntc -i src -f docx 从 src 生成 DOCX 报告\n ntc -i src -f xlsx 从 src 生成 XLSX 报告（带依赖分析）\n ntc -i file.txt 显示文件内容\n ntc --setL ON 启用行号\n ntc --showcg 显示配置\n ntc -say \"Hello World\" 打印 Hello World\n ntc --watch ON 启用文件监视器\n ntc -image screenshot.png to json 转换图像为 JSON 用于 AI\n".into());

    m.insert("cli.helpers.config_header".into(), "配置选项：".into());

    m.insert("cli.helpers.config_options".into(), " --setO [路径] 显示或设置输出目录（默认：桌面）\n --setD [深度] 显示或设置最大递归深度（最小：1，最大：20）\n --setL [on|off] 显示或切换行号\n --setC [on|off] 显示或切换颜色输出\n --setT [线程数] 显示或设置线程数（默认：4）\n --setH [路径|on|off] 显示/设置历史路径或启用/禁用\n --setA [on|off] 显示/设置自动建议（幽灵文本）\n --showcg 显示当前配置概览\n --watch [on|off] 显示/设置文件监视器状态\n --where 显示 ntc 可执行文件和配置位置\n".into());

    m.insert("cli.helpers.editor_header".into(), "编辑器选项：".into());

    m.insert("cli.helpers.editor_options".into(), " -e, --edit <文件> 在内置文本编辑器中打开文件\n --init <文件> 创建起始模板并在编辑器中打开\n --hyper <名称> --lang c 创建 C/C++ hyper 项目脚手架\n".into());

    m.insert("cli.helpers.examples_header".into(), "示例：".into());

    m.insert("cli.helpers.export_header".into(), "别名 / 忽略/关注导出：".into());

    m.insert("cli.helpers.export_options".into(), " --ral-export-all <名称> 导出所有运行别名为 <名称>.ntc.ral\n --ral-export-select <名称> 选择要导出的别名\n --ral-import <文件> 从 .ntc.ral 文件导入别名\n --igcare-export-all <名称> 导出所有忽略/关注设置\n --igcare-export-select <名称> 选择要导出的类别\n --igcare-import <文件> 从 .ntc.igcare 文件导入设置\n".into());

    m.insert("cli.helpers.igcare_options".into(), " --ignored 显示当前忽略的项目\n --ignore <名称> 忽略一个目录名称\n --cared <名称> 停止忽略一个目录名称\n --ignoref <扩展名> 忽略一个文件扩展名\n --caref <扩展名> 关注一个文件扩展名\n --ignoren <文件> 忽略一个特定文件（例如 Cargo.lock）\n --caren <文件> 关注一个特定文件\n".into());

    m.insert("cli.helpers.ignore_header".into(), "忽略/关注选项：".into());

    m.insert("cli.helpers.interactive_note".into(), "仅交互模式（运行 'ntc' 不含参数）：\n 导航：cd, back, gos, gosc\n 传送：tp jump, tp to, @name\n 教程：tutorial \u{2014} 开始交互式分步教程\n".into());

    m.insert("cli.helpers.other_header".into(), "其他选项：".into());

    m.insert("cli.helpers.other_options".into(), " -say, -print <文本> 打印文本到 stdout\n -image <路径> to <格式> 转换图像为 AI 文档（json|md|html|txt）\n -x, --explorer [路径] 启动 NTC 文件资源管理器 TUI\n --clear 清除终端屏幕\n --version 显示版本信息\n --math <表达式> 计算数学表达式（例如 --math \"3+4*5\"）\n --list, --fun 列出所有命令行功能\n --help 显示此帮助信息\n --generate-completions <SHELL> 生成补全（bash|zsh|fish|powershell）\n".into());

    m.insert("cli.helpers.report_header".into(), "报告选项：".into());

    m.insert("cli.helpers.report_options".into(), " -i, --input <路径> 处理文件或目录\n -o, --output <文件> 保存输出到指定文件\n -f, --format <格式> 输出格式：txt, html, json, md, pdf, docx, xlsx（默认：txt）\n --cp 复制报告到剪贴板而非保存到文件\n --size 显示当前目录大小\n --view 快速查看当前目录树\n --view --size 快速查看带目录大小\n".into());

    m.insert("cli.helpers.shell_commands".into(), " ntc pathcp 复制路径到剪贴板\n ntc say \"hello\" --loop 3 循环打印文本\n ntc tl dir . 统计目录中的行数\n ntc ne 打开文本编辑器\n ntc size 显示目录大小\n ntc view 显示目录树\n ntc serve 8080 在端口 8080 启动 HTTP 服务器\n ntc dl https://example.com/file.zip --progress 带进度下载\n ntc encode b64 hello 对字符串进行 Base64 编码\n ntc qr \"text\" 在终端生成二维码\n ntc math 3+4*5 计算表达式\n ntc ral add btr \"cargo build --release\" 从 CLI 添加别名\n".into());

    m.insert("cli.helpers.shell_header".into(), "Shell 命令（也可从 CLI 使用）：".into());

    m.insert("cli.helpers.tagline".into(), "集导航、项目工具包中心于一体。\n".into());

    m.insert("cli.helpers.tp_header".into(), "传送保存点：".into());

    m.insert("cli.helpers.tp_options".into(), " --tp-add <名称> 将当前目录保存为传送点\n --tp-list 列出所有传送点\n --tp-rm <名称> 删除一个传送点\n".into());

    m.insert("cli.helpers.usage_header".into(), "用法：".into());

    m.insert("cli.interactive.navigation".into(), " 导航：cd, back, gos, gosc".into());

    m.insert("cli.interactive.teleport".into(), " 传送：tp jump, tp to, @名称".into());

    m.insert("cli.interactive.tutorial".into(), " 教程：tutorial \u{2014} 启动交互式分步教程\n".into());

    m.insert("cli.known_flag.cared".into(), "--cared <名称> 停止忽略目录".into());

    m.insert("cli.known_flag.caref".into(), "--caref <扩展名> 关注文件扩展名".into());

    m.insert("cli.known_flag.caren".into(), "--caren <文件> 关注特定文件".into());

    m.insert("cli.known_flag.clear".into(), "--clear 清除终端屏幕".into());

    m.insert("cli.known_flag.cp".into(), "--cp 将报告复制到剪贴板".into());

    m.insert("cli.known_flag.explorer".into(), "-x, --explorer [路径] 启动NTC文件浏览器TUI".into());

    m.insert("cli.known_flag.generate_completions".into(), "--generate-completions <Shell> 生成Shell补全（bash|zsh|fish|powershell）".into());

    m.insert("cli.known_flag.help_flag".into(), "--help 显示帮助".into());

    m.insert("cli.known_flag.igcare_export_all".into(), "--igcare-export-all <名称> 导出所有忽略/关注设置到 <名称>.ntc.igcare".into());

    m.insert("cli.known_flag.igcare_export_select".into(), "--igcare-export-select <名称> 选择类别导出到 <名称>.ntc.igcare".into());

    m.insert("cli.known_flag.igcare_import".into(), "--igcare-import <文件> 从 .ntc.igcare 文件导入忽略/关注设置".into());

    m.insert("cli.known_flag.ignore".into(), "--ignore <名称> 忽略目录名称".into());

    m.insert("cli.known_flag.ignored".into(), "--ignored 显示已忽略的项目".into());

    m.insert("cli.known_flag.ignoref".into(), "--ignoref <扩展名> 忽略文件扩展名".into());

    m.insert("cli.known_flag.ignoren".into(), "--ignoren <文件> 忽略特定文件".into());

    m.insert("cli.known_flag.input".into(), "-i, --input <路径> 输入文件或目录".into());

    m.insert("cli.known_flag.list".into(), "--list, --fun 列出所有命令行功能".into());

    m.insert("cli.known_flag.math".into(), "--math <表达式> 计算数学表达式（如 --math \"3+4*5\"）".into());

    m.insert("cli.known_flag.no_args".into(), "（无参数） 启动交互模式".into());

    m.insert("cli.known_flag.output".into(), "-o, --output <文件> 输出文件名".into());

    m.insert("cli.known_flag.ral_export_all".into(), "--ral-export-all <名称> 将所有运行别名导出到 <名称>.ntc.ral".into());

    m.insert("cli.known_flag.ral_export_select".into(), "--ral-export-select <名称> 选择运行别名导出到 <名称>.ntc.ral".into());

    m.insert("cli.known_flag.ral_import".into(), "--ral-import <文件> 从 .ntc.ral 文件导入运行别名".into());

    m.insert("cli.known_flag.say".into(), "-say, -print <文本> 将文本输出到stdout".into());

    m.insert("cli.known_flag.setC".into(), "--setC [on|off] 显示或设置颜色输出".into());

    m.insert("cli.known_flag.setD".into(), "--setD [深度] 显示或设置最大深度（1-20）".into());

    m.insert("cli.known_flag.setH".into(), "--setH [路径|on|off] 显示或设置历史记录路径/状态".into());

    m.insert("cli.known_flag.setL".into(), "--setL [on|off] 显示或设置行号".into());

    m.insert("cli.known_flag.setO".into(), "--setO [路径] 显示或设置输出目录".into());

    m.insert("cli.known_flag.setT".into(), "--setT [线程数] 显示或设置线程数".into());

    m.insert("cli.known_flag.showcg".into(), "--showcg 显示当前配置概览".into());

    m.insert("cli.known_flag.size".into(), "--size 显示当前目录大小".into());

    m.insert("cli.known_flag.path".into(), "--path <路径|@tp> 目标路径或传送点（与 --size 或 --view 配合使用）".into());

    m.insert("cli.known_flag.teleport_at".into(), "ntc @<名称> 启动并传送到保存点".into());

    m.insert("cli.known_flag.tp_add".into(), "--tp-add <名称> 将当前目录保存为传送点".into());

    m.insert("cli.known_flag.tp_list".into(), "--tp-list 列出所有传送点".into());

    m.insert("cli.known_flag.tp_rm".into(), "--tp-rm <名称> 删除传送点".into());

    m.insert("cli.known_flag.upgrade".into(), "--upgrade 检查更新并升级ntc".into());

    m.insert("cli.known_flag.version".into(), "--version 显示版本信息".into());

    m.insert("cli.known_flag.view".into(), "--view 快速查看当前目录树".into());

    m.insert("cli.known_flag.view_size".into(), "--view --size 快速查看含目录大小".into());

    m.insert("cli.known_flag.watch".into(), "--watch [on|off] 显示或设置文件监视器状态".into());

    m.insert("cli.known_flag.where".into(), "--where 显示ntc可执行文件和配置位置".into());

    m.insert("cli.logo.c".into(), "C".into());

    m.insert("cli.logo.center".into(), "中心".into());

    m.insert("cli.logo.letters_format".into(), " {} {} {}".into());

    m.insert("cli.logo.n".into(), "N".into());

    m.insert("cli.logo.navigate".into(), "导航".into());

    m.insert("cli.logo.subtitle_format".into(), " {} {} {}".into());

    m.insert("cli.logo.t".into(), "T".into());

    m.insert("cli.logo.toolkit".into(), "工具包".into());

    m.insert("cli.run.about".into(), "导航、目录树、Cat - 目录树查看与文件连接工具".into());

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert("cli.run.help_clear".into(), "清空终端屏幕".into());

    m.insert("cli.run.help_copy".into(), "将报告复制到剪贴板而不是保存到文件".into());

    m.insert("cli.run.help_edit".into(), "在内置文本编辑器中打开文件".into());

    m.insert("cli.run.help_explorer".into(), "启动 NTC 文件资源管理器 TUI（可选起始路径）".into());

    m.insert("cli.run.help_format".into(), "输出格式：txt, html, json, md".into());

    m.insert("cli.run.help_generate_completions".into(), "生成 shell 补全脚本（bash|zsh|fish|powershell）".into());

    m.insert("cli.run.help_helpextra".into(), "显示子命令的扩展帮助".into());

    m.insert("cli.run.help_helpextra".into(), "显示子命令的扩展帮助".into());

    m.insert("cli.run.help_hyper".into(), "创建 hyper 项目脚手架（DSA 库）".into());

    m.insert("cli.run.help_igcare_export_all".into(), "将所有忽略/关注设置导出到 <name>.ntc.igcare".into());

    m.insert("cli.run.help_igcare_export_select".into(), "选择类别导出到 <name>.ntc.igcare".into());

    m.insert("cli.run.help_igcare_import".into(), "从 .ntc.igcare 文件导入忽略/关注设置".into());

    m.insert("cli.run.help_ignore".into(), "显示 ntc 可执行文件位置".into());

    m.insert("cli.run.help_ignore".into(), "显示 ntc 可执行文件位置".into());

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignoref".into(), "ignoref".into());

    m.insert("cli.run.help_ignoren".into(), "ignoren".into());

    m.insert("cli.run.help_image".into(), "将图片转换为文档（JSON, MD, HTML, TXT）".into());

    m.insert("cli.run.help_image_to".into(), "图片转换输出格式：json, md, html, txt".into());

    m.insert("cli.run.help_init".into(), "从模板创建起始文件并在编辑器中打开".into());

    m.insert("cli.run.help_input".into(), "输入文件或目录路径".into());

    m.insert("cli.run.help_lang".into(), "hyper 项目的语言".into());

    m.insert("cli.run.help_list".into(), "列出所有命令行功能".into());

    m.insert("cli.run.help_lsp".into(), "通过 stdio 启动 ntc-math LSP 服务器".into());

    m.insert("cli.run.help_lsp_log".into(), "LSP 调试日志写入路径".into());

    m.insert("cli.run.help_math".into(), "计算数学表达式".into());

    m.insert("cli.run.help_mode".into(), "在普通与管理员模式间切换".into());

    m.insert("cli.run.help_mode".into(), "在普通与管理员模式间切换".into());

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert("cli.run.help_output".into(), "报告的输出文件名".into());

    m.insert("cli.run.help_ral_export_all".into(), "将所有运行别名导出到 <name>.ntc.ral".into());

    m.insert("cli.run.help_ral_export_select".into(), "选择运行别名导出到 <name>.ntc.ral".into());

    m.insert("cli.run.help_ral_import".into(), "从 .ntc.ral 文件导入运行别名".into());

    m.insert("cli.run.help_say".into(), "向标准输出打印文本".into());

    m.insert("cli.run.help_seta".into(), "显示或设置自动建议（幽灵文本）（ON/OFF）".into());

    m.insert("cli.run.help_setc".into(), "显示或设置彩色输出（ON/OFF）".into());

    m.insert("cli.run.help_setd".into(), "显示或设置最大递归深度".into());

    m.insert("cli.run.help_seth".into(), "显示或设置历史记录（路径/ON/OFF/default）".into());

    m.insert("cli.run.help_setl".into(), "显示或设置行号显示（ON/OFF）".into());

    m.insert("cli.run.help_seto".into(), "显示或设置默认输出目录".into());

    m.insert("cli.run.help_sett".into(), "显示或设置线程数".into());

    m.insert("cli.run.help_showcg".into(), "显示当前配置总览".into());

    m.insert("cli.run.help_size".into(), "显示当前目录大小".into());

    m.insert("cli.run.help_path".into(), "目标路径或传送点（与 --size 或 --view 配合使用）".into());

    m.insert("cli.run.help_time".into(), "显示耗时（与 --size 或 --view 一起使用）".into());

    m.insert("cli.run.help_tp_add".into(), "将当前目录保存为传送点".into());

    m.insert("cli.run.help_tp_list".into(), "列出所有传送点".into());

    m.insert("cli.run.help_tp_rm".into(), "删除传送点".into());

    m.insert("cli.run.help_upgrade".into(), "检查更新并将 ntc 升级到最新版本".into());

    m.insert("cli.run.help_version".into(), "显示版本信息".into());

    m.insert("cli.run.help_view".into(), "快速查看当前目录树".into());

    m.insert("cli.run.help_watch".into(), "显示或设置文件监视器（ON/OFF）".into());

    m.insert("cli.run.help_where".into(), "显示 ntc 可执行文件位置".into());

    m.insert("cli.section.config".into(), "配置选项：".into());

    m.insert("cli.section.editor".into(), "编辑器选项：".into());

    m.insert("cli.section.examples".into(), "示例：".into());

    m.insert("cli.section.export".into(), "别名 / 忽略/关注导出：".into());

    m.insert("cli.section.igcare".into(), "忽略/关注选项：".into());

    m.insert("cli.section.interactive_only".into(), "仅交互模式（运行 'ntc' 不带参数）：".into());

    m.insert("cli.section.other".into(), "其他选项：".into());

    m.insert("cli.section.report".into(), "报告选项：".into());

    m.insert("cli.section.shell_commands".into(), "Shell命令（也可从CLI使用）：".into());

    m.insert("cli.section.teleport".into(), "传送保存点：".into());

    m.insert("cli.shell_cmd.dl".into(), " ntc dl https://example.com/file.zip --progress 带进度下载".into());

    m.insert("cli.shell_cmd.encode".into(), " ntc encode b64 hello 对字符串进行Base64编码".into());

    m.insert("cli.shell_cmd.math".into(), " ntc math 3+4*5 计算表达式".into());

    m.insert("cli.shell_cmd.ne".into(), " ntc ne 打开文本编辑器".into());

    m.insert("cli.shell_cmd.pathcp".into(), " ntc pathcp 将路径复制到剪贴板".into());

    m.insert("cli.shell_cmd.qr".into(), " ntc qr \"文本\" 在终端中生成QR码".into());

    m.insert("cli.shell_cmd.ral_add".into(), " ntc ral add btr \"cargo build --release\" 从CLI添加别名\n".into());

    m.insert("cli.shell_cmd.say_loop".into(), " ntc say \"hello\" --loop 3 带循环打印文本".into());

    m.insert("cli.shell_cmd.serve".into(), " ntc serve 8080 在8080端口启动HTTP服务器".into());

    m.insert("cli.shell_cmd.size".into(), " ntc size 显示目录大小".into());

    m.insert("cli.shell_cmd.tl".into(), " ntc tl dir . 统计目录中的代码行数".into());

    m.insert("cli.shell_cmd.view".into(), " ntc view 显示目录树".into());

    m.insert("cli.usage.basic".into(), " ntc [选项]".into());

    m.insert("cli.usage.clipboard".into(), " ntc -i <路径> --cp 将报告复制到剪贴板\n".into());

    m.insert("cli.usage.header".into(), "用法：".into());

    m.insert("cli.usage.input".into(), " ntc -i <路径> [-o <输出>]".into());

    m.insert("cmd.care.dir_desc".into(), "care dir <名称>[, ...] 停止忽略目录".into());

    m.insert("cmd.care.dir_usage".into(), "用法：care dir <名称>[, <名称>, ...]".into());

    m.insert("cmd.care.ext_desc".into(), "care ext <扩展名>[, ...] 添加额外支持的扩展名".into());

    m.insert("cmd.care.ext_usage".into(), "用法：care ext <扩展名>[, <扩展名>, ...]".into());

    m.insert("cmd.care.file_desc".into(), "care file <名称>[, ...] 添加额外支持的文件".into());

    m.insert("cmd.care.file_usage".into(), "用法：care file <名称>[, <名称>, ...]".into());

    m.insert("cmd.care.title".into(), "care \u{2014} 管理关注（取消忽略）的项目".into());

    m.insert("cmd.clipboard.access_fail".into(), "访问剪贴板失败：{}".into());

    m.insert("cmd.clipboard.already_empty".into(), "剪贴板历史已为空。".into());

    m.insert("cmd.clipboard.clear_cancelled".into(), "清除已取消。".into());

    m.insert("cmd.clipboard.clear_prompt".into(), "清除已取消。".into());

    m.insert("cmd.clipboard.clear_prompt".into(), "清除已取消。".into());

    m.insert("cmd.clipboard.cleared".into(), "剪贴板历史已清除。".into());

    m.insert("cmd.clipboard.copied_entry".into(), "已将第 #{} 条（{}）复制到剪贴板。".into());

    m.insert("cmd.clipboard.empty".into(), "剪贴板历史为空。".into());

    m.insert("cmd.clipboard.entry_not_found".into(), "未找到第 #{} 条（共有 {} 条）。".into());

    m.insert("cmd.clipboard.help_clear".into(), "  clip clear         清除剪贴板历史".into());

    m.insert("cmd.clipboard.help_list".into(), "  clip               显示剪贴板历史".into());

    m.insert("cmd.clipboard.help_select".into(), "  clip select <N>    将第 N 条复制回剪贴板".into());

    m.insert("cmd.clipboard.history_header".into(), "  剪贴板历史（最近 {} 条）：".into());

    m.insert("cmd.clipboard.select_usage".into(), "用法：clip select <N>".into());

    m.insert("cmd.clipboard.set_fail".into(), "设置剪贴板失败：{}".into());

    m.insert("cmd.clipboard.termux_unavailable".into(), "termux-clipboard-set 不可用：{}".into());

    m.insert("cmd.clipboard.unknown_sub".into(), "未知的 clip 子命令：{}".into());

    m.insert("cmd.completed".into(), "命令已成功完成。".into());

    m.insert("cmd.config.deinit.confirm".into(), "删除 {}？（y/N）：".into());

    m.insert("cmd.config.deinit.local_not_found".into(), "未找到 ntconfig.toml，跳过。".into());

    m.insert("cmd.config.deinit.local_remove_failed".into(), "删除 ntconfig.toml 失败：{}".into());

    m.insert("cmd.config.deinit.local_removed".into(), "ntconfig.toml 已删除。".into());

    m.insert("cmd.config.deinit.not_found".into(), "未找到 {}，跳过。".into());

    m.insert("cmd.config.deinit.remove_failed".into(), "删除 {} 失败：{}".into());

    m.insert("cmd.config.deinit.removed".into(), "已删除 {}".into());

    m.insert("cmd.config.deinit.skipped".into(), "已跳过。".into());

    m.insert("cmd.config.deinit.usage".into(), "用法：deinit [--all | --ran | --local]".into());

    m.insert("cmd.config.example_depth".into(), " config depth 4  # 设置最大深度为4".into());

    m.insert("cmd.config.example_history".into(), " config history off  # 禁用历史记录".into());

    m.insert("cmd.config.example_linenumbers".into(), " config linenumbers ON  # 启用行号显示".into());

    m.insert("cmd.config.example_output".into(), " config output C:\\reports  # 设置输出路径".into());

    m.insert("cmd.config.example_watch".into(), " config watch ON  # 启用文件监视器".into());

    m.insert("cmd.config.examples_header".into(), "示例：".into());

    m.insert("cmd.config.init.create_failed".into(), "创建 {} 失败：{}".into());

    m.insert("cmd.config.init.created".into(), "已创建 {}".into());

    m.insert("cmd.config.init.exists_warn".into(), "{} 已存在，跳过。".into());

    m.insert("cmd.config.init.local_exists".into(), "ntconfig.toml 已存在，跳过。".into());

    m.insert("cmd.config.init.usage".into(), "用法：init [--all | --ran | --local]".into());

    m.insert("cmd.config.lang.active".into(), "{}（活动中）".into());

    m.insert("cmd.config.lang.available".into(), "可用语言：".into());

    m.insert("cmd.config.lang.coming_soon".into(), "{}（已设置，即将推出）".into());

    m.insert("cmd.config.lang.set".into(), "语言已设置为：{}".into());

    m.insert("cmd.config.lang.set_coming_soon".into(), "语言已设置为：{}（即将推出）".into());

    m.insert("cmd.config.lang.unsupported".into(), "不支持的语言 '{}'。可用：{}".into());

    m.insert("cmd.config.local.create_failed".into(), "创建 ntconfig.toml 失败：{}".into());

    m.insert("cmd.config.local.created_export".into(), "已在 {} 创建 ntconfig.toml（当前设置）".into());

    m.insert("cmd.config.local.created_template".into(), "已在 {} 创建 ntconfig.toml 模板".into());

    m.insert("cmd.config.local.exists_warning".into(), "\u{26a0}\u{fe0f} ntconfig.toml 已存在！".into());

    m.insert("cmd.config.local.export_aliases".into(), " \u{2022} 运行别名".into());

    m.insert("cmd.config.local.export_extras".into(), " \u{2022} 额外支持的文件/扩展名".into());

    m.insert("cmd.config.local.export_ignores".into(), " \u{2022} 忽略的目录/扩展名/文件".into());

    m.insert("cmd.config.local.export_valid".into(), "\u{2705} 已导出有效 TOML 语法的活动配置：".into());

    m.insert("cmd.config.local.init_cancelled".into(), "初始化已取消。".into());

    m.insert("cmd.config.local.overwrite_prompt".into(), "是否覆盖？(y/N)：".into());

    m.insert("cmd.config.local.template_all_hint".into(), " 或运行：local init --all 导出当前设置".into());

    m.insert("cmd.config.local.template_edit_hint".into(), " 编辑文件并删除想启用的行前的 '#'".into());

    m.insert("cmd.config.local.template_hint".into(), "\u{1f4dd} 这是一个已注释的模板 — 尚无活动设置".into());

    m.insert("cmd.config.local.tip_global".into(), "\u{1f4a1} 提示：全局设置（传送点、输出路径、深度）保持全局".into());

    m.insert("cmd.config.local_deinit.cancelled".into(), "反初始化已取消。".into());

    m.insert("cmd.config.local_deinit.confirm".into(), "删除 ntconfig.toml？（y/N）：".into());

    m.insert("cmd.config.local_deinit.done".into(), "ntconfig.toml 已删除。全局配置现在是唯一的活动配置。".into());

    m.insert("cmd.config.local_deinit.global_safe".into(), "全局配置不会受影响。".into());

    m.insert("cmd.config.local_deinit.lose_settings".into(), "本地忽略/关注和运行别名设置将丢失。".into());

    m.insert("cmd.config.local_deinit.not_found".into(), "当前目录中未找到 ntconfig.toml。".into());

    m.insert("cmd.config.local_deinit.remove_failed".into(), "删除 ntconfig.toml 失败：{}".into());

    m.insert("cmd.config.local_deinit.title".into(), "\u{26A0}\u{FE0F} 本地配置删除".into());

    m.insert("cmd.config.local_deinit.will_delete".into(), "此操作将删除：{}".into());

    m.insert("cmd.config.opencg.created".into(), "已在以下位置创建配置文件：{}".into());

    m.insert("cmd.config.opencg.opening".into(), "正在打开配置：{}".into());

    m.insert("cmd.config.qsc.disabled".into(), "QSC热键已禁用。".into());

    m.insert("cmd.config.qsc.hotkey_set".into(), "QSC热键已设置为：{}".into());

    m.insert("cmd.config.qsc.hotkey_status".into(), "QSC热键：{}".into());

    m.insert("cmd.config.qsc.invalid_format".into(), "无效的QSC格式：'{}'。使用 '#c #s 0' 样式。".into());

    m.insert("cmd.config.qsc.not_configured".into(), "未配置".into());

    m.insert("cmd.config.qsc.usage1".into(), "用法：config qsc \"#c 0\"".into());

    m.insert("cmd.config.qsc.usage2".into(), " config qsc \"#c #s a\"".into());

    m.insert("cmd.config.qsc.usage3".into(), " config qsc \"#c #s #a F1\"".into());

    m.insert("cmd.config.qsc.usage4".into(), " config qsc off".into());

    m.insert("cmd.config.resetcg.backup_created".into(), "已创建备份：{}".into());

    m.insert("cmd.config.resetcg.backup_failed".into(), "创建备份失败：{}".into());

    m.insert("cmd.config.resetcg.backup_info".into(), "备份已保存。如有需要，使用 'restorecg' 恢复。".into());

    m.insert("cmd.config.resetcg.cancelled".into(), "重置已取消。".into());

    m.insert("cmd.config.resetcg.config_file_label".into(), "配置文件：".into());

    m.insert("cmd.config.resetcg.default_aliases".into(), " \u{2022} 运行别名 \u{2192} 已清除".into());

    m.insert("cmd.config.resetcg.default_custom_ignores".into(), " \u{2022} 自定义忽略 \u{2192} 已移除".into());

    m.insert("cmd.config.resetcg.default_depth".into(), " \u{2022} 最大深度 \u{2192} 2".into());

    m.insert("cmd.config.resetcg.default_history".into(), " \u{2022} 历史记录 \u{2192} 关闭".into());

    m.insert("cmd.config.resetcg.default_ignored_dirs".into(), " \u{2022} 忽略目录 \u{2192} target, build, venv, node_modules, installer, logs, .git".into());

    m.insert("cmd.config.resetcg.default_linenumbers".into(), " \u{2022} 行号 \u{2192} 关闭".into());

    m.insert("cmd.config.resetcg.default_output".into(), " \u{2022} 输出路径 \u{2192} 桌面".into());

    m.insert("cmd.config.resetcg.default_teleports".into(), " \u{2022} 传送点 \u{2192} 已清除".into());

    m.insert("cmd.config.resetcg.default_threads".into(), " \u{2022} 线程数 \u{2192} 4".into());

    m.insert("cmd.config.resetcg.default_watcher".into(), " \u{2022} 文件监视 \u{2192} 关闭".into());

    m.insert("cmd.config.resetcg.not_found".into(), "未找到配置文件。无需重置。".into());

    m.insert("cmd.config.resetcg.opt_backup".into(), " \u{2022} 输入 'backup' - 重置前创建备份".into());

    m.insert("cmd.config.resetcg.opt_no".into(), " \u{2022} 输入 'no' - 取消".into());

    m.insert("cmd.config.resetcg.opt_yes".into(), " \u{2022} 输入 'yes' - 重置配置（不备份）".into());

    m.insert("cmd.config.resetcg.options_label".into(), "选项：".into());

    m.insert("cmd.config.resetcg.reset_done".into(), "配置已重置为默认值！".into());

    m.insert("cmd.config.resetcg.serialize_failed".into(), "序列化默认配置失败".into());

    m.insert("cmd.config.resetcg.warning_desc".into(), "此操作将把配置重置为默认值：".into());

    m.insert("cmd.config.resetcg.warning_title".into(), "\u{26A0}\u{FE0F} 配置重置警告".into());

    m.insert("cmd.config.resetcg.write_failed".into(), "写入配置失败：{}".into());

    m.insert("cmd.config.resetcg.your_choice".into(), "您的选择：".into());

    m.insert("cmd.config.restorecg.backed_up".into(), "当前配置已备份到：{}".into());

    m.insert("cmd.config.restorecg.cancel_opt".into(), "0. 取消".into());

    m.insert("cmd.config.restorecg.cancelled".into(), "恢复已取消。".into());

    m.insert("cmd.config.restorecg.failed".into(), "恢复失败：{}".into());

    m.insert("cmd.config.restorecg.invalid".into(), "无效的选择。".into());

    m.insert("cmd.config.restorecg.none".into(), "未找到备份文件。".into());

    m.insert("cmd.config.restorecg.reloaded".into(), "配置已重新加载！运行 'showcg' 查看恢复的设置。".into());

    m.insert("cmd.config.restorecg.restored".into(), "已从以下位置恢复配置：{}".into());

    m.insert("cmd.config.restorecg.title".into(), "\u{1F4CB} 可用备份：".into());

    m.insert("cmd.config.mgmt_header".into(), "管理：".into());

    m.insert("cmd.config.settings_header".into(), "设置：".into());

    m.insert("cmd.config.showcg.lang_coming_soon".into(), "{}（即将推出）".into());

    m.insert("cmd.config.showcg.lang_label".into(), "语言：".into());

    m.insert("cmd.config.showcg.qsc_label".into(), "QSC热键：".into());

    m.insert("cmd.config.title".into(), "config \u{2014} 查看或更改配置设置".into());

    m.insert("cmd.config.watch.alias_not_found".into(), "未找到别名 '{}'。使用 'ral list' 查看别名。".into());

    m.insert("cmd.config.watch.trigger_cleared".into(), "监视器触发别名已清除。".into());

    m.insert("cmd.config.watch.trigger_label".into(), "触发别名：".into());

    m.insert("cmd.config.watch.trigger_off_usage".into(), "config watch trigger off 禁用自动运行".into());

    m.insert("cmd.config.watch.trigger_set".into(), "监视器触发已设置为别名：'{}'".into());

    m.insert("cmd.config.watch.trigger_usage".into(), "config watch trigger <别名> 更改时自动运行别名".into());

    m.insert("cmd.config.watch.limit_usage".into(), "config watch limit <数量> 更改框中显示的行数".into());

    m.insert("cmd.config.watch.usage".into(), "使用：config watch 开 | config watch 关 | config watch trigger <别名> | config watch trigger off | config watch limit <数量>".into());

    m.insert("cmd.config.watch.usage_line".into(), "用法：config watch on|off".into());

    m.insert("cmd.fmt.file_not_found".into(), "文件未找到：{}".into());

    m.insert("cmd.fmt.invalid_json".into(), "无效的 JSON：{}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "无效的 TOML：{}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "无效的 YAML：{}".into());

    m.insert("cmd.fmt.json_ser_err".into(), "JSON 序列化错误：{}".into());

    m.insert("cmd.fmt.json_ser_err".into(), "JSON 序列化错误：{}".into());

    m.insert("cmd.fmt.missing_file".into(), "缺少文件路径".into());

    m.insert("cmd.fmt.read_fail".into(), "读取文件失败：{}".into());

    m.insert("cmd.fmt.toml_conv_err".into(), "TOML 转换错误：{}".into());

    m.insert("cmd.fmt.toml_ser_err".into(), "TOML 序列化错误：{}".into());

    m.insert("cmd.fmt.unknown_format".into(), "未知格式：'{}'。请使用 json、yaml 或 toml。".into());

    m.insert("cmd.fmt.unknown_target".into(), "未知的目标格式：'{}'。请使用 json、yaml 或 toml。".into());

    m.insert("cmd.fmt.usage".into(), "用法：fmt json|yaml|toml <文件> [to json|yaml|toml] [-o <文件>]".into());

    m.insert("cmd.fmt.written_to".into(), "已写入：{}".into());

    m.insert("cmd.fmt.yaml_ser_err".into(), "YAML 序列化错误：{}".into());

    m.insert("cmd.gs.no_matches".into(), "未找到匹配项。".into());

    m.insert("cmd.http.authorization_label".into(), "授权：".into());

    m.insert("cmd.http.authorization_label".into(), "授权：".into());

    m.insert("cmd.http.basic_placeholder".into(), "Basic <编码>".into());

    m.insert("cmd.http.basic_placeholder".into(), "Basic <编码>".into());

    m.insert("cmd.http.bearer_placeholder".into(), "Bearer <令牌>".into());

    m.insert("cmd.http.bearer_placeholder".into(), "Bearer <令牌>".into());

    m.insert("cmd.http.byte_count".into(), "{} 字节".into());

    m.insert("cmd.http.byte_count".into(), "{} 字节".into());

    m.insert("cmd.http.help_auth".into(), "  -a, --auth <用户:密码>       基本认证".into());

    m.insert("cmd.http.help_bearer".into(), "  --bearer <令牌>             Bearer 令牌认证".into());

    m.insert("cmd.http.help_body".into(), "  -d, --data <请求体>             请求体".into());

    m.insert("cmd.http.help_example1".into(), "  http GET https://api.github.com/users/octocat".into());

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert("cmd.http.help_example3".into(), "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into());

    m.insert("cmd.http.help_example4".into(), "  http GET https://api.example.com/data --bearer mytoken -o response.json".into());

    m.insert("cmd.http.help_example5".into(), "  http GET https://api.example.com/health -s -t 5".into());

    m.insert("cmd.http.help_example6".into(), "  http GET https://api.example.com/users -a admin:secret123".into());

    m.insert("cmd.http.help_examples_title".into(), "示例：".into());

    m.insert("cmd.http.help_examples_title".into(), "示例：".into());

    m.insert("cmd.http.help_header".into(), "  -H, --header <\"键: 值\">    添加请求头（可重复）".into());

    m.insert("cmd.http.help_help".into(), "  -h, --help                     显示此帮助".into());

    m.insert("cmd.http.help_json".into(), "  --json <数据>                  以 JSON 发送（自动设置 Content-Type）".into());

    m.insert("cmd.http.help_methods".into(), "  GET, POST, PUT, DELETE, PATCH, HEAD".into());

    m.insert("cmd.http.help_methods_title".into(), "方法：".into());

    m.insert("cmd.http.help_methods_title".into(), "方法：".into());

    m.insert("cmd.http.help_no_pretty".into(), "  --no-pretty                    禁用 JSON 美化输出".into());

    m.insert("cmd.http.help_options_title".into(), "选项：".into());

    m.insert("cmd.http.help_options_title".into(), "选项：".into());

    m.insert("cmd.http.help_output".into(), "  -o, --output <文件>            将响应体保存到文件（原始）".into());

    m.insert("cmd.http.help_query".into(), "  -q, --query <键=值>             URL 查询参数（可重复）".into());

    m.insert("cmd.http.help_silent".into(), "  -s, --silent                   只显示响应体（不显示状态/请求头）".into());

    m.insert("cmd.http.help_timeout".into(), "  -t, --timeout <秒>             请求超时（默认 30）".into());

    m.insert("cmd.http.help_title".into(), "HTTP API 客户端".into());

    m.insert("cmd.http.help_title".into(), "HTTP API 客户端".into());

    m.insert("cmd.http.help_usage_default".into(), "  <url>                          （如果第一个参数是 URL 则默认为 GET）".into());

    m.insert("cmd.http.help_usage_line".into(), "  http <方法> <url> [选项]".into());

    m.insert("cmd.http.help_usage_title".into(), "用法：".into());

    m.insert("cmd.http.help_usage_title".into(), "用法：".into());

    m.insert("cmd.http.help_verbose".into(), "  -v, --verbose                 显示传出请求详情".into());

    m.insert("cmd.http.invalid_body".into(), "无效的请求体：{}".into());

    m.insert("cmd.http.invalid_body".into(), "无效的请求体：{}".into());

    m.insert("cmd.http.invalid_header".into(), "无效的请求头格式，应为 \"键: 值\"，实际为：{}".into());

    m.insert("cmd.http.invalid_header".into(), "无效的请求头格式，应为 \"键: 值\"，实际为：{}".into());

    m.insert("cmd.http.invalid_query".into(), "无效的查询格式，应为 键=值，实际为：{}".into());

    m.insert("cmd.http.invalid_query".into(), "无效的查询格式，应为 键=值，实际为：{}".into());

    m.insert("cmd.http.invalid_request".into(), "无效的请求：{}".into());

    m.insert("cmd.http.invalid_request".into(), "无效的请求：{}".into());

    m.insert("cmd.http.invalid_timeout".into(), "无效的超时：{}".into());

    m.insert("cmd.http.invalid_timeout".into(), "无效的超时：{}".into());

    m.insert("cmd.http.missing_auth".into(), "-a 后缺少 用户:密码".into());

    m.insert("cmd.http.missing_auth".into(), "-a 后缺少 用户:密码".into());

    m.insert("cmd.http.missing_bearer".into(), "--bearer 后缺少令牌".into());

    m.insert("cmd.http.missing_bearer".into(), "--bearer 后缺少令牌".into());

    m.insert("cmd.http.missing_body_data".into(), "-d 后缺少请求体数据".into());

    m.insert("cmd.http.missing_body_data".into(), "-d 后缺少请求体数据".into());

    m.insert("cmd.http.missing_filename".into(), "-o 后缺少文件名".into());

    m.insert("cmd.http.missing_filename".into(), "-o 后缺少文件名".into());

    m.insert("cmd.http.missing_header".into(), "-H 后缺少请求头值".into());

    m.insert("cmd.http.missing_header".into(), "-H 后缺少请求头值".into());

    m.insert("cmd.http.missing_json".into(), "--json 后缺少 JSON 数据".into());

    m.insert("cmd.http.missing_json".into(), "--json 后缺少 JSON 数据".into());

    m.insert("cmd.http.missing_query".into(), "-q 后缺少 键=值".into());

    m.insert("cmd.http.missing_query".into(), "-q 后缺少 键=值".into());

    m.insert("cmd.http.missing_timeout".into(), "-t 后缺少秒数".into());

    m.insert("cmd.http.missing_timeout".into(), "-t 后缺少秒数".into());

    m.insert("cmd.http.request_failed".into(), "{} {} 失败".into());

    m.insert("cmd.http.request_failed".into(), "{} {} 失败".into());

    m.insert("cmd.http.saved_bytes".into(), "已将 {} 字节保存到 {}".into());

    m.insert("cmd.http.saved_bytes".into(), "已将 {} 字节保存到 {}".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.unknown_status".into(), "未知".into());

    m.insert("cmd.http.unknown_status".into(), "未知".into());

    m.insert("cmd.http.url_required".into(), "需要 URL。用法：http <方法> <url>".into());

    m.insert("cmd.http.url_required".into(), "需要 URL。用法：http <方法> <url>".into());

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "写入 {} 失败".into());

    m.insert("cmd.http.write_fail".into(), "写入 {} 失败".into());

    m.insert("cmd.igcar.main_title".into(), "忽略/关注（igcare）命令：".into());

    m.insert("cmd.igcare.create_failed".into(), "创建文件失败：{}".into());

    m.insert("cmd.igcare.example_export_all".into(), "  igcare export --all myproject        # Creates myproject.ntc.igcare".into());

    m.insert("cmd.igcare.example_export_select".into(), "  igcare export --select myproject     # Pick which categories to export".into());

    m.insert("cmd.igcare.example_import".into(), "  igcare import myproject.ntc.igcare   # Import into current config".into());

    m.insert("cmd.igcare.examples_title".into(), "示例：".into());

    m.insert("cmd.igcare.examples_title".into(), "示例：".into());

    m.insert("cmd.igcare.export_cmd".into(), "igcare export --all <名称> 将所有设置导出到 <名称>.ntc.igcare".into());

    m.insert("cmd.igcare.export_name_empty".into(), "导出名称不能为空。".into());

    m.insert("cmd.igcare.export_select_cmd".into(), "igcare export --select <名称> 选择类别导出到 <名称>.ntc.igcare".into());

    m.insert("cmd.igcare.export_usage".into(), "用法：igcare export --all <名称> | igcare export --select <名称>".into());

    m.insert("cmd.igcare.file_exists".into(), "文件已存在：{}".into());

    m.insert("cmd.igcare.import_cmd".into(), "igcare import <文件> 从 .ntc.igcare 文件导入设置".into());

    m.insert("cmd.igcare.import_example".into(), "示例：igcare import myproject.ntc.igcare".into());

    m.insert("cmd.igcare.import_usage".into(), "用法：igcare import <文件>".into());

    m.insert("cmd.igcare.main_title".into(), "忽略/关注管理器".into());

    m.insert("cmd.igcare.main_title".into(), "忽略/关注管理器".into());

    m.insert("cmd.igcare.template_created".into(), "已创建 igcare 模板：{}".into());

    m.insert("cmd.igcare.template_hint".into(), "这是一个本地模板文件。使用 'igcare import' 应用设置。".into());

    m.insert("cmd.igcare.type_help".into(), "输入 'igcare' 获取帮助。".into());

    m.insert("cmd.igcare.unknown_flag".into(), "未知的导出标志：{}。使用 --all 或 --select".into());

    m.insert("cmd.igcare.unknown_subcmd".into(), "未知的 igcare 子命令：{}".into());

    m.insert("cmd.igcared.combined_hint".into(), "全局 + 本地设置合并：".into());

    m.insert("cmd.igcared.combined_hint".into(), "全局 + 本地设置合并：".into());

    m.insert("cmd.igcared.extra_exts".into(), "额外扩展名：{:?}".into());

    m.insert("cmd.igcared.extra_exts".into(), "额外扩展名：{:?}".into());

    m.insert("cmd.igcared.extra_files".into(), "额外文件：{:?}".into());

    m.insert("cmd.igcared.extra_files".into(), "额外文件：{:?}".into());

    m.insert("cmd.igcared.ignored_dirs".into(), "忽略的目录：{:?}".into());

    m.insert("cmd.igcared.ignored_dirs".into(), "忽略的目录：{:?}".into());

    m.insert("cmd.igcared.ignored_exts".into(), "忽略的扩展名：{:?}".into());

    m.insert("cmd.igcared.ignored_exts".into(), "忽略的扩展名：{:?}".into());

    m.insert("cmd.igcared.ignored_files".into(), "忽略的文件：{:?}".into());

    m.insert("cmd.igcared.ignored_files".into(), "忽略的文件：{:?}".into());

    m.insert("cmd.igcared.local_config".into(), "    本地配置：{}".into());

    m.insert("cmd.igcared.local_config".into(), "    本地配置：{}".into());

    m.insert("cmd.igcared.local_tip".into(), "运行 'igcared' 查看合并设置".into());

    m.insert("cmd.igcared.local_tip2".into(), "使用 'igcared' 查看当前状态".into());

    m.insert("cmd.igcared.separator".into(), "─".into());

    m.insert("cmd.igcared.separator".into(), "─".into());

    m.insert("cmd.igcared.title_combined".into(), "合并设置".into());

    m.insert("cmd.igcared.title_combined".into(), "合并设置".into());

    m.insert("cmd.igcared.title_global".into(), "全局设置".into());

    m.insert("cmd.igcared.title_global".into(), "全局设置".into());

    m.insert("cmd.ignore.applied".into(), "{} {} 个项目".into());

    m.insert("cmd.ignore.care_example_dir".into(), "  care dir build,dist".into());

    m.insert("cmd.ignore.care_example_ext".into(), "  care ext txt,md,json".into());

    m.insert("cmd.ignore.care_example_ext".into(), "  care ext txt,md,json".into());

    m.insert("cmd.ignore.care_example_file".into(), "  care file .env".into());

    m.insert("cmd.ignore.care_examples_title".into(), "关注示例：".into());

    m.insert("cmd.ignore.care_examples_title".into(), "关注示例：".into());

    m.insert("cmd.ignore.check_hint".into(), "运行 'igcared' 查看当前忽略/关注设置".into());

    m.insert("cmd.ignore.dir_desc".into(), "ignore dir <名称>[, ...] 忽略目录".into());

    m.insert("cmd.ignore.dir_usage".into(), "用法：ignore dir <名称>[, <名称>, ...]".into());

    m.insert("cmd.ignore.example_dir".into(), "  ignore dir temp,logs".into());

    m.insert("cmd.ignore.example_ext".into(), "  ignore ext log,tmp,bak".into());

    m.insert("cmd.ignore.example_file".into(), "  ignore file Cargo.lock".into());

    m.insert("cmd.ignore.examples_title".into(), "示例：".into());

    m.insert("cmd.ignore.examples_title".into(), "示例：".into());

    m.insert("cmd.ignore.ext_desc".into(), "ignore ext <扩展名>[, ...] 忽略文件扩展名".into());

    m.insert("cmd.ignore.ext_usage".into(), "用法：ignore ext <扩展名>[, <扩展名>, ...]".into());

    m.insert("cmd.ignore.file_desc".into(), "ignore file <名称>[, ...] 忽略特定文件".into());

    m.insert("cmd.ignore.file_usage".into(), "用法：ignore file <名称>[, <名称>, ...]".into());

    m.insert("cmd.ignore.title".into(), "ignore \u{2014} 管理已忽略的项目".into());

    m.insert("cmd.image.cancelled".into(), "已取消。".into());

    m.insert("cmd.image.cancelled".into(), "已取消。".into());

    m.insert("cmd.image.default_label".into(), "默认".into());

    m.insert("cmd.image.default_label".into(), "默认".into());

    m.insert("cmd.image.desc_cancel".into(), "  {}. 取消".into());

    m.insert("cmd.image.desc_html".into(), "  {}. HTML".into());

    m.insert("cmd.image.desc_json".into(), "  {}. JSON".into());

    m.insert("cmd.image.desc_markdown".into(), "  {}. Markdown（默认）".into());

    m.insert("cmd.image.desc_narrative".into(), "  {}. 叙述性描述（供 AI 使用）".into());

    m.insert("cmd.image.desc_txt".into(), "  {}. TXT".into());

    m.insert("cmd.image.enter_number".into(), "请输入编号：".into());

    m.insert("cmd.image.enter_number".into(), "请输入编号：".into());

    m.insert("cmd.image.missing_out_format".into(), "'out' 后缺少格式。".into());

    m.insert("cmd.image.missing_to_format".into(), "'to' 后缺少格式。".into());

    m.insert("cmd.image.no_files".into(), "当前目录中未找到图像文件。".into());

    m.insert("cmd.image.select_image".into(), "选择图片：".into());

    m.insert("cmd.image.select_image".into(), "选择图片：".into());

    m.insert("cmd.image.displaying".into(), "显示 {} ({}x{}) · {}".into());

    m.insert("cmd.image.usage".into(), "用法：image <路径> [display|show|to|out <格式>] [--ocr] [--layout]".into());

    m.insert("cmd.ipcfg.dns_label".into(), "DNS ......... :".into());

    m.insert("cmd.ipcfg.no_adapters".into(), "未找到网络适配器。".into());

    m.insert("cmd.ipcfg.no_such_adapter".into(), "没有这个网卡：'{}'。".into());

    m.insert("cmd.ipcfg.title".into(), "网络适配器：".into());

    m.insert("cmd.image_disabled".into(), "图像转换不可用（'image2doc' 功能已禁用）。".into());

    m.insert("cmd.mode.admin_relaunch".into(), "正在以管理员身份重新启动 ntc...".into());

    m.insert("cmd.mode.elevate_fail".into(), "启动提权进程失败：{}".into());

    m.insert("cmd.mode.exe_fail".into(), "获取可执行文件路径失败：{}".into());

    m.insert("cmd.mode.launch_fail".into(), "启动进程失败：{}".into());

    m.insert("cmd.mode.normal_relaunch".into(), "正在以普通模式重新启动 ntc...".into());

    m.insert("cmd.mode.windows_only".into(), "mode 是仅限 Windows 的功能。".into());

    m.insert("cmd.nav.back.invalid_number".into(), "无效数字：{}。用法：back [n]".into());

    m.insert("cmd.nav.back.invalid_step".into(), "第 {} 步空父目录 - 无处可返回".into());

    m.insert("cmd.nav.cd.example1".into(), "示例：cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), " cd subdir".into());

    m.insert("cmd.nav.cd.example3".into(), " cd to work # 传送到 'work' 保存点".into());

    m.insert("cmd.nav.cd.navigated".into(), "已导航至：{}".into());

    m.insert("cmd.nav.cd.tp_list".into(), "使用 'tp list' 查看所有保存点。".into());

    m.insert("cmd.nav.cd.tp_not_found".into(), "未找到传送点：'{}'".into());

    m.insert("cmd.nav.cd.usage1".into(), "用法：cd <目录路径>".into());

    m.insert("cmd.nav.cd.usage2".into(), " cd to <传送点名称> 传送到保存点".into());

    m.insert("cmd.nav.drive_windows_only".into(), "驱动器导航仅在 Windows 上支持。".into());

    m.insert("cmd.nav.fgo.cancel".into(), "0. 取消".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "已取消。".into());

    m.insert("cmd.nav.fgo.example1".into(), " fgo main.rs # 搜索然后选择文件导航到其父目录".into());

    m.insert("cmd.nav.fgo.example2".into(), " fgo main.rs -d 5 # 最多搜索 5 层深度".into());

    m.insert("cmd.nav.fgo.found".into(), "\u{1f50d} 找到 {} 个匹配 \"{}\" 的文件：".into());

    m.insert("cmd.nav.fgo.select".into(), "选择要导航到其父目录的文件：".into());

    m.insert("cmd.nav.fgo_usage".into(), "用法：fgo <模式> [-d <深度>]".into());

    m.insert("cmd.nav.gos.empty".into(), "（无子目录）".into());

    m.insert("cmd.nav.gos.exit".into(), "退出".into());

    m.insert("cmd.nav.gos.staying".into(), "停留在此处。".into());

    m.insert("cmd.nav.invalid_drive".into(), "无效的驱动器编号。".into());

    m.insert("cmd.nav.invalid_input".into(), "无效的输入。".into());

    m.insert("cmd.nav.invalid_number".into(), "无效的数字。".into());

    m.insert("cmd.nav.invalid_selection".into(), "无效的选择。".into());

    m.insert("cmd.nav.where.config_file".into(), "\u{2699}\u{fe0f} 配置文件：".into());

    m.insert("cmd.nav.where.config_missing".into(), "\u{26a0} 未找到配置文件（将在首次保存时创建）".into());

    m.insert("cmd.nav.where.config_size".into(), "\u{1f4cf} 配置大小：".into());

    m.insert("cmd.nav.where.current_dir".into(), "\u{1f4c2} 当前目录：".into());

    m.insert("cmd.nav.where.executable".into(), "\u{1f4c1} 可执行文件：".into());

    m.insert("cmd.nav.where.title".into(), "ntc 位置信息".into());

    m.insert("cmd.pathman.access_denied".into(), "拒绝访问 - 需要管理员权限。".into());

    m.insert("cmd.pathman.added_to_path".into(), "已添加到 {}：{}".into());

    m.insert("cmd.pathman.already_in_path".into(), "已在 PATH 中：{}".into());

    m.insert("cmd.pathman.empty_path".into(), "║  {}".into());

    m.insert("cmd.pathman.example_add".into(), "  {} add C:\\Tools".into());

    m.insert("cmd.pathman.example_add_cwd".into(), "  {} add --cwd".into());

    m.insert("cmd.pathman.example_move".into(), "  {} move 3 1               将第 3 条移到顶部".into());

    m.insert("cmd.pathman.example_rm_idx".into(), "  {} rm 3                    从列表中移除第 3 条".into());

    m.insert("cmd.pathman.example_rm_path".into(), "  {} rm C:\\OldTools".into());

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "示例：".into());

    m.insert("cmd.pathman.examples_title".into(), "示例：".into());

    m.insert("cmd.pathman.help_add".into(), "  {} add <路径>               向 PATH 添加目录".into());

    m.insert("cmd.pathman.help_add_cwd".into(), "  {} add --cwd | --here      将当前目录添加到 PATH".into());

    m.insert("cmd.pathman.help_move".into(), "  {} move <n> <m>            将第 n 条移动到第 m 位".into());

    m.insert("cmd.pathman.help_rm".into(), "  {} rm <路径或序号>          按路径或序号从 PATH 中移除".into());

    m.insert("cmd.pathman.help_show".into(), "  {} show                    显示带序号的 PATH 条目".into());

    m.insert("cmd.pathman.idx_out_of_range".into(), "序号超出范围：{}（1-{}）".into());

    m.insert("cmd.pathman.indices_numbers".into(), "索引必须是数字".into());

    m.insert("cmd.pathman.mode_admin_hint".into(), "然后重新运行命令。或使用：mode admin".into());

    m.insert("cmd.pathman.move_usage".into(), "用法：{} move <n> <m>".into());

    m.insert("cmd.pathman.moved".into(), "已移动到 {} 中的第 #{} 位：{}".into());

    m.insert("cmd.pathman.new_terminal".into(), "更改将应用于新的终端窗口。".into());

    m.insert("cmd.pathman.not_found_in_path".into(), "PATH 中未找到：{}".into());

    m.insert("cmd.pathman.path_update_fail".into(), "更新 PATH 失败：{}".into());

    m.insert("cmd.pathman.path_update_fail".into(), "更新 PATH 失败：{}".into());

    m.insert("cmd.pathman.reg_query_fail".into(), "查询注册表失败：{}".into());

    m.insert("cmd.pathman.reg_write_fail".into(), "写入注册表失败：{}".into());

    m.insert("cmd.pathman.removed_from_path".into(), "已从 {} 移除：{}".into());

    m.insert("cmd.pathman.rm_usage".into(), "用法：usrpath rm <路径或编号>".into());

    m.insert("cmd.pathman.run_as_admin".into(), "请以管理员身份运行终端（右键 → \"以管理员身份运行\"），".into());

    m.insert("cmd.pathman.same_source_target".into(), "源和目标相同 \u{2014} 无需移动".into());

    m.insert("cmd.pathman.system_level".into(), "系统".into());

    m.insert("cmd.pathman.system_level".into(), "系统".into());

    m.insert("cmd.pathman.total_entries".into(), "  总计：{} 条".into());

    m.insert("cmd.pathman.usage_title".into(), "用法：".into());

    m.insert("cmd.pathman.usage_title".into(), "用法：".into());

    m.insert("cmd.pathman.user_level".into(), "用户".into());

    m.insert("cmd.pathman.user_level".into(), "用户".into());

    m.insert("cmd.pathman.windows_only".into(), "usrpath/syspath 是仅限 Windows 的功能。".into());

    m.insert("cmd.qr.decode_fail".into(), "解码二维码 #{} 失败：{}".into());

    m.insert("cmd.qr.decode_requires".into(), "二维码解码需要 'image2doc' 功能（默认启用）。使用 --features image2doc 运行。".into());

    m.insert("cmd.qr.decode_usage".into(), "用法：qr decode <图像文件>".into());

    m.insert("cmd.qr.encode_usage".into(), "用法：qr encode <文本|图像> [-o <文件>]".into());

    m.insert("cmd.qr.example1".into(), "  qr \"你好，世界！\"".into());

    m.insert("cmd.qr.example2".into(), "  qr encode \"hello world\" -o qr.png".into());

    m.insert("cmd.qr.example3".into(), "  qr decode qr.png".into());

    m.insert("cmd.qr.example_image".into(), "  qr encode image.png -o imageqrcode.png".into());

    m.insert("cmd.qr.examples_title".into(), "示例：".into());

    m.insert("cmd.qr.file_not_found".into(), "未找到文件：{}".into());

    m.insert("cmd.qr.gen_fail".into(), "生成二维码失败：{}".into());

    m.insert("cmd.qr.help_decode".into(), "  qr decode <图像文件>            从图像解码二维码".into());

    m.insert("cmd.qr.help_encode".into(), "  qr <文本>                        编码文本并在终端中显示二维码".into());

    m.insert("cmd.qr.help_same_as_above".into(), "  qr encode <文本>                 同上".into());

    m.insert("cmd.qr.help_save_png".into(), "  qr encode <文本> -o <文件>       将二维码保存为 PNG 图像".into());

    m.insert("cmd.qr.help_encode_image".into(), "  qr encode <图像> -o <文件>       将图像编码为二维码（自动缩小）".into());

    m.insert("cmd.qr.image_save_requires".into(), "图像保存需要 'image2doc' 功能（默认启用）".into());

    m.insert("cmd.qr.image_requires_out".into(), "编码图像需要指定输出文件：qr encode <图像> -o <文件>.png".into());

    m.insert("cmd.qr.image_too_large".into(), "图像即使缩小到最小尺寸仍无法放入二维码。".into());

    m.insert("cmd.qr.no_qr_detected".into(), "图像中未检测到二维码。".into());

    m.insert("cmd.qr.no_text".into(), "没有要编码的文本".into());

    m.insert("cmd.qr.open_image_fail".into(), "打开图像失败：{}".into());

    m.insert("cmd.qr.qr_label".into(), "{} {}".into());

    m.insert("cmd.qr.saved".into(), "二维码已保存到：{}".into());

    m.insert("cmd.qr.usage_title".into(), "二维码（QR Code）：".into());

    m.insert("cmd.qsc.already_running".into(), "QSC 守护进程已在运行。先使用 'qsc stop'。".into());

    m.insert("cmd.qsc.current_setting".into(), "  当前：{}".into());

    m.insert("cmd.qsc.current_setting".into(), "  当前：{}".into());

    m.insert("cmd.qsc.daemon_running".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_stopped".into(), "  {}".into());

    m.insert("cmd.qsc.daemon_unknown".into(), "  {} ({})".into());

    m.insert("cmd.qsc.daemon_unknown_err".into(), "QSC 守护进程：未知（{}）".into());

    m.insert("cmd.qsc.help_conf_ex1".into(), "    config qsc \\\"#c 0\\\"        Set hotkey to Ctrl+0".into());

    m.insert("cmd.qsc.help_conf_ex2".into(), "    config qsc \\\"#c #s a\\\"     Set hotkey to Ctrl+Shift+A".into());

    m.insert("cmd.qsc.help_conf_ex3".into(), "    config qsc \\\"#c #s #a F1\\\"  Set hotkey to Ctrl+Shift+Alt+F1".into());

    m.insert("cmd.qsc.help_conf_off".into(), "    config qsc off            Disable QSC hotkey".into());

    m.insert("cmd.qsc.help_configure".into(), "  请先配置热键：".into());

    m.insert("cmd.qsc.help_listen".into(), "  qsc listen              启动 QSC 全局热键守护进程".into());

    m.insert("cmd.qsc.help_listen".into(), "  qsc listen              启动 QSC 全局热键守护进程".into());

    m.insert("cmd.qsc.help_status".into(), "  qsc status              检查 QSC 守护进程是否运行".into());

    m.insert("cmd.qsc.help_status".into(), "  qsc status              检查 QSC 守护进程是否运行".into());

    m.insert("cmd.qsc.help_stop".into(), "  qsc stop                停止 QSC 全局热键守护进程".into());

    m.insert("cmd.qsc.help_stop".into(), "  qsc stop                停止 QSC 全局热键守护进程".into());

    m.insert("cmd.qsc.hotkey_label".into(), "  热键：{}".into());

    m.insert("cmd.qsc.hotkey_not_configured".into(), "  {}".into());

    m.insert("cmd.qsc.not_configured".into(), "未配置 QSC 热键。先使用 'config qsc \"#c 0\"'。".into());

    m.insert("cmd.qsc.not_running".into(), "QSC 守护进程未在运行。".into());

    m.insert("cmd.qsc.started".into(), "QSC 守护进程已启动。".into());

    m.insert("cmd.qsc.startup_registered".into(), "QSC 守护进程已注册为启动应用程序。".into());

    m.insert("cmd.qsc.startup_removed".into(), "QSC 守护进程已从启动应用程序中删除。".into());

    m.insert("cmd.qsc.startup_usage".into(), "用法：qsc startup on|off".into());

    m.insert("cmd.qsc.status_running".into(), "QSC 守护进程：运行中".into());

    m.insert("cmd.qsc.status_stopped".into(), "QSC 守护进程：已停止".into());

    m.insert("cmd.qsc.stopped".into(), "QSC 守护进程已停止。".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC 全局热键：".into());

    m.insert("cmd.qsc.usage_title".into(), "QSC 全局热键：".into());

    m.insert("cmd.report.content_saved".into(), "文件内容已保存到：{}".into());

    m.insert("cmd.report.copied_clipboard".into(), "文件 '{}' 已复制到剪贴板！".into());

    m.insert("cmd.report.copy_failed".into(), "复制失败：{}".into());

    m.insert("cmd.report.copy_failed".into(), "复制失败：{}".into());

    m.insert("cmd.report.copying_clipboard".into(), "正在将 '{}' 复制到剪贴板...".into());

    m.insert("cmd.report.dir_only".into(), "{} 报告只能用于目录".into());

    m.insert("cmd.report.example_json".into(), "  report json ..         对父目录生成报告".into());

    m.insert("cmd.report.example_txt".into(), "  report txt             对当前目录生成报告".into());

    m.insert("cmd.report.examples".into(), "示例：".into());

    m.insert("cmd.report.examples".into(), "示例：".into());

    m.insert("cmd.report.file_not_found".into(), "文件未找到：{}".into());

    m.insert("cmd.report.help_docx".into(), "  report docx [路径]      生成 DOCX 报告".into());

    m.insert("cmd.report.help_html".into(), "  report html [路径]      生成 HTML 报告".into());

    m.insert("cmd.report.help_json".into(), "  report json [路径]      生成 JSON 报告".into());

    m.insert("cmd.report.help_md".into(), "  report md [路径]        生成 Markdown 报告".into());

    m.insert("cmd.report.help_pdf".into(), "  report pdf [路径]       生成 PDF 报告".into());

    m.insert("cmd.report.help_txt".into(), "  report txt [路径]       生成 TXT 报告".into());

    m.insert("cmd.report.help_xlsx".into(), "  report xlsx [路径]      生成 XLSX 报告".into());

    m.insert("cmd.report.invalid_depth".into(), "无效的深度：{}".into());

    m.insert("cmd.report.neovim_hint".into(), "可以用 Neovim 打开：:edit {}".into());

    m.insert("cmd.report.skipped_unsupported".into(), "已跳过（不支持的格式）：{}".into());

    m.insert("cmd.report.ext_usage".into(), "用法：view -ext <扩展名...>（例如 view -ext rs c）".into());

    m.insert("cmd.report.time_label".into(), "耗时：".into());

    m.insert("cmd.report.title".into(), "report — 生成项目报告".into());

    m.insert("cmd.report.unknown_option".into(), "未知的 view 选项：{}".into());

    m.insert("cmd.report.view_usage".into(), "用法：view [-s|--size] [-c|--cp] [--care] [-t|--time] [-d|--depth <n>] [-p|--path <路径|@tp>] [-ext|--ext <扩展名...>]".into());

    m.insert("cmd.scan.depth_usage".into(), "用法：scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]".into());
    m.insert("cmd.scan.examples_title".into(), "示例：".into());
    m.insert("cmd.scan.ext_usage".into(), "用法：scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]".into());
    m.insert("cmd.scan.help_ext".into(), "  scan ext <扩展名...> [-d|--depth <n>] [-s|--size] [--care]  按扩展名列出文件（不统计行数）".into());

    m.insert("cmd.scan.help_port".into(), "  scan port [主机] [--top N | --ports <范围> | --all] [--timeout MS]  TCP 端口扫描（默认本机，前 20 个端口）".into());

    m.insert("cmd.scan.help_ip".into(), "  scan ip [网段] [--top N | --ports <范围>] [--timeout MS]  存活主机扫描（默认自动本地 /24）".into());

    m.insert("cmd.scan.ip_arp_only".into(), "(arp)".into());

    m.insert("cmd.scan.ip_done".into(), "存活：{} / 已扫 {}".into());

    m.insert("cmd.scan.ip_none".into(), "{} 中没有存活主机".into());

    m.insert("cmd.scan.ip_start".into(), "正在扫描 {}（每主机 {} 个端口）...".into());

    m.insert("cmd.scan.ip_title".into(), "{} 中的存活主机：".into());

    m.insert("cmd.scan.missing_ports".into(), "'--ports' 后缺少值。".into());

    m.insert("cmd.scan.missing_timeout".into(), "'--timeout' 后缺少值。".into());

    m.insert("cmd.scan.missing_top".into(), "'--top' 后缺少值。".into());
    m.insert("cmd.scan.invalid_depth".into(), "无效的深度：{}".into());

    m.insert("cmd.scan.invalid_timeout".into(), "无效的 --timeout 值：'{}'".into());

    m.insert("cmd.scan.invalid_top".into(), "无效的 --top 值：'{}'".into());
    m.insert("cmd.scan.list_title".into(), "扫描：{}".into());
    m.insert("cmd.scan.no_files".into(), "未找到 {} 文件".into());

    m.insert("cmd.scan.port_done".into(), "开放：{} / 已扫 {}".into());

    m.insert("cmd.scan.port_none".into(), "{} 上没有开放端口".into());

    m.insert("cmd.scan.port_start".into(), "正在扫描 {}（{} 个端口）...".into());

    m.insert("cmd.scan.port_title".into(), "{} 上开放的端口：".into());

    m.insert("cmd.scan.progress".into(), "正在扫描 {} … {}/{} ({}%) · {} 个开放".into());
    m.insert("cmd.scan.total_files".into(), "共计：{} 个文件".into());
    m.insert("cmd.scan.total_files_size".into(), "共计：{} 个文件（{}）".into());
    m.insert("cmd.scan.unknown_option".into(), "未知的 scan 选项：{}".into());
    m.insert("cmd.scan.unknown_sub".into(), "未知的 scan 子命令：'{}'".into());

    m.insert("cmd.scan.unexpected_arg".into(), "多余的参数：'{}'".into());
    m.insert("cmd.scan.usage_title".into(), "用法：".into());

    m.insert("cmd.target.not_dir".into(), "“{}”不是目录".into());

    m.insert("cmd.target.not_found".into(), "路径“{}”不存在".into());

    m.insert("cmd.target.tp_not_found".into(), "传送点“{}”不存在".into());

    m.insert("cmd.serve.db_title".into(), "数据库：".into());

    m.insert("cmd.serve.db_title".into(), "数据库：".into());

    m.insert("cmd.serve.example_default".into(), "  serve                 HTTP 运行于 8080 端口".into());

    m.insert("cmd.serve.example_default".into(), "  serve                 HTTP 运行于 8080 端口".into());

    m.insert("cmd.serve.example_port".into(), "  serve 3000            HTTP 运行于 3000 端口".into());

    m.insert("cmd.serve.example_port".into(), "  serve 3000            HTTP 运行于 3000 端口".into());

    m.insert("cmd.serve.example_spa".into(), "  serve --spa           SPA 模式（React Router 回退）".into());

    m.insert("cmd.serve.example_spa".into(), "  serve --spa           SPA 模式（React Router 回退）".into());

    m.insert("cmd.serve.example_tunnel".into(), "  serve --tunnel        HTTP + cloudflared 隧道".into());

    m.insert("cmd.serve.example_tunnel".into(), "  serve --tunnel        HTTP + cloudflared 隧道".into());

    m.insert("cmd.serve.examples_title".into(), "示例：".into());

    m.insert("cmd.serve.examples_title".into(), "示例：".into());

    m.insert("cmd.serve.files_stored_title".into(), "文件存储位置：".into());

    m.insert("cmd.serve.files_stored_title".into(), "文件存储位置：".into());

    m.insert("cmd.serve.help_bind".into(), "  --bind, -b <地址>    绑定地址（默认：0.0.0.0）".into());

    m.insert("cmd.serve.help_bind".into(), "  --bind, -b <地址>    绑定地址（默认：0.0.0.0）".into());

    m.insert("cmd.serve.help_help".into(), "  --help, -h           显示此帮助".into());

    m.insert("cmd.serve.help_help".into(), "  --help, -h           显示此帮助".into());

    m.insert("cmd.serve.help_port".into(), "  --port, -p <端口>    监听端口（默认：8080）".into());

    m.insert("cmd.serve.help_port".into(), "  --port, -p <端口>    监听端口（默认：8080）".into());

    m.insert("cmd.serve.help_spa".into(), "  --spa                SPA 模式（回退到 index.html）".into());

    m.insert("cmd.serve.help_spa".into(), "  --spa                SPA 模式（回退到 index.html）".into());

    m.insert("cmd.serve.help_title".into(), "HTTP 文件服务器".into());

    m.insert("cmd.serve.help_title".into(), "HTTP 文件服务器".into());

    m.insert("cmd.serve.help_tunnel".into(), "  --tunnel, -t         通过 cloudflared 隧道暴露".into());

    m.insert("cmd.serve.help_tunnel".into(), "  --tunnel, -t         通过 cloudflared 隧道暴露".into());

    m.insert("cmd.serve.help_share".into(), "  --share [路径]        检查网络并显示局域网 URL + 二维码".into());

    m.insert("cmd.serve.help_qr".into(), "  --qr                  强制显示二维码（含 --share 效果）".into());

    m.insert("cmd.serve.help_no_qr".into(), "  --no-qr               不显示二维码".into());

    m.insert("cmd.serve.example_share".into(), "  serve 8080 --share      检查网络并打印局域网 URL + 二维码".into());

    m.insert("cmd.serve.share_checking".into(), "正在检查网络连接……".into());

    m.insert("cmd.serve.share_online".into(), "网络正常 —— 分享模式".into());

    m.insert("cmd.serve.share_offline".into(), "无网络连接 —— 仅显示本地地址。".into());

    m.insert("cmd.serve.share_lan_title".into(), "局域网：".into());

    m.insert("cmd.serve.share_path_title".into(), "分享：".into());

    m.insert("cmd.serve.share_no_lan".into(), "未找到局域网地址 —— 仅显示本地地址。".into());

    m.insert("cmd.serve.share_bound_loopback".into(), "已绑定到回环地址：局域网设备无法访问（请使用默认 0.0.0.0）。".into());

    m.insert("cmd.serve.share_not_private".into(), "检测到的地址 {} 不是私有局域网段。".into());

    m.insert("cmd.serve.share_not_private_hint".into(), "手机分享需要 10.x.x.x、172.16-31.x.x 或 192.168.x.x。".into());

    m.insert("cmd.serve.share_qr_fail".into(), "二维码渲染失败：{}".into());

    m.insert("cmd.serve.help_usage".into(), "serve [端口] [选项]".into());

    m.insert("cmd.serve.help_usage".into(), "serve [端口] [选项]".into());

    m.insert("cmd.serve.help_web_dir".into(), "  --web-dir, -w <目录> 使用自定义前端目录".into());
    m.insert("cmd.serve.help_auth".into(), "  --auth <用户名:密码>  访问 /api/* 需要登录（例如 --auth admin:secret）".into());
    m.insert("cmd.serve.example_auth".into(), "  serve 8080 --auth admin:secret   8080 端口 HTTP，API 需要登录".into());
    m.insert("cmd.serve.invalid_auth".into(), "无效的 --auth（用法：--auth <用户名:密码>）".into());
    m.insert("cmd.serve.missing_auth".into(), "缺少 --auth 的值".into());
    m.insert("cmd.serve.auth_on".into(), "已为用户 '{}' 启用 API 登录验证".into());

    m.insert("cmd.nssh.usage_title".into(), "用法：".into());
    m.insert("cmd.nssh.help_add".into(), "nssh add <名称> --host <主机> [--user 用户] [--port 端口] [--key 密钥] [--pass-env 变量]".into());
    m.insert("cmd.nssh.help_ssh".into(), "nssh ssh [名称] [--cmd \"命令\"] [--trust] [--ask-pass] [--timeout 秒]".into());
    m.insert("cmd.nssh.help_cp".into(), "nssh cp <本地> <名称>:<远端> | <名称>:<远端> <本地>".into());
    m.insert("cmd.nssh.help_scan".into(), "nssh scan <主机> [--ports 端口| --top 数量] [--timeout 毫秒]".into());
    m.insert("cmd.nssh.help_manage".into(), "nssh ls | nssh info <名称> | nssh rm <名称> | nssh edit <名称> [选项]".into());
    m.insert("cmd.nssh.help_keygen".into(), "nssh keygen [参数...]   通过 ssh-keygen 生成密钥".into());
    m.insert("cmd.nssh.help_help".into(), "nssh help   显示此帮助（无参数时选择配置）".into());
    m.insert("cmd.nssh.options_title".into(), "选项：".into());
    m.insert("cmd.nssh.help_host".into(), "  --host <主机>           服务器地址或 IP（add 必填）".into());
    m.insert("cmd.nssh.help_user".into(), "  --user, -u <用户>       登录用户（默认当前用户）".into());
    m.insert("cmd.nssh.help_port".into(), "  --port, -p <端口>       SSH 端口（默认 22）".into());
    m.insert("cmd.nssh.help_key".into(), "  --key <路径>            私钥文件（默认 ~/.ssh/id_ed25519、id_rsa）".into());
    m.insert("cmd.nssh.help_pass_env".into(), "  --pass-env <变量>      从环境变量读取密码（永不存储）".into());
    m.insert("cmd.nssh.help_cmd".into(), "  --cmd \"命令\"            运行单条命令而非交互式 shell".into());
    m.insert("cmd.nssh.help_trust".into(), "  --trust               接受并记录未知主机密钥（默认拒绝）".into());
    m.insert("cmd.nssh.help_ask_pass".into(), "  --ask-pass            提示输入密码（隐藏输入）".into());
    m.insert("cmd.nssh.help_timeout".into(), "  --timeout <数字>      连接超时秒数（ssh/cp）或毫秒（scan）".into());
    m.insert("cmd.nssh.help_top".into(), "  --top <数量>            扫描常用端口前 N 个（默认 20）".into());
    m.insert("cmd.nssh.help_scan_ports".into(), "  --ports <规格>        如 22,80,8000-8010（每段最多 2000 个）".into());
    m.insert("cmd.nssh.help_yes".into(), "  --yes, -y             跳过删除确认".into());
    m.insert("cmd.nssh.examples_title".into(), "示例：".into());
    m.insert("cmd.nssh.example_add".into(), "nssh add prod --host 203.0.113.10 --user root --key ~/.ssh/id_rsa".into());
    m.insert("cmd.nssh.example_ssh".into(), "nssh ssh prod --cmd \"uptime\"".into());
    m.insert("cmd.nssh.example_cp".into(), "nssh cp ./app.tar.gz prod:/tmp/".into());
    m.insert("cmd.nssh.example_scan".into(), "nssh scan example.com --top 20".into());
    m.insert("cmd.nssh.missing_host".into(), "--host 后缺少值".into());
    m.insert("cmd.nssh.missing_user".into(), "--user 后缺少值".into());
    m.insert("cmd.nssh.missing_port".into(), "--port 后缺少值".into());
    m.insert("cmd.nssh.missing_key".into(), "--key 后缺少值".into());
    m.insert("cmd.nssh.missing_pass_env".into(), "--pass-env 后缺少值".into());
    m.insert("cmd.nssh.missing_cmd".into(), "--cmd 后缺少值".into());
    m.insert("cmd.nssh.missing_timeout".into(), "--timeout 后缺少值".into());
    m.insert("cmd.nssh.missing_ports".into(), "--ports 后缺少值".into());
    m.insert("cmd.nssh.missing_top".into(), "--top 后缺少值".into());
    m.insert("cmd.nssh.invalid_port".into(), "无效的端口：'{}'".into());
    m.insert("cmd.nssh.invalid_timeout".into(), "无效的超时：'{}'".into());
    m.insert("cmd.nssh.invalid_top".into(), "无效的 --top 值：'{}'".into());
    m.insert("cmd.nssh.invalid_name".into(), "无效的配置名 '{}'（仅限字母数字 _ - .）".into());
    m.insert("cmd.nssh.unknown_option".into(), "未知：{}".into());
    m.insert("cmd.nssh.add_usage".into(), "用法：nssh add <名称> --host <主机> [--user 用户] [--port 端口] [--key 密钥] [--pass-env 变量]".into());
    m.insert("cmd.nssh.profile_exists".into(), "配置 '{}' 已存在（使用 nssh edit 修改）".into());
    m.insert("cmd.nssh.profile_saved".into(), "已保存配置：{}".into());
    m.insert("cmd.nssh.profile_not_found".into(), "未找到配置 '{}'".into());
    m.insert("cmd.nssh.key_not_found".into(), "密钥文件 '{}' 不存在（仍已存储）".into());
    m.insert("cmd.nssh.pass_env_missing".into(), "环境变量 '{}' 未设置（仍已存储）".into());
    m.insert("cmd.nssh.ls_empty".into(), "暂无配置。添加：nssh add <名称> --host <主机>".into());
    m.insert("cmd.nssh.ls_title".into(), "SSH 配置：".into());
    m.insert("cmd.nssh.info_title".into(), "配置 '{}'：".into());
    m.insert("cmd.nssh.info_usage".into(), "用法：nssh info <名称>".into());
    m.insert("cmd.nssh.info_host".into(), "主机：".into());
    m.insert("cmd.nssh.info_user".into(), "用户：".into());
    m.insert("cmd.nssh.info_port".into(), "端口：".into());
    m.insert("cmd.nssh.info_key".into(), "密钥：".into());
    m.insert("cmd.nssh.info_pass".into(), "密码变量：".into());
    m.insert("cmd.nssh.rm_usage".into(), "用法：nssh rm <名称> [--yes]".into());
    m.insert("cmd.nssh.rm_confirm".into(), "删除配置 '{}'？".into());
    m.insert("cmd.nssh.rm_deleted".into(), "已删除配置：{}".into());
    m.insert("cmd.nssh.rm_cancelled".into(), "已取消。".into());
    m.insert("cmd.nssh.edit_usage".into(), "用法：nssh edit <名称> [--host 主机] [--user 用户] [--port 端口] [--key 密钥|--no-key] [--pass-env 变量|--no-pass-env]".into());
    m.insert("cmd.nssh.edit_saved".into(), "已更新配置：{}".into());
    m.insert("cmd.nssh.pick_profile".into(), "选择 SSH 配置".into());
    m.insert("cmd.nssh.ask_pass".into(), "{}@{} 的密码：".into());
    m.insert("cmd.nssh.pass_env_unset".into(), "环境变量 '{}' 为空或未设置 —— 仅在使用 --ask-pass 时提示".into());
    m.insert("cmd.nssh.ssh_connecting".into(), "正在连接 {}@{}:{}……".into());
    m.insert("cmd.nssh.cmd_exit".into(), "远端命令退出码 {}".into());
    m.insert("cmd.nssh.cp_usage".into(), "用法：nssh cp <本地> <名称>:<远端> | nssh cp <名称>:<远端> <本地>".into());
    m.insert("cmd.nssh.cp_no_remote".into(), "必须有一端是 <配置>:<路径>".into());
    m.insert("cmd.nssh.cp_two_sides".into(), "两端都是远端 —— 每次只复制一端".into());
    m.insert("cmd.nssh.cp_local_missing".into(), "未找到本地文件 '{}'".into());
    m.insert("cmd.nssh.cp_read_fail".into(), "无法读取本地文件：{}".into());
    m.insert("cmd.nssh.cp_too_large".into(), "文件超过 256MB 传输限制".into());
    m.insert("cmd.nssh.cp_uploading".into(), "正在上传 {} 到 {}:{}……".into());
    m.insert("cmd.nssh.cp_downloading".into(), "正在下载 {}:{} 到 {}……".into());
    m.insert("cmd.nssh.cp_done".into(), "完成（{} 字节）".into());
    m.insert("cmd.nssh.scan_usage".into(), "用法：nssh scan <主机> [--ports 规格 | --top 数量] [--timeout 毫秒]".into());
    m.insert("cmd.nssh.scan_start".into(), "正在扫描 {}（{} 个端口）……".into());
    m.insert("cmd.nssh.scan_title".into(), "{} 开放的端口：".into());
    m.insert("cmd.nssh.scan_none".into(), "{} 没有开放端口".into());
    m.insert("cmd.nssh.scan_done".into(), "探测 {} 个，开放 {} 个".into());
    m.insert("cmd.nssh.keygen_missing".into(), "未找到 ssh-keygen：{}".into());
    m.insert("cmd.nssh.keygen_exit".into(), "ssh-keygen 退出码 {}".into());
    m.insert("cmd.nssh.shell_hint".into(), "交互式 shell —— 回车 ~ . 强制断开".into());
    m.insert("cmd.nssh.shell_back".into(), "已回到 ntc shell".into());
    m.insert("cmd.nssh.shell_forced".into(), "已强制断开（~.）".into());

    m.insert("cmd.nsheet.usage_title".into(), "用法：".into());
    m.insert("cmd.nsheet.help_open".into(), "nsheet open <文件> [选项]   全屏浏览器（CSV/TSV/JSON/JSONL）".into());
    m.insert("cmd.nsheet.help_view".into(), "nsheet view <文件> [选项]   打印表格（默认 50 行，--all 显示全部）".into());
    m.insert("cmd.nsheet.options_title".into(), "选项：".into());
    m.insert("cmd.nsheet.help_delimiter".into(), "  -d, --delimiter <字符|auto>  字段分隔符（默认自动检测 , ; tab |）".into());
    m.insert("cmd.nsheet.help_header".into(), "  --header / --no-header        强制有/无表头（默认自动检测）".into());
    m.insert("cmd.nsheet.help_limit".into(), "  -l, --limit <行数> / --all     view 模式打印行数".into());
    m.insert("cmd.nsheet.help_sort".into(), "  --sort <列> [--desc]            按列名或从 1 起的序号排序".into());
    m.insert("cmd.nsheet.help_filter".into(), "  --filter <表达式>             例如 age>30、city~york、name=bob（!=,>=,<=,&&,||）".into());
    m.insert("cmd.nsheet.help_export".into(), "  --export <json|md>             导出而不是打印".into());
    m.insert("cmd.nsheet.help_output".into(), "  -o, --output <文件>            导出目标（默认标准输出）".into());
    m.insert("cmd.nsheet.help_help".into(), "  --help, -h                     显示此帮助".into());
    m.insert("cmd.nsheet.examples_title".into(), "示例：".into());
    m.insert("cmd.nsheet.example_open".into(), "nsheet open users.csv".into());
    m.insert("cmd.nsheet.example_view".into(), "nsheet view users.csv -l 20 --sort age --desc".into());
    m.insert("cmd.nsheet.example_filter".into(), "nsheet view data.json --filter \"age>30 && city~york\"".into());
    m.insert("cmd.nsheet.example_export".into(), "nsheet view a.csv --export json -o out.json".into());
    m.insert("cmd.nsheet.missing_file".into(), "缺少文件。用法：nsheet open <文件> | nsheet view <文件>".into());
    m.insert("cmd.nsheet.file_not_found".into(), "未找到文件 '{}'".into());
    m.insert("cmd.nsheet.load_fail".into(), "无法加载表格：{}".into());
    m.insert("cmd.nsheet.empty_sheet".into(), "空表格（未找到列）".into());
    m.insert("cmd.nsheet.no_rows".into(), "没有匹配的行。".into());
    m.insert("cmd.nsheet.unknown_option".into(), "未知：{}".into());
    m.insert("cmd.nsheet.unknown_column".into(), "未知列：'{}'".into());
    m.insert("cmd.nsheet.invalid_limit".into(), "无效的行数：'{}'".into());
    m.insert("cmd.nsheet.invalid_delimiter".into(), "无效的分隔符：'{}'（使用单个字符或 auto）".into());
    m.insert("cmd.nsheet.invalid_export".into(), "无效的导出格式：'{}'（使用 json 或 md）".into());
    m.insert("cmd.nsheet.delimiter_usage".into(), "用法：--delimiter <字符|auto>".into());
    m.insert("cmd.nsheet.limit_usage".into(), "用法：--limit <行数>".into());
    m.insert("cmd.nsheet.sort_usage".into(), "用法：--sort <列>".into());
    m.insert("cmd.nsheet.filter_usage".into(), "用法：--filter <表达式>（例如 age>30）".into());
    m.insert("cmd.nsheet.export_usage".into(), "用法：--export <json|md>".into());
    m.insert("cmd.nsheet.output_usage".into(), "用法：--output <文件>".into());
    m.insert("cmd.nsheet.filter_fail".into(), "筛选表达式错误：{}".into());
    m.insert("cmd.nsheet.export_fail".into(), "导出失败：{}".into());
    m.insert("cmd.nsheet.exported".into(), "已导出：{}".into());
    m.insert("cmd.nsheet.rows_shown".into(), "显示 {} / {} 行（{} 列）".into());
    m.insert("cmd.nsheet.rows_truncated".into(), "已截断为 {} 行（使用 --all 或 -l <行数>）".into());
    m.insert("cmd.nsheet.tui_hint".into(), "方向键/hjkl 移动 · / 搜索 · n 下一个 · s 排序 · f 筛选 · c 清除 · e 导出 · Enter 查看 · q 退出".into());
    m.insert("cmd.nsheet.tui_search_prompt".into(), "搜索（留空清除）".into());
    m.insert("cmd.nsheet.tui_sort_prompt".into(), "按列排序（! 后缀 = 降序，留空清除）".into());
    m.insert("cmd.nsheet.tui_filter_prompt".into(), "筛选（留空清除）".into());
    m.insert("cmd.nsheet.tui_export_prompt".into(), "导出到（.json/.md 路径）".into());
    m.insert("cmd.nsheet.tui_cell_title".into(), "单元格（按任意键关闭）".into());
    m.insert("cmd.nsheet.tui_popup_hint".into(), "按任意键关闭".into());
    m.insert("cmd.nsheet.tui_no_matches".into(), "无匹配".into());
    m.insert("cmd.nsheet.tui_large".into(), "大表格（{} 行）—— 导航可能较慢".into());

    m.insert("cmd.serve.help_web_dir".into(), "  --web-dir, -w <目录> 使用自定义前端目录".into());

    m.insert("cmd.serve.invalid_port".into(), "无效的端口".into());

    m.insert("cmd.serve.invalid_port".into(), "无效的端口".into());

    m.insert("cmd.serve.missing_bind".into(), "缺少绑定地址".into());

    m.insert("cmd.serve.missing_bind".into(), "缺少绑定地址".into());

    m.insert("cmd.serve.missing_dir".into(), "缺少目录".into());

    m.insert("cmd.serve.missing_dir".into(), "缺少目录".into());

    m.insert("cmd.serve.missing_port".into(), "缺少端口".into());

    m.insert("cmd.serve.missing_port".into(), "缺少端口".into());

    m.insert("cmd.serve.options_title".into(), "选项：".into());

    m.insert("cmd.serve.options_title".into(), "选项：".into());

    m.insert("cmd.serve.stopped".into(), "服务器已停止。".into());

    m.insert("cmd.serve.unknown_option".into(), "未知：{}".into());

    m.insert("cmd.serve.unknown_option".into(), "未知：{}".into());

    m.insert("cmd.serve.usage_title".into(), "用法：".into());

    m.insert("cmd.serve.usage_title".into(), "用法：".into());

    m.insert("cmd.serve.cloudflared_starting".into(), "cloudflared 隧道正在启动（PID {}）".into());

    m.insert("cmd.serve.cloudflared_missing".into(), "未找到 cloudflared：{}。安装：{}".into());

    m.insert("cmd.smtp.build_fail".into(), "构建电子邮件消息失败".into());

    m.insert("cmd.smtp.create_one_hint".into(), "  创建：{} -i <服务器> -p <端口> -u <用户> -pwd <密码>".into());

    m.insert("cmd.smtp.create_syntax".into(), "smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into());

    m.insert("cmd.smtp.create_syntax".into(), "smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into());

    m.insert("cmd.smtp.create_usage".into(), "需要服务器、用户名和密码。用法：smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into());

    m.insert("cmd.smtp.created_label".into(), "已创建：".into());

    m.insert("cmd.smtp.created_label".into(), "已创建：".into());

    m.insert("cmd.smtp.decode_failed".into(), "解码存储的密码失败".into());

    m.insert("cmd.smtp.decode_failed".into(), "解码存储的密码失败".into());

    m.insert("cmd.smtp.help_attachments_desc".into(), "  -f 接受逗号分隔的文件路径和目录（目录中的文件会递归展开）".into());

    m.insert("cmd.smtp.help_attachments_example".into(), "  示例：smtp --id a1b2 --to user@test.com -s \"报告\" -b \"见附件\" -f \"./report.pdf,./images/\"".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "附件：".into());

    m.insert("cmd.smtp.help_attachments_title".into(), "附件：".into());

    m.insert("cmd.smtp.help_create_example".into(), "  示例：smtp -i smtp.gmail.com -p 587 -u me@gmail.com -pwd app123".into());

    m.insert("cmd.smtp.help_create_syntax".into(), "  smtp -i <服务器> -p <端口> -u <用户> -pwd <密码>".into());

    m.insert("cmd.smtp.help_create_title".into(), "创建配置：".into());

    m.insert("cmd.smtp.help_create_title".into(), "创建配置：".into());

    m.insert("cmd.smtp.help_info_syntax".into(), "  smtp --id-info <id>".into());

    m.insert("cmd.smtp.help_info_title".into(), "显示配置信息：".into());

    m.insert("cmd.smtp.help_info_title".into(), "显示配置信息：".into());

    m.insert("cmd.smtp.help_list_syntax".into(), "  smtp --id-ls".into());

    m.insert("cmd.smtp.help_list_title".into(), "列出配置：".into());

    m.insert("cmd.smtp.help_list_title".into(), "列出配置：".into());

    m.insert("cmd.smtp.help_port_25".into(), "   25   明文（若服务器支持）".into());

    m.insert("cmd.smtp.help_port_465".into(), "   465  TLS（SMTPS）".into());

    m.insert("cmd.smtp.help_port_587".into(), "   587  STARTTLS（默认）".into());

    m.insert("cmd.smtp.help_ports_title".into(), "端口：".into());

    m.insert("cmd.smtp.help_ports_title".into(), "端口：".into());

    m.insert("cmd.smtp.help_rm_syntax".into(), "  smtp --id-rm <id>".into());

    m.insert("cmd.smtp.help_rm_title".into(), "删除配置：".into());

    m.insert("cmd.smtp.help_rm_title".into(), "删除配置：".into());

    m.insert("cmd.smtp.help_send_syntax".into(), "  smtp --id <id> --to <邮箱> [--cc <邮箱>] [--bcc <邮箱>] -s <主题> -b <正文> [-f <文件>]".into());

    m.insert("cmd.smtp.help_send_title".into(), "发送邮件：".into());

    m.insert("cmd.smtp.help_send_title".into(), "发送邮件：".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP 邮件客户端".into());

    m.insert("cmd.smtp.help_title".into(), "SMTP 邮件客户端".into());

    m.insert("cmd.smtp.id_label".into(), "ID：".into());

    m.insert("cmd.smtp.id_label".into(), "ID：".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "无效的 BCC：{}".into());

    m.insert("cmd.smtp.invalid_bcc".into(), "无效的 BCC：{}".into());

    m.insert("cmd.smtp.invalid_cc".into(), "无效的 CC：{}".into());

    m.insert("cmd.smtp.invalid_port".into(), "无效的端口号".into());

    m.insert("cmd.smtp.invalid_pwd_encoding".into(), "无效的密码编码：{}".into());

    m.insert("cmd.smtp.invalid_pwd_encoding".into(), "无效的密码编码：{}".into());

    m.insert("cmd.smtp.invalid_recipient".into(), "无效的收件人：{}".into());

    m.insert("cmd.smtp.invalid_sender".into(), "无效的发件人邮箱地址".into());

    m.insert("cmd.smtp.invalid_sender".into(), "无效的发件人邮箱地址".into());

    m.insert("cmd.smtp.missing_body".into(), "-b 后缺少正文".into());

    m.insert("cmd.smtp.missing_body".into(), "-b 后缺少正文".into());

    m.insert("cmd.smtp.missing_id".into(), "--id 后缺少 ID".into());

    m.insert("cmd.smtp.missing_id".into(), "--id 后缺少 ID".into());

    m.insert("cmd.smtp.missing_id_info".into(), "--id-info 后缺少 ID".into());

    m.insert("cmd.smtp.missing_id_info".into(), "--id-info 后缺少 ID".into());

    m.insert("cmd.smtp.missing_id_rm".into(), "--id-rm 后缺少 ID".into());

    m.insert("cmd.smtp.missing_id_rm".into(), "--id-rm 后缺少 ID".into());

    m.insert("cmd.smtp.missing_password".into(), "-pwd 后缺少密码".into());

    m.insert("cmd.smtp.missing_password".into(), "-pwd 后缺少密码".into());

    m.insert("cmd.smtp.missing_path".into(), "-f 后缺少路径".into());

    m.insert("cmd.smtp.missing_port".into(), "-p 后缺少端口".into());

    m.insert("cmd.smtp.missing_port".into(), "-p 后缺少端口".into());

    m.insert("cmd.smtp.missing_recipient_bcc".into(), "--bcc 后缺少收件人".into());

    m.insert("cmd.smtp.missing_recipient_bcc".into(), "--bcc 后缺少收件人".into());

    m.insert("cmd.smtp.missing_recipient_cc".into(), "--cc 后缺少收件人".into());

    m.insert("cmd.smtp.missing_recipient_cc".into(), "--cc 后缺少收件人".into());

    m.insert("cmd.smtp.missing_recipient_to".into(), "--to 后缺少收件人".into());

    m.insert("cmd.smtp.missing_recipient_to".into(), "--to 后缺少收件人".into());

    m.insert("cmd.smtp.missing_server".into(), "-i 后缺少服务器".into());

    m.insert("cmd.smtp.missing_server".into(), "-i 后缺少服务器".into());

    m.insert("cmd.smtp.missing_subject".into(), "-s 后缺少主题".into());

    m.insert("cmd.smtp.missing_subject".into(), "-s 后缺少主题".into());

    m.insert("cmd.smtp.missing_user".into(), "-u 后缺少用户名".into());

    m.insert("cmd.smtp.missing_user".into(), "-u 后缺少用户名".into());

    m.insert("cmd.smtp.need_recipient".into(), "需要收件人。使用 --to <邮箱>".into());

    m.insert("cmd.smtp.need_recipient".into(), "需要收件人。使用 --to <邮箱>".into());

    m.insert("cmd.smtp.need_subject".into(), "需要主题。使用 -s <主题>".into());

    m.insert("cmd.smtp.no_files_in_dir".into(), "目录中未找到文件：{}".into());

    m.insert("cmd.smtp.no_profiles".into(), "  {}  未找到 SMTP 配置。".into());

    m.insert("cmd.smtp.password_label".into(), "密码：".into());

    m.insert("cmd.smtp.password_label".into(), "密码：".into());

    m.insert("cmd.smtp.path_not_found".into(), "路径未找到：{}".into());

    m.insert("cmd.smtp.port_label".into(), "端口：".into());

    m.insert("cmd.smtp.port_label".into(), "端口：".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP 配置：".into());

    m.insert("cmd.smtp.profile_label".into(), "SMTP 配置：".into());

    m.insert("cmd.smtp.profile_not_found".into(), "未找到配置：{}".into());

    m.insert("cmd.smtp.profile_saved".into(), "SMTP 配置已保存：{}".into());

    m.insert("cmd.smtp.profiles_title".into(), "SMTP 配置：".into());

    m.insert("cmd.smtp.read_attach_fail".into(), "读取附件失败：{}".into());

    m.insert("cmd.smtp.removed_profile".into(), "已移除 SMTP 配置：{}".into());

    m.insert("cmd.smtp.send_fail".into(), "发送邮件失败：{}".into());

    m.insert("cmd.smtp.send_ok".into(), "邮件发送成功{}".into());

    m.insert("cmd.smtp.sending".into(), "正在向 {} 发送邮件...".into());

    m.insert("cmd.smtp.server_label".into(), "服务器：".into());

    m.insert("cmd.smtp.server_label".into(), "服务器：".into());

    m.insert("cmd.smtp.skipping_exec".into(), "跳过可执行文件：{}".into());

    m.insert("cmd.smtp.skipping_not_file".into(), "已跳过（不是文件或目录）：{}".into());

    m.insert("cmd.smtp.transport_fail".into(), "配置 SMTP 传输失败：{}".into());

    m.insert("cmd.smtp.unknown_option".into(), "未知选项：{}".into());

    m.insert("cmd.smtp.unknown_option".into(), "未知选项：{}".into());

    m.insert("cmd.smtp.user_label".into(), "用户：".into());

    m.insert("cmd.smtp.user_label".into(), "用户：".into());

    m.insert("cmd.smtp.with_attachments".into(), " （{} 个附件{}）".into());

    m.insert("cmd.terminated".into(), "命令已终止（Ctrl+C）".into());

    m.insert("cmd.tl.dir_not_found".into(), "目录未找到：{}".into());

    m.insert("cmd.tl.dir_usage".into(), "用法：tl dir <路径>".into());

    m.insert("cmd.tl.ext_not_supported".into(), ".{} 不受支持 - 已阻止".into());

    m.insert("cmd.tl.ext_usage".into(), "用法：tl ext <扩展名>".into());

    m.insert("cmd.tl.file_not_found".into(), "文件未找到：{}".into());

    m.insert("cmd.tl.file_usage".into(), "用法：tl file <路径>".into());

    m.insert("cmd.tl.help_dir".into(), "  tl dir <路径>          显示目录中受支持扩展名和总行数".into());

    m.insert("cmd.tl.help_ext".into(), "  tl ext <扩展名>        显示所有带该扩展名的文件及总行数".into());

    m.insert("cmd.tl.help_file".into(), "  tl file <路径>         统计单个文件的行数".into());

    m.insert("cmd.tl.help_sort".into(), "  tl ext|dir <目标> -s <-n|-l|-r>   排序输出（-n 普通、-l 按行数升序、-r 降序）".into());

    m.insert("cmd.tl.highest".into(), "最高：{}  {} 行".into());

    m.insert("cmd.tl.no_ext_files".into(), "未找到 .{} 文件".into());

    m.insert("cmd.tl.no_supported_files".into(), "在 {} 中未找到受支持的文件".into());

    m.insert("cmd.tl.not_a_file".into(), "不是文件：{}".into());

    m.insert("cmd.tl.sort_usage".into(), "排序参数无效。请使用：-s -n（普通）| -s -l（按行数升序，从小到大）| -s -r（降序，从大到小）。".into());

    m.insert("cmd.tl.total_files_lines".into(), "总计：{} 个文件，{} 行".into());

    m.insert("cmd.tl.unknown_sub".into(), "未知子命令 '{}'。".into());

    m.insert("cmd.tl.usage_title".into(), "用法：".into());

    m.insert("cmd.todo.add_usage".into(), "用法：todo add \"任务名称\" [-> N]".into());

    m.insert("cmd.todo.all_cleared".into(), "所有任务已清除。".into());

    m.insert("cmd.todo.cancelled".into(), "已取消。".into());

    m.insert("cmd.todo.clear_prompt".into(), "清除所有 {} 个任务？[y/N] ".into());

    m.insert("cmd.todo.empty_name".into(), "任务名称不能为空".into());

    m.insert("cmd.todo.help_add".into(), "  todo add \"名称\" [-> N]  添加任务（可选在第 N 个位置）".into());

    m.insert("cmd.todo.help_clear".into(), "  todo clear              清除所有任务（需确认）".into());

    m.insert("cmd.todo.help_info".into(), "  todo info [N]           显示所有任务或第 N 个任务".into());

    m.insert("cmd.todo.help_rm".into(), "  todo rm <N>             移除第 N 个任务".into());

    m.insert("cmd.todo.help_show".into(), "  todo                    显示所有任务".into());

    m.insert("cmd.todo.info_usage".into(), "用法：todo info <N> - N 必须是数字".into());

    m.insert("cmd.todo.info_usage".into(), "用法：todo info <N> - N 必须是数字".into());

    m.insert("cmd.todo.invalid_position".into(), "无效的位置：'{}' 必须是数字".into());

    m.insert("cmd.todo.list_title".into(), "待办列表：".into());

    m.insert("cmd.todo.list_title".into(), "待办列表：".into());

    m.insert("cmd.todo.missing_position".into(), "-> 后缺少位置数字".into());

    m.insert("cmd.todo.missing_quote".into(), "缺少结束引号".into());

    m.insert("cmd.todo.must_be_quoted".into(), "任务名称必须用引号括起来".into());

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "没有要清除的任务。".into());

    m.insert("cmd.todo.one_based".into(), "任务编号从 1 开始（最小 1）".into());

    m.insert("cmd.todo.position_one_based".into(), "位置必须是 1 起始（最小为 1）".into());

    m.insert("cmd.todo.rm_usage".into(), "用法：todo rm <N>".into());

    m.insert("cmd.todo.rm_usage_num".into(), "用法：todo rm <N> - N 必须是数字".into());

    m.insert("cmd.todo.task_added".into(), "已添加任务：{}".into());

    m.insert("cmd.todo.task_not_found".into(), "未找到第 #{} 个任务（共有 {} 个任务）".into());

    m.insert("cmd.todo.task_removed".into(), "已移除任务：{}".into());

    m.insert("cmd.todo.unexpected_text".into(), "任务名称后出现意外文本：'{}'".into());

    m.insert("cmd.todo.unknown_sub".into(), "未知的 todo 子命令：'{}'".into());

    m.insert("cmd.todo.usage_title".into(), "用法：".into());

    m.insert("cmd.todo.usage_title".into(), "用法：".into());

    m.insert("cmd.todo.help_done".into(), "  todo done <N>           标记任务 N 为已完成".into());
    m.insert("cmd.todo.help_undo".into(), "  todo undo <N>           重新打开任务 N".into());
    m.insert("cmd.todo.help_edit".into(), "  todo edit <N> \"名称\"    重命名任务 N".into());
    m.insert("cmd.todo.done_usage".into(), "用法：todo done <N>".into());
    m.insert("cmd.todo.undo_usage".into(), "用法：todo undo <N>".into());
    m.insert("cmd.todo.edit_usage".into(), "用法：todo edit <N> \"新名称\"".into());
    m.insert("cmd.todo.task_done".into(), "任务已完成：{}".into());
    m.insert("cmd.todo.task_undone".into(), "任务已重新打开：{}".into());
    m.insert("cmd.todo.task_edited".into(), "任务已更新：{}".into());
    m.insert("cmd.todo.already_done".into(), "任务 #{} 已经完成".into());
    m.insert("cmd.todo.already_open".into(), "任务 #{} 已经是未完成状态".into());

    m.insert("cmd.arc.create_usage".into(), "用法：arc create <out.zip|tar.gz|tgz|tar> <文件...>".into());
    m.insert("cmd.arc.extract_usage".into(), "用法：arc extract <压缩包> [目标目录]".into());
    m.insert("cmd.arc.list_usage".into(), "用法：arc list <压缩包>".into());
    m.insert("cmd.arc.bad_ext".into(), "不支持的压缩包类型（请使用 .zip、.tar.gz/.tgz、.tar）".into());
    m.insert("cmd.arc.not_found".into(), "未找到：{}".into());
    m.insert("cmd.arc.no_inputs".into(), "没有要打包的输入文件。".into());
    m.insert("cmd.arc.created".into(), "已创建压缩包：{}（{} 个条目）".into());
    m.insert("cmd.arc.extracted".into(), "已解压 {} 个文件到 {}".into());
    m.insert("cmd.arc.list_title".into(), "压缩包：{}".into());
    m.insert("cmd.arc.total".into(), "共计：{} 个条目".into());
    m.insert("cmd.arc.unknown_sub".into(), "未知的 arc 子命令：'{}'".into());
    m.insert("cmd.arc.usage_title".into(), "用法：".into());
    m.insert("cmd.arc.help_create".into(), "  arc create <输出> <文件...>  创建压缩包（.zip、.tar.gz、.tar）".into());
    m.insert("cmd.arc.help_extract".into(), "  arc extract <文件> [目录]   解压压缩包".into());
    m.insert("cmd.arc.help_list".into(), "  arc list <文件>              列出压缩包内容".into());
    m.insert("cmd.arc.examples_title".into(), "示例：".into());

    m.insert("cmd.upgrade.after_restart".into(), "新版本将在重新启动后准备就绪。".into());

    m.insert("cmd.upgrade.ahead".into(), "本地版本（{}）高于最新发布（{}）。".into());
    m.insert("cmd.upgrade.already_latest".into(), "ntc {} 已是最新版本。".into());
    m.insert("cmd.upgrade.api_parse_fail".into(), "解析 GitHub API 响应失败".into());

    m.insert("cmd.upgrade.api_read_fail".into(), "读取 GitHub API 响应失败".into());

    m.insert("cmd.upgrade.api_request_fail".into(), "GitHub API 请求失败：{}".into());

    m.insert("cmd.upgrade.check_fail".into(), "检查更新失败：{}".into());

    m.insert("cmd.upgrade.checking".into(), "正在检查更新（当前版本：{}）...".into());

    m.insert("cmd.upgrade.copy_fail".into(), "复制二进制文件失败：{}".into());

    m.insert("cmd.upgrade.copy_fail_android".into(), "复制二进制文件失败。请检查磁盘空间或权限。".into());

    m.insert("cmd.upgrade.dev_build".into(), "您可能正在运行开发版本。".into());

    m.insert("cmd.upgrade.download_complete".into(), "下载完成。正在安装更新...".into());

    m.insert("cmd.upgrade.download_fail".into(), "下载失败：{}".into());

    m.insert("cmd.upgrade.downloading".into(), "正在下载更新...".into());

    m.insert("cmd.upgrade.downloading_url".into(), "正在下载：{}".into());

    m.insert("cmd.upgrade.exe_path_fail".into(), "获取可执行文件路径失败：{}".into());

    m.insert("cmd.upgrade.install_fail".into(), "安装失败：{}".into());

    m.insert("cmd.upgrade.missing_tag".into(), "GitHub 发布响应中缺少 'tag_name'".into());

    m.insert("cmd.upgrade.new_version".into(), "发现新版本：ntc {}".into());

    m.insert("cmd.upgrade.no_assets_release".into(), "在发布 v{} 中未找到资源".into());

    m.insert("cmd.upgrade.perm_fail".into(), "设置可执行权限失败：{}".into());

    m.insert("cmd.upgrade.progress_bytes".into(), "已下载：{} 字节".into());

    m.insert("cmd.upgrade.progress_pct".into(), "进度：{}%".into());

    m.insert("cmd.upgrade.restart".into(), "重新启动 ntc 以使用新版本。".into());

    m.insert("cmd.upgrade.stream_fail".into(), "读取下载流失败".into());

    m.insert("cmd.upgrade.sudo_hint".into(), "请使用 sudo/管理员权限运行".into());

    m.insert("cmd.upgrade.time_remaining".into(), "剩余时间：{:>3} 秒  ".into());

    m.insert("cmd.upgrade.tmpdir_fail".into(), "创建临时目录失败：{}".into());

    m.insert("cmd.upgrade.unsupported_platform".into(), "不支持的平台：{}".into());

    m.insert("cmd.upgrade.updated".into(), "ntc 已更新到 v{}！".into());

    m.insert("cmd.upgrade.write_fail".into(), "写入下载文件失败".into());

    m.insert("config.general.autosuggest".into(), " config autosuggest on|off 显示/设置自动建议".into());

    m.insert("config.general.color".into(), " config color on|off 显示/设置颜色".into());

    m.insert("config.general.depth".into(), " config depth [n] 显示/设置最大深度（1-20）".into());

    m.insert("config.general.examples".into(), "示例：".into());

    m.insert("config.general.history".into(), " config history [on|off|路径] 显示/设置历史记录".into());

    m.insert("config.general.lang".into(), " config lang [语言] 显示/设置界面语言".into());

    m.insert("config.general.linenumbers".into(), " config linenumbers on|off 显示/设置行号".into());

    m.insert("config.general.output".into(), " config output [路径] 显示/设置输出路径".into());

    m.insert("config.general.qsc".into(), " config qsc \"#c 0\" 显示/设置QSC全局热键".into());

    m.insert("config.general.settings".into(), "设置：".into());

    m.insert("config.general.threads".into(), " config threads [n] 显示/设置线程数".into());

    m.insert("config.general.title".into(), "config \u{2014} 查看或更改配置设置".into());

    m.insert("config.general.watch".into(), " config watch on|off 显示/设置文件监视器".into());

    m.insert("config.lang.set".into(), "语言已设置为：{}".into());

    m.insert("config.lang.set".into(), "语言已设置为：{}".into());

    m.insert("cvcode.formats".into(), "格式：b2, b10, b64（base64）, hex, url".into());

    m.insert("cvcode.missing_data".into(), "缺少要转换的数据".into());

    m.insert("cvcode.missing_to".into(), "'to' 后缺少目标格式".into());

    m.insert("cvcode.usage".into(), " cvcode <来源> <数据> to <目标> [-o <文件>]".into());

    m.insert("cvcode.usage_detailed".into(), "用法：cvcode <来源> <数据> to <目标> [-o <文件>]".into());

    m.insert("decode.unknown_format".into(), "未知的解码格式：'{}'。使用 b2, b10, b64, hex 或 url。".into());

    m.insert("decode.usage_cmd".into(), "用法：decode {} <目标> [-o <文件>]".into());

    m.insert("decode.usage_decode_b2".into(), " decode b2 <目标> [-o <文件>] 二进制解码".into());

    m.insert("decode.usage_decode_b10".into(), " decode b10 <目标> [-o <文件>] 十进制解码".into());

    m.insert("decode.usage_decode_b64".into(), " decode b64 <目标> [-o <文件>] Base64 解码".into());

    m.insert("decode.usage_decode_hex".into(), " decode hex <目标> [-o <文件>] 十六进制解码".into());

    m.insert("decode.usage_decode_url".into(), " decode url <目标> [-o <文件>] URL 百分比解码".into());

    m.insert("dl.cancel_hint".into(), "使用 'dl --cancel <编号|url>' 删除记录。".into());

    m.insert("dl.completed".into(), "已下载 {}（{}，耗时 {}，速度 {}）".into());

    m.insert("dl.downloading".into(), "正在下载 {}".into());

    m.insert("dl.failed_append".into(), "打开 {} 以追加失败".into());

    m.insert("dl.failed_create".into(), "创建 {} 失败".into());

    m.insert("dl.failed_download".into(), "下载 {} 失败".into());

    m.insert("dl.failed_read".into(), "读取 {} 失败".into());

    m.insert("dl.file_still_exists".into(), "文件 '{}' 仍然存在。如需，请手动删除。".into());

    m.insert("dl.help.examples".into(), "示例：".into());

    m.insert("dl.help.opt_cancel".into(), " --cancel <编号|url> 删除下载记录".into());

    m.insert("dl.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert("dl.help.opt_o".into(), " -o, --output <文件> 保存为文件名".into());

    m.insert("dl.help.opt_progress".into(), " --progress 显示下载进度条".into());

    m.insert("dl.help.opt_resume".into(), " --resume 恢复部分下载".into());

    m.insert("dl.help.opt_stop".into(), " --stop 显示下载历史".into());

    m.insert("dl.help.options".into(), "选项：".into());

    m.insert("dl.help.title".into(), "下载（dl）".into());

    m.insert("dl.help.usage".into(), "用法：".into());

    m.insert("dl.help.usage_line".into(), " dl <url> [选项]".into());

    m.insert("dl.history_title".into(), "下载历史：".into());

    m.insert("dl.http_error".into(), "HTTP {}：{}".into());

    m.insert("dl.invalid_header".into(), "无效的标头格式，应为 \"Key: Value\"，实际为：{}".into());

    m.insert("dl.invalid_index".into(), "无效的索引".into());

    m.insert("dl.missing_cancel".into(), "--cancel 后缺少URL/名称".into());

    m.insert("dl.missing_data".into(), "-d 后缺少数据".into());

    m.insert("dl.missing_header".into(), "-H 后缺少标头值".into());

    m.insert("dl.missing_o".into(), "-o 后缺少文件名".into());

    m.insert("dl.no_records".into(), "尚无下载记录。".into());

    m.insert("dl.not_found".into(), "未找到下载记录：{}".into());

    m.insert("dl.not_found_404".into(), "未找到URL（404）：{}".into());

    m.insert("dl.removed_record".into(), "已删除下载记录：{}".into());

    m.insert("dl.up_url_required".into(), "需要URL。用法：up <文件> <url>".into());

    m.insert("dl.uploading".into(), "正在上传 {}（{}）".into());

    m.insert("dl.url_required".into(), "需要URL。用法：dl <url>".into());

    m.insert("dns.help.examples".into(), "示例：".into());

    m.insert("dns.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert("dns.help.opt_type".into(), " -t, --type <类型> 记录类型：A, AAAA, ALL, MX, NS, CNAME 等（默认 A）".into());

    m.insert("dns.help.opt_type_sub".into(), " MX/NS/CNAME 等使用系统 dig/nslookup".into());

    m.insert("dns.help.options".into(), "选项：".into());

    m.insert("dns.help.title".into(), "DNS 查询".into());

    m.insert("dns.help.usage".into(), "用法：".into());

    m.insert("dns.help.usage_line".into(), " dns <主机> [选项]".into());

    m.insert("editor.already_root".into(), "已在根目录".into());

    m.insert("editor.auto_save_off".into(), "退出时自动保存：关".into());

    m.insert("editor.auto_save_on".into(), "退出时自动保存：开".into());

    m.insert("editor.cleared_cursors".into(), "已清除额外光标".into());

    m.insert("editor.clip_copied_chars".into(), "已复制 {} 个字符".into());

    m.insert("editor.clip_copied_chars".into(), "已复制 {} 个字符".into());

    m.insert("editor.clip_copied_line".into(), "已复制行".into());

    m.insert("editor.clip_copied_line".into(), "已复制行".into());

    m.insert("editor.clip_cut_chars".into(), "已剪切 {} 个字符".into());

    m.insert("editor.clip_cut_chars".into(), "已剪切 {} 个字符".into());

    m.insert("editor.clip_cut_line".into(), "已剪切行".into());

    m.insert("editor.clip_cut_line".into(), "已剪切行".into());

    m.insert("editor.clip_empty".into(), "剪贴板为空".into());

    m.insert("editor.clip_empty".into(), "剪贴板为空".into());

    m.insert("editor.clip_yanked".into(), "已拉出行".into());

    m.insert("editor.clip_yanked".into(), "已拉出行".into());

    m.insert("editor.clipboard_empty".into(), "剪贴板为空".into());

    m.insert("editor.close_btn".into(), "\u{2715}".into());

    m.insert("editor.cmd_already_at_root".into(), "已在根目录".into());

    m.insert("editor.cmd_auto_off".into(), "退出时自动保存：关".into());

    m.insert("editor.cmd_auto_on".into(), "退出时自动保存：开".into());

    m.insert("editor.cmd_cd_prefix".into(), "cd：".into());

    m.insert("editor.cmd_color_off".into(), "颜色：关".into());

    m.insert("editor.cmd_color_on".into(), "颜色：开".into());

    m.insert("editor.cmd_created_dir".into(), "已创建目录：".into());

    m.insert("editor.cmd_current_theme".into(), "当前主题：".into());

    m.insert("editor.cmd_exited_hex".into(), "已退出十六进制模式（更改已丢弃）".into());

    m.insert("editor.cmd_gos_only".into(), "gos：只有 {} 个子目录".into());

    m.insert("editor.cmd_gos_usage".into(), "gos：用法 gos <N>".into());

    m.insert("editor.cmd_goto_usage".into(), "goto：用法 goto <地址> 或 goto 0x<十六进制>".into());

    m.insert("editor.cmd_hex_save_failed".into(), "十六进制保存失败：".into());

    m.insert("editor.cmd_hex_saved".into(), "十六进制已保存".into());

    m.insert("editor.cmd_jumped_to_line".into(), "已跳转到行".into());

    m.insert("editor.cmd_jumped_to_offset".into(), "已跳转到偏移".into());

    m.insert("editor.cmd_mkd_failed".into(), "mkd：".into());

    m.insert("editor.cmd_mkd_missing".into(), "mkd：缺少操作数".into());

    m.insert("editor.cmd_mkdir_failed".into(), "mkdir：".into());

    m.insert("editor.cmd_mkdir_missing".into(), "mkdir：缺少操作数".into());

    m.insert("editor.cmd_mkf_failed".into(), "mkf：".into());

    m.insert("editor.cmd_mkf_missing".into(), "mkf：缺少操作数".into());

    m.insert("editor.cmd_no_cwd".into(), "无法获取当前目录".into());

    m.insert("editor.cmd_no_subdirs".into(), "没有子目录".into());

    m.insert("editor.cmd_no_tp".into(), "没有传送点".into());

    m.insert("editor.cmd_not_dir".into(), "不是目录".into());

    m.insert("editor.cmd_not_exist".into(), "不存在".into());

    m.insert("editor.cmd_removed".into(), "已删除：".into());

    m.insert("editor.cmd_removed_dir".into(), "已删除目录：".into());

    m.insert("editor.cmd_rmd".into(), "rmd：".into());

    m.insert("editor.cmd_rmd_missing".into(), "rmd：缺少操作数".into());

    m.insert("editor.cmd_rmf".into(), "rmf：".into());

    m.insert("editor.cmd_rmf_failed".into(), "rmf：".into());

    m.insert("editor.cmd_rmf_missing".into(), "rmf：缺少操作数".into());

    m.insert("editor.cmd_run_missing".into(), "run：缺少命令".into());

    m.insert("editor.cmd_save_failed".into(), "保存失败：".into());

    m.insert("editor.cmd_saved".into(), "已保存".into());

    m.insert("editor.cmd_syntax_off".into(), "语法高亮：关".into());

    m.insert("editor.cmd_syntax_on".into(), "语法高亮：开".into());

    m.insert("editor.cmd_theme_list_hint".into(), "使用 'theme list' 查看可用主题。".into());

    m.insert("editor.cmd_theme_not_found".into(), "未找到主题".into());

    m.insert("editor.cmd_theme_switched".into(), "已切换主题".into());

    m.insert("editor.cmd_themes".into(), "主题：".into());

    m.insert("editor.cmd_touch_failed".into(), "touch：".into());

    m.insert("editor.cmd_touch_missing".into(), "touch：缺少操作数".into());

    m.insert("editor.cmd_tp_add_error".into(), "tp-add：".into());

    m.insert("editor.cmd_tp_add_missing".into(), "tp-add：缺少名称".into());

    m.insert("editor.cmd_tp_added".into(), "已创建传送点".into());

    m.insert("editor.cmd_tp_error".into(), "tp：".into());

    m.insert("editor.cmd_tp_jumped".into(), "已传送到".into());

    m.insert("editor.cmd_tp_list".into(), "传送点：".into());

    m.insert("editor.cmd_tp_not_found".into(), "未找到传送点：".into());

    m.insert("editor.cmd_unknown".into(), "未知命令：".into());

    m.insert("editor.cmd_visual_hint".into(), "可视模式 方向键选择，Esc 退出".into());

    m.insert("editor.color_off".into(), "颜色：关".into());

    m.insert("editor.color_on".into(), "颜色：开".into());

    m.insert("editor.compl_builtin".into(), "[内置]".into());

    m.insert("editor.compl_builtin".into(), "[内置]".into());

    m.insert("editor.compl_math_abs".into(), "绝对值".into());

    m.insert("editor.compl_math_abs".into(), "绝对值".into());

    m.insert("editor.compl_math_acos".into(), "反余弦（弧度）".into());

    m.insert("editor.compl_math_acos".into(), "反余弦（弧度）".into());

    m.insert("editor.compl_math_acot".into(), "反余切（弧度）".into());

    m.insert("editor.compl_math_acot".into(), "反余切（弧度）".into());

    m.insert("editor.compl_math_acsc".into(), "反余割（弧度）".into());

    m.insert("editor.compl_math_acsc".into(), "反余割（弧度）".into());

    m.insert("editor.compl_math_asec".into(), "反正割（弧度）".into());

    m.insert("editor.compl_math_asec".into(), "反正割（弧度）".into());

    m.insert("editor.compl_math_asin".into(), "反正弦（弧度）".into());

    m.insert("editor.compl_math_asin".into(), "反正弦（弧度）".into());

    m.insert("editor.compl_math_atan".into(), "反正切（弧度）".into());

    m.insert("editor.compl_math_atan".into(), "反正切（弧度）".into());

    m.insert("editor.compl_math_avg".into(), "平均值".into());

    m.insert("editor.compl_math_avg".into(), "平均值".into());

    m.insert("editor.compl_math_ceil".into(), "向上取整".into());

    m.insert("editor.compl_math_ceil".into(), "向上取整".into());

    m.insert("editor.compl_math_cos".into(), "余弦（弧度）".into());

    m.insert("editor.compl_math_cos".into(), "余弦（弧度）".into());

    m.insert("editor.compl_math_cot".into(), "余切（弧度）".into());

    m.insert("editor.compl_math_cot".into(), "余切（弧度）".into());

    m.insert("editor.compl_math_csc".into(), "余割（弧度）".into());

    m.insert("editor.compl_math_csc".into(), "余割（弧度）".into());

    m.insert("editor.compl_math_floor".into(), "向下取整".into());

    m.insert("editor.compl_math_floor".into(), "向下取整".into());

    m.insert("editor.compl_math_ln".into(), "自然对数".into());

    m.insert("editor.compl_math_ln".into(), "自然对数".into());

    m.insert("editor.compl_math_log10".into(), "以 10 为底的对数".into());

    m.insert("editor.compl_math_log10".into(), "以 10 为底的对数".into());

    m.insert("editor.compl_math_log2".into(), "以 2 为底的对数".into());

    m.insert("editor.compl_math_log2".into(), "以 2 为底的对数".into());

    m.insert("editor.compl_math_max".into(), "最大值".into());

    m.insert("editor.compl_math_max".into(), "最大值".into());

    m.insert("editor.compl_math_min".into(), "最小值".into());

    m.insert("editor.compl_math_min".into(), "最小值".into());

    m.insert("editor.compl_math_pow".into(), "幂（x^y）".into());

    m.insert("editor.compl_math_pow".into(), "幂（x^y）".into());

    m.insert("editor.compl_math_rand".into(), "随机数".into());

    m.insert("editor.compl_math_rand".into(), "随机数".into());

    m.insert("editor.compl_math_round".into(), "四舍五入".into());

    m.insert("editor.compl_math_round".into(), "四舍五入".into());

    m.insert("editor.compl_math_sec".into(), "正割（弧度）".into());

    m.insert("editor.compl_math_sec".into(), "正割（弧度）".into());

    m.insert("editor.compl_math_sin".into(), "正弦（弧度）".into());

    m.insert("editor.compl_math_sin".into(), "正弦（弧度）".into());

    m.insert("editor.compl_math_sqrt".into(), "平方根".into());

    m.insert("editor.compl_math_sqrt".into(), "平方根".into());

    m.insert("editor.compl_math_sum".into(), "总和".into());

    m.insert("editor.compl_math_sum".into(), "总和".into());

    m.insert("editor.compl_math_tan".into(), "正切（弧度）".into());

    m.insert("editor.compl_math_tan".into(), "正切（弧度）".into());

    m.insert("editor.compl_math_tobinary".into(), "转换为二进制".into());

    m.insert("editor.compl_math_tobinary".into(), "转换为二进制".into());

    m.insert("editor.compl_math_todecimal".into(), "转换为十进制".into());

    m.insert("editor.compl_math_todecimal".into(), "转换为十进制".into());

    m.insert("editor.compl_math_tohb".into(), "转换为人类可读字节数".into());

    m.insert("editor.compl_math_tohb".into(), "转换为人类可读字节数".into());

    m.insert("editor.compl_math_tohex".into(), "转换为十六进制".into());

    m.insert("editor.compl_math_tohex".into(), "转换为十六进制".into());

    m.insert("editor.compl_math_tooctal".into(), "转换为八进制".into());

    m.insert("editor.compl_math_tooctal".into(), "转换为八进制".into());

    m.insert("editor.compl_other".into(), "[其他]".into());

    m.insert("editor.compl_other".into(), "[其他]".into());

    m.insert("editor.compl_print_values".into(), "打印值".into());

    m.insert("editor.compl_print_values".into(), "打印值".into());

    m.insert("editor.compl_return_value".into(), "返回值".into());

    m.insert("editor.compl_return_value".into(), "返回值".into());

    m.insert("editor.completion_title".into(), "补全".into());

    m.insert("editor.copied_line".into(), "已复制行".into());

    m.insert("editor.cut_line".into(), "已剪切行".into());

    m.insert("editor.edit_added_cursor".into(), "已添加光标（共 {} 个）".into());

    m.insert("editor.edit_added_cursor".into(), "已添加光标（共 {} 个）".into());

    m.insert("editor.edit_already_selected".into(), "已选中".into());

    m.insert("editor.edit_already_selected".into(), "已选中".into());

    m.insert("editor.edit_no_more_occurrences".into(), "没有更多匹配项".into());

    m.insert("editor.edit_no_more_occurrences".into(), "没有更多匹配项".into());

    m.insert("editor.edit_wrap".into(), "换行".into());

    m.insert("editor.edit_wrap".into(), "换行".into());

    m.insert("editor.exited_hex".into(), "已退出十六进制模式（更改已放弃）".into());

    m.insert("editor.exited_multicursor".into(), "已退出多光标模式".into());

    m.insert("editor.ff_prompt_find_empty".into(), "查找：输入搜索...".into());

    m.insert("editor.ff_prompt_find_prefix".into(), "查找：".into());

    m.insert("editor.ff_prompt_gs_prefix".into(), "gs:".into());

    m.insert("editor.file_changed_reload".into(), "文件已外部更改 \u{2014} 已重新加载".into());

    m.insert("editor.gosc_exit_hint".into(), "0=退出 ".into());

    m.insert("editor.gosc_hint".into(), " Enter=进入 -N=后退 N Esc=关闭 ".into());

    m.insert("editor.gosc_title".into(), " gosc — 连续导航 ".into());

    m.insert("editor.gs_loading".into(), " 正在查找... {} 请稍候".into());

    m.insert("editor.help_screen".into(), " 普通模式（vim 风格）\n i=插入 v=可视 q=查询 n=保存 c=复制 e=导航 f=查找 b=文件 t=终端 u=编码 g=命令 :=命令 ?=帮助 0=退出（未保存会提示）\n hjkl/awsd/方向键移动 \u{B7} Home/End 行首尾 \u{B7} PgUp/PgDn 翻页 \u{B7} ^J=跳转\n\n GMODE（按 g，输入，Enter）\n t/T=上下标签 qt=关闭标签 aqt=关闭全部 数字=跳转 q=退出 Esc=取消\n\n NFE 侧栏（按 e）\n hjkl/方向键/wasd 移动 \u{B7} Enter=目录进入/文件打开（停留） \u{B7} Esc=返回\n\n 插入模式（编辑）\n 输入即插入 \u{B7} 方向键移动 \u{B7} Esc=普通 ^S=保存 ^Z=撤销 ^R=重做 ^D=多光标 ^F=查找 ^N=文件 ^P=粘贴 ^T=终端 ^J=跳转 ^E=命令 \u{B7} Home/End 行首尾 \u{B7} PgUp/PgDn 翻页 \u{B7} F3/Sh+F3 查找上下\n\n 可视模式（选择）\n hjkl/方向键移动（Shift 选择） \u{B7} a=全选切换 c=复制 x=剪切 y=复制行 p=粘贴 :=命令 e=导航 f=查找 g=命令 \u{B7} Esc=退出\n\n 命令模式\n :<number> 跳转到行号\n :l N 或 :line N 跳转到第 N 行\n :q 或 :quit 退出编辑器（带保存提示）\n :wq 保存并退出\n :w 或 :write 保存文件\n :auto on 启用自动保存\n :auto off 禁用自动保存\n :syntax on 启用语法高亮\n :syntax off 禁用语法高亮\n :color on|off 切换彩色输出\n :cd <路径> 更改目录（导航器）\n :back [N] 返回父目录（N 级）\n :gos <N> 导航到第 N 个子目录\n :gosc 列出编号的子目录\n :mkdir <目录> 创建目录\n :mkd <目录> 创建目录（:mkdir 别名）\n :touch <文件> 创建新文件\n :mkf <文件> 创建新文件（:touch 别名）\n :rmd <目标> 递归删除目录\n :rmf <目标> 强制删除文件或目录\n :rnmf <源> <目标> 重命名文件\n :rnmd <源> <目标> 重命名目录\n :ne <文件> 导航并编辑（不存在则创建）\n :ne . 在侧边栏显示当前目录\n :theme <名称> 按名称切换主题\n :theme list 列出可用主题\n :theme 显示当前主题\n :git diff 或 :gd 显示当前文件的 git 差异\n :git blame 或 :blame 显示当前行的 blame 信息\n :tp <名称> 跳转到传送点\n :tp-add <名称> 将当前目录保存为传送点\n :gs <模式> 在项目中搜索文件内容\n :run <命令> 在持久运行面板中执行命令\n Esc 取消\n\n 查询模式（普通模式 q，插入模式 Ctrl+Alt+Q，命令模式 :x — 开头 / 可省略）\n /s '文本' 选中光标后下一个匹配（循环）；空 /s 选中光标处单词\n /l N 跳转到第 N 行（光标到行首）\n /sa '文本' 选中全文所有匹配\n /sal 全选所有行\n /sl N 选中第 N 行；/sl A B 选中 A..B 行（顺序不限）；/sl A , B , … 选中离散行\n /rp <查找> <替换> 在选区内替换（需选择；多词加引号：/rp 'a b' 'c'）\n /add <文本> to #end|#start|#hm 在选中行追加/前插（需选择）\n /set <模式> to #end|#start|#hm 对齐首处匹配（需选择）\n /c | /cs 清除选择\n /upper 选择转大写（需选择）\n /lower 选择转小写（需选择）\n /qal add <n> <q> 添加查询别名（如 /qal sw /rp ${1} ${2}；s/l/sa/sal/sl/rp/c 等为保留名）\n /qal rm <n> 删除别名\n /qal info <n> 显示别名\n /qal edit <n> <q> 更新别名\n /qal rnm <a> to <b> 重命名别名\n /qal list 列出别名\n <名称> [参数] 直接运行别名（无需 /）\n /undo 撤销\n /redo 重做\n /exit /quit /Esc 退出查询模式\n ; 链式执行多个命令\n ${1} ${2} 别名位置参数\n ${1:-默认值} 带默认值的别名参数\n ${1[]} 别名数组参数（所有剩余参数）\n\n 终端 / 运行面板\n Ctrl+T 切换终端面板\n :run <命令> 执行命令，打开持久运行面板\n mouse 单击面板聚焦，单击编辑器返回\n Esc（面板内） 运行：关闭 | 终端：返回编辑器\n drag \u{2500}\u{2500}\u{2500} row 调整面板高度".into());

    m.insert("editor.hex_cannot_read".into(), "hex：无法读取文件：{}".into());

    m.insert("editor.hex_cannot_read".into(), "hex：无法读取文件：{}".into());

    m.insert("editor.hex_mode_hint".into(), "Hex 模式 - 0-9/a-f 编辑字节  u=撤销  Ctrl+R=重做  :=命令  Esc=退出".into());

    m.insert("editor.hex_mode_hint".into(), "Hex 模式 - 0-9/a-f 编辑字节  u=撤销  Ctrl+R=重做  :=命令  Esc=退出".into());

    m.insert("editor.hex_saved".into(), "十六进制已保存".into());

    m.insert("editor.hex_unsaved".into(), "未保存的十六进制更改。使用 :w 保存，:q! 放弃。".into());

    m.insert("editor.hex_unsaved_warning".into(), "Hex 更改未保存。使用 :w 保存，:q! 放弃。".into());

    m.insert("editor.hex_unsaved_warning".into(), "Hex 更改未保存。使用 :w 保存，:q! 放弃。".into());

    m.insert("editor.hint_command_actions".into(), "Enter=执行 Esc=取消".into());

    m.insert("editor.hint_command_nav".into(), "cd|back|gos|gs|mkdir|touch|ne|tp|tp-add".into());

    m.insert("editor.hint_file_finder_results".into(), "结果 ↑↓=导航 Enter=打开 Esc=关闭".into());

    m.insert("editor.hint_find".into(), "查找：{}".into());

    m.insert("editor.hint_find".into(), "查找：{}".into());

    m.insert("editor.hint_find_prefix".into(), "查找 —".into());

    m.insert("editor.hint_gosc".into(), "转到：{}".into());

    m.insert("editor.hint_gosc".into(), "转到：{}".into());

    m.insert("editor.hint_gosc_close".into(), "Esc=关闭".into());

    m.insert("editor.hint_gosc_nav".into(), "目录 输入编号+Enter=导航 0=退出".into());

    m.insert("editor.hint_gosc_prefix".into(), "导航 —".into());

    m.insert("editor.hint_gs_close".into(), "Esc=关闭".into());

    m.insert("editor.hint_gs_done".into(), "完成：{} 个匹配".into());

    m.insert("editor.hint_gs_done".into(), "完成：{} 个匹配".into());

    m.insert("editor.hint_gs_done_prefix".into(), "gs —".into());

    m.insert("editor.hint_gs_loading".into(), "gs — 正在搜索...".into());

    m.insert("editor.hint_help".into(), " 帮助 — Esc/q=关闭 ↑↓=滚动".into());

    m.insert("editor.hint_hex_actions".into(), "0-9/a-f=编辑 u=撤销 ^R=重做 :=cmd :w=保存 Esc=退出".into());

    m.insert("editor.hint_hex_edit".into(), "Hex 编辑 - 输入十六进制字节".into());

    m.insert("editor.hint_hex_edit".into(), "Hex 编辑 - 输入十六进制字节".into());

    m.insert("editor.hint_insert".into(), " 插入模式 — Esc=普通 ^S=保存 ^Z=撤销 ^R=重做 ^D=多光标 ^F=查找 ^N=文件 ^P=粘贴 ^T=终端 ^J=跳转".into());

    m.insert("editor.hint_insert_msg".into(), " Esc=普通".into());

    m.insert("editor.hint_insert_multi".into(), " Insert* — Esc=退出多光标 ^D=下一处".into());

    m.insert("editor.hint_insert_multi_msg".into(), " Esc=退出多光标".into());

    m.insert("editor.hint_jump".into(), "跳转到行 —".into());

    m.insert("editor.hint_jump_actions".into(), "Enter=跳转 Esc=取消".into());

    m.insert("editor.hint_gmode".into(), "g".into());

    m.insert("editor.hint_gmode_actions".into(), "t/T=标签 qt/aqt=关闭 数字=跳转 q=退出 Enter=运行 Esc=取消".into());

    m.insert("editor.gmode_unknown".into(), "未知 gmode 命令：'{}'".into());

    m.insert("editor.hint_normal".into(), " i=插入 v=可视 f=查找 b=文件 e=导航 t=终端 g=命令 n=保存 c=复制 Sh=选择 :命令 ?=帮助 0=退出".into());

    m.insert("editor.hint_normal_msg".into(), " i=插入 v=可视 :命令 ?=帮助 0=退出".into());

    m.insert("editor.hint_query".into(), "查询：{}".into());

    m.insert("editor.hint_query".into(), "查询：{}".into());

    m.insert("editor.hint_query_actions".into(), "操作：/s 下一个, /l 跳转, /sa 全部, /sal 所有行, /sl 行, /rp 替换, /add, /set, /upper, /lower, /qal, /c 清除".into());

    m.insert("editor.hint_query_actions".into(), "操作：/s 下一个, /l 跳转, /sa 全部, /sal 所有行, /sl 行, /rp 替换, /add, /set, /upper, /lower, /qal, /c 清除".into());

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_results".into(), "结果：{}".into());

    m.insert("editor.hint_results".into(), "结果：{}".into());

    m.insert("editor.hint_results_so_far".into(), "结果：目前 {} 个".into());

    m.insert("editor.hint_results_so_far".into(), "结果：目前 {} 个".into());

    m.insert("editor.hint_run_empty".into(), "运行输出 —（空）".into());

    m.insert("editor.hint_run_entry".into(), "Enter=执行 Esc=取消".into());

    m.insert("editor.hint_run_output".into(), "运行输出 — 行".into());

    m.insert("editor.hint_run_scroll".into(), ":=cmd ↑↓/j/k=滚动 Esc=关闭".into());

    m.insert("editor.hint_search".into(), "搜索：".into());

    m.insert("editor.hint_search_actions".into(), "Enter=下一个 Shift+Enter=上一个 Esc=关闭".into());

    m.insert("editor.hint_term_actions".into(), "行 PgUp/PgDn=滚动 Ctrl+Shift+C=复制 Ctrl+C=中断".into());

    m.insert("editor.hint_term_close".into(), "Esc/q=关闭".into());

    m.insert("editor.hint_term_exited".into(), "终端已退出".into());

    m.insert("editor.hint_term_lines".into(), "终端 —".into());

    m.insert("editor.hint_terminal".into(), "终端 - {} — {}".into());

    m.insert("editor.hint_terminal".into(), "终端 - {} — {}".into());

    m.insert("editor.hint_visual".into(), " hjkl=移动 Sh=选择 a=全选 c=复制 x=剪切 y=复制行 p=粘贴 Esc=退出".into());

    m.insert("editor.hint_visual_no_msg".into(), " 可视模式 — hjkl/方向键移动 · Shift 选择 · a=全选 c=复制 x=剪切 y=复制行 p=粘贴".into());

    m.insert("editor.hover_title".into(), " 信息 ".into());

    m.insert("editor.hyper_created".into(), "hyper 项目 '{}' 创建成功！".into());

    m.insert("editor.hyper_created".into(), "hyper 项目 '{}' 创建成功！".into());

    m.insert("editor.hyper_creating".into(), "正在创建 hyper 项目 '{}'（C）...".into());

    m.insert("editor.hyper_creating".into(), "正在创建 hyper 项目 '{}'（C）...".into());

    m.insert("editor.hyper_dir_exists".into(), "目录 '{}' 已存在。".into());

    m.insert("editor.hyper_dir_exists".into(), "目录 '{}' 已存在。".into());

    m.insert("editor.hyper_name_with_ext".into(), "项目名 '{}' 包含扩展名。请使用不含点号的名称。".into());

    m.insert("editor.hyper_name_with_ext".into(), "项目名 '{}' 包含扩展名。请使用不含点号的名称。".into());

    m.insert("editor.hyper_unsupported_lang".into(), "不支持的语言 '{}'。仅支持 'c'。".into());

    m.insert("editor.hyper_unsupported_lang".into(), "不支持的语言 '{}'。仅支持 'c'。".into());

    m.insert("editor.line_deleted".into(), "行已删除".into());

    m.insert("editor.line_selected".into(), "行已选中".into());

    m.insert("editor.line_yanked".into(), "行已复制".into());

    m.insert("editor.max_tabs".into(), "已达到最大标签页数（{}）".into());

    m.insert("editor.mkd_missing".into(), "mkd：缺少操作数".into());

    m.insert("editor.mkdir_missing".into(), "mkdir：缺少操作数".into());

    m.insert("editor.mkf_missing".into(), "mkf：缺少操作数".into());

    m.insert("editor.mode.command".into(), "[命令]".into());

    m.insert("editor.mode.finder".into(), "[查找器]".into());

    m.insert("editor.mode.gosc".into(), "[Gosc]".into());

    m.insert("editor.mode.help".into(), "[帮助]".into());

    m.insert("editor.mode.hex".into(), "[十六进制]".into());

    m.insert("editor.mode.insert".into(), "[插入]".into());

    m.insert("editor.mode.insert_star".into(), "[插入 *]".into());

    m.insert("editor.mode.jump".into(), "[跳转]".into());

    m.insert("editor.mode.normal".into(), "[普通]".into());

    m.insert("editor.mode.query".into(), "[查询]".into());

    m.insert("editor.mode.run".into(), "[运行]".into());

    m.insert("editor.mode.search".into(), "[搜索]".into());

    m.insert("editor.mode.term".into(), "[终端]".into());

    m.insert("editor.mode.visual".into(), "[可视]".into());

    m.insert("editor.mode_cleared_cursors".into(), "已清除额外光标".into());

    m.insert("editor.mode_cleared_cursors".into(), "已清除额外光标".into());

    m.insert("editor.mode_command".into(), "[命令]".into());

    m.insert("editor.mode_exited_multi".into(), "已退出多光标模式".into());

    m.insert("editor.mode_exited_multi".into(), "已退出多光标模式".into());

    m.insert("editor.mode_finder".into(), "[查找]".into());

    m.insert("editor.mode_encoding".into(), "[编码]".into());

    m.insert("editor.mode_git_diff".into(), "[Git差异]".into());
    m.insert("editor.git_no_diff".into(), "git：此文件无差异".into());
    m.insert("editor.git_no_blame".into(), "git：此行为无 blame 信息".into());
    m.insert("editor.git_disabled".into(), "git 集成已禁用（config git ON 开启）".into());
    m.insert("editor.git_quit_hint".into(), "Esc/q 关闭".into());

    m.insert("editor.mode_gosc".into(), "[导航]".into());

    m.insert("editor.mode_help".into(), "[帮助]".into());

    m.insert("editor.mode_hex".into(), "[十六进制]".into());

    m.insert("editor.mode_insert".into(), "[插入]".into());

    m.insert("editor.mode_insert_multi".into(), "[插入 *]".into());

    m.insert("editor.mode_jump".into(), "[跳转]".into());

    m.insert("editor.mode_jump_range".into(), "第 {} 行超出范围（1-{}）".into());

    m.insert("editor.mode_jump_range".into(), "第 {} 行超出范围（1-{}）".into());

    m.insert("editor.mode_normal".into(), "[普通]".into());

    m.insert("editor.mode_gmode".into(), "[Gmode]".into());

    m.insert("editor.mode_nfe".into(), "[Nfe]".into());

    m.insert("editor.hint_nfe".into(), " hjkl/wasd/方向键移动 · Enter 打开 · :=宽度 · Esc 返回".into());

    m.insert("editor.hint_nfe_cmd_actions".into(), "Enter=应用 Esc=取消".into());

    m.insert("editor.nfe_unknown".into(), "未知 nfe 命令：'{}'（试试 size full|def|N%）".into());

    m.insert("editor.nfe_size_usage".into(), "用法：size full|def|N%（收到 '{}'）".into());

    m.insert("editor.nfe_size_set".into(), "侧栏宽度：{}".into());

    m.insert("editor.nfe_size_default".into(), "默认（30 列）".into());

    m.insert("editor.nfe_size_full".into(), "全宽".into());

    m.insert("editor.mode_query".into(), "[查询]".into());

    m.insert("editor.mode_run".into(), "[运行]".into());

    m.insert("editor.mode_run_err".into(), "run：{}".into());

    m.insert("editor.mode_run_err".into(), "run：{}".into());

    m.insert("editor.mode_search".into(), "[搜索]".into());

    m.insert("editor.mode_term_copied".into(), "已复制 {} 个字符".into());

    m.insert("editor.mode_term_copied".into(), "已复制 {} 个字符".into());

    m.insert("editor.mode_terminal".into(), "[终端]".into());

    m.insert("editor.mode_terminal_err".into(), "终端错误：{}".into());

    m.insert("editor.mode_terminal_err".into(), "终端错误：{}".into());

    m.insert("editor.mode_visual".into(), "[可视]".into());

    m.insert("editor.mode_visual_start".into(), "可视模式 - 用方向键选择，Esc 退出".into());

    m.insert("editor.mode_visual_start".into(), "可视模式 - 用方向键选择，Esc 退出".into());

    m.insert("editor.mode_zoom_hint".into(), "缩放：在终端模拟器中按 Ctrl++ / Ctrl+-".into());

    m.insert("editor.mode_zoom_hint".into(), "缩放：在终端模拟器中按 Ctrl++ / Ctrl+-".into());

    m.insert("editor.modified_marker".into(), " \u{25CF}".into());

    m.insert("editor.mouse_closed_all".into(), "已关闭所有标签页".into());

    m.insert("editor.mouse_closed_all".into(), "已关闭所有标签页".into());

    m.insert("editor.nav_line_selected".into(), "已选择行".into());

    m.insert("editor.nav_line_selected".into(), "已选择行".into());

    m.insert("editor.nav_selected_all".into(), "已全选".into());

    m.insert("editor.nav_selected_all".into(), "已全选".into());

    m.insert("editor.nav_split_cursor".into(), "拆分光标（共 {} 个）".into());

    m.insert("editor.nav_split_cursor".into(), "拆分光标（共 {} 个）".into());

    m.insert("editor.no_subdirs".into(), "没有子目录".into());

    m.insert("editor.no_teleports".into(), "没有传送保存点".into());

    m.insert("editor.nothing_redo".into(), "没有可重做的操作".into());

    m.insert("editor.nothing_undo".into(), "没有可撤销的操作".into());

    m.insert("editor.opened_file".into(), "已打开 {}".into());

    m.insert("editor.panel_executing".into(), "正在执行：".into());

    m.insert("editor.panel_lines".into(), "行".into());

    m.insert("editor.panel_run".into(), "运行：".into());

    m.insert("editor.panel_run_hint".into(), "j/k 滚动，Esc/q 关闭".into());

    m.insert("editor.panel_run_output".into(), "运行输出".into());

    m.insert("editor.panel_terminal".into(), "终端".into());

    m.insert("editor.pattern_not_found".into(), "未找到模式：{}".into());

    m.insert("editor.query_add_format".into(), "格式：/add <文本> to #end|#start|#hm".into());

    m.insert("editor.query_add_format".into(), "格式：/add <文本> to #end|#start|#hm".into());

    m.insert("editor.query_add_position".into(), "位置必须是 #end、#start 或 #hm".into());

    m.insert("editor.query_add_position".into(), "位置必须是 #end、#start 或 #hm".into());

    m.insert("editor.query_add_usage".into(), "/add <文本> to #end|#start|#hm".into());

    m.insert("editor.query_add_usage".into(), "/add <文本> to #end|#start|#hm".into());

    m.insert("editor.query_added_text".into(), "已向 {} 行添加文本".into());

    m.insert("editor.query_added_text".into(), "已向 {} 行添加文本".into());

    m.insert("editor.query_alias_not_found".into(), "未找到别名：{}".into());

    m.insert("editor.query_alias_not_found".into(), "未找到别名：{}".into());

    m.insert("editor.query_empty_pattern".into(), "空的查找模式".into());

    m.insert("editor.query_empty_pattern".into(), "空的查找模式".into());

    m.insert("editor.query_l_usage".into(), "/l <行号> — 跳转到行".into());

    m.insert("editor.query_l_usage".into(), "/l <行号> — 跳转到行".into());

    m.insert("editor.query_lowercased".into(), "已转为小写".into());

    m.insert("editor.query_lowercased".into(), "已转为小写".into());

    m.insert("editor.query_no_matches".into(), "未找到 '{}' 的匹配".into());

    m.insert("editor.query_no_matches".into(), "未找到 '{}' 的匹配".into());

    m.insert("editor.query_no_pattern_matches".into(), "没有包含 '{}' 的行".into());

    m.insert("editor.query_no_pattern_matches".into(), "没有包含 '{}' 的行".into());

    m.insert("editor.query_qal_add_usage".into(), "/qal add <名称> <查询>".into());

    m.insert("editor.query_qal_add_usage".into(), "/qal add <名称> <查询>".into());

    m.insert("editor.query_qal_added".into(), "已添加别名 '{}'".into());

    m.insert("editor.query_qal_added".into(), "已添加别名 '{}'".into());

    m.insert("editor.query_qal_edit_usage".into(), "/qal edit <名称> <新查询>".into());

    m.insert("editor.query_qal_edit_usage".into(), "/qal edit <名称> <新查询>".into());

    m.insert("editor.query_qal_exists".into(), "别名 '{}' 已存在（请用 /qal edit）".into());

    m.insert("editor.query_qal_exists".into(), "别名 '{}' 已存在（请用 /qal edit）".into());

    m.insert("editor.query_qal_help".into(), "/qal：add|rm|info|edit|rnm|list|run".into());

    m.insert("editor.query_qal_help".into(), "/qal：add|rm|info|edit|rnm|list|run".into());

    m.insert("editor.query_qal_info_usage".into(), "/qal info <名称>".into());

    m.insert("editor.query_qal_info_usage".into(), "/qal info <名称>".into());

    m.insert("editor.query_qal_list".into(), "别名：{}".into());

    m.insert("editor.query_qal_list".into(), "别名：{}".into());

    m.insert("editor.query_qal_no_aliases".into(), "没有查询别名".into());

    m.insert("editor.query_qal_no_aliases".into(), "没有查询别名".into());

    m.insert("editor.query_qal_not_found".into(), "未找到别名 '{}'".into());

    m.insert("editor.query_qal_not_found".into(), "未找到别名 '{}'".into());

    m.insert("editor.query_qal_removed".into(), "已移除别名 '{}'".into());

    m.insert("editor.query_qal_removed".into(), "已移除别名 '{}'".into());

    m.insert("editor.query_qal_reserved".into(), "保留名称：'{}'".into());

    m.insert("editor.query_qal_reserved".into(), "保留名称：'{}'".into());

    m.insert("editor.query_qal_renamed".into(), "已将 '{}' 重命名为 '{}'".into());

    m.insert("editor.query_qal_renamed".into(), "已将 '{}' 重命名为 '{}'".into());

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <名称>".into());

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <名称>".into());

    m.insert("editor.query_qal_rnm_usage".into(), "/qal rnm <旧名> to <新名>".into());

    m.insert("editor.query_qal_rnm_usage".into(), "/qal rnm <旧名> to <新名>".into());

    m.insert("editor.query_qal_updated".into(), "已更新别名 '{}'".into());

    m.insert("editor.query_qal_updated".into(), "已更新别名 '{}'".into());

    m.insert("editor.query_replace_format".into(), "格式：/p <查找> to <替换>".into());

    m.insert("editor.query_replace_format".into(), "格式：/p <查找> to <替换>".into());

    m.insert("editor.query_replace_usage".into(), "/p <查找> to <替换>".into());

    m.insert("editor.query_replace_usage".into(), "/p <查找> to <替换>".into());

    m.insert("editor.query_replaced_count".into(), "已替换 {} 处".into());

    m.insert("editor.query_replaced_count".into(), "已替换 {} 处".into());

    m.insert("editor.query_rp_usage".into(), "/rp <查找> <替换> — 在选区内替换".into());

    m.insert("editor.query_rp_usage".into(), "/rp <查找> <替换> — 在选区内替换".into());

    m.insert("editor.query_s_usage".into(), "/s \"文本\" — 选中下一个匹配（空 /s = 选中光标处单词）".into());

    m.insert("editor.query_s_usage".into(), "/s \"文本\" — 选中下一个匹配（空 /s = 选中光标处单词）".into());

    m.insert("editor.query_sa_usage".into(), "/sa \"文本\" — 选中所有匹配".into());

    m.insert("editor.query_sa_usage".into(), "/sa \"文本\" — 选中所有匹配".into());

    m.insert("editor.query_selected_all_lines".into(), "已选择所有行".into());

    m.insert("editor.query_selected_all_lines".into(), "已选择所有行".into());

    m.insert("editor.query_selected_line".into(), "已选择第 {} 行".into());

    m.insert("editor.query_selected_line".into(), "已选择第 {} 行".into());

    m.insert("editor.query_selected_lines".into(), "已选择 {} 行".into());

    m.insert("editor.query_selected_lines".into(), "已选择 {} 行".into());

    m.insert("editor.query_selected_lines_range".into(), "已选择第 {}-{} 行".into());

    m.insert("editor.query_selected_lines_range".into(), "已选择第 {}-{} 行".into());

    m.insert("editor.query_selected_next".into(), "已选中下一个 '{}'".into());

    m.insert("editor.query_selected_next".into(), "已选中下一个 '{}'".into());

    m.insert("editor.query_selected_occurrences".into(), "已选中 {} 处 '{}' 的匹配".into());

    m.insert("editor.query_selected_occurrences".into(), "已选中 {} 处 '{}' 的匹配".into());

    m.insert("editor.query_selected_word".into(), "已选中单词".into());

    m.insert("editor.query_selected_word".into(), "已选中单词".into());

    m.insert("editor.query_selection_cleared".into(), "已清除选择".into());

    m.insert("editor.query_selection_cleared".into(), "已清除选择".into());

    m.insert("editor.query_selection_required".into(), "需要先选择（用 /s、/sa、/sl 或 /sal 选择）".into());

    m.insert("editor.query_selection_required".into(), "需要先选择（用 /s、/sa、/sl 或 /sal 选择）".into());

    m.insert("editor.query_set_alignment".into(), "已设置 {} 行对齐".into());

    m.insert("editor.query_set_alignment".into(), "已设置 {} 行对齐".into());

    m.insert("editor.query_set_format".into(), "格式：/set <模式> to #end|#start|#hm".into());

    m.insert("editor.query_set_format".into(), "格式：/set <模式> to #end|#start|#hm".into());

    m.insert("editor.query_set_usage".into(), "/set <模式> to #end|#start|#hm".into());

    m.insert("editor.query_set_usage".into(), "/set <模式> to #end|#start|#hm".into());

    m.insert("editor.query_sl_usage".into(), "/sl <行号> [行号] | <行号> , ... — 选择行".into());

    m.insert("editor.query_sl_usage".into(), "/sl <行号> [行号] | <行号> , ... — 选择行".into());

    m.insert("editor.query_unknown".into(), "未知查询：{}".into());

    m.insert("editor.query_unknown".into(), "未知查询：{}".into());

    m.insert("editor.query_uppercased".into(), "已转为大写".into());

    m.insert("editor.query_uppercased".into(), "已转为大写".into());

    m.insert("editor.redo".into(), "重做".into());

    m.insert("editor.rmd_missing".into(), "rmd：缺少操作数".into());

    m.insert("editor.rmf_missing".into(), "rmf：缺少操作数".into());

    m.insert("editor.run_exited".into(), "进程已退出，代码 {}".into());

    m.insert("editor.run_exited".into(), "进程已退出，代码 {}".into());

    m.insert("editor.run_missing".into(), "run：缺少命令".into());

    m.insert("editor.run_terminated".into(), "进程已终止".into());

    m.insert("editor.run_terminated".into(), "进程已终止".into());

    m.insert("editor.save_auto_error".into(), "自动保存错误：{}".into());

    m.insert("editor.save_auto_error".into(), "自动保存错误：{}".into());

    m.insert("editor.save_failed".into(), "保存失败：{}".into());

    m.insert("editor.encoding_changed".into(), "编码已更改".into());

    m.insert("editor.enc_title".into(), "编码".into());
    m.insert("editor.enc_hint".into(), "↑↓ 选择 · 回车确认 · Esc 取消".into());
    m.insert("editor.hint_encoding".into(), "编码：".into());
    m.insert("editor.encoding_reload_error".into(), "重新加载失败：{}".into());
    m.insert(
        "editor.encoding_applied_on_save".into(),
        "保存时应用".into(),
    );

    m.insert("editor.save_prompt".into(), "保存文件 [Y|N]？".into());

    m.insert("editor.save_prompt".into(), "保存文件 [Y|N]？".into());

    m.insert("editor.saved".into(), "已保存 \u{2713}".into());

    m.insert("editor.saved_full".into(), " 已保存 ✓".into());

    m.insert("editor.saved_full".into(), " 已保存 ✓".into());

    m.insert("editor.saved_marker".into(), " \u{2713}".into());

    m.insert("editor.scratch".into(), "[草稿]".into());

    m.insert("editor.scratch_label".into(), "[临时]".into());

    m.insert("editor.scratch_label".into(), "[临时]".into());

    m.insert("editor.search_pattern_not_found".into(), "未找到模式：{}".into());

    m.insert("editor.search_pattern_not_found".into(), "未找到模式：{}".into());

    m.insert("editor.selected_all".into(), "已全选".into());

    m.insert("editor.status_modified".into(), "已修改".into());

    m.insert("editor.status_modified_tag".into(), " \u{25cf}".into());

    m.insert("editor.status_saved".into(), "已保存".into());

    m.insert("editor.status_saved_tag".into(), " \u{2713}".into());

    m.insert("editor.syntax_off".into(), "语法高亮：关".into());

    m.insert("editor.syntax_on".into(), "语法高亮：开".into());

    m.insert("editor.tab_cannot_open".into(), "无法打开 {}".into());

    m.insert("editor.tab_cannot_open".into(), "无法打开 {}".into());

    m.insert("editor.tab_error_reading".into(), "读取 {} 出错：{}".into());

    m.insert("editor.tab_error_reading".into(), "读取 {} 出错：{}".into());

    m.insert("editor.tab_max_reached".into(), "已达到最大标签页数（{}）".into());

    m.insert("editor.tab_max_reached".into(), "已达到最大标签页数（{}）".into());

    m.insert("editor.tab_opened".into(), "已打开 {}".into());

    m.insert("editor.tab_opened".into(), "已打开 {}".into());

    m.insert("editor.term_title".into(), "终端".into());

    m.insert("editor.term_title".into(), "终端".into());

    m.insert("editor.tmpl_last_saved".into(), "上次保存：{}".into());

    m.insert("editor.tmpl_last_saved".into(), "上次保存：{}".into());

    m.insert("editor.tmpl_recover_prompt".into(), "恢复未保存的更改？[Y/N]：".into());

    m.insert("editor.tmpl_recover_prompt".into(), "恢复未保存的更改？[Y/N]：".into());

    m.insert("editor.tmpl_recovery_found".into(), "为 {} 找到崩溃恢复文件：".into());

    m.insert("editor.tmpl_recovery_found".into(), "为 {} 找到崩溃恢复文件：".into());

    m.insert("editor.touch_missing".into(), "touch：缺少操作数".into());

    m.insert("editor.tp_add_missing".into(), "tp-add：缺少名称".into());

    m.insert("editor.undo".into(), "撤销".into());

    m.insert("editor.unknown_cmd".into(), "未知命令：{}".into());

    m.insert("editor.untitled".into(), "未命名".into());

    m.insert("editor.untitled_label".into(), "未命名".into());

    m.insert("editor.untitled_label".into(), "未命名".into());

    m.insert("editor.visual_hint".into(), "可视模式 \u{2014} 方向键选择，Esc退出".into());

    m.insert("editor.welcome.title".into(), "一个用Rust构建的快速、基于终端的代码编辑器。".into());

    m.insert("editor.zoom_hint".into(), "缩放：在终端模拟器中按 Ctrl++ / Ctrl+-".into());

    m.insert("encode.binary_hint".into(), "使用 -o <文件> 保存二进制输出。".into());

    m.insert("encode.binary_warning".into(), "输出为二进制数据。显示十六进制预览：".into());

    m.insert("encode.hex_even".into(), "十六进制字符串长度必须为偶数".into());

    m.insert("encode.input_must_be_text".into(), "输入数据必须是文本".into());

    m.insert("encode.input_not_text".into(), "{} 输入必须是文本（非二进制）".into());

    m.insert("encode.invalid_b2".into(), "无效的二进制字符串：'{}'".into());

    m.insert("encode.invalid_b10".into(), "无效的十进制字节：'{}'".into());

    m.insert("encode.invalid_b64".into(), "无效的 base64 输入：{}".into());

    m.insert("encode.invalid_hex_pct".into(), "百分比编码中的无效十六进制：%{}".into());

    m.insert("encode.invalid_hex_pos".into(), "位置 {} 处无效的十六进制字符".into());

    m.insert("encode.invalid_pct".into(), "无效的百分比编码".into());

    m.insert("encode.truncated_pct".into(), "截断的百分比编码".into());

    m.insert("encode.unknown_format".into(), "未知的编码格式：'{}'。使用 b2, b10, b64, hex 或 url。".into());

    m.insert("encode.unknown_format_fmt".into(), "（未知格式：{}）".into());

    m.insert("encode.unknown_format_generic".into(), "未知格式：'{}'。使用 b2, b10, b64, hex 或 url。".into());

    m.insert("encode.usage_cmd".into(), "用法：encode {} <目标> [-o <文件>]".into());

    m.insert("encode.usage_encode_b2".into(), " encode b2 <目标> [-o <文件>] 二进制编码（0101...）".into());

    m.insert("encode.usage_encode_b10".into(), " encode b10 <目标> [-o <文件>] 十进制编码（0-255...）".into());

    m.insert("encode.usage_encode_b64".into(), " encode b64 <目标> [-o <文件>] Base64 编码".into());

    m.insert("encode.usage_encode_hex".into(), " encode hex <目标> [-o <文件>] 十六进制编码".into());

    m.insert("encode.usage_encode_url".into(), " encode url <目标> [-o <文件>] URL 百分比编码".into());

    m.insert("encode.usage_examples".into(), "示例：".into());

    m.insert("encode.usage_title".into(), "用法：".into());

    m.insert("encode.written_to".into(), "已写入：{}".into());

    m.insert("entry.goodbye".into(), "再见！".into());

    m.insert("entry.ntpkg_active".into(), "ntpkg：{} 个运行时在 PATH 中处于活动状态".into());

    m.insert("entry.startup_hint".into(), "输入 'help' 查看可用命令，输入 'exit' 退出。".into());

    m.insert("entry.watch_trigger".into(), "监视触发：{}".into());

    m.insert("entry.watcher_failed".into(), "监视器失败：{}".into());

    m.insert("entry.watcher_started".into(), "文件监视器已启动（递归）".into());

    m.insert("entry.welcome_fmt".into(), " 欢迎使用 ntc {} - 导航, 工具包, 中心 ".into());

    m.insert("error.failed_write_completions".into(), "写入补全失败：{}".into());

    m.insert("error.unsupported_shell".into(), "不支持的Shell：{}。使用：bash, zsh, fish, powershell".into());

    m.insert("fmt.help_example1".into(), "  fmt json package.json to yaml".into());

    m.insert("fmt.help_example2".into(), "  fmt json config.json to toml -o config.toml".into());

    m.insert("fmt.help_example3".into(), "  fmt yaml config.yaml to json".into());

    m.insert("fmt.help_example4".into(), "  fmt toml Cargo.toml to json".into());

    m.insert("fmt.help_example5".into(), "  fmt json data.json                      # 美化输出 JSON".into());

    m.insert("fmt.help_examples".into(), "示例：".into());

    m.insert("fmt.help_json_line".into(), "  fmt json <文件> [to yaml|toml] [-o <文件>]    将 JSON 转换为 YAML 或 TOML".into());

    m.insert("fmt.help_no_target".into(), "  fmt json <文件>                          美化输出 JSON".into());

    m.insert("fmt.help_no_target".into(), "  fmt json <文件>                          美化输出 JSON".into());

    m.insert("fmt.help_toml_line".into(), "  fmt toml <文件> [to json|yaml] [-o <文件>]    将 TOML 转换为 JSON 或 YAML".into());

    m.insert("fmt.help_usage".into(), "用法：".into());

    m.insert("fmt.help_yaml_line".into(), "  fmt yaml <文件> [to json|toml] [-o <文件>]    将 YAML 转换为 JSON 或 TOML".into());

    m.insert("gosc.calculating_size".into(), "正在计算目录大小...".into());

    m.insert("gosc.exiting".into(), "正在退出 gosc...".into());

    m.insert("gosc.output_zone".into(), "─── 输出区 ───────────────────────────────────".into());

    m.insert("gosc.output_zone_end".into(), "──────────────────────────────────────────────────".into());

    m.insert("gosc.panel.back_1".into(), "返回 1 级".into());

    m.insert("gosc.panel.back_2".into(), "返回 2 级".into());

    m.insert("gosc.panel.back_n".into(), "返回 n 级".into());

    m.insert("gosc.panel.empty".into(), "（无子目录）".into());

    m.insert("gosc.panel.exit".into(), "退出 gosc".into());

    m.insert("gosc.panel.size_care".into(), "大小 --care（总大小）".into());

    m.insert("gosc.panel.title".into(), "gosc — 持续导航".into());

    m.insert("gosc.panel.view_care".into(), "查看 --care -d 1（文件 + 关注）".into());

    m.insert("gosc.panel.view_size_care".into(), "查看 -s --care -d 1（文件 + 大小 + 关注）".into());

    m.insert("gosc.press_enter".into(), "按回车继续...".into());

    m.insert("gosc.prompt".into(), "gosc>".into());

    m.insert("gosc.size_bytes".into(), "字节：".into());

    m.insert("gosc.size_human".into(), "可读：".into());

    m.insert("gosc.size_title".into(), "当前目录大小".into());

    m.insert("help.examples_header".into(), "示例：".into());

    m.insert("help.context.cr".into(), "<cr> 按回车运行".into());

    m.insert("help.alias.add".into(), "ral add <名称> \"<命令>\" ## 创建别名".into());

    m.insert("help.alias.add_array".into(), "ral add <名称>(x[],y) \"<命令>\" ## 带数组参数的别名".into());

    m.insert("help.alias.add_defaults".into(), "ral add <名称>({x=a,e=txt}) \"<命令>\" ## 带默认值的别名".into());

    m.insert("help.alias.add_param".into(), "ral add <名称>(x) \"<命令>\" ## 创建带参数的别名（使用 $x）".into());

    m.insert("help.alias.cls".into(), "ral cls ## 清除所有别名（需确认）".into());

    m.insert("help.alias.edit".into(), "ral edit <名称> \"<命令>\" ## 更新别名".into());

    m.insert("help.alias.export_all".into(), "ral export --all <名称> ## 将所有别名导出到 <名称>.ntc.ral".into());

    m.insert("help.alias.export_select".into(), "ral export --select <名称> ## 选择要导出的别名".into());

    m.insert("help.alias.import".into(), "ral import <文件> ## 从 .ntc.ral 文件导入别名".into());

    m.insert("help.alias.info".into(), "ral info <名称> [--edit] ## 显示别名详情（--edit：将编辑命令复制到剪贴板）".into());

    m.insert("help.alias.list".into(), "ral list ## 显示所有别名".into());

    m.insert("help.alias.make".into(), "ral make [名称] ## 在当前目录创建别名模板".into());

    m.insert("help.alias.rm".into(), "ral rm <名称>[, ...] ## 删除别名".into());

    m.insert("help.alias.rnm".into(), "ral rnm <旧名称> to <新名称> ## 重命名别名".into());

    m.insert("help.alias.run".into(), "run <别名> ## 使用 'run' 执行别名".into());

    m.insert("help.alias.run_direct".into(), "<别名> ## 直接执行别名".into());

    m.insert("help.alias.run_param".into(), "<别名>(<参数>) ## 执行带参数的别名".into());

    m.insert("help.alias.title".into(), "运行别名命令：".into());

    m.insert("help.autoup.add".into(), "autoup add <名称> \"<命令>\" ## 将启动条目添加到HKCU".into());

    m.insert("help.autoup.disable".into(), "autoup disable <名称> ## 禁用条目（使用StartupApproved）".into());

    m.insert("help.autoup.enable".into(), "autoup enable <名称> ## 重新启用禁用的条目".into());

    m.insert("help.autoup.list".into(), "autoup list ## 列出所有启动条目（CU/LM/SF来源）".into());

    m.insert("help.autoup.rm".into(), "autoup rm <名称> ## 删除启动条目".into());

    m.insert("help.autoup.sources".into(), "[CU]=HKCU [LM]=HKLM [SF]=启动文件夹".into());

    m.insert("help.autoup.title".into(), "启动管理器（Windows）：".into());

    m.insert("help.backup.clear".into(), "backup clear ## 删除所有备份（需确认）".into());

    m.insert("help.backup.clear_force".into(), "backup clear --force ## 删除所有备份（无需确认）".into());

    m.insert("help.backup.create".into(), "backup create ## 创建当前项目的备份".into());

    m.insert("help.backup.diff".into(), "backup diff [num] ## 比较当前项目与备份的差异".into());

    m.insert("help.backup.list".into(), "backup list ## 列出所有备份".into());

    m.insert("help.backup.recreate".into(), "backup recreate ## 清除全部+撤销+创建全新备份".into());

    m.insert("help.backup.restore".into(), "backup restore [num] ## 恢复备份（未指定编号时交互式）".into());

    m.insert("help.backup.restore_choose".into(), "backup restore --choose ## 从交互式菜单中选择备份".into());

    m.insert("help.backup.restore_deleted".into(), "backup restore --deleted ## 仅恢复已删除的文件".into());

    m.insert("help.backup.title".into(), "备份命令：".into());

    m.insert("help.backup.undo".into(), "backup undo ## 撤销上次恢复".into());

    m.insert("help.backup.undo_cls".into(), "backup undo --cls ## 清除撤销历史（需确认）".into());

    m.insert("help.backup.undo_force".into(), "backup undo --force ## 清除撤销历史（无需确认）".into());

    m.insert("help.clipboard.clear".into(), "clip clear ## 清除剪贴板历史".into());

    m.insert("help.clipboard.list".into(), "clip list ## 显示剪贴板历史".into());

    m.insert("help.clipboard.pathcp".into(), "pathcp ## 将当前目录路径复制到剪贴板".into());

    m.insert("help.clipboard.select".into(), "clip select <N> ## 将条目N复制回剪贴板".into());

    m.insert("help.clipboard.show".into(), "clip ## 显示剪贴板历史".into());

    m.insert("help.clipboard.title".into(), "剪贴板命令：".into());

    m.insert("help.config.autosuggest".into(), "config autosuggest on|off ## 显示/设置自动建议（幽灵文本）".into());

    m.insert("help.config.color".into(), "config color on|off ## 显示/设置颜色输出".into());

    m.insert("help.config.depth".into(), "config depth [n] ## 显示/设置最大深度（1-20）".into());

    m.insert("help.config.history".into(), "config history [on|off|路径] ## 显示/设置历史记录".into());

    m.insert("help.config.lang".into(), "config lang [语言] ## 显示/设置界面语言".into());

    m.insert("help.config.linenumbers".into(), "config linenumbers on|off ## 显示/设置行号".into());

    m.insert("help.config.local_deinit".into(), "local deinit ## 从当前目录删除 ntconfig.toml".into());

    m.insert("help.config.local_help".into(), "local help ## 显示本地命令用法".into());

    m.insert("help.config.local_init".into(), "local init ## 创建 ntconfig.toml 模板（已注释）".into());

    m.insert("help.config.local_init_all".into(), "local init --all ## 将当前设置导出到 ntconfig.toml".into());

    m.insert("help.config.opencg".into(), "opencg ## 在编辑器中打开 config.toml".into());

    m.insert("help.config.output".into(), "config output [路径] ## 显示/设置输出路径".into());

    m.insert("help.config.qsc".into(), "config qsc \"#c 0\" ## 设置QSC全局热键".into());

    m.insert("help.config.qsc_off".into(), "config qsc off ## 禁用QSC热键".into());

    m.insert("help.config.resetcg".into(), "resetcg ## 重置配置为默认值（可选备份）".into());

    m.insert("help.config.restorecg".into(), "restorecg ## 从备份恢复配置".into());

    m.insert("help.config.showcg".into(), "showcg ## 显示当前配置概览".into());

    m.insert("help.config.showenv".into(), "showenv ## 显示环境变量（前5个PATH条目）".into());

    m.insert("help.config.showenv_all".into(), "showenv --all ## 显示所有PATH条目".into());

    m.insert("help.config.showenv_d".into(), "showenv -d <n> ## 显示前n个PATH条目".into());

    m.insert("help.config.threads".into(), "config threads [n] ## 显示/设置线程数".into());

    m.insert("help.config.title".into(), "配置命令：".into());

    m.insert("help.config.watch".into(), "config watch on|off ## 启用/禁用文件监视器".into());

    m.insert("help.config.watch_trigger".into(), "config watch trigger <别名> ## 文件监视器触发时自动运行别名".into());

    m.insert("help.config.watch_trigger_off".into(), "config watch trigger off ## 禁用自动运行".into());

    m.insert("help.config.watch_limit".into(), "config watch limit <数量> ## 监视器更改框中显示的行数".into());

    m.insert("help.config.wi".into(), "wi [路径] ## 实时文件更改流（q=退出）".into());

    m.insert("help.editor.ne".into(), "ne, ntceditor <文件> ## 在内置文本编辑器中打开文件".into());

    m.insert("help.editor.ne_hyper".into(), "ne --hyper <名称> --lang <l> ## 创建C/C++ hyper项目".into());

    m.insert("help.editor.ne_init".into(), "ne --init <文件> ## 创建空文件模板".into());

    m.insert("help.editor.ne_line".into(), "ne, ntceditor <文件>:<n> ## 在第n行打开文件".into());

    m.insert("help.editor.ne_super".into(), "ne --super <文件> ## 创建超级文件模板".into());

    m.insert("help.editor.ne_welcome".into(), "ne, ntceditor ## 在当前目录打开欢迎屏幕".into());

    m.insert("help.editor.title".into(), "编辑器命令：".into());

    m.insert("help.encdec.cvcode".into(), "cvcode <来源> <数据> to <目标> ## 在格式之间转换".into());

    m.insert("help.encdec.decode_b2".into(), "decode b2 <目标> [-o <文件>] ## 二进制解码".into());

    m.insert("help.encdec.decode_b10".into(), "decode b10 <目标> [-o <文件>] ## 十进制解码".into());

    m.insert("help.encdec.decode_b64".into(), "decode b64 <目标> [-o <文件>] ## Base64解码".into());

    m.insert("help.encdec.decode_hex".into(), "decode hex <目标> [-o <文件>] ## 十六进制解码".into());

    m.insert("help.encdec.decode_url".into(), "decode url <目标> [-o <文件>] ## URL百分比解码".into());

    m.insert("help.encdec.encode_b2".into(), "encode b2 <目标> [-o <文件>] ## 二进制编码（0101...）".into());

    m.insert("help.encdec.encode_b10".into(), "encode b10 <目标> [-o <文件>] ## 十进制编码（0-255...）".into());

    m.insert("help.encdec.encode_b64".into(), "encode b64 <目标> [-o <文件>] ## Base64编码（文件或原始文本）".into());

    m.insert("help.encdec.encode_hex".into(), "encode hex <目标> [-o <文件>] ## 十六进制编码".into());

    m.insert("help.encdec.encode_url".into(), "encode url <目标> [-o <文件>] ## URL百分比编码".into());

    m.insert("help.encdec.title".into(), "编码/解码命令：".into());

    m.insert("help.explorer.hint1".into(), "h=帮助, l=跳转到行, b=父目录, f=查找".into());

    m.insert("help.explorer.hint2".into(), "/=过滤, e=编辑器, r=重命名, d=删除".into());

    m.insert("help.explorer.hint3".into(), "n=新文件, m=新目录, i=切换隐藏文件".into());

    m.insert("help.explorer.hint4".into(), "s=切换排序, S=反向排序, F5=刷新".into());

    m.insert("help.explorer.hint5".into(), "q/Esc=退出, 鼠标=点击/滚动".into());

    m.insert("help.explorer.launch".into(), "explorer, nfe [路径] ## 启动全屏终端文件浏览器".into());

    m.insert("help.explorer.title".into(), "文件浏览器（NFE）：".into());

    m.insert("help.fileops.cp".into(), "cp <源> <目标|@tp> [-f] ## 复制文件或目录".into());

    m.insert("help.fileops.mkd".into(), "mkd <目录>[, ...] ## 创建新目录".into());

    m.insert("help.fileops.mkf".into(), "mkf <文件>[, ...] ## 创建新文件".into());

    m.insert("help.fileops.rmd".into(), "rmd <目录>[, ...] ## 删除目录（需确认）".into());

    m.insert("help.fileops.rmf".into(), "rmf <文件>[, ...] ## 删除文件（需确认）".into());

    m.insert("help.fileops.rnmd".into(), "rnmd <旧名称> <新名称> ## 重命名目录".into());

    m.insert("help.fileops.rnmf".into(), "rnmf <旧名称> <新名称> ## 重命名文件".into());

    m.insert("help.fileops.mv".into(), "mv <源> <目标|@tp> [-f] ## 移动/重命名文件或目录".into());

    m.insert("help.fileops.title".into(), "文件操作：".into());

    m.insert("help.fmt.json".into(), "fmt json <文件> [to yaml|toml] ## 将JSON转换为YAML或TOML".into());

    m.insert("help.fmt.output".into(), "（添加 -o <文件> 以保存输出）".into());

    m.insert("help.fmt.title".into(), "格式转换器：".into());

    m.insert("help.fmt.toml".into(), "fmt toml <文件> [to json|yaml] ## 将TOML转换为JSON或YAML".into());

    m.insert("help.fmt.yaml".into(), "fmt yaml <文件> [to json|toml] ## 将YAML转换为JSON或TOML".into());

    m.insert("help.igcare.care_dir".into(), "care dir <名称>[, ...] ## 停止忽略目录".into());

    m.insert("help.igcare.care_ext".into(), "care ext <扩展名>[, ...] ## 添加额外支持的扩展名".into());

    m.insert("help.igcare.care_file".into(), "care file <名称>[, ...] ## 添加额外支持的文件".into());

    m.insert("help.igcare.export_all".into(), "igcare export --all <n> ## 将所有设置导出到 <名称>.ntc.igcare".into());

    m.insert("help.igcare.export_select".into(), "igcare export --select <n> ## 选择要导出的类别".into());

    m.insert("help.igcare.ignore_dir".into(), "ignore dir <名称>[, ...] ## 按名称忽略目录".into());

    m.insert("help.igcare.ignore_ext".into(), "ignore ext <扩展名>[, ...] ## 忽略文件扩展名".into());

    m.insert("help.igcare.ignore_file".into(), "ignore file <名称>[, ...] ## 忽略特定文件".into());

    m.insert("help.igcare.import".into(), "igcare import <文件> ## 从 .ntc.igcare 文件导入设置".into());

    m.insert("help.igcare.make".into(), "igcare make [名称] ## 在当前目录创建 igcare 模板".into());

    m.insert("help.igcare.show".into(), "igcared ## 显示所有忽略/关注设置".into());

    m.insert("help.igcare.title".into(), "忽略 / 关注命令：".into());

    m.insert("help.image.convert".into(), "image <路径> to <格式> ## 转换并显示输出（json|md|html|txt|desc）".into());

    m.insert("help.image.display".into(), "image display <路径> ## 在终端内直接渲染图像".into());

    m.insert("help.image.show".into(), "image show <路径> ## 在终端内直接渲染图像".into());

    m.insert("help.image.interactive".into(), "image ## 交互式图像转换器（从当前目录选取）".into());

    m.insert("help.image.layout".into(), "image <路径> --layout ## 启用UI布局检测".into());

    m.insert("help.image.ocr".into(), "image <路径> --ocr ## 启用OCR文本提取".into());

    m.insert("help.image.open".into(), "image <路径> ## 在默认查看器中打开图像".into());

    m.insert("help.image.save".into(), "image <路径> out <格式> ## 转换并保存到输出目录".into());

    m.insert("help.image.title".into(), "图像命令：".into());

    m.insert("help.math.eval".into(), "math <表达式> ## 计算数学表达式".into());

    m.insert("help.math.file".into(), "math file.ntc.math ## 编译并运行数学文件".into());

    m.insert("help.math.fun".into(), "math fun add/rm/edit/info/ls ## 管理自定义数学函数".into());

    m.insert("help.math.timer".into(), "math timer [秒] ## 计时器或带闹钟的倒计时".into());

    m.insert("help.math.title".into(), "数学命令：".into());

    m.insert("help.mode.admin".into(), "mode admin ## 退出并以管理员身份重新启动ntc".into());

    m.insert("help.mode.normal".into(), "mode normal ## 退出并以非提升模式重新启动ntc".into());

    m.insert("help.mode.title".into(), "Windows模式（Windows）：".into());

    m.insert("help.nav.back".into(), "back [n] ## 返回父目录（或n层）".into());

    m.insert("help.nav.cd".into(), "cd <路径> ## 导航到目录".into());

    m.insert("help.nav.cd_tp".into(), "cd to <tp名称> ## 传送到已保存的传送点".into());

    m.insert("help.nav.fgo".into(), "fgo <模式> [-d <n>] ## 查找文件，导航到所选文件的父目录".into());

    m.insert("help.nav.godrive".into(), "godrive ## 列出并选择Windows驱动器".into());

    m.insert("help.nav.gos".into(), "gos ## 列出子目录并选择一个".into());

    m.insert("help.nav.gosc".into(), "gosc ## 连续导航（0退出，v/s/vs查看）".into());

    m.insert("help.nav.title".into(), "导航命令：".into());

    m.insert("help.nav.where".into(), "where ## 显示ntc可执行文件和配置位置".into());

    m.insert("help.nav.example1".into(), "cd src ## 导航到 src 目录".into());

    m.insert("help.nav.example2".into(), "back ## 返回父目录".into());

    m.insert("help.nav.example3".into(), "gos ## 交互式子目录选择".into());

    m.insert("help.nav.example4".into(), "cd to web ## 传送到 'web' 保存点".into());

    m.insert("help.network.dl".into(), "dl <url> ## 下载文件（--progress, --resume, --stop, --cancel）".into());

    m.insert("help.network.dns".into(), "dns <主机> ## DNS查询（-t 记录类型：A, AAAA, ALL, MX, NS 等）".into());

    m.insert("help.network.http".into(), "http <方法> <url> ## HTTP API客户端（-H, -d, --json, -q, -a, --bearer, -s, -o, -v, -t）".into());

    m.insert("help.network.ping".into(), "ping <主机> ## ICMP ping（-c 次数）".into());

    m.insert("help.network.ipcfg".into(), "ipcfg [名称] ## 显示网卡信息（类似 ipconfig/ifconfig）".into());

    m.insert("help.network.serve".into(), "serve [端口] ## HTTP文件服务器（--auth, --tunnel, --share, --spa, --web-dir）".into());

    m.insert("help.network.smtp".into(), "smtp ## SMTP邮件客户端（--id, -i, -u, -pwd, -s, -b, -f, --to）".into());

    m.insert("help.network.spec".into(), "spec ## 显示电脑/系统规格（CPU、显卡、内存、存储、系统、序列号）".into());

    m.insert("help.network.sys".into(), "sys, top ## 实时系统监视器（CPU、内存、磁盘、进程）".into());

    m.insert("help.network.title".into(), "系统与网络命令：".into());

    m.insert("help.network.up".into(), "up <文件> <url> ## 上传文件（--post, -H, -d）".into());

    m.insert("help.ntpkg.available".into(), "ntpkg available [语言] ## 列出语言或可下载版本".into());

    m.insert("help.ntpkg.help".into(), "ntpkg help ## 显示 ntpkg 用法".into());

    m.insert("help.ntpkg.info".into(), "ntpkg info <语言> ## 显示包信息（版本、位置、二进制文件）".into());

    m.insert("help.ntpkg.install".into(), "ntpkg install <语言> [版本] ## 下载并安装运行时".into());

    m.insert("help.ntpkg.list".into(), "ntpkg list ## 显示已安装的运行时".into());

    m.insert("help.ntpkg.ntpx".into(), "ntpx <包> [二进制] [参数] ## 运行已安装包的二进制文件".into());

    m.insert("help.ntpkg.search".into(), "ntpkg search [关键词] ## 搜索可用包".into());

    m.insert("help.ntpkg.title".into(), "包管理器（ntpkg）+ ntpx：".into());

    m.insert("help.ntpkg.uninstall".into(), "ntpkg uninstall <语言> [版本] ## 删除已安装的运行时".into());

    m.insert("help.ntpkg.use".into(), "ntpkg use <语言> <版本> ## 激活特定版本".into());

    m.insert("help.ntpkg.which".into(), "ntpkg which <语言> ## 显示活动运行时的路径".into());

    m.insert("help.other.clear".into(), "clear ## 清除屏幕".into());

    m.insert("help.other.deinit".into(), "deinit [--ran|--local] ## 删除项目文件".into());

    m.insert("help.other.exit".into(), "exit, quit ## 退出ntc".into());

    m.insert("help.other.help".into(), "help ## 显示此帮助".into());

    m.insert("help.other.init".into(), "init [--ran|--local] ## 初始化项目文件（NTCRANFILE.toml + ntconfig.toml）".into());

    m.insert("help.other.print".into(), "print <文本> [--loop N] [-n|-t] ## say的别名".into());

    m.insert("help.other.say".into(), "say <文本> [--loop N] [-n|-t] ## 打印文本（无需文件重定向）".into());

    m.insert("help.other.title".into(), "其他命令：".into());

    m.insert("help.other.tutorial".into(), "tutorial ## 启动交互式分步教程".into());
    m.insert("help.unknown_topic".into(), "未知帮助主题：'{}'。运行 'help' 查看全部。".into());

    m.insert("help.other.ui".into(), "ui ## 显示当前UI模式".into());

    m.insert("help.other.version".into(), "version ## 显示版本".into());

    m.insert("help.pathman.syspath_add".into(), "syspath add <路径> ## 将目录添加到系统PATH".into());

    m.insert("help.pathman.syspath_add_cwd".into(), "syspath add --cwd | --here ## 将当前目录添加到系统PATH".into());

    m.insert("help.pathman.syspath_move".into(), "syspath move <n> <m> ## 将条目#n移动到位置#m".into());

    m.insert("help.pathman.syspath_rm".into(), "syspath rm <路径或编号> ## 从系统PATH中删除（按路径或索引）".into());

    m.insert("help.pathman.syspath_show".into(), "syspath show ## 显示系统级PATH条目".into());

    m.insert("help.pathman.title".into(), "路径管理器（Windows）：".into());

    m.insert("help.pathman.usrpath_add".into(), "usrpath add <路径> ## 将目录添加到用户PATH".into());

    m.insert("help.pathman.usrpath_add_cwd".into(), "usrpath add --cwd | --here ## 将当前目录添加到用户PATH".into());

    m.insert("help.pathman.usrpath_move".into(), "usrpath move <n> <m> ## 将条目#n移动到位置#m".into());

    m.insert("help.pathman.usrpath_rm".into(), "usrpath rm <路径或编号> ## 从用户PATH中删除（按路径或索引）".into());

    m.insert("help.pathman.usrpath_show".into(), "usrpath show ## 显示用户级PATH条目".into());

    m.insert("help.python.activate".into(), "py_activate ## 激活Python虚拟环境（自动检测）".into());

    m.insert("help.python.activate_path".into(), "py_activate <路径> ## 激活指定venv路径".into());

    m.insert("help.python.deactivate".into(), "py_deactivate ## 停用当前虚拟环境".into());

    m.insert("help.python.title".into(), "Python命令：".into());

    m.insert("help.qr.decode".into(), "qr decode <图像> ## 从图像解码QR码".into());

    m.insert("help.qr.encode".into(), "qr <文本> ## 编码文本并在终端中显示QR码".into());

    m.insert("help.qr.save".into(), "qr encode <文本> -o <文件> ## 将QR码保存为PNG图像".into());

    m.insert("help.qr.image".into(), "qr encode <图像> -o <文件> ## 将图像编码为二维码（扫描即可查看图像）".into());

    m.insert("help.qr.title".into(), "QR码命令：".into());

    m.insert("help.qsc.listen".into(), "qsc listen ## 启动QSC守护进程（后台进程）".into());

    m.insert("help.qsc.off".into(), "config qsc off ## 禁用QSC热键".into());

    m.insert("help.qsc.set".into(), "config qsc \"#c 0\" ## 设置热键（Ctrl+0）".into());

    m.insert("help.qsc.set_cs".into(), "config qsc \"#c #s a\" ## 设置热键（Ctrl+Shift+A）".into());

    m.insert("help.qsc.startup".into(), "qsc startup on|off ## 注册/取消注册为启动应用".into());

    m.insert("help.qsc.status".into(), "qsc status ## 检查QSC守护进程是否在运行".into());

    m.insert("help.qsc.stop".into(), "qsc stop ## 停止QSC守护进程".into());

    m.insert("help.qsc.title".into(), "QSC（全局热键）命令：".into());

    m.insert("help.ran.deinit".into(), "ran deinit ## 删除 NTCRANFILE.toml".into());

    m.insert("help.ran.help".into(), "ran help ## 显示 ran 用法".into());

    m.insert("help.ran.init".into(), "ran init ## 创建 NTCRANFILE.toml 模板".into());

    m.insert("help.ran.list".into(), "ran list ## 列出 NTCRANFILE.toml 中的所有目标".into());

    m.insert("help.ran.run".into(), "ran ## 运行 'standalone' 目标（默认）".into());

    m.insert("help.ran.target".into(), "ran <目标> ## 从 NTCRANFILE.toml 运行指定目标".into());

    m.insert("help.ran.title".into(), "RAN / 任务运行器命令：".into());

    m.insert("help.report.cp".into(), "report <格式> --cp ## 将报告复制到剪贴板而非保存".into());

    m.insert("help.report.docx".into(), "report docx [路径] ## 生成DOCX报告".into());

    m.insert("help.report.html".into(), "report html [路径] ## 生成HTML报告".into());

    m.insert("help.report.json".into(), "report json [路径] ## 生成JSON报告".into());

    m.insert("help.report.md".into(), "report md [路径] ## 生成Markdown报告".into());

    m.insert("help.report.pdf".into(), "report pdf [路径] ## 生成PDF报告".into());

    m.insert("help.report.title".into(), "报告命令：".into());

    m.insert("help.report.txt".into(), "report txt [路径] ## 生成TXT报告".into());

    m.insert("help.report.txtc".into(), "txtc <文件> ## 将文件内容复制到剪贴板".into());

    m.insert("help.report.txtf".into(), "txtf <文件> ## 显示文件内容".into());

    m.insert("help.report.xlsx".into(), "report xlsx [路径] ## 生成XLSX报告（含依赖分析）".into());

    m.insert("help.scr.continue".into(), "scr -c <file.ntc> [args...] ## 运行脚本，出错后继续".into());

    m.insert("help.scr.example1".into(), "scr deploy.ntc ## 运行 deploy.ntc".into());

    m.insert("help.scr.example2".into(), "scr build.ntc dev prod ## 传入参数 $1=dev, $2=prod".into());

    m.insert("help.scr.example3".into(), "scr -c setup.ntc ## 运行并在出错后继续".into());

    m.insert("help.scr.help".into(), "scr help ## 显示 scr 用法".into());

    m.insert("help.scr.init".into(), "scr init [name] ## 创建脚本模板".into());

    m.insert("help.scr.ls".into(), "scr ls ## 列出当前目录中的 *.ntc 脚本".into());

    m.insert("help.scr.run".into(), "scr <file.ntc> [args...] ## 运行自动化脚本".into());

    m.insert("help.scr.title".into(), "SCR / 自动化脚本命令：".into());

    m.insert("help.search.ds".into(), "ds <模式> [-d <n>] ## 按名称搜索目录".into());

    m.insert("help.search.fs".into(), "fs <模式> [-d <n>] ## 按名称搜索文件（模糊匹配）".into());

    m.insert("help.search.fsc".into(), "fsc <模式> [-d <n>] ## 搜索文件，选择后显示并可选择复制".into());

    m.insert("help.search.gs".into(), "gs <模式> [-d <n>] [-l <n>] ## 搜索文件内容（简单grep）".into());

    m.insert("help.search.gs_interactive".into(), "gs -i <模式> [-d <n>] ## 交互式grep（每页10条，Enter打开）".into());

    m.insert("help.search.locate".into(), "locate <模式> [-d <n>] ## 同时搜索文件和目录".into());

    m.insert("help.search.title".into(), "搜索命令：".into());

    m.insert("help.size.care".into(), "size --care ## 计算所有文件的大小（care 模式）".into());

    m.insert("help.size.path".into(), "size -p, --path <路径|@tp> ## 显示指定路径或传送点的大小".into());

    m.insert("help.size.size".into(), "size ## 显示当前目录大小".into());

    m.insert("help.size.time".into(), "size -t, --time ## 显示耗时".into());

    m.insert("help.size.title".into(), "大小命令：".into());

    m.insert("help.syscmd.any".into(), "<任意命令> ## NTC将未知命令传递给系统".into());

    m.insert("help.syscmd.native".into(), "shell -- echo, git, dir/ls, ping 等。".into());

    m.insert("help.syscmd.native2".into(), "所有命令均可与NTC命令一起本地运行。".into());

    m.insert("help.syscmd.title".into(), "系统命令：".into());

    m.insert("help.teleport.add".into(), "tp add <名称> ## 保存当前目录".into());

    m.insert("help.teleport.add_path".into(), "tp add <名称> <路径> ## 保存指定路径".into());

    m.insert("help.teleport.at".into(), "@<名称> ## 快速传送快捷方式".into());

    m.insert("help.teleport.cls".into(), "tp cls ## 清除所有保存点".into());

    m.insert("help.teleport.info".into(), "tp info <名称> ## 显示保存点详情".into());

    m.insert("help.teleport.jump".into(), "tp jump <名称> ## 传送到已保存位置".into());

    m.insert("help.teleport.jump_num".into(), "tp jump <数字> ## 按编号传送".into());

    m.insert("help.teleport.list".into(), "tp list ## 列出所有传送点".into());

    m.insert("help.teleport.menu".into(), "tp ## 交互式菜单".into());

    m.insert("help.teleport.rm".into(), "tp rm <名称> ## 删除传送点".into());

    m.insert("help.teleport.rnm".into(), "tp rnm <旧名称> to <新名称> ## 重命名保存点".into());

    m.insert("help.teleport.title".into(), "传送命令：".into());

    m.insert("help.teleport.tpb".into(), "tpb ## 传送到上一个位置（撤销传送）".into());

    m.insert("help.teleport.tpb_clear".into(), "tpb clear ## 清除传送历史".into());

    m.insert("help.teleport.tpb_history".into(), "tpb history ## 显示传送导航历史".into());

    m.insert("help.theme.add".into(), "theme add <名称> ## 创建新主题（在编辑器中打开）".into());

    m.insert("help.theme.current".into(), "theme current ## 显示当前主题名称和详情".into());

    m.insert("help.theme.edit".into(), "theme edit [名称] ## 编辑主题（或打开主题目录）".into());

    m.insert("help.theme.export".into(), "theme export <名称> ## 导出主题为 .ntc_theme 文件".into());

    m.insert("help.theme.import".into(), "theme import <文件> ## 从 .ntc_theme 文件导入主题".into());

    m.insert("help.theme.info".into(), "theme info <名称> ## 显示主题元数据和颜色数量".into());

    m.insert("help.theme.interactive".into(), "theme ## 交互式主题管理器（菜单）".into());

    m.insert("help.theme.list".into(), "theme list ## 列出所有可用主题".into());

    m.insert("help.theme.make".into(), "theme make [名称] ## 在当前目录创建主题模板".into());

    m.insert("help.theme.reload".into(), "theme reload ## 从磁盘重新加载主题".into());

    m.insert("help.theme.rm".into(), "theme rm <名称> ## 删除主题".into());

    m.insert("help.theme.rnm".into(), "theme rnm <旧名称> to <新名称> ## 重命名主题".into());

    m.insert("help.theme.switch".into(), "theme <名称> ## 按名称切换主题".into());

    m.insert("help.theme.title".into(), "主题命令：".into());

    m.insert("help.theme.where".into(), "theme where ## 显示主题目录位置".into());

    m.insert("help.tl.dir".into(), "tl dir <路径> [-s <-n|-l|-r>] ## 显示目录中支持的扩展名和总行数（可排序）".into());

    m.insert("help.tl.ext".into(), "tl ext <扩展名> [-s <-n|-l|-r>] ## 显示所有<扩展名>文件及其总行数（遵循igcare，可排序）".into());

    m.insert("help.tl.file".into(), "tl file <路径> ## 统计单个文件的行数".into());

    m.insert("help.tl.sort".into(), "tl ext|dir <目标> -s <-n|-l|-r> ## 排序：-n 普通、-l 按行数（从小到大）、-r 反向（从大到小）".into());

    m.insert("help.tl.title".into(), "代码行数统计命令：".into());

    m.insert("help.todo.add".into(), "todo add \"名称\" ## 添加新任务".into());

    m.insert("help.todo.add_at".into(), "todo add \"名称\" -> N ## 在位置N插入任务".into());

    m.insert("help.todo.clear".into(), "todo clear ## 清除所有任务（需确认）".into());

    m.insert("help.todo.info".into(), "todo info [N] ## 显示所有任务或任务N的详情".into());

    m.insert("help.todo.rm".into(), "todo rm <N> ## 删除任务N".into());

    m.insert("help.todo.show".into(), "todo ## 显示所有任务（来自当前目录的 future_todo.txt）".into());

    m.insert("help.todo.title".into(), "待办事项命令：".into());
    m.insert("help.todo.done".into(), "todo done <N> ## 标记任务 N 为已完成".into());
    m.insert("help.todo.undo".into(), "todo undo <N> ## 重新打开任务 N".into());
    m.insert("help.todo.edit".into(), "todo edit <N> \"名称\" ## 重命名任务 N".into());

    m.insert("help.arc.title".into(), "压缩包命令：".into());
    m.insert("help.arc.create".into(), "arc create <out.zip|tar.gz|tar> <文件...> ## 创建压缩包".into());
    m.insert("help.arc.extract".into(), "arc extract <文件> [目录] ## 解压压缩包".into());
    m.insert("help.arc.list".into(), "arc list <文件> ## 列出压缩包内容".into());
    m.insert("help.arc.example1".into(), "arc create backup.zip src Cargo.toml ## 创建 zip".into());
    m.insert("help.arc.example2".into(), "arc list backup.zip ## 列出内容".into());
    m.insert("help.arc.example3".into(), "arc extract backup.zip restored ## 解压".into());

    m.insert("help.scan.depth".into(), "scan ext <扩展名...> -d, --depth <n> ## 限制扫描深度".into());
    m.insert("help.scan.example1".into(), "scan ext rs ## 列出所有 .rs 文件".into());
    m.insert("help.scan.example2".into(), "scan ext rs c -d 2 ## 列出深度 2 内的 .rs/.c 文件".into());
    m.insert("help.scan.example3".into(), "scan ext rs c -s ## 列出并显示大小与总计".into());

    m.insert("help.scan.example4".into(), "scan port 192.168.1.1 --top 10 ## 扫描前 10 个端口".into());

    m.insert("help.scan.example5".into(), "scan ip 192.168.1.0/24 ## 扫描存活主机".into());
    m.insert("help.scan.ext".into(), "scan ext <扩展名...> ## 按扩展名列出文件（不统计行数）".into());

    m.insert("help.scan.port".into(), "scan port [主机] [--top N | --ports <范围> | --all] ## TCP 端口扫描".into());

    m.insert("help.scan.ip".into(), "scan ip [网段] [--top N | --ports <范围>] ## 存活主机扫描（含 ARP）".into());
    m.insert("help.scan.size".into(), "scan ext <扩展名...> -s, --size ## 显示大小与总计".into());
    m.insert("help.scan.care".into(), "scan ext <扩展名...> --care ## 包含所有文件（care 模式）".into());
    m.insert("help.scan.title".into(), "扫描命令：".into());

    m.insert("help.nssh.title".into(), "SSH 命令：".into());
    m.insert("help.nssh.add".into(), "nssh add <名称> --host <主机> ## 保存 SSH 配置".into());
    m.insert("help.nssh.ssh".into(), "nssh ssh [名称] [--cmd \"命令\"] ## 交互式 shell 或单条命令".into());
    m.insert("help.nssh.cp".into(), "nssh cp <本地> <名称>:<远端> ## SFTP 上传/下载".into());
    m.insert("help.nssh.scan".into(), "nssh scan <主机> [--top 数量] ## TCP 端口扫描".into());
    m.insert("help.nssh.manage".into(), "nssh ls|info|rm|edit ## 管理配置".into());
    m.insert("help.nssh.keygen".into(), "nssh keygen ## 通过 ssh-keygen 生成密钥".into());
    m.insert("help.nssh.example1".into(), "nssh add prod --host 203.0.113.10 --user root ## 保存配置".into());
    m.insert("help.nssh.example2".into(), "nssh ssh prod --cmd \"uptime\" ## 运行远端命令".into());
    m.insert("help.nssh.example3".into(), "nssh cp ./app.tar.gz prod:/tmp/ ## 上传文件".into());

    m.insert("help.nsheet.title".into(), "表格命令：".into());
    m.insert("help.nsheet.open".into(), "nsheet open <文件> [选项] ## 全屏表格浏览器（CSV/TSV/JSON）".into());
    m.insert("help.nsheet.view".into(), "nsheet view <文件> [选项] ## 打印表格（默认 50 行）".into());
    m.insert("help.nsheet.sort".into(), "nsheet view <文件> --sort <列> [--desc] ## 按列排序".into());
    m.insert("help.nsheet.filter".into(), "nsheet view <文件> --filter <表达式> ## 筛选行（col~文本、col=值、col>数值）".into());
    m.insert("help.nsheet.export".into(), "nsheet view <文件> --export <json|md> [-o <文件>] ## 导出".into());
    m.insert("help.nsheet.tui".into(), "全屏按键：方向键/hjkl 移动 · / 搜索 · n 下一个 · s 排序 · f 筛选 · e 导出 · Enter 查看 · q 退出".into());
    m.insert("help.nsheet.example1".into(), "nsheet open users.csv ## 浏览表格".into());
    m.insert("help.nsheet.example2".into(), "nsheet view data.json --filter \"age>30\" --sort age ## 筛选并排序".into());
    m.insert("help.nsheet.example3".into(), "nsheet view a.csv --export md -o out.md ## 导出 Markdown".into());

    m.insert("help.view.care".into(), "view --care ## 显示所有文件的树（不忽略）".into());

    m.insert("help.view.cp".into(), "view -c, --cp ## 显示树并复制到剪贴板".into());

    m.insert("help.view.depth".into(), "view -d, --depth <n> ## 使用自定义深度显示树".into());

    m.insert("help.view.dirsize".into(), "view -ds, --dirsize ## 显示仅含目录大小的树".into());

    m.insert("help.view.ext".into(), "view -ext, --ext <扩展名...> ## 仅显示匹配扩展名的树".into());

    m.insert("help.view.filesize".into(), "view -fs, --filesize ## 显示仅含文件大小的树".into());

    m.insert("help.view.path".into(), "view -p, --path <路径|@tp> ## 显示指定路径或传送点的树".into());

    m.insert("help.view.size".into(), "view -s, --size ## 显示含目录和文件大小的树".into());

    m.insert("help.view.time".into(), "view --time, -t ## 显示含执行时间的树".into());

    m.insert("help.view.title".into(), "查看命令：".into());

    m.insert("help.view.tree".into(), "view ## 显示目录树（使用配置的深度）".into());

    m.insert("helpers.building_tree".into(), "正在构建目录树...".into());

    m.insert("helpers.calculating_sizes".into(), "正在计算目录大小...".into());

    m.insert("helpers.export_cancelled".into(), "导出已取消。".into());

    m.insert("helpers.exported_all".into(), "已导出所有 {} 个别名到：{}".into());

    m.insert("helpers.exported_categories".into(), "已导出 {} 个类别到：{}".into());

    m.insert("helpers.exported_count".into(), "已导出 {} 个别名到：{}".into());

    m.insert("helpers.exported_igcare_all".into(), "已导出所有忽略/关注设置到：{}".into());

    m.insert("helpers.file_not_found".into(), "未找到文件：{}".into());

    m.insert("helpers.imported_igcare".into(), "已从以下位置导入忽略/关注设置：{}".into());

    m.insert("helpers.imported_ral".into(), "已从以下位置导入运行别名：{}".into());

    m.insert("helpers.invalid_back_count".into(), "无效的返回计数：{}".into());

    m.insert("helpers.invalid_input".into(), "无效的输入：{}".into());

    m.insert("helpers.invalid_num".into(), "无效的数字：{}".into());

    m.insert("helpers.invalid_num_range".into(), "无效的数字：{}。使用 1-{} 或 0 完成。".into());

    m.insert("helpers.no_editor".into(), "未找到外部编辑器。打开内置编辑器...".into());

    m.insert("helpers.no_ral_to_export".into(), "没有要导出的运行别名。".into());

    m.insert("igcare_export.all".into(), "全选".into());

    m.insert("igcare_export.cancel".into(), "取消".into());

    m.insert("igcare_export.extra_supported_extensions".into(), "额外支持的扩展名".into());

    m.insert("igcare_export.extra_supported_files".into(), "额外支持的文件".into());

    m.insert("igcare_export.finish".into(), "完成导出".into());

    m.insert("igcare_export.ignored_dirs".into(), "忽略的目录".into());

    m.insert("igcare_export.ignored_extensions".into(), "忽略的扩展名".into());

    m.insert("igcare_export.ignored_files".into(), "忽略的文件".into());

    m.insert("igcare_export.items".into(), "{} 个项目".into());

    m.insert("igcare_export.prompt".into(), "igcare-导出>".into());

    m.insert("igcare_export.select".into(), "按数字选择/取消选择".into());

    m.insert("igcare_export.selected".into(), "已选择".into());

    m.insert("igcare_export.selected_mark".into(), "（已选择）".into());

    m.insert("igcare_export.title".into(), "igcare 导出 — 选择类别".into());

    m.insert("image2doc.ocr_disabled".into(), "此平台或构建未启用 OCR。使用 'ocr' 功能重新编译或安装 tesseract。".into());

    m.insert("image2doc.ocr_missing".into(), "未找到 Tesseract。OCR 不可用。".into());

    m.insert("image2doc.ocr_install".into(), "通过以下命令安装：ntpkg install tesseract".into());

    m.insert("image2doc.ocr_path_win".into(), "然后将其添加到 PATH：usrpath add <目录>，接着执行 'mode admin' + 'syspath add <目录>'，或重新打开终端。".into());

    m.insert("image2doc.ocr_path_unix".into(), "然后将其添加到 PATH，例如：export PATH=\"/<目录>:$PATH\"".into());

    m.insert("image2doc.ocr_rerun".into(), "然后再次运行相同的命令。".into());

    m.insert("image2doc.ocr_failed".into(), "Tesseract 处理图像失败。".into());

    m.insert("interactive.cancelled".into(), "已取消。".into());

    m.insert("interactive.invalid_input".into(), "无效的输入。".into());

    m.insert("interactive.invalid_selection".into(), "无效的选择。".into());

    m.insert("interactive.no_items".into(), "（无项目）".into());

    m.insert("math.builtin_agg".into(), "  agg:  sum, min, max, avg".into());

    m.insert("math.builtin_constants".into(), "  constants: PI, E, PHI, TAU".into());

    m.insert("math.builtin_conv".into(), "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into());

    m.insert("math.builtin_header".into(), "内置函数：".into());

    m.insert("math.builtin_inv_trig".into(), "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc".into());

    m.insert("math.builtin_math".into(), "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into());

    m.insert("math.builtin_print".into(), "  print: print values (like Python)".into());

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert("math.builtin_trig".into(), "  trig: sin, cos, tan, cot, sec, csc".into());

    m.insert("math.compiled_header".into(), "已编译：{}".into());

    m.insert("math.compiled_header".into(), "已编译：{}".into());

    m.insert("math.countdown".into(), "倒计时：{} 秒".into());

    m.insert("math.error".into(), "  {} 在第 {} 行：{}".into());

    m.insert("math.example_add".into(), "  math 3+4*5                           → 23".into());

    m.insert("math.example_fun_call".into(), "  math square(5)                        → 25".into());

    m.insert("math.example_fun_def".into(), "  math fun add square(x) = x^2".into());

    m.insert("math.example_hexbin".into(), "  math 0xFF + 0b1010                   → 265".into());

    m.insert("math.example_rand".into(), "  math rand(1,100)                      → 42".into());

    m.insert("math.example_sin".into(), "  math sin(PI/2)                       → 1".into());

    m.insert("math.example_sqrt".into(), "  math sqrt(144)                        → 12".into());

    m.insert("math.example_timer".into(), "  math timer 10                        → countdown 10s".into());

    m.insert("math.example_tobinary".into(), "  math toBinary(255)                    → 11111111".into());

    m.insert("math.example_tohb".into(), "  math toHB(1048576)                    → 1.00 MB".into());

    m.insert("math.example_tohex".into(), "  math toHex(255)                       → FF".into());

    m.insert("math.examples_header".into(), "示例：".into());

    m.insert("math.exec_end".into(), "\u{255A}\u{2550} 结束：{} \u{2550}\u{255D}".into());

    m.insert("math.failed_read".into(), "读取 '{}' 失败：{}".into());

    m.insert("math.file_not_found".into(), "未找到文件：'{}'".into());

    m.insert("math.fun_commands_header".into(), "数学函数命令：".into());

    m.insert("math.fun_list_header".into(), "用户定义的数学函数：".into());

    m.insert("math.fun_none".into(), " 没有用户定义的函数。使用 'math fun add <名称>(<参数>) = <主体>'".into());

    m.insert("math.fun_removed".into(), " 已删除：{}".into());

    m.insert("math.help_expr".into(), "  math <表达式>         计算数学表达式".into());

    m.insert("math.help_file".into(), "  math <文件>.ntc.math  编译并运行数学文件".into());

    m.insert("math.help_fun_add".into(), "  math fun add <名称>(<参数>) = <正文>   定义函数".into());

    m.insert("math.help_fun_edit".into(), "  math fun edit <名称> = <正文>  更新函数".into());

    m.insert("math.help_fun_info".into(), "  math fun info <名称>    显示函数详情".into());

    m.insert("math.help_fun_ls".into(), "  math fun ls             列出所有自定义函数".into());

    m.insert("math.help_fun_rm".into(), "  math fun rm <名称>      移除函数".into());

    m.insert("math.help_func_call".into(), "  math <函数>(<参数>)     调用内置或自定义函数".into());

    m.insert("math.help_timer".into(), "  math timer [秒]        秒表计时或倒计时+闹铃".into());

    m.insert("math.help_title".into(), "数学模块 - 用法：".into());

    m.insert("math.is_directory".into(), "'{}' 是目录，不是文件".into());

    m.insert("math.lap_time".into(), "圈速：{} 秒".into());

    m.insert("math.time_remaining".into(), "剩余时间：{}".into());

    m.insert("math.timer_started".into(), "计时器已启动...".into());

    m.insert("math.timer_usage".into(), "用法：math timer <秒>".into());

    m.insert("math.times_up".into(), "时间到！".into());

    m.insert("misc.clear.type_help".into(), "输入 'help' 查看可用命令，输入 'exit' 退出。".into());

    m.insert("misc.clear.welcome".into(), "欢迎使用 ntc {} - 导航, 工具包, 中心".into());

    m.insert("misc.ds.example1".into(), " ds src # 使用配置深度搜索".into());

    m.insert("misc.ds.example2".into(), " ds src -d 3 # 搜索最多3层深度".into());

    m.insert("misc.ds.example3".into(), " ds test # 部分匹配：找到 test_utils/ 等".into());

    m.insert("misc.ds.usage".into(), "用法：ds <模式> [-d <深度>]".into());

    m.insert("misc.fs.example1".into(), " fs main.c # 使用配置深度搜索".into());

    m.insert("misc.fs.example2".into(), " fs main.c -d 5 # 搜索最多5层深度".into());

    m.insert("misc.fs.example3".into(), " fs main # 部分匹配：找到 main_helper.c 等".into());

    m.insert("misc.fs.usage".into(), "用法：fs <模式> [-d <深度>]".into());

    m.insert("misc.fsc.copied".into(), "'{}' 已复制到剪贴板！".into());

    m.insert("misc.fsc.copy_prompt".into(), "复制到剪贴板？（y/N）：".into());

    m.insert("misc.fsc.done_opt".into(), "0. 完成".into());

    m.insert("misc.fsc.done_str".into(), "完成。".into());

    m.insert("misc.fsc.example1".into(), " fsc main.rs # 搜索然后选择文件显示".into());

    m.insert("misc.fsc.example2".into(), " fsc main.rs -d 5 # 搜索最多5层深度".into());

    m.insert("misc.fsc.found".into(), "\u{1F50D} 找到 {} 个文件匹配 \"{}\"：".into());

    m.insert("misc.fsc.invalid_input".into(), "无效的输入。".into());

    m.insert("misc.fsc.invalid_selection".into(), "无效的选择。".into());

    m.insert("misc.fsc.not_file".into(), "所选路径不是文件。".into());

    m.insert("misc.fsc.select_prompt".into(), "选择要显示的文件：".into());

    m.insert("misc.fsc.skipped".into(), "已跳过（不支持的格式）：{}".into());

    m.insert("misc.fsc.usage".into(), "用法：fsc <模式> [-d <深度>]".into());

    m.insert("misc.gs.example1".into(), " gs main # 使用配置深度搜索文件内容".into());

    m.insert("misc.gs.example2".into(), " gs main -d 5 # 搜索最多5层深度".into());

    m.insert("misc.gs.example3".into(), " gs -i main # 交互模式（浏览结果，Enter打开）".into());

    m.insert("misc.gs.example4".into(), " gs -i main -d 3 # 带深度限制的交互模式".into());

    m.insert("misc.gs.example5".into(), " gs main -d 5 -l 10 # 仅显示前10个匹配".into());

    m.insert("misc.gs.searching".into(), "正在搜索内容...".into());

    m.insert("misc.gs.many_results".into(), "gs：找到 {} 个匹配，显示前20,000个。请缩小搜索范围。".into());

    m.insert("misc.gs.usage".into(), "用法：gs <模式> [-d <深度>] [-l <数量>] [-i]".into());

    m.insert("misc.wi.not_dir".into(), "wi：不是目录：'{}'".into());

    m.insert("misc.wi.start_error".into(), "wi：无法监视：{}".into());

    m.insert("misc.wi.usage".into(), "用法：wi [路径]".into());

    m.insert("misc.locate.example1".into(), " locate main.c # 搜索文件和目录（配置深度）".into());

    m.insert("misc.locate.example2".into(), " locate main.c -d 5 # 搜索最多5层深度".into());

    m.insert("misc.locate.example3".into(), " locate main # 部分匹配：找到 main.c, main_helper/ 等".into());

    m.insert("misc.locate.header".into(), "\u{1F50D} {} \"{}\"（{} 个结果）：".into());

    m.insert("misc.locate.label_didyoumean".into(), "您是不是要找？".into());

    m.insert("misc.locate.label_exact".into(), "精确匹配".into());

    m.insert("misc.locate.label_partial".into(), "部分匹配".into());

    m.insert("misc.locate.no_results".into(), "\n\u{1F50D} 未找到 \"{}\" 的结果\n".into());

    m.insert("misc.locate.usage".into(), "用法：locate <模式> [-d <深度>]".into());

    m.insert("misc.mkd.created".into(), "已创建目录：{}".into());

    m.insert("misc.mkd.error".into(), "mkd：{}：{}".into());

    m.insert("misc.mkd.missing".into(), "mkd：缺少操作数".into());

    m.insert("misc.mkf.created".into(), "已创建文件：{}".into());

    m.insert("misc.mkf.error".into(), "mkf：{}：{}".into());

    m.insert("misc.mkf.exists".into(), "已存在：{}".into());

    m.insert("misc.mkf.missing".into(), "mkf：缺少操作数".into());

    m.insert("misc.ne.created_hyper".into(), "已创建 hyper 项目：{}".into());

    m.insert("misc.ne.created_super".into(), "已创建超级模板：{}".into());

    m.insert("misc.ne.created_template".into(), "已创建模板：{}".into());

    m.insert("misc.ne.editor_error".into(), "编辑器错误：{}".into());

    m.insert("misc.ne.hyper_error".into(), "Hyper 项目错误：{}".into());

    m.insert("misc.ne.opened_welcome".into(), "已在 {} 中打开欢迎屏幕".into());

    m.insert("misc.ne.usage_hyper".into(), "用法：ne --hyper <名称> --lang <语言>（名称不能包含扩展名）".into());

    m.insert("misc.pathcp.copied".into(), "路径已复制到剪贴板：{}".into());

    m.insert("misc.pip.found".into(), "\u{2713} 在以下位置找到 pip：{}".into());

    m.insert("misc.pip.not_found".into(), "在 venv/Scripts 目录中未找到 pip".into());

    m.insert("misc.rmd.error".into(), "rmd：{}：{}".into());

    m.insert("misc.rmd.missing".into(), "rmd：缺少操作数".into());

    m.insert("misc.rmd.not_dir".into(), "rmd：'{}' 不是目录".into());

    m.insert("misc.rmd.not_exist".into(), "rmd：'{}' 不存在".into());

    m.insert("misc.rmd.removed".into(), "已删除目录：{}".into());

    m.insert("misc.rmf.error".into(), "rmf：{}：{}".into());

    m.insert("misc.rmf.is_dir".into(), "rmf：'{}' 是目录（请使用 rmd）".into());

    m.insert("misc.rmf.missing".into(), "rmf：缺少操作数".into());

    m.insert("misc.rmf.not_exist".into(), "rmf：'{}' 不存在".into());

    m.insert("misc.rmf.no_match".into(), "rmf：没有文件匹配 '{}'".into());

    m.insert("misc.rmf.pattern_matches".into(), "有 {} 个文件匹配 '{}'：".into());

    m.insert("misc.rmf.confirm_delete".into(), "删除 {} 个文件吗？".into());

    m.insert("misc.rmf.cancelled".into(), "已取消。".into());

    m.insert("misc.rmf.removed".into(), "已删除：{}".into());

    m.insert("misc.rnmd.error".into(), "rnmd：{}".into());

    m.insert("misc.rnmd.exists".into(), "rnmd：'{}' 已存在".into());

    m.insert("misc.rnmd.missing".into(), "rnmd：缺少操作数".into());

    m.insert("misc.rnmd.not_dir".into(), "rnmd：'{}' 不是目录".into());

    m.insert("misc.rnmd.not_exist".into(), "rnmd：'{}' 不存在".into());

    m.insert("misc.rnmd.renamed".into(), "目录已重命名 '{}' -> '{}'".into());

    m.insert("misc.rnmd.usage".into(), " 用法：rnmd <旧名称> <新名称>".into());

    m.insert("misc.rnmf.error".into(), "rnmf：{}".into());

    m.insert("misc.rnmf.exists".into(), "rnmf：'{}' 已存在".into());

    m.insert("misc.rnmf.is_dir".into(), "rnmf：'{}' 是目录（请使用 rnmd）".into());

    m.insert("misc.rnmf.missing".into(), "rnmf：缺少操作数".into());

    m.insert("misc.rnmf.not_exist".into(), "rnmf：'{}' 不存在".into());

    m.insert("misc.rnmf.renamed".into(), "已重命名 '{}' -> '{}'".into());

    m.insert("misc.rnmf.usage".into(), " 用法：rnmf <旧名称> <新名称>".into());

    m.insert("misc.mv.error".into(), "mv：{}".into());

    m.insert("misc.mv.exists".into(), "mv：目标已存在：'{}'（使用 -f 覆盖）".into());

    m.insert("misc.mv.missing".into(), "mv：缺少操作数".into());

    m.insert("misc.mv.moved".into(), "已移动 '{}' -> '{}'".into());

    m.insert("misc.mv.not_exist".into(), "mv：源不存在：'{}'".into());

    m.insert("misc.mv.usage".into(), "用法：mv <源> <目标|@tp> [-f]".into());

    m.insert("misc.cp.error".into(), "cp：{}".into());

    m.insert("misc.cp.exists".into(), "cp：目标已存在：'{}'（使用 -f 覆盖）".into());

    m.insert("misc.cp.missing".into(), "cp：缺少操作数".into());

    m.insert("misc.cp.copied".into(), "已复制 '{}' -> '{}'".into());

    m.insert("misc.cp.not_exist".into(), "cp：源不存在：'{}'".into());

    m.insert("misc.cp.same".into(), "cp：源和目标是同一文件：'{}'".into());

    m.insert("misc.cp.usage".into(), "用法：cp <源> <目标|@tp> [-f]".into());

    m.insert("misc.say.example1".into(), " say \"hello world\" --loop 5 -n # 5行".into());

    m.insert("misc.say.example2".into(), " say \"hello world\" --loop 3 -t # 1行，重复3次".into());

    m.insert("misc.say.loop_exceed".into(), "--loop 值超过最大值（{}），已限制".into());

    m.insert("misc.say.loop_invalid".into(), "无效的 --loop 值 '{}'，使用1".into());

    m.insert("misc.say.loop_requires".into(), "--loop 需要一个数字，使用1".into());

    m.insert("misc.say.loop_warning".into(), "--loop 值不能小于1，使用1".into());

    m.insert("misc.say.usage".into(), "用法：say <文本> [--loop N] [-n|-t]".into());

    m.insert("misc.show_env.activate_hint".into(), "运行 'python_activate' 激活虚拟环境".into());

    m.insert("misc.show_env.active".into(), "虚拟环境".into());

    m.insert("misc.show_env.and_more".into(), "... 及另外 {} 个".into());

    m.insert("misc.show_env.cwd_label".into(), "当前目录：".into());

    m.insert("misc.show_env.deactivate_hint".into(), "运行 'deactivate' 退出虚拟环境".into());

    m.insert("misc.show_env.not_active".into(), "虚拟环境".into());

    m.insert("misc.show_env.not_set".into(), "（未设置）".into());

    m.insert("misc.show_env.os_label".into(), "操作系统：".into());

    m.insert("misc.show_env.other_header".into(), "\u{1F527} 其他Python/Venv变量：".into());

    m.insert("misc.show_env.path_full".into(), "\u{1F4C2} PATH（完整）：".into());

    m.insert("misc.show_env.path_header".into(), "\u{1F4C2} PATH（前几个条目）：".into());

    m.insert("misc.show_env.shell_label".into(), "Shell：".into());

    m.insert("misc.show_env.summary_header".into(), "\u{1F4CA} 摘要：".into());

    m.insert("misc.show_env.sys_header".into(), "\u{1F4BB} 系统信息：".into());

    m.insert("misc.show_env.venv_header".into(), "\u{1F40D} Python虚拟环境：".into());

    m.insert("misc.size.bytes_label".into(), "字节：".into());

    m.insert("misc.size.calculating".into(), "正在计算目录大小...".into());

    m.insert("misc.size.human_label".into(), "可读格式：".into());

    m.insert("misc.size.path_label".into(), "路径：".into());

    m.insert("misc.size.time_label".into(), "耗时：".into());

    m.insert("misc.size.title".into(), "当前目录大小".into());

    m.insert("misc.tp.example_info".into(), "示例：tp info my_project".into());

    m.insert("misc.tp.info_canonical".into(), "规范路径".into());

    m.insert("misc.tp.info_exists".into(), "存在".into());

    m.insert("misc.tp.info_global".into(), "全局".into());

    m.insert("misc.tp.info_index".into(), "索引".into());

    m.insert("misc.tp.info_name".into(), "名称".into());

    m.insert("misc.tp.info_path".into(), "路径".into());

    m.insert("misc.tp.info_source".into(), "来源".into());

    m.insert("misc.tp.info_title".into(), "\u{1F4CC} 传送保存点信息".into());

    m.insert("misc.tp.invalid_format".into(), "无效的格式。使用：tp rnm <旧名称> to <新名称>".into());

    m.insert("misc.tp.not_found".into(), "未找到保存点 '{}'".into());

    m.insert("misc.tp.type_help".into(), "输入 'tp help' 查看用法。".into());

    m.insert("misc.tp.unknown_subcmd".into(), "未知的 tp 子命令：{}".into());

    m.insert("misc.tp.usage_add".into(), "用法：tp add <名称> [路径]".into());

    m.insert("misc.tp.usage_info".into(), "用法：tp info <名称>".into());

    m.insert("misc.tp.usage_jump".into(), "用法：tp jump <名称|编号>".into());

    m.insert("misc.tp.usage_rm".into(), "用法：tp rm <名称|编号>[, <名称|编号>, ...]".into());

    m.insert("misc.tp.usage_rnm".into(), "用法：tp rnm <旧名称> to <新名称>".into());

    m.insert("misc.tpb.unknown".into(), "未知的 tpb 选项：{}".into());

    m.insert("misc.tpb.usage".into(), "用法：".into());

    m.insert("misc.tpb.usage_basic".into(), " tpb - 传送到上一个位置".into());

    m.insert("misc.tpb.usage_clear".into(), " tpb clear - 清除传送历史".into());

    m.insert("misc.tpb.usage_history".into(), " tpb history - 显示传送历史".into());

    m.insert("misc.ui.removed".into(), "现代UI已被移除。仅经典Shell可用。".into());

    m.insert("misc.venva.activated".into(), "已激活：{}".into());

    m.insert("misc.venva.activating".into(), "正在激活虚拟环境：{}".into());

    m.insert("misc.venva.active_info".into(), "虚拟环境现已激活。pip和python命令将使用此环境。".into());

    m.insert("misc.venva.already_in".into(), "已在虚拟环境中。请先使用 'deactivate' 退出。".into());

    m.insert("misc.venva.failed".into(), "激活失败：{}".into());

    m.insert("misc.venva.no_script".into(), "未在以下位置找到激活脚本：{}".into());

    m.insert("misc.venva.not_found".into(), "未找到虚拟环境。用法：python_activate <环境名称>".into());

    m.insert("misc.venvd.deactivated".into(), "虚拟环境已停用".into());

    m.insert("misc.venvd.not_in_venv".into(), "不在虚拟环境中".into());

    m.insert("mode.help_admin".into(), "  mode admin              退出并以管理员身份重新启动".into());

    m.insert("mode.help_admin".into(), "  mode admin              退出并以管理员身份重新启动".into());

    m.insert("mode.help_example_admin".into(), "  mode admin               （将出现 UAC 提示）".into());

    m.insert("mode.help_example_admin".into(), "  mode admin               （将出现 UAC 提示）".into());

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_example_normal".into(), "  mode normal".into());

    m.insert("mode.help_examples".into(), "示例：".into());

    m.insert("mode.help_normal".into(), "  mode normal             退出并以普通身份重新启动".into());

    m.insert("mode.help_normal".into(), "  mode normal             退出并以普通身份重新启动".into());

    m.insert("mode.help_usage".into(), "用法：".into());

    m.insert("msg.ignore.dir_added".into(), "正在忽略目录：{}".into());

    m.insert("msg.ignore.dir_removed".into(), "不再忽略目录：{}".into());

    m.insert("msg.ignore.ext_added".into(), "正在忽略 .{} 文件".into());

    m.insert("msg.ignore.ext_removed".into(), "正在关注 .{} 文件".into());

    m.insert("msg.ignore.file_added".into(), "正在忽略文件：{}".into());

    m.insert("msg.ignore.file_removed".into(), "正在关注文件：{}".into());

    m.insert("msg.ignored.dirs".into(), "已忽略的目录：{}".into());

    m.insert("msg.ignored.extensions".into(), "已忽略的扩展名：{}".into());

    m.insert("msg.ignored.extra_extensions".into(), "额外支持的扩展名：{}".into());

    m.insert("msg.ignored.extra_files".into(), "额外支持的文件：{}".into());

    m.insert("msg.ignored.files".into(), "已忽略的文件：{}".into());

    m.insert("msg.image.converted".into(), "图像已转换并保存到：{}".into());

    m.insert("msg.image.not_found".into(), "未找到图像：{}".into());

    m.insert("msg.init.template_created".into(), "已创建模板：{}".into());

    m.insert("msg.list.entry_format".into(), " {}".into());

    m.insert("msg.list.title".into(), "ntc {} - 可用的命令行功能：\n".into());

    m.insert("msg.report.binary_clipboard_warning".into(), "{} 报告无法复制到剪贴板（二进制格式）".into());

    m.insert("msg.report.copied_to_clipboard".into(), "{} 报告已复制到剪贴板！".into());

    m.insert("msg.report.file_saved".into(), "文件已保存到：{}".into());

    m.insert("msg.report.path_not_found".into(), "未找到路径：{}".into());

    m.insert("msg.report.skipped_unsupported".into(), "已跳过（不支持的格式）：{}".into());

    m.insert("msg.setA.error".into(), "setA的值无效：{}。使用 开 或 关。".into());

    m.insert("msg.setA.off".into(), "自动建议：关".into());

    m.insert("msg.setA.on".into(), "自动建议：开".into());

    m.insert("msg.setA.status".into(), "自动建议（幽灵文本）：{}".into());

    m.insert("msg.setC.error".into(), "setC的值无效：{}。使用 开 或 关。".into());

    m.insert("msg.setC.status".into(), "颜色输出：{}".into());

    m.insert("msg.setC.success".into(), "颜色：{}".into());

    m.insert("msg.setD.error_out_of_range".into(), "无效的深度值：{}。必须在1到20之间。".into());

    m.insert("msg.setD.error_parse".into(), "无效的深度值：{}。必须是1到20之间的正整数。".into());

    m.insert("msg.setD.status".into(), "当前最大深度：{}".into());

    m.insert("msg.setD.success".into(), "最大深度已设置为：{}".into());

    m.insert("msg.setH.off".into(), "历史记录：关".into());

    m.insert("msg.setH.on".into(), "历史记录：开".into());

    m.insert("msg.setH.path".into(), "历史记录路径：{}".into());

    m.insert("msg.setH.path_default".into(), "历史记录路径：默认".into());

    m.insert("msg.setH.path_reset".into(), "历史记录路径已重置为默认".into());

    m.insert("msg.setH.path_set".into(), "历史记录路径已设置为：{}".into());

    m.insert("msg.setH.status".into(), "历史记录：{}".into());

    m.insert("msg.setL.error".into(), "setL的值无效：{}。使用 开 或 关。".into());

    m.insert("msg.setL.status".into(), "行号：{}".into());

    m.insert("msg.setL.success".into(), "行号：{}".into());

    m.insert("msg.setO.status".into(), "当前输出路径：{}".into());

    m.insert("msg.setO.success".into(), "输出路径已设置为：{}".into());

    m.insert("msg.setT.error_out_of_range".into(), "无效的线程数：{}。必须在1到64之间。".into());

    m.insert("msg.setT.error_parse".into(), "无效的线程数：{}。必须是1到64之间的正整数。".into());

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

    m.insert("msg.showcg.title".into(), "当前配置".into());

    m.insert("msg.size.bytes_label".into(), "字节：".into());

    m.insert("msg.size.human_label".into(), "可读格式：".into());

    m.insert("msg.size.path_label".into(), "路径：{}".into());

    m.insert("msg.size.time_label".into(), "耗时：".into());

    m.insert("msg.size.title".into(), "当前目录大小".into());

    m.insert("msg.teleport.error".into(), "传送到 '{}' 失败：{}".into());

    m.insert("msg.teleport.hint_list".into(), "使用 'ntc --tp-list' 查看所有保存点。".into());

    m.insert("msg.teleport.not_found".into(), "未找到传送点：'{}'".into());

    m.insert("msg.teleport.success".into(), "\u{2713} 已传送到 '{}' -> {}".into());

    m.insert("msg.version.format".into(), "ntc {}".into());

    m.insert("msg.watch.invalid".into(), "使用 --watch 开 或 --watch 关".into());

    m.insert("msg.watch.off".into(), "文件监视器：关（重启ntc以停用）".into());

    m.insert("msg.watch.on".into(), "文件监视器：开（重启ntc以激活）".into());

    m.insert("msg.watch.status".into(), "文件监视器：{}".into());

    m.insert("msg.watch.limit_status".into(), "监视器显示行数：{}".into());

    m.insert("msg.watch.limit_success".into(), "监视器显示行数已设为 {}".into());

    m.insert("msg.watch.limit_error".into(), "无效的监视器行数：'{}'。请输入正数。".into());

    m.insert("msg.where.current_dir".into(), "当前目录：{}".into());

    m.insert("msg.where.executable".into(), "ntc 可执行文件：{}".into());

    m.insert("nav.available_drives".into(), "可用驱动器：".into());

    m.insert("nav.cancelled".into(), "已取消。".into());

    m.insert("nav.config_exists".into(), " \u{2713} 配置文件存在".into());

    m.insert("nav.drive_prompt".into(), "输入驱动器字母或编号（或 'cancel'）：".into());

    m.insert("nav.gos_where".into(), "gos 哪里？".into());

    m.insert("navigator.already_root".into(), "已在根目录，无法再后退".into());

    m.insert("navigator.cannot_navigate_to".into(), "无法导航到：{}".into());

    m.insert("navigator.cannot_navigate_to".into(), "无法导航到：{}".into());

    m.insert("navigator.cannot_read_dir".into(), "无法读取目录：{}".into());

    m.insert("navigator.cannot_read_dir".into(), "无法读取目录：{}".into());

    m.insert("navigator.drive_windows_only".into(), "盘符导航仅在 Windows 上受支持".into());

    m.insert("navigator.not_a_directory".into(), "{} 不是目录".into());

    m.insert("navigator.venv_activate_unix".into(), "    运行：source ./venv/bin/activate".into());

    m.insert("navigator.venv_activate_win".into(), "   Run: .\\\\\\\\venv\\\\\\\\Scripts\\\\\\\\activate".into());

    m.insert("navigator.venv_tip".into(), "    运行：source ./venv/bin/activate".into());

    m.insert("navigator.venv_tip".into(), "    运行：source ./venv/bin/activate".into());

    m.insert("nfe.binary_file".into(), "（二进制/不支持的格式）".into());

    m.insert("nfe.binary_file_preview".into(), "（二进制文件）".into());

    m.insert("nfe.empty_dir".into(), "（空目录）".into());

    m.insert("nfe.file_truncated".into(), "...（文件已截断）".into());

    m.insert("nfe.hint_bar".into(), "h 帮助 | b 返回 | f 查找 | / 过滤 | q 退出".into());

    m.insert("nfe.no_matches".into(), "（无匹配）".into());

    m.insert("nfe.preview_empty".into(), "（空）".into());

    m.insert("nfe.preview_error".into(), "（无法读取）".into());

    m.insert("nfe.preview_more".into(), "...（更多）".into());

    m.insert("nfe.sort_date".into(), "日期".into());

    m.insert("nfe.sort_name".into(), "名称".into());

    m.insert("nfe.sort_size".into(), "大小".into());

    m.insert("nfe.sort_type".into(), "类型".into());

    m.insert("nfe.status_bar".into(), "{} 个项目 | {} 个目录 | {} 个文件 | 排序：{} {}".into());

    m.insert("ntpkg.add_to_path".into(), "添加到您的PATH：".into());

    m.insert("ntpkg.add_to_sys_path".into(), "添加到您的系统PATH（需要管理员权限）：".into());

    m.insert("ntpkg.add_to_user_path".into(), "添加到您的用户PATH：".into());

    m.insert("ntpkg.already_installed".into(), "{} {} 已安装在 {}".into());

    m.insert("ntpkg.available_hint".into(), " 使用 'ntpkg available <语言>' 列出版本。".into());

    m.insert("ntpkg.configuring".into(), "正在配置（./configure）...".into());

    m.insert("ntpkg.downloading".into(), "正在下载 {}".into());

    m.insert("ntpkg.enter_number".into(), "输入数字（1-{}）或0：".into());

    m.insert("ntpkg.extracting".into(), "正在解压...".into());

    m.insert("ntpkg.extracting_indent".into(), " 正在解压...".into());

    m.insert("ntpkg.extracting_source".into(), "正在解压源代码压缩包...".into());

    m.insert("ntpkg.filter_label".into(), "（过滤：\"{}\"）".into());

    m.insert("ntpkg.git_not_found".into(), "在PATH中未找到Git。正在下载MinGit...".into());

    m.insert("ntpkg.help_usage".into(), "ntpkg \u{2014} ntc 语言运行时包管理器".into());

    m.insert("ntpkg.help_usage_title".into(), "  用法：".into());

    m.insert("ntpkg.help_install".into(), "    ntpkg install <语言> [版本]   下载并安装运行时".into());

    m.insert("ntpkg.help_uninstall".into(), "    ntpkg uninstall <语言> [版本]  卸载已安装的运行时".into());

    m.insert("ntpkg.help_list".into(), "    ntpkg list                          显示已安装的运行时".into());

    m.insert("ntpkg.help_use".into(), "    ntpkg use <语言> <版本>       激活特定版本".into());

    m.insert("ntpkg.help_which".into(), "    ntpkg which <语言>               显示活动运行时的路径".into());

    m.insert("ntpkg.help_available".into(), "    ntpkg available [语言]            列出语言或可下载的版本".into());

    m.insert("ntpkg.help_help".into(), "    ntpkg help                           显示此帮助".into());

    m.insert("ntpkg.ntpx_header".into(), "  ntpx \u{2014} 运行已安装软件包的二进制文件".into());

    m.insert("ntpkg.ntpx_usage_line".into(), "    ntpx <包> [二进制] [参数...]".into());

    m.insert("ntpkg.help_examples_title".into(), "  示例：".into());

    m.insert("ntpkg.help_ex_ntpx_go".into(), "    ntpx go version              # 运行 'go version'".into());

    m.insert("ntpkg.help_ex_ntpx_py".into(), "    ntpx python --version        # 运行 'python --version'".into());

    m.insert("ntpkg.help_ex_ntpx_msys2".into(), "    ntpx msys2 gcc --version     # 运行 'gcc --version'（显式指定二进制）".into());

    m.insert("ntpkg.available_title".into(), "可用软件包".into());

    m.insert("ntpkg.package_info_title".into(), "软件包信息：{}".into());

    m.insert("ntpkg.active_label".into(), "活动版本：".into());

    m.insert("ntpkg.installed_label".into(), "已安装：".into());

    m.insert("ntpkg.location_label".into(), "位置：".into());

    m.insert("ntpkg.binaries_label".into(), "二进制：".into());

    m.insert("ntpkg.not_set".into(), "（未设置）".into());

    m.insert("ntpkg.none".into(), "（无）".into());

    m.insert("ntpkg.active_marker".into(), " （活动）".into());

    m.insert("ntpkg.info_usage".into(), "用法：ntpkg info <语言>".into());

    m.insert("ntpkg.install_failed".into(), "安装失败：{}".into());

    m.insert("ntpkg.install_loc".into(), "安装位置：{}".into());

    m.insert("ntpkg.install_usage".into(), "用法：ntpkg install <语言> [版本]".into());

    m.insert("ntpkg.installed_msg".into(), "{} {} 已安装！".into());

    m.insert("ntpkg.installing".into(), "正在安装（make install）...".into());

    m.insert("ntpkg.latest_suffix".into(), "（最新）".into());

    m.insert("ntpkg.launch_installer".into(), "正在启动安装程序。完成设置后关闭安装程序窗口...".into());

    m.insert("ntpkg.magick_macos".into(), "macOS上的ImageMagick：".into());

    m.insert("ntpkg.magick_manual_reg".into(), "如果已安装，请手动注册：ntpkg use imagemagick <版本>".into());

    m.insert("ntpkg.magick_not_found".into(), "安装后无法自动检测 magick.exe。".into());

    m.insert("ntpkg.msys2_gcc_incomplete".into(), "GCC安装可能未完成。您可以稍后通过pacman安装。".into());

    m.insert("ntpkg.msys2_gcc_not_found".into(), "尚未找到GCC。运行pacman安装：".into());

    m.insert("ntpkg.msys2_installed".into(), "MSYS2已安装！通过pacman安装可选包：".into());

    m.insert("ntpkg.msys2_linux_gcc".into(), "在此平台上，请使用包管理器安装GCC：".into());

    m.insert("ntpkg.msys2_running_pacman".into(), "正在运行：pacman -S mingw-w64-ucrt-x86_64-gcc".into());

    m.insert("ntpkg.no_active_version".into(), "'{}' 没有活动版本且未指定版本".into());

    m.insert("ntpkg.no_match".into(), "没有匹配 '{}' 的包".into());

    m.insert("ntpkg.no_packages".into(), "未安装任何包。使用 'ntpkg install <语言>'".into());

    m.insert("ntpkg.no_versions".into(), "'{}' 没有追踪的版本".into());

    m.insert("ntpkg.no_versions_for".into(), "'{}' 没有可用版本".into());

    m.insert("ntpkg.not_installed".into(), "{} {} 未安装".into());

    m.insert("ntpkg.now_using".into(), "正在使用 {} {}".into());

    m.insert("ntpkg.npm_bin_in_path".into(), "npm全局二进制文件在PATH中".into());

    m.insert("ntpkg.npm_detected".into(), "已检测到Node.js和npm".into());

    m.insert("ntpkg.npm_not_detected".into(), "未检测到Node.js/npm。".into());

    m.insert("ntpkg.npm_section_node_first".into(), "  ── 先安装 Node.js ──".into());

    m.insert("ntpkg.npm_node_found_at".into(), "  ✓ 在 {} 找到 Node.js".into());

    m.insert("ntpkg.npm_node_not_found".into(), "  ✗ 未找到 Node.js".into());

    m.insert("ntpkg.npm_install_via_pkg".into(), "  通过 pkg 安装：".into());

    m.insert("ntpkg.npm_install_via_ntpkg".into(), "  通过 ntpkg 安装：".into());

    m.insert("ntpkg.npm_ntpkg_cmd".into(), "    ntpkg install {}".into());

    m.insert("ntpkg.npm_cmd".into(), "    npm i -g {}".into());

    m.insert("ntpkg.npm_or_download_nodejs".into(), "  或从以下地址下载：https://nodejs.org".into());

    m.insert("ntpkg.npm_or_fnm_nvm_volta".into(), "  或通过 fnm/nvm/volta 安装".into());

    m.insert("ntpkg.npm_found_at".into(), "  ✓ 在 {} 找到 npm".into());

    m.insert("ntpkg.npm_not_found".into(), "  ✗ 未找到 npm".into());

    m.insert("ntpkg.npm_then_run".into(), "  然后运行：".into());

    m.insert("ntpkg.npm_then_rerun".into(), "  然后重新运行：".into());

    m.insert("ntpkg.npm_bin_not_in_path".into(), "npm 全局二进制目录不在 PATH 中（{}）".into());

    m.insert("ntpkg.path_add_label".into(), "  添加到 PATH：".into());

    m.insert("ntpkg.npm_install_section".into(), "  ── 安装 {} ──".into());

    m.insert("ntpkg.npm_run_cmd_label".into(), "  运行此命令：".into());

    m.insert("ntpkg.ntpx_usage".into(), "用法：ntpx <包> [二进制] [参数...]".into());

    m.insert("ntpkg.package_count".into(), "{} 个包".into());

    m.insert("ntpkg.php_skip_register".into(), "跳过用 ntpkg 注册PHP。".into());

    m.insert("ntpkg.removed_msg".into(), "{} {} 已删除".into());

    m.insert("ntpkg.select_jdk_version".into(), "选择 JDK {} 版本".into());

    m.insert("ntpkg.select_version".into(), "选择 {} 版本".into());

    m.insert("ntpkg.skip_register".into(), "跳过用 ntpkg 注册 {}。".into());

    m.insert("ntpkg.termux_use_pkg".into(), "在 Termux 上请用 pkg 而非 ntpkg 安装：".into());

    m.insert("ntpkg.termux_cmd".into(), "    pkg install {}".into());

    m.insert("ntpkg.termux_no_pkg".into(), "Termux 中没有与 '{}' 对应的 pkg 包。".into());

    m.insert("ntpkg.src_act".into(), "act - 本地运行 GitHub Actions".into());

    m.insert("ntpkg.src_act".into(), "act - 本地运行 GitHub Actions".into());

    m.insert("ntpkg.src_cloudflare".into(), "cloudflared - Cloudflare 隧道客户端".into());

    m.insert("ntpkg.src_cloudflare".into(), "cloudflared - Cloudflare 隧道客户端".into());

    m.insert("ntpkg.src_devops_tool".into(), "Azure DevOps CLI 工具".into());

    m.insert("ntpkg.src_devops_tool".into(), "Azure DevOps CLI 工具".into());

    m.insert("ntpkg.src_docker".into(), "Docker - 容器运行时".into());

    m.insert("ntpkg.src_docker".into(), "Docker - 容器运行时".into());

    m.insert("ntpkg.src_docker_compose".into(), "Docker Compose - 多容器编排".into());

    m.insert("ntpkg.src_docker_compose".into(), "Docker Compose - 多容器编排".into());

    m.insert("ntpkg.src_dotnet".into(), ".NET SDK - 跨平台开发".into());

    m.insert("ntpkg.src_dotnet".into(), ".NET SDK - 跨平台开发".into());

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - 媒体转换器".into());

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - 媒体转换器".into());

    m.insert("ntpkg.src_freebuff".into(), "Freebuff - 免费 AI 编码代理".into());

    m.insert("ntpkg.src_freebuff".into(), "Freebuff - 免费 AI 编码代理".into());

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI 工具包".into());

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI 工具包".into());

    m.insert("ntpkg.src_gh".into(), "GitHub CLI - 终端使用 GitHub".into());

    m.insert("ntpkg.src_gh".into(), "GitHub CLI - 终端使用 GitHub".into());

    m.insert("ntpkg.src_git".into(), "Git - 版本控制".into());

    m.insert("ntpkg.src_git".into(), "Git - 版本控制".into());

    m.insert("ntpkg.src_go".into(), "Go - 编程语言".into());

    m.insert("ntpkg.src_go".into(), "Go - 编程语言".into());

    m.insert("ntpkg.src_helm".into(), "Helm - Kubernetes 包管理器".into());

    m.insert("ntpkg.src_helm".into(), "Helm - Kubernetes 包管理器".into());

    m.insert("ntpkg.src_imagemagick".into(), "ImageMagick - 图像处理".into());

    m.insert("ntpkg.src_imagemagick".into(), "ImageMagick - 图像处理".into());

    m.insert("ntpkg.src_java".into(), "Java - 运行时环境".into());

    m.insert("ntpkg.src_java".into(), "Java - 运行时环境".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - AI 代码助手".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - AI 代码助手".into());

    m.insert("ntpkg.src_kind".into(), "kind - Kubernetes（Docker 内）".into());

    m.insert("ntpkg.src_kind".into(), "kind - Kubernetes（Docker 内）".into());

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - 编程语言".into());

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - 编程语言".into());

    m.insert("ntpkg.src_kubectl".into(), "kubectl - Kubernetes 命令行工具".into());

    m.insert("ntpkg.src_kubectl".into(), "kubectl - Kubernetes 命令行工具".into());

    m.insert("ntpkg.src_kustomize".into(), "Kustomize - Kubernetes 配置自定义".into());

    m.insert("ntpkg.src_kustomize".into(), "Kustomize - Kubernetes 配置自定义".into());

    m.insert("ntpkg.src_lua".into(), "Lua - 脚本语言".into());

    m.insert("ntpkg.src_lua".into(), "Lua - 脚本语言".into());

    m.insert("ntpkg.src_mimocode".into(), "Mimic - AI 编码代理".into());

    m.insert("ntpkg.src_mimocode".into(), "Mimic - AI 编码代理".into());

    m.insert("ntpkg.src_msys2".into(), "MSYS2 - 软件发行与构建平台".into());

    m.insert("ntpkg.src_msys2".into(), "MSYS2 - 软件发行与构建平台".into());

    m.insert("ntpkg.src_nim".into(), "Nim - 编程语言".into());

    m.insert("ntpkg.src_nim".into(), "Nim - 编程语言".into());

    m.insert("ntpkg.src_node".into(), "Node.js - JavaScript 运行时".into());

    m.insert("ntpkg.src_node".into(), "Node.js - JavaScript 运行时".into());

    m.insert("ntpkg.src_omniroute".into(), "OmniRoute - 可视化工具".into());

    m.insert("ntpkg.src_omniroute".into(), "OmniRoute - 可视化工具".into());

    m.insert("ntpkg.src_opencode".into(), "opencode - CLI 编码代理".into());

    m.insert("ntpkg.src_opencode".into(), "opencode - CLI 编码代理".into());

    m.insert("ntpkg.src_php".into(), "PHP - 脚本语言".into());

    m.insert("ntpkg.src_php".into(), "PHP - 脚本语言".into());

    m.insert("ntpkg.src_python".into(), "Python - 编程语言".into());

    m.insert("ntpkg.src_python".into(), "Python - 编程语言".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - 编程语言".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - 编程语言".into());

    m.insert("ntpkg.src_rust".into(), "Rust - 系统编程语言".into());

    m.insert("ntpkg.src_rust".into(), "Rust - 系统编程语言".into());

    m.insert("ntpkg.src_scala".into(), "Scala - 编程语言".into());

    m.insert("ntpkg.src_scala".into(), "Scala - 编程语言".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - 嵌入式数据库".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - 嵌入式数据库".into());

    m.insert("ntpkg.src_sqlitestudio".into(), "SQLiteStudio - SQLite 管理器".into());

    m.insert("ntpkg.src_sqlitestudio".into(), "SQLiteStudio - SQLite 管理器".into());

    m.insert("ntpkg.src_swift".into(), "Swift - 编程语言".into());

    m.insert("ntpkg.src_swift".into(), "Swift - 编程语言".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - 代码编辑器".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - 代码编辑器".into());

    m.insert("ntpkg.src_zig".into(), "Zig - 编程语言".into());

    m.insert("ntpkg.src_zig".into(), "Zig - 编程语言".into());

    m.insert("ntpkg.src_tesseract".into(), "Tesseract - OCR 引擎".into());

    m.insert("ntpkg.tesseract_running_installer".into(), "正在以静默方式运行 Tesseract 安装程序（提升权限）...".into());

    m.insert("ntpkg.tesseract_not_found".into(), "安装后无法自动检测 tesseract.exe。请用以下命令将其所在文件夹添加到 PATH：usrpath add <目录>（然后执行 'mode admin' + 'syspath add <目录>'）。".into());

    m.insert("ntpkg.tesseract_linux".into(), "在 Linux 上通过包管理器安装 Tesseract：".into());

    m.insert("ntpkg.tesseract_macos".into(), "在 macOS 上通过 Homebrew 安装 Tesseract：".into());

    m.insert("ntpkg.tesseract_unsupported".into(), "此平台不支持安装 Tesseract。".into());

    m.insert("ntpkg.src_nginx".into(), "nginx - Web 服务器 / 反向代理".into());

    m.insert("ntpkg.nginx_unsupported".into(), "此平台不支持安装 nginx。".into());

    m.insert("ntpkg.nginx_system_hint_linux".into(), "在 Linux 上通过包管理器安装 nginx：".into());

    m.insert("ntpkg.nginx_system_hint_macos".into(), "在 macOS 上通过 Homebrew 安装 nginx：".into());

    m.insert("ntpkg.nginx_termux".into(), "在 Termux 上通过其包管理器安装 nginx：".into());

    m.insert("ntpkg.nginx_choose_method".into(), "没有官方的 Linux/macOS 预编译二进制。请选择安装方式：".into());

    m.insert("ntpkg.nginx_option_source".into(), "从源码构建（下载 nginx 源码包）".into());

    m.insert("ntpkg.nginx_option_system".into(), "通过系统包管理器安装".into());

    m.insert("ntpkg.nginx_run_commands_system".into(), "使用上面显示的包管理器命令安装 nginx（不受 ntpkg 跟踪）。".into());

    m.insert("ntpkg.nginx_build_confirm".into(), "将 nginx {} 源码构建到 ntpkg 包目录？需要 make/gcc 以及 PCRE/Zlib/OpenSSL 开发包。".into());

    m.insert("ntpkg.nginx_build_deps".into(), "构建依赖提示：build-essential libpcre2-dev zlib1g-dev libssl-dev (Debian/Ubuntu)。".into());

    m.insert("ntpkg.nginx_configuring".into(), "正在执行 ./configure...".into());

    m.insert("ntpkg.nginx_compiling".into(), "正在编译...".into());

    m.insert("ntpkg.supported_langs".into(), " 支持的语言：".into());

    m.insert("ntpkg.uninstall_failed".into(), "卸载失败：{}".into());

    m.insert("ntpkg.uninstall_usage".into(), "用法：ntpkg uninstall <语言> [版本]".into());

    m.insert("ntpkg.unknown_subcmd".into(), "未知的 ntpkg 子命令：'{}'。使用 'ntpkg help'".into());

    m.insert("ntpkg.use_usage".into(), "用法：ntpkg use <语言> <版本>".into());

    m.insert("ntpkg.which_usage".into(), "用法：ntpkg which <语言> [版本]".into());

    m.insert("output.filename_header".into(), "{}：".into());

    m.insert("output.line_prefix".into(), "行 {} >>".into());

    m.insert("output.mkdir_failed".into(), "创建目录失败：{}".into());

    m.insert("output.mkfile_failed".into(), "创建文件失败：{}".into());

    m.insert("output.print_error".into(), "\u{2717} {}".into());

    m.insert("output.print_info".into(), "\u{2139} {}".into());

    m.insert("output.print_success".into(), "\u{2713} {}".into());

    m.insert("output.print_warning".into(), "\u{26A0} {}".into());

    m.insert("output.read_file_failed".into(), "读取文件 '{}' 失败".into());

    m.insert("output.write_failed".into(), "写入文件失败：{}".into());

    m.insert("ping.help.examples".into(), "示例：".into());

    m.insert("ping.help.opt_count".into(), " -c, --count <n> Ping 次数（默认 4）".into());

    m.insert("ping.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert("ping.help.options".into(), "选项：".into());

    m.insert("ping.help.title".into(), "Ping".into());

    m.insert("ping.help.usage".into(), "用法：".into());

    m.insert("ping.help.usage_line".into(), " ping <主机> [选项]".into());

    m.insert("qsc.macos_listen_hint".into(), "QSC：在添加支持后运行 'qsc listen'。".into());

    m.insert("qsc.macos_not_implemented".into(), "QSC：macOS 全局热键支持尚未实现。".into());

    m.insert("qsc.unsupported_platform".into(), "QSC：不支持的平台".into());

    m.insert("ral_export.all".into(), "全选".into());

    m.insert("ral_export.cancel".into(), "取消".into());

    m.insert("ral_export.export_cancelled".into(), "导出已取消。".into());

    m.insert("ral_export.finish".into(), "完成导出".into());

    m.insert("ral_export.prompt".into(), "ral-导出>".into());

    m.insert("ral_export.select".into(), "按数字选择/取消选择".into());

    m.insert("ral_export.selected".into(), "已选择".into());

    m.insert("ral_export.selected_mark".into(), "（已选择）".into());

    m.insert("ral_export.title".into(), "ral 导出 — 选择要导出的别名".into());

    m.insert("ran.already_exists".into(), "{} 已存在于 {} 中".into());

    m.insert("ran.calling_not_allowed".into(), "不允许在 NTCRANFILE 目标中调用 'ran'".into());

    m.insert("ran.completed".into(), "命令已成功完成。".into());

    m.insert("ran.created".into(), "已创建 {}".into());

    m.insert("ran.desc_aliases".into(), " 命令通过 ntc 的运行别名系统展开，因此别名也可以工作。".into());

    m.insert("ran.desc_aliases2".into(), " 添加 'run ' 前缀以绕过别名展开，直接作为系统命令运行。".into());

    m.insert("ran.desc_deps".into(), " 目标可以依赖其他目标。依赖项按顺序先运行。".into());

    m.insert("ran.desc_note".into(), " 不允许在 NTCRANFILE 目标命令中调用 'ran'。".into());

    m.insert("ran.desc_reserved".into(), " init, deinit, help, list, ls \u{2014} 这些不能用作目标名称。".into());

    m.insert("ran.desc_vars".into(), " 在 [vars] 下定义变量，并在命令中使用 $(VAR_NAME) 引用。".into());

    m.insert("ran.desc_vars2".into(), " 它们像 Make 变量一样工作，但使用 $(...) 语法。".into());

    m.insert("ran.error_prefix".into(), "ran：{}".into());

    m.insert("ran.exec_failed".into(), "执行命令失败：{}".into());

    m.insert("ran.executing".into(), "正在执行：{}".into());

    m.insert("ran.exit_code".into(), "命令退出代码：{}".into());

    m.insert("ran.get_cwd_failed".into(), "获取当前目录失败：{}".into());

    m.insert("ran.help_deinit_fmt".into(), " ran deinit 从当前目录删除 {}".into());

    m.insert("ran.help_help".into(), " ran help 显示此帮助".into());

    m.insert("ran.help_init_fmt".into(), " ran init 在当前目录创建 {}".into());

    m.insert("ran.help_list".into(), " ran list 列出所有可用目标".into());

    m.insert("ran.help_standalone".into(), " ran 运行 'standalone' 目标（默认）".into());

    m.insert("ran.help_target".into(), " ran <目标> 运行特定目标".into());

    m.insert("ran.help_title".into(), "ran \u{2014} ntc 任务运行器（类似 Make 但 ntc 原生）".into());

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

    m.insert("ranfile.available_title".into(), "可用目标：".into());

    m.insert("ranfile.has_command".into(), " (包含命令)".into());

    m.insert("ranfile.depends_on".into(), " ─▶ 依赖：".into());

    m.insert("ranfile.runner.hint".into(), "运行 {} 查看可用目标。".into());

    m.insert("report.generating_html".into(), "\u{1F4CA} 正在生成 {} 的 HTML 报告".into());

    m.insert("report.generating_json".into(), "\u{1F4CA} 正在生成 {} 的 JSON 报告".into());

    m.insert("report.generating_md".into(), "\u{1F4CA} 正在生成 {} 的 Markdown 报告".into());

    m.insert("report.saved_html".into(), "\u{2705} HTML 报告已保存到：{}".into());

    m.insert("report.saved_json".into(), "\u{2705} JSON 报告已保存到：{}".into());

    m.insert("report.saved_md".into(), "\u{2705} Markdown 报告已保存到：{}".into());

    m.insert("report.saved_to".into(), "报告已保存到：{}".into());

    m.insert("scr.calling_not_allowed".into(), "脚本内不允许嵌套调用 scr".into());

    m.insert("scr.cmd_failed".into(), "命令执行失败".into());

    m.insert("scr.completed".into(), "脚本 '{}' 成功完成（{} 条命令）".into());

    m.insert("scr.continue_warn".into(), "在第 {} 行出错后继续".into());

    m.insert("scr.desc_args".into(), " $1..$9, $@ 位置参数".into());

    m.insert("scr.desc_force".into(), " @ <cmd> 强制系统命令（绕过 ntc 内建命令）".into());

    m.insert("scr.desc_interp".into(), " $var / ${var} / ${var:-default} 变量插值".into());

    m.insert("scr.desc_set".into(), " set NAME = value 定义变量".into());

    m.insert("scr.done_errors".into(), "脚本 '{}' 完成，共 {} 个错误".into());

    m.insert("scr.error_at".into(), "错误：{}:{} — {}".into());

    m.insert("scr.executing".into(), "[{}] 执行：{}".into());

    m.insert("scr.failed_abort".into(), "脚本 '{}' 在第 {} 行中止（{} 个错误）".into());

    m.insert("scr.help_continue".into(), " scr -c <file.ntc> [args...] 出错后继续".into());

    m.insert("scr.help_help".into(), " scr help 显示此帮助".into());

    m.insert("scr.help_init".into(), " scr init [name] 创建脚本模板".into());

    m.insert("scr.help_ls".into(), " scr ls 列出当前目录中的 *.ntc 脚本".into());

    m.insert("scr.help_run".into(), " scr <file.ntc> [args...] 运行自动化脚本".into());

    m.insert("scr.help_title".into(), "scr — ntc 自动化脚本运行器".into());

    m.insert("scr.help_usage".into(), "用法：".into());

    m.insert("scr.init_created".into(), "已创建模板 {}".into());

    m.insert("scr.init_exists".into(), "{} 已存在".into());

    m.insert("scr.ls_empty".into(), "在 {} 中未找到 *.ntc 脚本".into());

    m.insert("scr.ls_title".into(), "自动化脚本：".into());

    m.insert("scr.no_commands".into(), "{} 中没有要运行的命令".into());

    m.insert("scr.not_found".into(), "找不到脚本：{}（{}）".into());

    m.insert("scr.start".into(), "正在运行脚本 {}...".into());

    m.insert("scr.syscmd".into(), "执行：{}".into());

    m.insert("scr.syscmd_exit".into(), "命令退出代码：{}".into());

    m.insert("scr.syscmd_terminated".into(), "命令已终止（Ctrl+C）".into());

    m.insert("status.none".into(), "无".into());

    m.insert("status.off".into(), "关".into());

    m.insert("status.on".into(), "开".into());

    m.insert("teleport.added".into(), "保存点 '{}' 已创建 -> {}".into());

    m.insert("teleport.all_cleared".into(), "所有保存点已清除。".into());

    m.insert("teleport.back_hint".into(), "使用 'tpb' 返回上一位置。".into());

    m.insert("teleport.back_opt".into(), "b. 传送返回".into());

    m.insert("teleport.cancel_opt".into(), "0. 取消".into());

    m.insert("teleport.choose_prompt".into(), "输入数字传送到（1-{}）或 b/h/0：".into());

    m.insert("teleport.clear_cancelled".into(), "清除已取消。".into());

    m.insert("teleport.clear_confirm".into(), "\u{26A0} 您确定要删除所有保存点吗？（y/N）：".into());

    m.insert("teleport.clear_hint".into(), "使用 'tp clear' 清除历史。".into());

    m.insert("teleport.fs_failed".into(), "访问文件系统失败：{}".into());

    m.insert("teleport.history_cleared".into(), "传送历史已清除。".into());

    m.insert("teleport.history_opt".into(), "h. 显示历史".into());

    m.insert("teleport.history_title".into(), "\u{1F4DC} 传送历史（最近的在最后）".into());

    m.insert("teleport.invalid_char".into(), "名称中包含无效字符：'{}'".into());

    m.insert("teleport.invalid_index".into(), "无效的索引：{}。使用 1-{}".into());

    m.insert("teleport.invalid_input_interactive".into(), "无效的输入：{}。输入数字、'b' 或 'h'。".into());

    m.insert("teleport.invalid_name".into(), "无效的名称：'{}' 被保留或为空".into());

    m.insert("teleport.jump_failed".into(), "无法传送：'{}'".into());

    m.insert("teleport.jump_hint".into(), "使用 'tp jump <名称>' 传送。".into());

    m.insert("teleport.jumped".into(), "已传送到 '{}' -> {}".into());

    m.insert("teleport.list_title".into(), "\u{1F4CC} 您的传送保存点（全局）".into());

    m.insert("teleport.menu_title".into(), "\u{1F4CC} 传送 - 您的保存点（全局）".into());

    m.insert("teleport.name_exists".into(), "无法重命名：'{}' 已作为保存点存在".into());

    m.insert("teleport.name_taken".into(), "保存点 '{}' 已存在。使用 'tp edit' 更新。".into());

    m.insert("teleport.name_taken_edit".into(), "保存点 '{}' 已存在".into());

    m.insert("teleport.no_history_first".into(), "没有传送历史。先使用 'tp jump <名称>' 创建历史。".into());

    m.insert("teleport.no_history_start".into(), "没有传送历史。使用 'tp jump <名称>' 开始建立历史。".into());

    m.insert("teleport.no_savepoints_to_clear".into(), "没有要清除的保存点。".into());

    m.insert("teleport.no_savepoints_yet".into(), "还没有保存点。使用 'tp add <名称>' 创建。".into());

    m.insert("teleport.not_found".into(), "未找到保存点：'{}'".into());

    m.insert("teleport.overwrite_cancelled".into(), "覆盖已取消。".into());

    m.insert("teleport.overwrite_prompt".into(), "保存点 '{}' 已存在于不同位置。覆盖？（y/N）：".into());

    m.insert("teleport.prev_gone".into(), "上一位置不再存在：{}".into());

    m.insert("teleport.removed".into(), "已删除保存点：'{}'".into());

    m.insert("teleport.removing_from_history".into(), "正在从历史中删除。使用 'tp history' 查看剩余条目。".into());

    m.insert("teleport.rename_failed".into(), "重命名保存点失败：{}".into());

    m.insert("teleport.rename_same".into(), "新名称与旧名称相同".into());

    m.insert("teleport.renamed".into(), "已重命名保存点 '{}' \u{2192} '{}'".into());

    m.insert("teleport.renamed_with_path".into(), "已重命名保存点 '{}' -> '{}'（-> {}）".into());

    m.insert("teleport.teleport_cancelled".into(), "传送已取消。".into());

    m.insert("teleport.teleported_back".into(), "已传送回：{}".into());

    m.insert("teleport.tpb_hint".into(), "使用 'tpb' 返回上一位置。".into());

    m.insert("teleport.updated".into(), "保存点 '{}' 已更新：{} -> {}".into());

    m.insert("theme.create_failed".into(), "创建主题失败：{}".into());

    m.insert("theme.create_file_failed".into(), "创建文件失败：{}".into());

    m.insert("theme.create_first".into(), "运行 'theme add <名称>' 创建您的第一个主题".into());

    m.insert("theme.created".into(), "已创建主题 '{}'（在编辑器中打开）".into());

    m.insert("theme.created_hint".into(), "编辑文件，保存，然后运行 'theme reload' 应用更改".into());

    m.insert("theme.current_title".into(), "当前主题：{}".into());

    m.insert("theme.dir_title".into(), "主题目录：".into());

    m.insert("theme.edit_hint".into(), "编辑 '{}'，保存，然后运行 'theme reload'".into());

    m.insert("theme.export_failed".into(), "导出主题失败：{}".into());

    m.insert("theme.export_failed_simple".into(), "导出失败：{}".into());

    m.insert("theme.exported_simple".into(), "已导出到：{}".into());

    m.insert("theme.exported_to".into(), "已导出主题 '{}' 到：{}".into());

    m.insert("theme.file_exists".into(), "文件已存在：{}".into());

    m.insert("theme.file_not_found".into(), "未找到文件：{}".into());

    m.insert("theme.files_title".into(), "主题文件：".into());

    m.insert("theme.import_failed".into(), "导入主题失败：{}".into());

    m.insert("theme.import_failed_simple".into(), "导入失败：{}".into());

    m.insert("theme.import_hint".into(), "运行 'theme list' 查看可用主题".into());

    m.insert("theme.imported".into(), "已从以下位置导入主题：{}".into());

    m.insert("theme.info_author".into(), "作者".into());

    m.insert("theme.info_defined".into(), "\u{2713} 已定义".into());

    m.insert("theme.info_description".into(), "描述".into());

    m.insert("theme.info_editor".into(), "编辑器颜色".into());

    m.insert("theme.info_shell".into(), "Shell 颜色".into());

    m.insert("theme.info_syntax".into(), "语法颜色".into());

    m.insert("theme.info_title".into(), "\u{1F3A8} 主题：{}".into());

    m.insert("theme.invalid_number".into(), "无效的编号：{}。使用 1-{}".into());

    m.insert("theme.name_empty".into(), "主题名称不能为空".into());

    m.insert("theme.no_dir".into(), "无法确定主题目录".into());

    m.insert("theme.no_dir_yet".into(), "（目录尚不存在）".into());

    m.insert("theme.no_files".into(), "（无主题文件）".into());

    m.insert("theme.not_found".into(), "未找到主题 '{}'".into());

    m.insert("theme.not_found_hint".into(), "未找到主题：{}。使用 'theme list' 查看可用主题。".into());

    m.insert("theme.opened_dir".into(), "已打开主题目录：{}".into());

    m.insert("theme.reloaded".into(), "主题已从磁盘重新加载".into());

    m.insert("theme.remove_failed".into(), "删除主题失败：{}".into());

    m.insert("theme.removed".into(), "已删除主题：{}".into());

    m.insert("theme.rename_failed".into(), "重命名主题失败：{}".into());

    m.insert("theme.rename_failed_simple".into(), "重命名失败：{}".into());

    m.insert("theme.renamed".into(), "已重命名主题 '{}' -> '{}'".into());

    m.insert("theme.renamed_simple".into(), "已重命名 '{}' -> '{}'".into());

    m.insert("theme.switch_failed".into(), "切换主题失败：{}".into());

    m.insert("theme.switch_failed_simple".into(), "切换主题失败：{}".into());

    m.insert("theme.switched".into(), "已切换到主题：{}".into());

    m.insert("theme.template_created".into(), "已创建主题模板：{}".into());

    m.insert("theme.template_info".into(), "这是一个本地模板文件。使用 'theme import' 将其添加到主题列表。".into());

    m.insert("theme.type_interactive".into(), "输入 'theme' 进入交互菜单".into());

    m.insert("theme.ui.current".into(), "当前主题".into());

    m.insert("theme.ui.exit".into(), "退出主题管理器。".into());

    m.insert("theme.ui.name_prompt".into(), "主题名称：".into());

    m.insert("theme.ui.theme_prompt".into(), "theme>".into());

    m.insert("theme.ui.title".into(), "\u{1F3A8} ntc 主题管理器".into());

    m.insert("theme.unknown_cmd".into(), "未知命令：{}".into());

    m.insert("theme.usage_add".into(), "用法：theme add <名称>".into());

    m.insert("theme.usage_export".into(), "用法：theme export <名称> [输出文件]".into());

    m.insert("theme.usage_import".into(), "用法：theme import <文件.ntc_theme>".into());

    m.insert("theme.usage_info".into(), "用法：theme info <名称>".into());

    m.insert("theme.usage_rename".into(), "用法：theme rename <旧名> to <新名>".into());

    m.insert("theme.usage_rm".into(), "用法：theme rm <名称>".into());

    m.insert("theme.usage_rnm".into(), "用法：theme rnm <旧名> to <新名>".into());

    m.insert("tutorial.complete_msg".into(), "您现在已掌握NTC的基本功能：".into());

    m.insert("tutorial.complete_title".into(), " 教程完成！ ".into());

    m.insert("tutorial.demonstrating".into(), "正在演示：".into());

    m.insert("tutorial.exit_hint".into(), "输入 'exit' 退出NTC。".into());

    m.insert("tutorial.help_hint".into(), "输入 'help' 查看完整命令参考。".into());

    m.insert("tutorial.prompt_continue".into(), "按Enter继续".into());

    m.insert("tutorial.prompt_exit_label".into(), "退出".into());

    m.insert("tutorial.prompt_skip_label".into(), "跳过".into());

    m.insert("tutorial.return_hint".into(), "按Enter返回Shell".into());

    m.insert("tutorial.step_title".into(), " NTC 教程 \u{2014} 步骤 {}/{} ".into());

    m.insert("tutorial.tip_label".into(), "\u{1F4A1} 提示：".into());

    m.insert("up.help.examples".into(), "示例：".into());

    m.insert("up.help.opt_data".into(), " -d, --data <主体> 上传内联数据".into());

    m.insert("up.help.opt_header".into(), " -H, --header <\"Key: Value\"> 添加请求标头（可重复）".into());

    m.insert("up.help.opt_help".into(), " -h, --help 显示此帮助".into());

    m.insert("up.help.opt_post".into(), " --post 使用 POST 代替 PUT".into());

    m.insert("up.help.options".into(), "选项：".into());

    m.insert("up.help.title".into(), "上传（up）".into());

    m.insert("up.help.usage".into(), "用法：".into());

    m.insert("up.help.usage_data".into(), " up -d \"数据\" <url> 上传内联文本数据".into());

    m.insert("up.help.usage_line".into(), " up <文件> <url> 通过 PUT 上传文件".into());

    m.insert("up.help.usage_post".into(), " up <文件> <url> --post 通过 POST 上传".into());

    m.insert("watcher.file_changed".into(), "\u{2190} 检测到文件变化".into());

    m.insert("watcher.more_events".into(), "\u{2026} 还有 {} 个".into());

    m.insert("watcher.tree_refreshed".into(), "监视器 \u{2192} 树已刷新".into());

    // help config
    m.insert("help.config.example1".into(), "config output ~/Desktop ## 设置输出路径到桌面".into());
    m.insert("help.config.example2".into(), "config depth 3 ## 设置最大深度为 3".into());
    m.insert("help.config.example3".into(), "config color ON ## 启用彩色输出".into());
    m.insert("help.config.example4".into(), "config threads 8 ## 设置线程数为 8".into());
    m.insert("help.config.example5".into(), "showcg ## 显示当前配置概览".into());

    // help editor
    m.insert("help.editor.example1".into(), "ne main.rs ## 在编辑器中打开 main.rs".into());
    m.insert("help.editor.example2".into(), "ne src/main.rs:42 ## 在 42 行打开 main.rs".into());
    m.insert("help.editor.example3".into(), "ne --init newfile.md ## 创建模板文件".into());
    m.insert("help.editor.example4".into(), "ne --hyper myproject --lang c ## 创建 C 语言 hyper 项目".into());

    // help theme
    m.insert("help.theme.example1".into(), "theme list ## 列出所有可用主题".into());
    m.insert("help.theme.example2".into(), "theme dark ## 切换到 'dark' 主题".into());
    m.insert("help.theme.example3".into(), "theme current ## 显示当前主题信息".into());
    m.insert("help.theme.example4".into(), "theme add mytheme ## 创建新主题".into());

    // help teleport
    m.insert("help.teleport.example1".into(), "tp add web ## 将当前目录保存为 'web'".into());
    m.insert("help.teleport.example2".into(), "tp add docs ~/Documents ## 保存指定路径".into());
    m.insert("help.teleport.example3".into(), "tp jump web ## 传送到 'web'".into());
    m.insert("help.teleport.example4".into(), "tp list ## 列出所有保存点".into());
    m.insert("help.teleport.example5".into(), "@web ## 快速传送快捷方式".into());

    // help ran
    m.insert("help.ran.example1".into(), "ran ## 运行默认独立目标".into());
    m.insert("help.ran.example2".into(), "ran build ## 运行 'build' 目标".into());
    m.insert("help.ran.example3".into(), "ran list ## 列出所有目标".into());

    // help alias
    m.insert("help.alias.example1".into(), "ral add build \"cargo build\" ## 创建简单别名".into());
    m.insert("help.alias.example2".into(), "ral add greet(x) \"echo Hello $x\" ## 创建参数化别名".into());
    m.insert("help.alias.example3".into(), "ral add copy(x) \"cp $x ${{x}}.bak\" ## 带默认值的别名".into());
    m.insert("help.alias.example4".into(), "ral list ## 显示所有别名".into());
    m.insert("help.alias.example5".into(), "ral export --all myaliases ## 导出别名".into());
    m.insert("help.alias.example6".into(), "build ## 直接运行别名".into());

    // help view
    m.insert("help.view.example1".into(), "view ## 显示目录树".into());
    m.insert("help.view.example2".into(), "view --size ## 显示带大小的目录树".into());
    m.insert("help.view.example3".into(), "view -d 2 ## 显示深度为 2 的目录树".into());
    m.insert("help.view.example4".into(), "view --cp ## 显示目录树并复制到剪贴板".into());
    m.insert("help.view.example5".into(), "view --time ## 显示带执行时间的目录树".into());
    m.insert("help.view.example6".into(), "view -p C:\\code\\project ## 显示指定路径的目录树".into());
    m.insert("help.view.example7".into(), "view @web ## 显示传送点的目录树".into());

    // help size
    m.insert("help.size.example1".into(), "size ## 显示当前目录大小".into());
    m.insert("help.size.example2".into(), "size ../src ## 显示其他目录的大小".into());
    m.insert("help.size.example3".into(), "size -p C:\\code\\project ## 显示指定路径的大小".into());
    m.insert("help.size.example4".into(), "size @web --care -t ## 传送点使用 care 模式并显示耗时".into());

    // help report
    m.insert("help.report.example1".into(), "report txt ## 生成 TXT 报告".into());
    m.insert("help.report.example2".into(), "report html --cp ## 复制 HTML 报告到剪贴板".into());
    m.insert("help.report.example3".into(), "report pdf ## 生成 PDF 报告".into());
    m.insert("help.report.example4".into(), "txtc file.txt ## 复制文件内容到剪贴板".into());

    // help backup
    m.insert("help.backup.example1".into(), "backup create ## 创建新备份".into());
    m.insert("help.backup.example2".into(), "backup list ## 列出所有备份".into());
    m.insert("help.backup.example3".into(), "backup restore 1 ## 恢复备份 #1".into());
    m.insert("help.backup.example4".into(), "backup diff 1 ## 对比备份 #1".into());

    // help search
    m.insert("help.search.example1".into(), "gs fn main ## 在文件中搜索 'fn main'".into());
    m.insert("help.search.example2".into(), "gs -i config ## 交互式搜索结果".into());
    m.insert("help.search.example3".into(), "fs *.rs ## 查找 .rs 文件".into());
    m.insert("help.search.example4".into(), "ds src ## 查找名为 'src' 的目录".into());
    m.insert("help.search.example5".into(), "locate README ## 搜索文件与目录中的 'README'".into());

    // help igcare
    m.insert("help.igcare.example1".into(), "ignore dir node_modules ## 忽略 node_modules".into());
    m.insert("help.igcare.example2".into(), "ignore ext .log ## 忽略 .log 文件".into());
    m.insert("help.igcare.example3".into(), "care dir src ## 取消忽略 src 目录".into());
    m.insert("help.igcare.example4".into(), "igcared ## 显示所有忽略/关注设置".into());

    // help network
    m.insert("help.network.example1".into(), "sys ## 显示系统监视器".into());
    m.insert("help.network.example2".into(), "http GET https://api.example.com ## 发送 GET 请求".into());
    m.insert("help.network.example3".into(), "dl https://example.com/file.zip --progress ## 下载并显示进度".into());
    m.insert("help.network.example4".into(), "ping google.com ## Ping google.com".into());
    m.insert("help.network.example5".into(), "dns example.com -t MX ## 查询 DNS MX 记录".into());
    m.insert("help.network.example6".into(), "serve 8080 ## 在端口 8080 启动 HTTP 服务器".into());

    // help clipboard
    m.insert("help.clipboard.example1".into(), "clip ## 显示剪贴板历史".into());
    m.insert("help.clipboard.example2".into(), "clip select 1 ## 将条目 #1 复制回剪贴板".into());
    m.insert("help.clipboard.example3".into(), "pathcp ## 复制当前目录路径".into());

    // help qsc
    m.insert("help.qsc.example1".into(), "config qsc \"#c 0\" ## 设置 Ctrl+0 热键".into());
    m.insert("help.qsc.example2".into(), "qsc listen ## 启动 QSC 守护进程".into());
    m.insert("help.qsc.example3".into(), "qsc status ## 检查守护进程状态".into());

    // help todo
    m.insert("help.todo.example1".into(), "todo ## 显示所有任务".into());
    m.insert("help.todo.example2".into(), "todo add \"Fix bug\" ## 添加新任务".into());
    m.insert("help.todo.example3".into(), "todo rm 1 ## 删除任务 #1".into());

    // help other
    m.insert("help.other.example1".into(), "init --ran ## 初始化 NTCRANFILE.toml".into());
    m.insert("help.other.example2".into(), "clear ## 清除屏幕".into());
    m.insert("help.other.example3".into(), "version ## 显示版本信息".into());
    m.insert("help.other.example4".into(), "tutorial ## 启动交互式教程".into());

    // help fileops
    m.insert("help.fileops.example1".into(), "mkf index.html ## 创建新文件".into());
    m.insert("help.fileops.example2".into(), "mkd src/components ## 创建目录".into());
    m.insert("help.fileops.example3".into(), "cp file.txt backup.txt ## 复制文件".into());

    // help tl
    m.insert("help.tl.example1".into(), "tl file main.rs ## 统计 main.rs 的行数".into());
    m.insert("help.tl.example2".into(), "tl ext .rs ## 统计所有 .rs 文件的行数".into());
    m.insert("help.tl.example3".into(), "tl dir . ## 统计当前目录的行数".into());
    m.insert("help.tl.example4".into(), "tl ext .rs -s -r ## 按行数从大到小列出 .rs 文件".into());

    // help ntpkg
    m.insert("help.ntpkg.example1".into(), "ntpkg install node 18 ## 安装 Node.js 18".into());
    m.insert("help.ntpkg.example2".into(), "ntpkg list ## 显示已安装的运行时".into());
    m.insert("help.ntpkg.example3".into(), "ntpkg use node 18 ## 激活 Node.js 18".into());
    m.insert("help.ntpkg.example4".into(), "ntpkg which node ## 显示 Node.js 路径".into());
    m.insert("help.ntpkg.example5".into(), "ntpkg search python ## 搜索 Python 包".into());
    m.insert("help.ntpkg.example6".into(), "ntpkg info node ## 显示 Node.js 包信息".into());
    m.insert("help.ntpkg.example7".into(), "ntpkg available ## 列出可下载的语言".into());
    m.insert("help.ntpkg.example8".into(), "ntpx node npm init ## 通过 ntpx 运行 npm".into());
    m.insert("help.ntpkg.example9".into(), "ntpkg help ## 显示 ntpkg 用法".into());

    // help qr
    m.insert("help.qr.example1".into(), "qr \"Hello World\" ## 在终端显示二维码".into());
    m.insert("help.qr.example2".into(), "qr encode \"Hello\" -o hello.png ## 保存二维码为 PNG".into());
    m.insert("help.qr.example3".into(), "qr decode qr.png ## 从图片解码二维码".into());

    m.insert("help.qr.example4".into(), "qr encode image.png -o imageqrcode.png ## 将图像编码为二维码，扫描即显示".into());

    // help pathman
    m.insert("help.pathman.example1".into(), "usrpath show ## 显示用户 PATH 条目".into());
    m.insert("help.pathman.example2".into(), "usrpath add C:\\tools ## 添加到用户 PATH".into());
    m.insert("help.pathman.example3".into(), "usrpath rm 2 ## 从用户 PATH 删除条目 #2".into());
    m.insert("help.pathman.example4".into(), "syspath show ## 显示系统 PATH 条目".into());
    m.insert("help.pathman.example5".into(), "syspath add C:\\bin ## 添加到系统 PATH".into());
    m.insert("help.pathman.example6".into(), "syspath move 3 1 ## 移动条目到位置 1".into());

    // help autoup
    m.insert("help.autoup.example1".into(), "autoup list ## 列出启动项".into());
    m.insert("help.autoup.example2".into(), "autoup add ntc \"C:\\ntc\\ntc.exe\" ## 添加到启动项".into());
    m.insert("help.autoup.example3".into(), "autoup rm ntc ## 删除启动项".into());
    m.insert("help.autoup.example4".into(), "autoup disable ntc ## 禁用启动项".into());

    // help mode
    m.insert("help.mode.example1".into(), "mode admin ## 以管理员身份重新启动".into());

    // help explorer
    m.insert("help.explorer.example1".into(), "explorer ## 在当前目录启动文件管理器".into());
    m.insert("help.explorer.example2".into(), "nfe ~/projects ## 在 ~/projects 打开文件管理器".into());

    // help math
    m.insert("help.math.example1".into(), "math 3+4*5 ## 计算算术表达式".into());
    m.insert("help.math.example2".into(), "math sin(PI/2) ## 使用函数的数学计算".into());
    m.insert("help.math.example3".into(), "math timer 30 ## 开始 30 秒倒计时".into());
    m.insert("help.math.example4".into(), "math fun add myfn \"x+1\" ## 添加自定义函数".into());
    m.insert("help.math.example5".into(), "math fun ls ## 列出自定义函数".into());
    m.insert("help.math.example6".into(), "math calc.ntc.math ## 运行数学文件".into());

    // help python
    m.insert("help.python.example1".into(), "py_activate ## 自动激活 Python 虚拟环境".into());
    m.insert("help.python.example2".into(), "py_activate .venv ## 激活指定虚拟环境".into());
    m.insert("help.python.example3".into(), "py_deactivate ## 停用虚拟环境".into());

    // help encdec
    m.insert("help.encdec.example1".into(), "encode b64 \"hello\" ## Base64 编码字符串".into());
    m.insert("help.encdec.example2".into(), "encode hex file.bin ## Hex 编码文件".into());
    m.insert("help.encdec.example3".into(), "decode b64 aGVsbG8= ## Base64 解码".into());
    m.insert("help.encdec.example4".into(), "cvcode b64 \"aGVsbG8=\" to hex ## 在格式间转换".into());

    // help fmt
    m.insert("help.fmt.example1".into(), "fmt json config.json to yaml ## 将 JSON 转换为 YAML".into());
    m.insert("help.fmt.example2".into(), "fmt toml Cargo.toml to json -o output.json ## 将 TOML 转换为 JSON".into());

    // help image
    m.insert("help.image.example1".into(), "image screenshot.png to json ## 将图片转换为 JSON 供 AI 使用".into());
    m.insert("help.image.example2".into(), "image photo.jpg --ocr ## 通过 OCR 从图片提取文本".into());

    m.insert("help.tmprt.title".into(), "温度与天气命令：".into());

    m.insert("help.tmprt.com".into(), "tmprt com [秒] ## 实时硬件温度监控（q=退出 s=冻结 c=切换'oC/'oF）".into());

    m.insert("help.tmprt.weather".into(), "tmprt weather [城市] ## 通过 OpenWeather API 获取当前天气".into());

    m.insert("help.tmprt.key".into(), "tmprt key [API_KEY] ## 保存/显示 OpenWeather API 密钥".into());

    m.insert("help.tmprt.city".into(), "tmprt city [名称] ## 保存/显示天气默认城市".into());

    m.insert("help.tmprt.sensors".into(), "tmprt sensors <status|install|uninstall|debug> ## 管理硬件传感器后端（LHM）".into());

    m.insert("help.tmprt.example1".into(), "tmprt com 5 ## 每 5 秒刷新一次的实时视图".into());

    m.insert("help.tmprt.example2".into(), "tmprt weather Bangkok ## 查询曼谷天气".into());

    m.insert("help.tmprt.example3".into(), "tmprt key 1234567890abcdef ## 保存 API 密钥".into());

    m.insert("help.tmprt.example4".into(), "tmprt city Bangkok ## 设置默认城市".into());

    m.insert("help.git.title".into(), "GIT 集成命令：".into());

    m.insert("help.git.status".into(), "git status | st ## 漂亮的仓库状态输出".into());

    m.insert("help.git.branch".into(), "git branch | b ## 列出分支（当前分支高亮）".into());

    m.insert("help.git.log".into(), "git log | l ## 彩色最近提交日志".into());

    m.insert("help.git.diff".into(), "git diff ## 显示工作区差异".into());

    m.insert("help.git.passthrough".into(), "git <任意> ## 其他 git 子命令直接透传".into());

    m.insert("help.git.example1".into(), "git status ## 显示分支和暂存/未暂存/未跟踪".into());

    m.insert("help.git.example2".into(), "git branch -a ## 列出全部分支（透传）".into());

    m.insert("help.git.example3".into(), "git commit -am \"消息\" ## 常规 git 流程（透传）".into());

    m.insert("help.config.git".into(), "config git ON|OFF ## 启用/禁用 git 集成（提示符和符号）".into());

    m.insert("help.config.dir_view".into(), "config dir_view ON|OFF ## 在shell中显示/隐藏目录树".into());

    m.insert("git.no_git_binary".into(), "未安装 Git。请运行 `ntpkg install git` 安装。".into());

    m.insert("git.not_repo".into(), "不是 git 仓库（或其父目录）。".into());

    m.insert("git.executing".into(), "正在执行：{}".into());

    m.insert("git.completed".into(), "git 命令已完成。".into());

    m.insert("git.exit_code".into(), "命令退出码：{}".into());

    m.insert("git.terminated".into(), "命令已被终止。".into());

    m.insert("git.failed".into(), "执行命令失败：{}".into());

    m.insert("git.checking_status".into(), "正在检查状态".into());

    m.insert("git.status_failed".into(), "无法读取 git 状态。".into());

    m.insert("git.branch_header".into(), "当前分支".into());

    m.insert("git.upstream".into(), "跟踪 {}".into());

    m.insert("git.ahead".into(), "领先 {}".into());

    m.insert("git.behind".into(), "落后 {}".into());

    m.insert("git.detached_header".into(), "分离的 HEAD".into());

    m.insert("git.section_conflicts".into(), "未合并的路径：".into());

    m.insert("git.section_staged".into(), "待提交的更改：".into());

    m.insert("git.section_unstaged".into(), "尚未暂存的更改：".into());

    m.insert("git.section_untracked".into(), "未跟踪的文件：".into());

    m.insert("git.clean".into(), "工作区干净。".into());

    m.insert("git.branch_failed".into(), "无法列出分支。".into());

    m.insert("git.log_failed".into(), "无法读取 git 日志。".into());

    m.insert("git.log_empty".into(), "未找到提交。".into());

    m.insert("git.help_title".into(), "GIT 集成：".into());

    m.insert("git.help_status".into(), "git status ## 显示分支和暂存/未暂存/未跟踪".into());

    m.insert("git.help_branch".into(), "git branch ## 列出分支".into());

    m.insert("git.help_log".into(), "git log ## 最近的彩色提交".into());

    m.insert("git.help_diff".into(), "git diff ## 工作区差异".into());

    m.insert("git.help_passthrough".into(), "git <任意> ## 其他都透传给真实 git".into());

    m.insert("git.examples_header".into(), "示例：".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());

    m.insert("msg.setGit.status".into(), "Git 集成：{}".into());

    m.insert("msg.setGit.on".into(), "已启用 Git 集成。".into());

    m.insert("msg.setGit.off".into(), "已禁用 Git 集成。".into());

    m.insert("msg.setGit.error".into(), "用法：config git ON|OFF".into());

    m.insert("msg.setDV.status".into(), "目录视图：{}".into());

    m.insert("msg.setDV.on".into(), "已启用目录视图。".into());

    m.insert("msg.setDV.off".into(), "已禁用目录视图。".into());

    m.insert("msg.setDV.error".into(), "用法：config dir_view ON|OFF".into());

    m.insert("cmd.tmprt.unknown_sub".into(), "未知的 tmprt 子命令：{}".into());

    m.insert("cmd.spec.other".into(), "其他".into());

    m.insert("cmd.spec.title".into(), "系统规格".into());

    m.insert("cmd.spec.cpu".into(), "CPU".into());

    m.insert("cmd.spec.vendor".into(), "厂商".into());

    m.insert("cmd.spec.generation".into(), "代数".into());

    m.insert("cmd.spec.model".into(), "型号".into());

    m.insert("cmd.spec.cores".into(), "核心 / 线程".into());

    m.insert("cmd.spec.gpu".into(), "显卡".into());

    m.insert("cmd.spec.ram".into(), "内存".into());

    m.insert("cmd.spec.storage".into(), "存储".into());

    m.insert("cmd.spec.total".into(), "总计".into());

    m.insert("cmd.spec.os".into(), "操作系统".into());

    m.insert("cmd.spec.manufacturer".into(), "制造商".into());

    m.insert("cmd.spec.computer_model".into(), "电脑型号".into());

    m.insert("cmd.spec.serial".into(), "序列号".into());

    m.insert("cmd.spec.hostname".into(), "主机名".into());

    m.insert("cmd.spec.kernel".into(), "内核".into());

    m.insert("cmd.spec.uptime".into(), "运行时间".into());

    m.insert("cmd.spec.unknown".into(), "未知".into());

    m.insert("cmd.spec.custom".into(), "自定义".into());

    m.insert("cmd.tmprt.usage_title".into(), "用法：".into());

    m.insert("cmd.tmprt.usage_com".into(), "  tmprt com [秒]        实时硬件温度监控".into());

    m.insert("cmd.tmprt.usage_weather".into(), "  tmprt weather [城市]  通过 OpenWeather API 获取当前天气".into());

    m.insert("cmd.tmprt.usage_key".into(), "  tmprt key [API_KEY]   保存/显示 OpenWeather API 密钥".into());

    m.insert("cmd.tmprt.usage_city".into(), "  tmprt city [名称]     保存/显示天气默认城市".into());

    m.insert("cmd.tmprt.usage_sensors".into(), "  tmprt sensors <子命令>  管理硬件温度传感器后端".into());

    m.insert("cmd.tmprt.examples_title".into(), "示例：".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert("cmd.tmprt.example4".into(), "tmprt key 1234567890abcdef".into());

    m.insert("cmd.tmprt.com.no_sensors".into(), "未检测到温度传感器。".into());

    m.insert("cmd.tmprt.com.no_sensors_hint".into(), "在 Windows 上可能需要管理员权限；在 Linux 上需要存在 hwmon/thermal 传感器。".into());

    m.insert("cmd.tmprt.com.footer".into(), " 按 q 退出".into());

    m.insert("cmd.tmprt.com.lhm_missing".into(), "未找到 CPU 温度后端（此机器的 ACPI 未暴露任何传感器）。".into());

    m.insert("cmd.tmprt.com.lhm_prompt".into(), "是否安装 LibreHardwareMonitorLib（自动下载，约 7 MB）以获取 CPU/GPU/主板/硬盘温度？".into());

    m.insert("cmd.tmprt.com.lhm_confirm".into(), "立即安装？[Y/n]".into());

    m.insert("cmd.tmprt.com.lhm_installing".into(), "正在下载并安装 LibreHardwareMonitorLib...".into());

    m.insert("cmd.tmprt.com.lhm_installed".into(), "LibreHardwareMonitorLib 已安装。".into());

    m.insert("cmd.tmprt.com.lhm_install_failed".into(), "LibreHardwareMonitorLib 安装失败：{}".into());

    m.insert("cmd.tmprt.com.admin_hint".into(), "CPU 温度需 PawnIO 驱动与管理员权限。运行 'mode admin' 以管理员身份重新启动 ntc。".into());
    m.insert("cmd.tmprt.com.pawnio_hint".into(), "CPU 温度需要 PawnIO 驱动。运行 'tmprt sensors install'（需要管理员权限）。".into());

    m.insert("cmd.tmprt.weather.no_key".into(), "未找到 OpenWeather API 密钥。".into());

    m.insert("cmd.tmprt.weather.no_key_hint".into(), "请设置：tmprt key <API_KEY>（或导出 OPENWEATHER_API_KEY）".into());

    m.insert("cmd.tmprt.weather.no_city".into(), "未提供城市且未配置默认城市。".into());

    m.insert("cmd.tmprt.weather.no_city_hint".into(), "用法：tmprt weather <城市> 或设置默认：tmprt city <名称>".into());

    m.insert("cmd.tmprt.weather.request_failed".into(), "天气请求失败：{}".into());

    m.insert("cmd.tmprt.weather.read_failed".into(), "读取天气响应失败：{}".into());

    m.insert("cmd.tmprt.weather.bad_key".into(), "OpenWeather 拒绝了该 API 密钥（HTTP 401）。".into());

    m.insert("cmd.tmprt.weather.city_not_found".into(), "未找到城市：{}（HTTP 404）".into());

    m.insert("cmd.tmprt.weather.http_error".into(), "OpenWeather 错误（HTTP {}）：{}".into());

    m.insert("cmd.tmprt.weather.parse_failed".into(), "解析天气数据失败：{}".into());

    m.insert("cmd.tmprt.weather.title".into(), "天气：".into());

    m.insert("cmd.tmprt.weather.condition".into(), "天气状况".into());

    m.insert("cmd.tmprt.weather.temp".into(), "温度".into());

    m.insert("cmd.tmprt.weather.feels".into(), "体感".into());

    m.insert("cmd.tmprt.weather.range".into(), "最低/最高".into());

    m.insert("cmd.tmprt.weather.humidity".into(), "湿度".into());

    m.insert("cmd.tmprt.weather.pressure".into(), "气压".into());

    m.insert("cmd.tmprt.weather.wind".into(), "风力".into());

    m.insert("cmd.tmprt.weather.visibility".into(), "能见度".into());

    m.insert("cmd.tmprt.weather.clouds".into(), "云量".into());

    m.insert("cmd.tmprt.weather.sunrise".into(), "日出".into());

    m.insert("cmd.tmprt.weather.sunset".into(), "日落".into());

    m.insert("cmd.tmprt.weather.coords".into(), "坐标".into());

    m.insert("cmd.tmprt.key.env_source".into(), "OpenWeather API 密钥（来自 OPENWEATHER_API_KEY 环境变量）：{}".into());

    m.insert("cmd.tmprt.key.status".into(), "OpenWeather API 密钥：{}".into());

    m.insert("cmd.tmprt.key.not_set".into(), "未配置 OpenWeather API 密钥。".into());

    m.insert("cmd.tmprt.key.saved".into(), "OpenWeather API 密钥已保存：{}".into());

    m.insert("cmd.tmprt.city.status".into(), "默认天气城市：{}".into());

    m.insert("cmd.tmprt.city.not_set".into(), "未配置默认天气城市。".into());

    m.insert("cmd.tmprt.city.saved".into(), "默认天气城市已保存：{}".into());

    m.insert("cmd.tmprt.sensors.help_title".into(), "tmprt sensors 子命令：".into());

    m.insert("cmd.tmprt.sensors.help_status".into(), "  tmprt sensors status     显示可用的传感器后端".into());

    m.insert("cmd.tmprt.sensors.help_install".into(), "  tmprt sensors install    下载 LibreHardwareMonitorLib（Windows）".into());

    m.insert("cmd.tmprt.sensors.help_uninstall".into(), "  tmprt sensors uninstall  删除已下载的 LibreHardwareMonitorLib 文件".into());

    m.insert("cmd.tmprt.sensors.status_title".into(), "硬件传感器后端：".into());

    m.insert("cmd.tmprt.sensors.src_acpi".into(), "ACPI/hwmon（sysinfo）".into());

    m.insert("cmd.tmprt.sensors.src_nvidia".into(), "NVIDIA（nvidia-smi）".into());

    m.insert("cmd.tmprt.sensors.src_lhm".into(), "LibreHardwareMonitor".into());

    m.insert("cmd.tmprt.sensors.src_unavailable".into(), "不可用".into());

    m.insert("cmd.tmprt.sensors.src_installed".into(), "已安装".into());

    m.insert("cmd.tmprt.sensors.src_not_installed".into(), "未安装".into());

    m.insert("cmd.tmprt.sensors.status_hint".into(), "运行 'tmprt sensors install' 可启用基于 LibreHardwareMonitor 的传感器（Windows）。".into());

    m.insert("cmd.tmprt.sensors.installed".into(), "LibreHardwareMonitorLib 已安装。".into());

    m.insert("cmd.tmprt.sensors.install_failed".into(), "LibreHardwareMonitorLib 安装失败：{}".into());

    m.insert("cmd.tmprt.sensors.uninstalled".into(), "LibreHardwareMonitorLib 文件已删除。".into());

    m.insert("cmd.tmprt.sensors.uninstall_failed".into(), "删除 LibreHardwareMonitorLib 文件失败：{}".into());

    m.insert("cmd.tmprt.sensors.not_installed".into(), "LibreHardwareMonitorLib 尚未安装。".into());

    m.insert("cmd.tmprt.sensors.windows_only".into(), "LibreHardwareMonitor 仅支持 Windows。".into());

    m.insert("cmd.tmprt.sensors.downloading".into(), "正在下载 LibreHardwareMonitorLib".into());

    m.insert("cmd.tmprt.sensors.elevation".into(), "权限级别".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "管理员".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "标准用户".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "通过 LHM 读取 CPU 温度需要管理员权限（PawnIO 驱动）。运行 'mode admin' 以管理员身份重新启动 ntc。".into());
    m.insert("cmd.tmprt.sensors.src_pawnio".into(), "PawnIO 驱动".into());
    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "未安装 PawnIO 驱动 - CPU 温度将保持 '--'。运行 'tmprt sensors install' 安装（需要管理员权限）。".into());
    m.insert("cmd.tmprt.sensors.pawnio_present".into(), "PawnIO 驱动已安装。".into());
    m.insert("cmd.tmprt.sensors.pawnio_elevating".into(), "正在提升权限以安装 PawnIO 驱动（请接受 UAC 提示）...".into());
    m.insert("cmd.tmprt.sensors.pawnio_downloading".into(), "正在下载 PawnIO 驱动".into());
    m.insert("cmd.tmprt.sensors.pawnio_installed".into(), "PawnIO 驱动已安装。".into());
    m.insert("cmd.tmprt.sensors.pawnio_reboot".into(), "PawnIO 驱动已安装 - 请重启电脑以启用。".into());
    m.insert("cmd.tmprt.sensors.pawnio_failed".into(), "PawnIO 驱动安装失败：{}".into());

    m.insert("cmd.tmprt.sensors.help_debug".into(), "  tmprt sensors debug         转储检测到的硬件与传感器值（诊断）".into());

    m.insert("cmd.tmprt.sensors.debug_title".into(), "传感器诊断（LibreHardwareMonitor）：".into());

    m.insert("cmd.tmprt.sensors.debug_none".into(), "LibreHardwareMonitor 尚未安装 - 请先运行 'tmprt sensors install'。".into());

    m.insert("dl.dns_host_required".into(), "缺少主机。用法：dns <主机>".into());
    m.insert("dl.dns_looking_up".into(), "正在通过系统工具查询 {} {} 记录...".into());
    m.insert("dl.dns_lookup_failed".into(), "DNS 查询失败（退出码：{}）".into());
    m.insert("dl.dns_missing_type".into(), "缺少 -t 后的记录类型".into());
    m.insert("dl.dns_no_records".into(), "  未找到 {} 条 {} 记录".into());
    m.insert("dl.dns_tool_failed".into(), "运行 DNS 工具失败：{}。请安装 dig 或 nslookup 以支持 {} 记录。".into());
    m.insert("dl.invalid_request".into(), "无效的请求：{}".into());
    m.insert("dl.ping_exit_code".into(), "ping 退出码：{}".into());
    m.insert("dl.ping_failed".into(), "运行 ping 失败：{}".into());
    m.insert("dl.ping_host_required".into(), "缺少主机。用法：ping <主机>".into());
    m.insert("dl.ping_invalid_count".into(), "无效的数量".into());
    m.insert("dl.ping_missing_count".into(), "缺少 -c 后的数量".into());
    m.insert("dl.request_failed".into(), "请求 {} 失败".into());
    m.insert("dl.upload_failed".into(), "上传到 {} 失败".into());



    m

}

