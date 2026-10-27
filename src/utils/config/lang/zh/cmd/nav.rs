use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.nav.back.invalid_number".into(),
        "无效数字：{}。用法：back [n]".into(),
    );

    m.insert(
        "cmd.nav.back.invalid_step".into(),
        "第 {} 步空父目录 - 无处可返回".into(),
    );

    m.insert("cmd.nav.cd.example1".into(), "示例：cd C:\\Users".into());

    m.insert("cmd.nav.cd.example2".into(), " cd subdir".into());

    m.insert(
        "cmd.nav.cd.example3".into(),
        " cd to work # 传送到 'work' 保存点".into(),
    );

    m.insert("cmd.nav.cd.navigated".into(), "已导航至：{}".into());

    m.insert(
        "cmd.nav.cd.tp_list".into(),
        "使用 'tp list' 查看所有保存点。".into(),
    );

    m.insert(
        "cmd.nav.cd.tp_not_found".into(),
        "未找到传送点：'{}'".into(),
    );

    m.insert("cmd.nav.cd.usage1".into(), "用法：cd <目录路径>".into());

    m.insert(
        "cmd.nav.cd.usage2".into(),
        " cd to <传送点名称> 传送到保存点".into(),
    );

    m.insert(
        "cmd.nav.drive_windows_only".into(),
        "驱动器导航仅在 Windows 上支持。".into(),
    );

    m.insert("cmd.nav.fgo.cancel".into(), "0. 取消".into());

    m.insert("cmd.nav.fgo.cancelled".into(), "已取消。".into());

    m.insert(
        "cmd.nav.fgo.example1".into(),
        " fgo main.rs # 搜索然后选择文件导航到其父目录".into(),
    );

    m.insert(
        "cmd.nav.fgo.example2".into(),
        " fgo main.rs -d 5 # 最多搜索 5 层深度".into(),
    );

    m.insert(
        "cmd.nav.fgo.found".into(),
        "\u{1f50d} 找到 {} 个匹配 \"{}\" 的文件：".into(),
    );

    m.insert(
        "cmd.nav.fgo.select".into(),
        "选择要导航到其父目录的文件：".into(),
    );

    m.insert(
        "cmd.nav.fgo_usage".into(),
        "用法：fgo <模式> [-d <深度>]".into(),
    );

    m.insert("cmd.nav.gos.empty".into(), "（无子目录）".into());

    m.insert("cmd.nav.gos.exit".into(), "退出".into());

    m.insert("cmd.nav.gos.staying".into(), "停留在此处。".into());

    m.insert("cmd.nav.invalid_drive".into(), "无效的驱动器编号。".into());

    m.insert("cmd.nav.invalid_input".into(), "无效的输入。".into());

    m.insert("cmd.nav.invalid_number".into(), "无效的数字。".into());

    m.insert("cmd.nav.invalid_selection".into(), "无效的选择。".into());

    m.insert(
        "cmd.nav.where.config_file".into(),
        "\u{2699}\u{fe0f} 配置文件：".into(),
    );

    m.insert(
        "cmd.nav.where.config_missing".into(),
        "\u{26a0} 未找到配置文件（将在首次保存时创建）".into(),
    );

    m.insert(
        "cmd.nav.where.config_size".into(),
        "\u{1f4cf} 配置大小：".into(),
    );

    m.insert(
        "cmd.nav.where.current_dir".into(),
        "\u{1f4c2} 当前目录：".into(),
    );

    m.insert(
        "cmd.nav.where.executable".into(),
        "\u{1f4c1} 可执行文件：".into(),
    );

    m.insert(
        "cmd.nav.where.not_found".into(),
        "\u{2717} 在 PATH 中未找到 '{}'".into(),
    );

    m.insert(
        "cmd.nav.where.path_title".into(),
        "\u{1f50d} '{}' 在 PATH 中的匹配项：".into(),
    );

    m.insert("cmd.nav.where.title".into(), "ntc 位置信息".into());
}
