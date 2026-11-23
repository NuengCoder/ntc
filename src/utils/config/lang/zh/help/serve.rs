use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.serve.title".into(), "SERVE 命令：".into());

    m.insert(
        "help.serve.service".into(),
        "serve <service> ## 选择服务（单独 `serve` 列出全部）".into(),
    );

    m.insert(
        "help.serve.chess".into(),
        "serve chess [--online] ## 国际象棋 hotseat/AI/教练（--online：大厅 + 房间）".into(),
    );

    m.insert(
        "help.serve.git".into(),
        "serve git [目录] ## Git 图形界面：状态、diff、暂存、提交、分支、标签、stash、push/pull"
            .into(),
    );

    m.insert(
        "help.serve.arc".into(),
        "serve arc [目录] ## 压缩包图形界面：创建、查看和解压".into(),
    );

    m.insert(
        "help.serve.ndb".into(),
        "serve ndb ## 浏览 SQLite：选择此文件夹中的数据库、查看结构/数据，可选 --write".into(),
    );

    m.insert(
        "help.serve.theme".into(),
        "serve theme ## 主题图形界面：编辑、预览、保存与应用主题".into(),
    );

    m.insert(
        "help.serve.fake".into(),
        "serve fake ## 假数据界面：类型或已保存分组，预览 txt/json/csv/sql".into(),
    );

    m.insert(
        "help.serve.config".into(),
        "serve config ## 配置界面：设置与服务默认值（命令行编辑器仍在 config serve）".into(),
    );

    m.insert(
        "help.serve.backup".into(),
        "serve backup [路径] ## 备份界面：创建、对比、恢复、撤销、清空此文件夹".into(),
    );

    m.insert(
        "help.serve.example1".into(),
        "serve files --port 8080 ## 在端口 8080 启动文件网盘服务器".into(),
    );
}
