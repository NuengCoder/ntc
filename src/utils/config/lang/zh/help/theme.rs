use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.theme.add".into(),
        "theme add <名称> ## 创建新主题（在编辑器中打开）".into(),
    );

    m.insert(
        "help.theme.current".into(),
        "theme current ## 显示当前主题名称和详情".into(),
    );

    m.insert(
        "help.theme.edit".into(),
        "theme edit [名称] ## 编辑主题（或打开主题目录）".into(),
    );

    m.insert(
        "help.theme.export".into(),
        "theme export <名称> ## 导出主题为 .ntc_theme 文件".into(),
    );

    m.insert(
        "help.theme.import".into(),
        "theme import <文件> ## 从 .ntc_theme 文件导入主题".into(),
    );

    m.insert(
        "help.theme.info".into(),
        "theme info <名称> ## 显示主题元数据和颜色数量".into(),
    );

    m.insert(
        "help.theme.interactive".into(),
        "theme ## 交互式主题管理器（菜单）".into(),
    );

    m.insert(
        "help.theme.list".into(),
        "theme list ## 列出所有可用主题".into(),
    );

    m.insert(
        "help.theme.make".into(),
        "theme make [名称] ## 在当前目录创建主题模板".into(),
    );

    m.insert(
        "help.theme.reload".into(),
        "theme reload ## 从磁盘重新加载主题".into(),
    );

    m.insert("help.theme.rm".into(), "theme rm <名称> ## 删除主题".into());

    m.insert(
        "help.theme.rnm".into(),
        "theme rnm <旧名称> to <新名称> ## 重命名主题".into(),
    );

    m.insert(
        "help.theme.switch".into(),
        "theme <名称> ## 按名称切换主题".into(),
    );

    m.insert("help.theme.title".into(), "主题命令：".into());

    m.insert(
        "help.theme.where".into(),
        "theme where ## 显示主题目录位置".into(),
    );

    m.insert(
        "help.theme.example1".into(),
        "theme list ## 列出所有可用主题".into(),
    );

    m.insert(
        "help.theme.example2".into(),
        "theme dark ## 切换到 'dark' 主题".into(),
    );

    m.insert(
        "help.theme.example3".into(),
        "theme current ## 显示当前主题信息".into(),
    );

    m.insert(
        "help.theme.example4".into(),
        "theme add mytheme ## 创建新主题".into(),
    );
}
