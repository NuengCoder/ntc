use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.igcare.care_dir".into(),
        "care dir <名称>[, ...] ## 停止忽略目录".into(),
    );

    m.insert(
        "help.igcare.care_ext".into(),
        "care ext <扩展名>[, ...] ## 添加额外支持的扩展名".into(),
    );

    m.insert(
        "help.igcare.care_file".into(),
        "care file <名称>[, ...] ## 添加额外支持的文件".into(),
    );

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## 将所有设置导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## 选择要导出的类别".into(),
    );

    m.insert(
        "help.igcare.ignore_dir".into(),
        "ignore dir <名称>[, ...] ## 按名称忽略目录".into(),
    );

    m.insert(
        "help.igcare.ignore_ext".into(),
        "ignore ext <扩展名>[, ...] ## 忽略文件扩展名".into(),
    );

    m.insert(
        "help.igcare.ignore_file".into(),
        "ignore file <名称>[, ...] ## 忽略特定文件".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <文件> ## 从 .ntc.igcare 文件导入设置".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [名称] ## 在当前目录创建 igcare 模板".into(),
    );

    m.insert(
        "help.igcare.show".into(),
        "igcared ## 显示所有忽略/关注设置".into(),
    );

    m.insert("help.igcare.title".into(), "忽略 / 关注命令：".into());

    m.insert(
        "help.igcare.example1".into(),
        "ignore dir node_modules ## 忽略 node_modules".into(),
    );

    m.insert(
        "help.igcare.example2".into(),
        "ignore ext .log ## 忽略 .log 文件".into(),
    );

    m.insert(
        "help.igcare.example3".into(),
        "care dir src ## 取消忽略 src 目录".into(),
    );

    m.insert(
        "help.igcare.example4".into(),
        "igcared ## 显示所有忽略/关注设置".into(),
    );
}
