use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ndb.title".into(), "NDB（数据库）命令：".into());

    m.insert(
        "help.ndb.launch".into(),
        "ndb ## 在当前目录打开 ndb 数据库外壳（.db 浏览器）".into(),
    );

    m.insert(
        "help.ndb.root_help".into(),
        "help ## 显示所有 ndb 命令（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.root_view".into(),
        "view | ls ## 列出可用的 .db 文件（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.root_interface".into(),
        "interface <文件.db> ## 打开数据库：提示符变为 ndb [文件.db]>".into(),
    );

    m.insert(
        "help.ndb.root_sql".into(),
        "sql <查询> ## 打开数据库后执行只读查询".into(),
    );

    m.insert(
        "help.ndb.root_exit".into(),
        "exit ## 关闭数据库，然后离开 ndb".into(),
    );

    m.insert(
        "help.ndb.db_viewf".into(),
        "viewf | v ## 显示表和列的结构树".into(),
    );

    m.insert(
        "help.ndb.db_tables".into(),
        "tables | t ## 列出表和视图名称".into(),
    );

    m.insert(
        "help.ndb.db_tablef".into(),
        "tablef <表> ## 显示表的行（类似 txtf）".into(),
    );

    m.insert(
        "help.ndb.db_schema".into(),
        "schema <表> ## 显示 CREATE TABLE SQL 和列详情".into(),
    );

    m.insert(
        "help.ndb.db_sql".into(),
        "sql <查询> ## 执行只读查询（-l N 行、--all）".into(),
    );

    m.insert(
        "help.ndb.db_write".into(),
        "write YES <语句> ## 执行一条写入，需先 edit on（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.db_edit".into(),
        "edit on|off ## 允许或禁止写入（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.db_backup".into(),
        "backup ## 在旁边复制一份 .db 文件（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.more".into(),
        "more ## 显示上次查询的下一页（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.indexes".into(),
        "indexes [表] ## 列出索引（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.info".into(),
        "info ## 显示数据库文件、引擎和对象统计（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.dbal".into(),
        "dbal ## 管理已保存的只读查询：add|run|list|rm（在 ndb 内）".into(),
    );

    m.insert(
        "help.ndb.db_exit".into(),
        "exit ## 关闭数据库并返回 ndb [目录]>".into(),
    );

    m.insert("help.ndb.readonly".into(), "只读 ## 写入被阻止".into());

    m.insert(
        "help.ndb.example1".into(),
        "ndb ## 然后：interface users.db".into(),
    );

    m.insert(
        "help.ndb.example2".into(),
        "ndb ## 然后：tablef users --all".into(),
    );
}
