use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ndb.banner".into(), "ndb {} - NTC 数据库外壳".into());

    m.insert("ndb.cwd_line".into(), "当前目录：{}".into());

    m.insert(
        "ndb.no_db_files".into(),
        "当前目录中没有找到 .db 文件。".into(),
    );

    m.insert(
        "ndb.no_db_files_hint".into(),
        "请在此处添加数据库文件，或 cd 进入包含数据库的目录。".into(),
    );

    m.insert("ndb.db_files_header".into(), "可用数据库（{}）：".into());

    m.insert("ndb.tables_count".into(), "{} 个表".into());

    m.insert("ndb.not_a_db".into(), "不是数据库".into());

    m.insert("ndb.pick_db".into(), "选择要打开的数据库".into());

    m.insert("ndb.pick_table".into(), "选择要显示的表".into());

    m.insert("ndb.pick_prompt".into(), "输入编号（1-{}）或 0：".into());

    m.insert("ndb.open_failed".into(), "打开 '{}' 失败：{}".into());

    m.insert("ndb.opened".into(), "已打开数据库：{}（只读）".into());

    m.insert(
        "ndb.hint_next".into(),
        "可用：tables、viewf、tablef <表>、schema <表>、sql <查询>、exit。".into(),
    );

    m.insert("ndb.no_tables".into(), "此数据库没有表。".into());

    m.insert("ndb.query_failed".into(), "查询失败：{}".into());

    m.insert("ndb.unknown_cmd".into(), "未知的 ndb 命令：'{}'".into());

    m.insert(
        "ndb.hint_help".into(),
        "输入 'help' 查看所有 ndb 命令。".into(),
    );

    m.insert(
        "ndb.interface_usage".into(),
        "用法：interface <文件.db>（或用 'view' 列出数据库）".into(),
    );

    m.insert(
        "ndb.sql_needs_db".into(),
        "请先打开数据库：interface <文件.db>".into(),
    );

    m.insert(
        "ndb.sql_usage".into(),
        "用法：sql <查询>   例如 sql SELECT * FROM users".into(),
    );

    m.insert(
        "ndb.sql_blocked".into(),
        "ndb 是只读的：仅允许 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into(),
    );

    m.insert("ndb.sql_error".into(), "SQL 错误：{}".into());

    m.insert("ndb.table_not_found".into(), "未找到表：{}".into());

    m.insert("ndb.schema_header".into(), "'{}' 的结构：".into());

    m.insert("ndb.schema_usage".into(), "用法：schema <表>".into());

    m.insert("ndb.showing_table".into(), "表：{}（只读）".into());

    m.insert("ndb.editor_fail".into(), "创建行编辑器失败：{}".into());

    m.insert(
        "ndb.start_hint".into(),
        "输入 'help' 查看命令，输入 'exit' 返回 ntc 外壳。".into(),
    );

    m.insert("ndb.closed_db".into(), "数据库已关闭。".into());

    m.insert("ndb.bye".into(), "已退出 ndb 外壳。".into());

    m.insert(
        "ndb.no_args".into(),
        "用法：ndb（打开 ndb 数据库外壳）".into(),
    );

    m.insert("ndb.null".into(), "NULL".into());

    m.insert("ndb.kind_view".into(), "视图".into());

    m.insert("ndb.kind_table".into(), "表".into());

    m.insert("ndb.columns_count".into(), "{} 列".into());

    m.insert("ndb.rows_count".into(), "{} 行".into());

    m.insert("ndb.rows_empty".into(), "0 行".into());

    m.insert("ndb.rows_shown".into(), "已显示 {} 行".into());

    m.insert(
        "ndb.rows_truncated".into(),
        "仅显示前 {} 行 - 使用 --all 或 -l <n> 查看更多。".into(),
    );

    m.insert("ndb.no_columns".into(), "该语句没有返回列。".into());

    m.insert("ndb.col_name".into(), "名称".into());

    m.insert("ndb.col_kind".into(), "类型".into());

    m.insert("ndb.col_columns".into(), "列数".into());

    m.insert("ndb.col_rows".into(), "行数".into());

    m.insert("ndb.col_type".into(), "数据类型".into());

    m.insert("ndb.col_constraints".into(), "约束".into());

    m.insert(
        "ndb.help_root_title".into(),
        "NDB - 数据库外壳（未打开数据库）：".into(),
    );

    m.insert(
        "ndb.help_root_help".into(),
        "help | h | ?              显示此命令列表".into(),
    );

    m.insert(
        "ndb.help_root_view".into(),
        "view | ls                 列出当前目录中的 .db 文件".into(),
    );

    m.insert(
        "ndb.help_root_interface".into(),
        "interface <文件.db>      打开数据库（提示符变为 ndb [文件.db]>）".into(),
    );

    m.insert(
        "ndb.help_root_sql".into(),
        "sql                       打开数据库后可用（先 interface）".into(),
    );

    m.insert(
        "ndb.help_root_clear".into(),
        "clear | cls               清屏并重新显示数据库".into(),
    );

    m.insert(
        "ndb.help_root_exit".into(),
        "exit | q                  返回 ntc 外壳".into(),
    );

    m.insert(
        "ndb.help_db_title".into(),
        "NDB - 数据库外壳（只读）：".into(),
    );

    m.insert(
        "ndb.help_db_help".into(),
        "help | h | ?              显示此命令列表".into(),
    );

    m.insert(
        "ndb.help_db_viewf".into(),
        "viewf | v                 显示结构树（表 + 列）".into(),
    );

    m.insert(
        "ndb.help_db_tables".into(),
        "tables [筛选]             列出表和视图名称（可选筛选）".into(),
    );

    m.insert(
        "ndb.help_db_tablef".into(),
        "tablef <表>               显示表的行（类似 txtf）".into(),
    );

    m.insert(
        "ndb.help_db_schema".into(),
        "schema <表>               显示 CREATE SQL、列、索引和键".into(),
    );

    m.insert(
        "ndb.help_db_sql".into(),
        "sql <查询>                执行只读查询（-l N、--all）".into(),
    );

    m.insert(
        "ndb.help_db_write".into(),
        "write YES <语句>          执行一条写入（INSERT/UPDATE/DELETE，需先 edit on）".into(),
    );

    m.insert(
        "ndb.help_db_edit".into(),
        "edit on|off              允许或禁止写入当前数据库".into(),
    );

    m.insert(
        "ndb.help_db_backup".into(),
        "backup                    在旁边复制一份 .db 文件".into(),
    );

    m.insert(
        "ndb.help_db_more".into(),
        "more | next             显示上次查询的下一页".into(),
    );

    m.insert(
        "ndb.help_db_indexes".into(),
        "indexes [表]             列出索引（全部或指定表）".into(),
    );

    m.insert(
        "ndb.help_db_info".into(),
        "info                    显示文件、引擎和对象统计".into(),
    );

    m.insert(
        "ndb.help_db_interface".into(),
        "interface <文件.db>      切换到另一个数据库".into(),
    );

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

    m.insert(
        "ndb.no_query_yet".into(),
        "先执行 'sql <查询>' 或 'tablef <表>'，再用 'more'。".into(),
    );

    m.insert(
        "ndb.multiline_hint".into(),
        "输入以 ';' 结尾的 SQL（Ctrl-C 中止）：".into(),
    );

    m.insert("ndb.yes".into(), "是".into());

    m.insert("ndb.no".into(), "否".into());

    m.insert(
        "ndb.help_db_clear".into(),
        "clear | cls               清屏并重新显示表".into(),
    );

    m.insert(
        "ndb.help_db_exit".into(),
        "exit | q                  关闭数据库，返回 ndb [目录]>".into(),
    );

    m.insert(
        "ndb.help_dbal".into(),
        "dbal                        管理已保存的查询（数据库别名）".into(),
    );

    m.insert(
        "ndb.dbal_title".into(),
        "DBAL - 数据库别名（已保存的只读查询）：".into(),
    );

    m.insert(
        "ndb.dbal_add".into(),
        "dbal add <名称> \"<SELECT ...>\"   保存查询".into(),
    );

    m.insert(
        "ndb.dbal_edit".into(),
        "dbal edit <名称> \"<SELECT ...>\"  更新已保存的查询".into(),
    );

    m.insert(
        "ndb.dbal_rnm".into(),
        "dbal rnm <旧名> to <新名>        重命名别名".into(),
    );

    m.insert(
        "ndb.dbal_rm".into(),
        "dbal rm <名称>                删除别名".into(),
    );

    m.insert(
        "ndb.dbal_info".into(),
        "dbal info <名称>              显示别名内容".into(),
    );

    m.insert(
        "ndb.dbal_list".into(),
        "dbal list                   列出已保存的查询".into(),
    );

    m.insert(
        "ndb.dbal_run".into(),
        "dbal run <名称> [参数] [-l N|--all]  运行已保存的查询（$参数）".into(),
    );

    m.insert(
        "ndb.dbal_cls".into(),
        "dbal cls --force             删除所有别名".into(),
    );

    m.insert(
        "ndb.dbal_exec".into(),
        "<名称>                        直接运行已保存的查询".into(),
    );

    m.insert("ndb.dbal_note".into(), "别名仅保存只读查询；写入会被拒绝。直接输入别名即可运行。参数值按字面替换——字符串请自行加引号：dbal run q \"'2026-01-01'\"。".into());

    m.insert("ndb.dbal_params".into(), "参数：{}".into());

    m.insert("ndb.dbal_missing_args".into(), "缺少参数值：{}".into());

    m.insert(
        "ndb.dbal_single_only".into(),
        "dbal 别名只保存一条语句——用 && 运行多条。".into(),
    );

    m.insert(
        "ndb.dbal_add_usage".into(),
        "用法：dbal add <名称> \"<SELECT ...>\"".into(),
    );

    m.insert(
        "ndb.dbal_edit_usage".into(),
        "用法：dbal edit <名称> \"<SELECT ...>\"".into(),
    );

    m.insert(
        "ndb.dbal_rnm_usage".into(),
        "用法：dbal rnm <旧名> to <新名>".into(),
    );

    m.insert("ndb.dbal_rm_usage".into(), "用法：dbal rm <名称>".into());

    m.insert(
        "ndb.dbal_info_usage".into(),
        "用法：dbal info <名称>".into(),
    );

    m.insert(
        "ndb.dbal_run_usage".into(),
        "用法：dbal run <名称> [-l N | --all]".into(),
    );

    m.insert(
        "ndb.dbal_cls_usage".into(),
        "用法：dbal cls --force（删除所有数据库别名）".into(),
    );

    m.insert("ndb.dbal_added".into(), "已添加别名 '{}'（{}）".into());

    m.insert("ndb.dbal_updated".into(), "已更新别名 '{}'".into());

    m.insert("ndb.dbal_removed".into(), "已删除别名 '{}'".into());

    m.insert("ndb.dbal_renamed".into(), "已将 '{}' 重命名为 '{}'".into());

    m.insert(
        "ndb.dbal_cleared".into(),
        "已清除 {} 个数据库别名（{}）".into(),
    );

    m.insert("ndb.dbal_not_found".into(), "未找到别名 '{}'".into());

    m.insert("ndb.dbal_invalid_name".into(), "无效的别名 '{}'".into());

    m.insert(
        "ndb.dbal_name_rules".into(),
        "名称不能是 ndb 命令，也不能包含空格或引号。".into(),
    );

    m.insert(
        "ndb.dbal_blocked".into(),
        "dbal 仅保存只读查询：只允许 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into(),
    );

    m.insert(
        "ndb.dbal_unknown_sub".into(),
        "未知的 dbal 命令：'{}'".into(),
    );

    m.insert(
        "ndb.dbal_no_aliases".into(),
        "还没有数据库别名。添加一个：dbal add <名称> \"<SELECT ...>\"".into(),
    );

    m.insert("ndb.dbal_scope_local".into(), "本地".into());

    m.insert("ndb.dbal_scope_global".into(), "全局".into());

    m.insert("ndb.col_sql".into(), "SQL".into());

    m.insert(
        "ndb.help_note".into(),
        "所有数据库都以只读方式打开；ndb 绝不会修改你的文件。".into(),
    );

    m.insert(
        "ndb.help_readonly".into(),
        "只读：仅执行 SELECT、PRAGMA、EXPLAIN、WITH 和 VALUES。".into(),
    );

    m.insert(
        "ndb.help_limit".into(),
        "行数限制：默认 50（-l <n> 修改，--all 显示全部）。".into(),
    );

    m.insert(
        "ndb.edit_on".into(),
        "已为 {} 开启编辑模式（允许写入）".into(),
    );

    m.insert(
        "ndb.edit_warn".into(),
        "写入需要显式确认：write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into(),
    );

    m.insert("ndb.edit_failed".into(), "开启编辑模式失败：{}".into());

    m.insert("ndb.edit_off".into(), "编辑模式已关闭（只读）".into());

    m.insert("ndb.edit_status".into(), "编辑模式：{}".into());

    m.insert("ndb.edit_state_on".into(), "开".into());

    m.insert("ndb.edit_state_off".into(), "关".into());

    m.insert("ndb.edit_usage".into(), "用法：edit on|off".into());

    m.insert(
        "ndb.write_needs_edit".into(),
        "写入已禁用：请先运行 `edit on`".into(),
    );

    m.insert(
        "ndb.write_usage".into(),
        "用法：write YES <INSERT|UPDATE|DELETE|REPLACE ...>".into(),
    );

    m.insert(
        "ndb.write_blocked".into(),
        "`write` 只允许 INSERT、UPDATE、DELETE 和 REPLACE".into(),
    );

    m.insert("ndb.write_confirm".into(), "将要执行：{}".into());

    m.insert(
        "ndb.write_confirm_hint".into(),
        "用 YES 重新运行以确认：write YES <语句>".into(),
    );

    m.insert("ndb.write_done".into(), "写入成功：影响 {} 行".into());

    m.insert("ndb.write_failed".into(), "写入失败：{}".into());

    m.insert("ndb.backup_done".into(), "备份已保存：{}".into());

    m.insert("ndb.backup_failed".into(), "备份失败：{}".into());
}
