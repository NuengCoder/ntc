use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "clap.arg.cared.help".into(),
        "停止忽略一个或多个目录".into(),
    );

    m.insert(
        "clap.arg.caref.help".into(),
        "关注一个或多个文件扩展名".into(),
    );

    m.insert(
        "clap.arg.caren.help".into(),
        "关注一个或多个特定文件".into(),
    );

    m.insert("clap.arg.clear.help".into(), "清除终端屏幕".into());

    m.insert(
        "clap.arg.copy.help".into(),
        "将报告复制到剪贴板而非保存到文件".into(),
    );

    m.insert(
        "clap.arg.edit.help".into(),
        "在内置文本编辑器中打开文件".into(),
    );

    m.insert(
        "clap.arg.explorer.help".into(),
        "启动NTC文件浏览器TUI（可指定起始路径）".into(),
    );

    m.insert(
        "clap.arg.format_cli.help".into(),
        "输出格式：txt, html, json, md".into(),
    );

    m.insert(
        "clap.arg.generate_completions.help".into(),
        "生成Shell补全脚本（bash|zsh|fish|powershell）".into(),
    );

    m.insert("clap.arg.help_arg.help".into(), "显示详细帮助".into());

    m.insert(
        "clap.arg.hyper.help".into(),
        "创建hyper项目脚手架（DSA库）".into(),
    );

    m.insert(
        "clap.arg.igcare_export_all.help".into(),
        "将所有忽略/关注设置导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_export_select.help".into(),
        "选择要导出的类别到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "clap.arg.igcare_import.help".into(),
        "从 .ntc.igcare 文件导入忽略/关注设置".into(),
    );

    m.insert(
        "clap.arg.ignore.help".into(),
        "忽略一个或多个目录名称".into(),
    );

    m.insert("clap.arg.ignored.help".into(), "显示已忽略的项目".into());

    m.insert(
        "clap.arg.ignoref.help".into(),
        "忽略一个或多个文件扩展名".into(),
    );

    m.insert(
        "clap.arg.ignoren.help".into(),
        "忽略一个或多个特定文件".into(),
    );

    m.insert(
        "clap.arg.image.help".into(),
        "将图像转换为文档（JSON, MD, HTML, TXT）".into(),
    );

    m.insert(
        "clap.arg.image_to.help".into(),
        "图像转换的输出格式：json, md, html, txt".into(),
    );

    m.insert(
        "clap.arg.init.help".into(),
        "从模板创建入门文件并在编辑器中打开".into(),
    );

    m.insert("clap.arg.input.help".into(), "输入文件或目录路径".into());

    m.insert("clap.arg.lang_cli.help".into(), "hyper项目的语言".into());

    m.insert("clap.arg.list.help".into(), "列出所有命令行功能".into());

    m.insert(
        "clap.arg.lsp.help".into(),
        "通过stdio启动ntc-math LSP服务器".into(),
    );

    m.insert(
        "clap.arg.lsp_log.help".into(),
        "写入LSP调试日志的路径".into(),
    );

    m.insert("clap.arg.math.help".into(), "计算数学表达式".into());

    m.insert("clap.arg.output.help".into(), "报告的输出文件名".into());

    m.insert(
        "clap.arg.ral_export_all.help".into(),
        "将所有运行别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_export_select.help".into(),
        "选择要导出的运行别名到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "clap.arg.ral_import.help".into(),
        "从 .ntc.ral 文件导入运行别名".into(),
    );

    m.insert("clap.arg.say.help".into(), "将文本输出到stdout".into());

    m.insert(
        "clap.arg.setA.help".into(),
        "显示或设置自动建议（幽灵文本）（开/关）".into(),
    );

    m.insert(
        "clap.arg.setC.help".into(),
        "显示或设置颜色输出（开/关）".into(),
    );

    m.insert("clap.arg.setD.help".into(), "显示或设置最大递归深度".into());

    m.insert(
        "clap.arg.setH.help".into(),
        "显示或设置历史记录（路径/开/关/默认）".into(),
    );

    m.insert(
        "clap.arg.setL.help".into(),
        "显示或设置行号显示（开/关）".into(),
    );

    m.insert("clap.arg.setO.help".into(), "显示或设置默认输出目录".into());

    m.insert("clap.arg.setT.help".into(), "显示或设置线程数".into());

    m.insert("clap.arg.showcg.help".into(), "显示当前配置概览".into());

    m.insert("clap.arg.size_cli.help".into(), "显示当前目录大小".into());

    m.insert(
        "clap.arg.time.help".into(),
        "显示耗时（与 --size 或 --view 配合使用）".into(),
    );

    m.insert(
        "clap.arg.tp_add.help".into(),
        "将当前目录保存为传送点".into(),
    );

    m.insert("clap.arg.tp_list.help".into(), "列出所有传送点".into());

    m.insert("clap.arg.tp_rm.help".into(), "删除传送点".into());

    m.insert(
        "clap.arg.upgrade.help".into(),
        "检查更新并将ntc升级到最新版本".into(),
    );

    m.insert("clap.arg.version.help".into(), "显示版本信息".into());

    m.insert("clap.arg.view_cli.help".into(), "快速查看当前目录树".into());

    m.insert(
        "clap.arg.watch_cli.help".into(),
        "显示或设置文件监视器（开/关）".into(),
    );

    m.insert(
        "clap.arg.where_cli.help".into(),
        "显示ntc可执行文件位置".into(),
    );

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
}
