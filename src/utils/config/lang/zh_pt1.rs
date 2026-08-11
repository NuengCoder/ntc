use std::collections::HashMap;

pub fn build_map() -> HashMap<String, String> {
    let mut m = HashMap::new();

    // ========== CLI Logo ==========
    m.insert("cli.logo.letters_format".into(), "  {}  {}  {}".into());
    m.insert("cli.logo.n".into(), "N".into());
    m.insert("cli.logo.t".into(), "T".into());
    m.insert("cli.logo.c".into(), "C".into());
    m.insert("cli.logo.subtitle_format".into(), "  {} {} {}".into());
    m.insert("cli.logo.navigate".into(), "导航".into());
    m.insert("cli.logo.toolkit".into(), "工具包".into());
    m.insert("cli.logo.center".into(), "中心".into());

    // ========== CLI Help ==========
    m.insert("cli.help.title".into(), "ntc <version> \u{2014} 导航, 工具包, 中心".into());
    m.insert("cli.help.description".into(), "集导航、项目工具包中心于一体。\n".into());

    // ========== CLI Usage ==========
    m.insert("cli.usage.header".into(), "用法：".into());
    m.insert("cli.usage.basic".into(), "    ntc [选项]".into());
    m.insert("cli.usage.input".into(), "    ntc -i <路径> [-o <输出>]".into());
    m.insert("cli.usage.clipboard".into(), "    ntc -i <路径> --cp              复制报告到剪贴板\n".into());

    // ========== CLI Sections ==========
    m.insert("cli.section.report".into(), "报告选项：".into());
    m.insert("cli.section.config".into(), "配置选项：".into());
    m.insert("cli.section.editor".into(), "编辑器选项：".into());
    m.insert("cli.section.igcare".into(), "忽略/关注选项：".into());
    m.insert("cli.section.teleport".into(), "传送保存点：".into());
    m.insert("cli.section.export".into(), "别名 / 忽略/关注导出：".into());
    m.insert("cli.section.other".into(), "其他选项：".into());
    m.insert("cli.section.examples".into(), "示例：".into());
    m.insert("cli.section.shell_commands".into(), "Shell 命令（也可从 CLI 使用）：".into());
    m.insert("cli.section.interactive_only".into(), "仅交互模式（运行 'ntc' 无参数）：".into());

    // ========== CLI Flags ==========
    m.insert("cli.flag.input".into(), "    -i, --input <路径>      处理文件或目录".into());
    m.insert("cli.flag.output".into(), "    -o, --output <文件>     保存输出到指定文件".into());
    m.insert("cli.flag.format".into(), "    -f, --format <格式>   输出格式：txt, html, json, md, pdf, docx, xlsx（默认：txt）".into());
    m.insert("cli.flag.copy".into(), "    --cp                    复制报告到剪贴板而非保存到文件".into());
    m.insert("cli.flag.size".into(), "    --size                  显示当前目录大小".into());
    m.insert("cli.flag.view".into(), "    --view                  快速查看当前目录树".into());
    m.insert("cli.flag.view_size".into(), "    --view --size           快速查看带目录大小\n".into());
    m.insert("cli.flag.setO".into(), "    --setO [路径]           显示或设置输出目录（默认：桌面）".into());
    m.insert("cli.flag.setD".into(), "    --setD [深度]          显示或设置最大递归深度（最小：1，最大：20）".into());
    m.insert("cli.flag.setL".into(), "    --setL [开|关]         显示或切换文件显示行号".into());
    m.insert("cli.flag.setC".into(), "    --setC [开|关]         显示或切换颜色输出".into());
    m.insert("cli.flag.setT".into(), "    --setT [线程数]        显示或设置线程数（默认：4）".into());
    m.insert("cli.flag.setH".into(), "    --setH [路径|开|关]    显示/设置历史路径或启用/禁用".into());
    m.insert("cli.flag.setA".into(), "    --setA [开|关]         显示/设置自动建议（幽灵文本）".into());
    m.insert("cli.flag.showcg".into(), "    --showcg                显示当前配置概览".into());
    m.insert("cli.flag.watch_flag".into(), "    --watch [开|关]        显示/设置文件监视器状态".into());
    m.insert("cli.flag.where".into(), "    --where                 显示 ntc 可执行文件和配置位置\n".into());
    m.insert("cli.flag.edit".into(), "    -e, --edit <文件>       在内置文本编辑器中打开文件".into());
    m.insert("cli.flag.init".into(), "    --init <文件>           创建入门模板并在编辑器中打开".into());
    m.insert("cli.flag.hyper".into(), "    --hyper <名称> --lang c 创建 C/C++ 超项目脚手架\n".into());
    m.insert("cli.flag.ignored".into(), "    --ignored               显示当前忽略的项目".into());
    m.insert("cli.flag.ignore".into(), "    --ignore <名称>         忽略目录名".into());
    m.insert("cli.flag.cared".into(), "    --cared <名称>          停止忽略目录名".into());
    m.insert("cli.flag.ignoref".into(), "    --ignoref <扩展名>     忽略文件扩展名".into());
    m.insert("cli.flag.caref".into(), "    --caref <扩展名>        关注文件扩展名".into());
    m.insert("cli.flag.ignoren".into(), "    --ignoren <文件>       忽略特定文件（例如：Cargo.lock）".into());
    m.insert("cli.flag.caren".into(), "    --caren <文件>          关注特定文件\n".into());
    m.insert("cli.flag.tp_add".into(), "    --tp-add <名称>         保存当前目录为传送点".into());
    m.insert("cli.flag.tp_list".into(), "    --tp-list               列出所有传送点".into());
    m.insert("cli.flag.tp_rm".into(), "    --tp-rm <名称>          删除传送点\n".into());
    m.insert("cli.flag.ral_export_all".into(), "    --ral-export-all <名称>     导出所有运行别名到 <名称>.ntc.ral".into());
    m.insert("cli.flag.ral_export_select".into(), "    --ral-export-select <名称>  选择要导出的别名".into());
    m.insert("cli.flag.ral_import".into(), "    --ral-import <文件>         从 .ntc.ral 文件导入别名".into());
    m.insert("cli.flag.igcare_export_all".into(), "    --igcare-export-all <名称>  导出所有忽略/关注设置".into());
    m.insert("cli.flag.igcare_export_select".into(), "    --igcare-export-select <名称> 选择要导出的类别".into());
    m.insert("cli.flag.igcare_import".into(), "    --igcare-import <文件>      从 .ntc.igcare 文件导入设置\n".into());
    m.insert("cli.flag.say".into(), "    -say, -print <文本>     打印文本到标准输出".into());
    m.insert("cli.flag.image".into(), "    -image <路径> to <格式>  转换图像为 AI 用文档（json|md|html|txt）".into());
    m.insert("cli.flag.explorer".into(), "    -x, --explorer [路径]   启动 NTC 文件浏览器 TUI".into());
    m.insert("cli.flag.dino".into(), "    --dino                  玩恐龙跑步游戏".into());
    m.insert("cli.flag.clear".into(), "    --clear                 清除终端屏幕".into());
    m.insert("cli.flag.version".into(), "    --version               显示版本信息".into());
    m.insert("cli.flag.math".into(), "    --math <表达式>          计算数学表达式（例如：--math \"3+4*5\"）".into());
    m.insert("cli.flag.list".into(), "    --list, --fun           列出所有命令行函数".into());
    m.insert("cli.flag.help_flag".into(), "    --help                  显示此帮助信息".into());
    m.insert("cli.flag.generate_completions".into(), "    --generate-completions <SHELL>  生成补全脚本（bash|zsh|fish|powershell）\n".into());

