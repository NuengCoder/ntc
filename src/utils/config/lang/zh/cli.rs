use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cli.completions.shell.bash".into(), "bash".into());

    m.insert("cli.completions.shell.fish".into(), "fish".into());

    m.insert(
        "cli.completions.shell.powershell".into(),
        "powershell".into(),
    );

    m.insert("cli.completions.shell.zsh".into(), "zsh".into());

    m.insert(
        "cli.example.cat_file".into(),
        " ntc -i file.txt 显示文件内容".into(),
    );

    m.insert(
        "cli.example.image".into(),
        " ntc -image screenshot.png to json 将图像转换为JSON以供AI使用\n".into(),
    );

    m.insert(
        "cli.example.interactive".into(),
        " ntc 启动交互式Shell".into(),
    );

    m.insert(
        "cli.example.math_arithmetic".into(),
        " ntc --math \"3+4*5\" 计算数学表达式".into(),
    );

    m.insert(
        "cli.example.math_func".into(),
        " ntc --math \"sin(PI/2)\" 使用函数和常量的数学计算".into(),
    );

    m.insert(
        "cli.example.report_cp".into(),
        " ntc -i src --cp 将目录树复制到剪贴板".into(),
    );

    m.insert(
        "cli.example.report_docx".into(),
        " ntc -i src -f docx 生成DOCX报告".into(),
    );

    m.insert(
        "cli.example.report_json_cp".into(),
        " ntc -i src -f json --cp 将JSON报告复制到剪贴板".into(),
    );

    m.insert(
        "cli.example.report_pdf".into(),
        " ntc -i src -f pdf 从src生成PDF报告".into(),
    );

    m.insert(
        "cli.example.report_txt".into(),
        " ntc -i src 生成src目录的TXT报告".into(),
    );

    m.insert(
        "cli.example.report_xlsx".into(),
        " ntc -i src -f xlsx 生成XLSX报告（含依赖分析）".into(),
    );

    m.insert(
        "cli.example.say".into(),
        " ntc -say \"Hello World\" 打印Hello World".into(),
    );

    m.insert("cli.example.setL".into(), " ntc --setL 开 启用行号".into());

    m.insert("cli.example.showcg".into(), " ntc --showcg 显示配置".into());

    m.insert(
        "cli.example.teleport_at".into(),
        " ntc @web 启动并传送至 'web' 保存点".into(),
    );

    m.insert(
        "cli.example.watch".into(),
        " ntc --watch 开 启用文件监视器".into(),
    );

    m.insert(
        "cli.flag.cared".into(),
        " --cared <名称> 停止忽略目录名称".into(),
    );

    m.insert(
        "cli.flag.caref".into(),
        " --caref <扩展名> 关注文件扩展名".into(),
    );

    m.insert(
        "cli.flag.caren".into(),
        " --caren <文件> 关注特定文件\n".into(),
    );

    m.insert("cli.flag.clear".into(), " --clear 清除终端屏幕".into());

    m.insert(
        "cli.flag.copy".into(),
        " --cp 将报告复制到剪贴板而非保存到文件".into(),
    );

    m.insert(
        "cli.flag.edit".into(),
        " -e, --edit <文件> 在内置文本编辑器中打开文件".into(),
    );

    m.insert(
        "cli.flag.explorer".into(),
        " -x, --explorer [路径] 启动NTC文件浏览器TUI".into(),
    );

    m.insert(
        "cli.flag.format".into(),
        " -f, --format <格式> 输出格式：txt, html, json, md, pdf, docx, xlsx（默认：txt）".into(),
    );

    m.insert(
        "cli.flag.generate_completions".into(),
        " --generate-completions <Shell> 生成自动补全（bash|zsh|fish|powershell）\n".into(),
    );

    m.insert("cli.flag.help_flag".into(), " --help 显示此帮助信息".into());

    m.insert(
        "cli.flag.hyper".into(),
        " --hyper <名称> --lang c 创建C/C++ hyper项目脚手架\n".into(),
    );

    m.insert(
        "cli.flag.igcare_export_all".into(),
        " --igcare-export-all <名称> 导出所有忽略/关注设置".into(),
    );

    m.insert(
        "cli.flag.igcare_export_select".into(),
        " --igcare-export-select <名称> 选择要导出的类别".into(),
    );

    m.insert(
        "cli.flag.igcare_import".into(),
        " --igcare-import <文件> 从 .ntc.igcare 文件导入设置\n".into(),
    );

    m.insert(
        "cli.flag.ignore".into(),
        " --ignore <名称> 忽略目录名称".into(),
    );

    m.insert(
        "cli.flag.ignored".into(),
        " --ignored 显示当前忽略的项目".into(),
    );

    m.insert(
        "cli.flag.ignoref".into(),
        " --ignoref <扩展名> 忽略文件扩展名".into(),
    );

    m.insert(
        "cli.flag.ignoren".into(),
        " --ignoren <文件> 忽略特定文件（如 Cargo.lock）".into(),
    );

    m.insert(
        "cli.flag.image".into(),
        " -image <路径> to <格式> 将图像转换为AI可用的文档（json|md|html|txt）".into(),
    );

    m.insert(
        "cli.flag.init".into(),
        " --init <文件> 创建入门模板并在编辑器中打开".into(),
    );

    m.insert(
        "cli.flag.input".into(),
        " -i, --input <路径> 处理文件或目录".into(),
    );

    m.insert(
        "cli.flag.list".into(),
        " --list, --fun 列出所有命令行功能".into(),
    );

    m.insert(
        "cli.flag.math".into(),
        " --math <表达式> 计算数学表达式（如 --math \"3+4*5\"）".into(),
    );

    m.insert(
        "cli.flag.output".into(),
        " -o, --output <文件> 将输出保存到指定文件".into(),
    );

    m.insert(
        "cli.flag.ral_export_all".into(),
        " --ral-export-all <名称> 将所有运行别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "cli.flag.ral_export_select".into(),
        " --ral-export-select <名称> 选择要导出的别名".into(),
    );

    m.insert(
        "cli.flag.ral_import".into(),
        " --ral-import <文件> 从 .ntc.ral 文件导入别名".into(),
    );

    m.insert(
        "cli.flag.say".into(),
        " -say, -print <文本> 将文本输出到stdout".into(),
    );

    m.insert(
        "cli.flag.setA".into(),
        " --setA [on|off] 显示/设置自动建议（幽灵文本）".into(),
    );

    m.insert(
        "cli.flag.setC".into(),
        " --setC [on|off] 显示或切换颜色输出".into(),
    );

    m.insert(
        "cli.flag.setD".into(),
        " --setD [深度] 显示或设置最大递归深度（最小：1，最大：20）".into(),
    );

    m.insert(
        "cli.flag.setH".into(),
        " --setH [路径|on|off] 显示/设置历史路径或启用/禁用".into(),
    );

    m.insert(
        "cli.flag.setL".into(),
        " --setL [on|off] 显示或切换文件显示的行号".into(),
    );

    m.insert(
        "cli.flag.setO".into(),
        " --setO [路径] 显示或设置输出目录（默认：桌面）".into(),
    );

    m.insert(
        "cli.flag.setT".into(),
        " --setT [线程数] 显示或设置线程数（默认：4）".into(),
    );

    m.insert(
        "cli.flag.showcg".into(),
        " --showcg 显示当前配置概览".into(),
    );

    m.insert("cli.flag.size".into(), " --size 显示当前目录大小".into());

    m.insert(
        "cli.flag.tp_add".into(),
        " --tp-add <名称> 将当前目录保存为传送点".into(),
    );

    m.insert(
        "cli.flag.tp_list".into(),
        " --tp-list 列出所有传送点".into(),
    );

    m.insert(
        "cli.flag.tp_rm".into(),
        " --tp-rm <名称> 删除传送点\n".into(),
    );

    m.insert("cli.flag.version".into(), " --version 显示版本信息".into());

    m.insert("cli.flag.view".into(), " --view 快速查看当前目录树".into());

    m.insert(
        "cli.flag.view_size".into(),
        " --view --size 快速查看含目录大小\n".into(),
    );

    m.insert(
        "cli.flag.watch_flag".into(),
        " --watch [on|off] 显示/设置文件监视器状态".into(),
    );

    m.insert(
        "cli.flag.where".into(),
        " --where 显示ntc可执行文件和配置位置\n".into(),
    );

    m.insert(
        "cli.help.description".into(),
        "导航、项目工具包中心的结合体。\n".into(),
    );

    m.insert(
        "cli.help.title".into(),
        "ntc <版本> \u{2014} 导航, 工具包, 中心".into(),
    );

    m.insert(
        "cli.helpers.basic_usage".into(),
        " ntc [选项]\n ntc -i <路径> [-o <输出>]\n ntc -i <路径> --cp 复制报告到剪贴板\n".into(),
    );

    m.insert("cli.helpers.cli_examples".into(), " ntc 启动交互式 shell\n ntc @web 启动并传送到 'web' 保存点\n ntc --math \"3+4*5\" 计算数学表达式\n ntc --math \"sin(PI/2)\" 带函数和常量的数学计算\n ntc -i src 生成 src 目录的 TXT 报告\n ntc -i src --cp 复制目录树到剪贴板\n ntc -i src -f json --cp 复制 JSON 报告到剪贴板\n ntc -i src -f pdf 从 src 生成 PDF 报告\n ntc -i src -f docx 从 src 生成 DOCX 报告\n ntc -i src -f xlsx 从 src 生成 XLSX 报告（带依赖分析）\n ntc -i file.txt 显示文件内容\n ntc --setL ON 启用行号\n ntc --showcg 显示配置\n ntc -say \"Hello World\" 打印 Hello World\n ntc --watch ON 启用文件监视器\n ntc -image screenshot.png to json 转换图像为 JSON 用于 AI\n".into());

    m.insert("cli.helpers.config_header".into(), "配置选项：".into());

    m.insert("cli.helpers.config_options".into(), " --setO [路径] 显示或设置输出目录（默认：桌面）\n --setD [深度] 显示或设置最大递归深度（最小：1，最大：20）\n --setL [on|off] 显示或切换行号\n --setC [on|off] 显示或切换颜色输出\n --setT [线程数] 显示或设置线程数（默认：4）\n --setH [路径|on|off] 显示/设置历史路径或启用/禁用\n --setA [on|off] 显示/设置自动建议（幽灵文本）\n --showcg 显示当前配置概览\n --watch [on|off] 显示/设置文件监视器状态\n --where 显示 ntc 可执行文件和配置位置\n".into());

    m.insert("cli.helpers.editor_header".into(), "编辑器选项：".into());

    m.insert("cli.helpers.editor_options".into(), " -e, --edit <文件> 在内置文本编辑器中打开文件\n --init <文件> 创建起始模板并在编辑器中打开\n --hyper <名称> --lang c 创建 C/C++ hyper 项目脚手架\n".into());

    m.insert("cli.helpers.examples_header".into(), "示例：".into());

    m.insert(
        "cli.helpers.export_header".into(),
        "别名 / 忽略/关注导出：".into(),
    );

    m.insert("cli.helpers.export_options".into(), " --ral-export-all <名称> 导出所有运行别名为 <名称>.ntc.ral\n --ral-export-select <名称> 选择要导出的别名\n --ral-import <文件> 从 .ntc.ral 文件导入别名\n --igcare-export-all <名称> 导出所有忽略/关注设置\n --igcare-export-select <名称> 选择要导出的类别\n --igcare-import <文件> 从 .ntc.igcare 文件导入设置\n".into());

    m.insert("cli.helpers.igcare_options".into(), " --ignored 显示当前忽略的项目\n --ignore <名称> 忽略一个目录名称\n --cared <名称> 停止忽略一个目录名称\n --ignoref <扩展名> 忽略一个文件扩展名\n --caref <扩展名> 关注一个文件扩展名\n --ignoren <文件> 忽略一个特定文件（例如 Cargo.lock）\n --caren <文件> 关注一个特定文件\n".into());

    m.insert("cli.helpers.ignore_header".into(), "忽略/关注选项：".into());

    m.insert("cli.helpers.interactive_note".into(), "仅交互模式（运行 'ntc' 不含参数）：\n 导航：cd, back, gos, gosc\n 传送：tp jump, tp to, @name\n 教程：tutorial \u{2014} 开始交互式分步教程\n".into());

    m.insert("cli.helpers.other_header".into(), "其他选项：".into());

    m.insert("cli.helpers.other_options".into(), " -say, -print <文本> 打印文本到 stdout\n -image <路径> to <格式> 转换图像为 AI 文档（json|md|html|txt）\n -x, --explorer [路径] 启动 NTC 文件资源管理器 TUI\n --clear 清除终端屏幕\n --version 显示版本信息\n --math <表达式> 计算数学表达式（例如 --math \"3+4*5\"）\n --list, --fun 列出所有命令行功能\n --help 显示此帮助信息\n --generate-completions <SHELL> 生成补全（bash|zsh|fish|powershell）\n".into());

    m.insert("cli.helpers.report_header".into(), "报告选项：".into());

    m.insert("cli.helpers.report_options".into(), " -i, --input <路径> 处理文件或目录\n -o, --output <文件> 保存输出到指定文件\n -f, --format <格式> 输出格式：txt, html, json, md, pdf, docx, xlsx（默认：txt）\n --cp 复制报告到剪贴板而非保存到文件\n --size 显示当前目录大小\n --view 快速查看当前目录树\n --view --size 快速查看带目录大小\n".into());

    m.insert("cli.helpers.shell_commands".into(), " ntc pathcp 复制路径到剪贴板\n ntc say \"hello\" --loop 3 循环打印文本\n ntc tl dir . 统计目录中的行数\n ntc ne 打开文本编辑器\n ntc size 显示目录大小\n ntc view 显示目录树\n ntc serve files --port 8080 在端口 8080 启动文件网盘服务器\n ntc dl https://example.com/file.zip --progress 带进度下载\n ntc encode b64 hello 对字符串进行 Base64 编码\n ntc qr \"text\" 在终端生成二维码\n ntc math 3+4*5 计算表达式\n ntc ral add btr \"cargo build --release\" 从 CLI 添加别名\n".into());

    m.insert(
        "cli.helpers.shell_header".into(),
        "Shell 命令（也可从 CLI 使用）：".into(),
    );

    m.insert(
        "cli.helpers.tagline".into(),
        "集导航、项目工具包中心于一体。\n".into(),
    );

    m.insert("cli.helpers.tp_header".into(), "传送保存点：".into());

    m.insert("cli.helpers.tp_options".into(), " --tp-add <名称> 将当前目录保存为传送点\n --tp-list 列出所有传送点\n --tp-rm <名称> 删除一个传送点\n".into());

    m.insert("cli.helpers.usage_header".into(), "用法：".into());

    m.insert("cli.new.usage_header".into(), "用法：".into());

    m.insert(
        "cli.new.usage_shell".into(),
        "  ntc                       启动交互式 shell".into(),
    );

    m.insert(
        "cli.new.usage_cmd".into(),
        "  ntc <command> [args...]   非交互运行 shell 命令".into(),
    );

    m.insert(
        "cli.new.usage_new".into(),
        "  ntc -n, --new [command]   打开新终端（在其中运行命令并保持交互）".into(),
    );

    m.insert(
        "cli.new.usage_upgrade".into(),
        "  ntc --upgrade             升级 ntc 到最新版本".into(),
    );

    m.insert(
        "cli.new.usage_version".into(),
        "  ntc --version, -v         显示版本信息".into(),
    );

    m.insert(
        "cli.new.usage_help".into(),
        "  ntc --help, -h            显示此帮助信息".into(),
    );

    m.insert("cli.new.examples_header".into(), "示例：".into());

    m.insert(
        "cli.new.ex_help".into(),
        "  ntc help                  显示 shell 帮助".into(),
    );

    m.insert(
        "cli.new.ex_version".into(),
        "  ntc version               通过 shell 显示版本".into(),
    );

    m.insert(
        "cli.new.ex_alias".into(),
        "  ntc t                     运行 ral 别名 't'（例如 taskmgr）".into(),
    );

    m.insert(
        "cli.new.ex_new".into(),
        "  ntc -n                    打开新的交互式终端".into(),
    );

    m.insert(
        "cli.new.ex_new_cmd".into(),
        "  ntc -n t                  打开新终端，运行 't' 并保持交互".into(),
    );

    m.insert("cli.new.opened".into(), "已打开新终端。".into());

    m.insert(
        "cli.new.no_terminal".into(),
        "未找到终端（已尝试：{}）".into(),
    );

    m.insert(
        "cli.new.unsupported".into(),
        "ntc -n/--new 仅支持 Windows、Linux 和 macOS".into(),
    );

    m.insert(
        "cli.interactive.navigation".into(),
        " 导航：cd, back, gos, gosc".into(),
    );

    m.insert(
        "cli.interactive.teleport".into(),
        " 传送：tp jump, tp to, @名称".into(),
    );

    m.insert(
        "cli.interactive.tutorial".into(),
        " 教程：tutorial \u{2014} 启动交互式分步教程\n".into(),
    );

    m.insert(
        "cli.known_flag.cared".into(),
        "--cared <名称> 停止忽略目录".into(),
    );

    m.insert(
        "cli.known_flag.caref".into(),
        "--caref <扩展名> 关注文件扩展名".into(),
    );

    m.insert(
        "cli.known_flag.caren".into(),
        "--caren <文件> 关注特定文件".into(),
    );

    m.insert("cli.known_flag.clear".into(), "--clear 清除终端屏幕".into());

    m.insert("cli.known_flag.cp".into(), "--cp 将报告复制到剪贴板".into());

    m.insert(
        "cli.known_flag.explorer".into(),
        "-x, --explorer [路径] 启动NTC文件浏览器TUI".into(),
    );

    m.insert(
        "cli.known_flag.generate_completions".into(),
        "--generate-completions <Shell> 生成Shell补全（bash|zsh|fish|powershell）".into(),
    );

    m.insert("cli.known_flag.help_flag".into(), "--help 显示帮助".into());

    m.insert(
        "cli.known_flag.igcare_export_all".into(),
        "--igcare-export-all <名称> 导出所有忽略/关注设置到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "cli.known_flag.igcare_export_select".into(),
        "--igcare-export-select <名称> 选择类别导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "cli.known_flag.igcare_import".into(),
        "--igcare-import <文件> 从 .ntc.igcare 文件导入忽略/关注设置".into(),
    );

    m.insert(
        "cli.known_flag.ignore".into(),
        "--ignore <名称> 忽略目录名称".into(),
    );

    m.insert(
        "cli.known_flag.ignored".into(),
        "--ignored 显示已忽略的项目".into(),
    );

    m.insert(
        "cli.known_flag.ignoref".into(),
        "--ignoref <扩展名> 忽略文件扩展名".into(),
    );

    m.insert(
        "cli.known_flag.ignoren".into(),
        "--ignoren <文件> 忽略特定文件".into(),
    );

    m.insert(
        "cli.known_flag.input".into(),
        "-i, --input <路径> 输入文件或目录".into(),
    );

    m.insert(
        "cli.known_flag.list".into(),
        "--list, --fun 列出所有命令行功能".into(),
    );

    m.insert(
        "cli.known_flag.math".into(),
        "--math <表达式> 计算数学表达式（如 --math \"3+4*5\"）".into(),
    );

    m.insert(
        "cli.known_flag.no_args".into(),
        "（无参数） 启动交互模式".into(),
    );

    m.insert(
        "cli.known_flag.output".into(),
        "-o, --output <文件> 输出文件名".into(),
    );

    m.insert(
        "cli.known_flag.ral_export_all".into(),
        "--ral-export-all <名称> 将所有运行别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "cli.known_flag.ral_export_select".into(),
        "--ral-export-select <名称> 选择运行别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "cli.known_flag.ral_import".into(),
        "--ral-import <文件> 从 .ntc.ral 文件导入运行别名".into(),
    );

    m.insert(
        "cli.known_flag.say".into(),
        "-say, -print <文本> 将文本输出到stdout".into(),
    );

    m.insert(
        "cli.known_flag.setC".into(),
        "--setC [on|off] 显示或设置颜色输出".into(),
    );

    m.insert(
        "cli.known_flag.setD".into(),
        "--setD [深度] 显示或设置最大深度（1-20）".into(),
    );

    m.insert(
        "cli.known_flag.setH".into(),
        "--setH [路径|on|off] 显示或设置历史记录路径/状态".into(),
    );

    m.insert(
        "cli.known_flag.setL".into(),
        "--setL [on|off] 显示或设置行号".into(),
    );

    m.insert(
        "cli.known_flag.setO".into(),
        "--setO [路径] 显示或设置输出目录".into(),
    );

    m.insert(
        "cli.known_flag.setT".into(),
        "--setT [线程数] 显示或设置线程数".into(),
    );

    m.insert(
        "cli.known_flag.showcg".into(),
        "--showcg 显示当前配置概览".into(),
    );

    m.insert(
        "cli.known_flag.size".into(),
        "--size 显示当前目录大小".into(),
    );

    m.insert(
        "cli.known_flag.path".into(),
        "--path <路径|@tp> 目标路径或传送点（与 --size 或 --view 配合使用）".into(),
    );

    m.insert(
        "cli.known_flag.teleport_at".into(),
        "ntc @<名称> 启动并传送到保存点".into(),
    );

    m.insert(
        "cli.known_flag.tp_add".into(),
        "--tp-add <名称> 将当前目录保存为传送点".into(),
    );

    m.insert(
        "cli.known_flag.tp_list".into(),
        "--tp-list 列出所有传送点".into(),
    );

    m.insert(
        "cli.known_flag.tp_rm".into(),
        "--tp-rm <名称> 删除传送点".into(),
    );

    m.insert(
        "cli.known_flag.upgrade".into(),
        "--upgrade 检查更新并升级ntc".into(),
    );

    m.insert(
        "cli.known_flag.version".into(),
        "--version 显示版本信息".into(),
    );

    m.insert(
        "cli.known_flag.view".into(),
        "--view 快速查看当前目录树".into(),
    );

    m.insert(
        "cli.known_flag.view_size".into(),
        "--view --size 快速查看含目录大小".into(),
    );

    m.insert(
        "cli.known_flag.watch".into(),
        "--watch [on|off] 显示或设置文件监视器状态".into(),
    );

    m.insert(
        "cli.known_flag.where".into(),
        "--where 显示ntc可执行文件和配置位置".into(),
    );

    m.insert("cli.logo.c".into(), "C".into());

    m.insert("cli.logo.center".into(), "中心".into());

    m.insert("cli.logo.letters_format".into(), " {} {} {}".into());

    m.insert("cli.logo.n".into(), "N".into());

    m.insert("cli.logo.navigate".into(), "导航".into());

    m.insert("cli.logo.subtitle_format".into(), " {} {} {}".into());

    m.insert("cli.logo.t".into(), "T".into());

    m.insert("cli.logo.toolkit".into(), "工具包".into());

    m.insert(
        "cli.run.about".into(),
        "导航、目录树、Cat - 目录树查看与文件连接工具".into(),
    );

    m.insert("cli.run.help_cared".into(), "cared".into());

    m.insert("cli.run.help_caref".into(), "caref".into());

    m.insert("cli.run.help_caren".into(), "caren".into());

    m.insert("cli.run.help_clear".into(), "清空终端屏幕".into());

    m.insert(
        "cli.run.help_copy".into(),
        "将报告复制到剪贴板而不是保存到文件".into(),
    );

    m.insert(
        "cli.run.help_edit".into(),
        "在内置文本编辑器中打开文件".into(),
    );

    m.insert(
        "cli.run.help_explorer".into(),
        "启动 NTC 文件资源管理器 TUI（可选起始路径）".into(),
    );

    m.insert(
        "cli.run.help_format".into(),
        "输出格式：txt, html, json, md".into(),
    );

    m.insert(
        "cli.run.help_generate_completions".into(),
        "生成 shell 补全脚本（bash|zsh|fish|powershell）".into(),
    );

    m.insert(
        "cli.run.help_helpextra".into(),
        "显示子命令的扩展帮助".into(),
    );

    m.insert(
        "cli.run.help_hyper".into(),
        "创建 hyper 项目脚手架（DSA 库）".into(),
    );

    m.insert(
        "cli.run.help_igcare_export_all".into(),
        "将所有忽略/关注设置导出到 <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.run.help_igcare_export_select".into(),
        "选择类别导出到 <name>.ntc.igcare".into(),
    );

    m.insert(
        "cli.run.help_igcare_import".into(),
        "从 .ntc.igcare 文件导入忽略/关注设置".into(),
    );

    m.insert(
        "cli.run.help_ignore".into(),
        "显示 ntc 可执行文件位置".into(),
    );

    m.insert("cli.run.help_ignored".into(), "ignoren".into());

    m.insert("cli.run.help_ignoref".into(), "ignoref".into());

    m.insert("cli.run.help_ignoren".into(), "ignoren".into());

    m.insert(
        "cli.run.help_image".into(),
        "将图片转换为文档（JSON, MD, HTML, TXT）".into(),
    );

    m.insert(
        "cli.run.help_image_to".into(),
        "图片转换输出格式：json, md, html, txt".into(),
    );

    m.insert(
        "cli.run.help_init".into(),
        "从模板创建起始文件并在编辑器中打开".into(),
    );

    m.insert("cli.run.help_input".into(), "输入文件或目录路径".into());

    m.insert("cli.run.help_lang".into(), "hyper 项目的语言".into());

    m.insert("cli.run.help_list".into(), "列出所有命令行功能".into());

    m.insert(
        "cli.run.help_lsp".into(),
        "通过 stdio 启动 ntc-math LSP 服务器".into(),
    );

    m.insert("cli.run.help_lsp_log".into(), "LSP 调试日志写入路径".into());

    m.insert("cli.run.help_math".into(), "计算数学表达式".into());

    m.insert(
        "cli.run.help_mode".into(),
        "在普通与管理员模式间切换".into(),
    );

    m.insert("cli.run.help_modn".into(), "modn".into());

    m.insert("cli.run.help_output".into(), "报告的输出文件名".into());

    m.insert(
        "cli.run.help_ral_export_all".into(),
        "将所有运行别名导出到 <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.run.help_ral_export_select".into(),
        "选择运行别名导出到 <name>.ntc.ral".into(),
    );

    m.insert(
        "cli.run.help_ral_import".into(),
        "从 .ntc.ral 文件导入运行别名".into(),
    );

    m.insert("cli.run.help_say".into(), "向标准输出打印文本".into());

    m.insert(
        "cli.run.help_seta".into(),
        "显示或设置自动建议（幽灵文本）（ON/OFF）".into(),
    );

    m.insert(
        "cli.run.help_setc".into(),
        "显示或设置彩色输出（ON/OFF）".into(),
    );

    m.insert("cli.run.help_setd".into(), "显示或设置最大递归深度".into());

    m.insert(
        "cli.run.help_seth".into(),
        "显示或设置历史记录（路径/ON/OFF/default）".into(),
    );

    m.insert(
        "cli.run.help_setl".into(),
        "显示或设置行号显示（ON/OFF）".into(),
    );

    m.insert("cli.run.help_seto".into(), "显示或设置默认输出目录".into());

    m.insert("cli.run.help_sett".into(), "显示或设置线程数".into());

    m.insert("cli.run.help_showcg".into(), "显示当前配置总览".into());

    m.insert("cli.run.help_size".into(), "显示文件或目录大小".into());

    m.insert(
        "cli.run.help_path".into(),
        "目标路径或传送点（与 --size 或 --view 配合使用）".into(),
    );

    m.insert(
        "cli.run.help_time".into(),
        "显示耗时（与 --size 或 --view 一起使用）".into(),
    );

    m.insert(
        "cli.run.help_tp_add".into(),
        "将当前目录保存为传送点".into(),
    );

    m.insert("cli.run.help_tp_list".into(), "列出所有传送点".into());

    m.insert("cli.run.help_tp_rm".into(), "删除传送点".into());

    m.insert(
        "cli.run.help_upgrade".into(),
        "检查更新并将 ntc 升级到最新版本".into(),
    );

    m.insert("cli.run.help_version".into(), "显示版本信息".into());

    m.insert("cli.run.help_view".into(), "快速查看当前目录树".into());

    m.insert(
        "cli.run.help_watch".into(),
        "显示或设置文件监视器（ON/OFF）".into(),
    );

    m.insert(
        "cli.run.help_where".into(),
        "显示 ntc 可执行文件位置".into(),
    );

    m.insert("cli.section.config".into(), "配置选项：".into());

    m.insert("cli.section.editor".into(), "编辑器选项：".into());

    m.insert("cli.section.examples".into(), "示例：".into());

    m.insert("cli.section.export".into(), "别名 / 忽略/关注导出：".into());

    m.insert("cli.section.igcare".into(), "忽略/关注选项：".into());

    m.insert(
        "cli.section.interactive_only".into(),
        "仅交互模式（运行 'ntc' 不带参数）：".into(),
    );

    m.insert("cli.section.other".into(), "其他选项：".into());

    m.insert("cli.section.report".into(), "报告选项：".into());

    m.insert(
        "cli.section.shell_commands".into(),
        "Shell命令（也可从CLI使用）：".into(),
    );

    m.insert("cli.section.teleport".into(), "传送保存点：".into());

    m.insert(
        "cli.shell_cmd.dl".into(),
        " ntc dl https://example.com/file.zip --progress 带进度下载".into(),
    );

    m.insert(
        "cli.shell_cmd.encode".into(),
        " ntc encode b64 hello 对字符串进行Base64编码".into(),
    );

    m.insert(
        "cli.shell_cmd.math".into(),
        " ntc math 3+4*5 计算表达式".into(),
    );

    m.insert("cli.shell_cmd.ne".into(), " ntc ne 打开文本编辑器".into());

    m.insert(
        "cli.shell_cmd.pathcp".into(),
        " ntc pathcp 将路径复制到剪贴板".into(),
    );

    m.insert(
        "cli.shell_cmd.qr".into(),
        " ntc qr \"文本\" 在终端中生成QR码".into(),
    );

    m.insert(
        "cli.shell_cmd.ral_add".into(),
        " ntc ral add btr \"cargo build --release\" 从CLI添加别名\n".into(),
    );

    m.insert(
        "cli.shell_cmd.say_loop".into(),
        " ntc say \"hello\" --loop 3 带循环打印文本".into(),
    );

    m.insert(
        "cli.shell_cmd.serve".into(),
        " ntc serve files --port 8080 在8080端口启动文件网盘服务器".into(),
    );

    m.insert("cli.shell_cmd.size".into(), " ntc size 显示目录大小".into());

    m.insert(
        "cli.shell_cmd.tl".into(),
        " ntc tl dir . 统计目录中的代码行数".into(),
    );

    m.insert("cli.shell_cmd.view".into(), " ntc view 显示目录树".into());

    m.insert("cli.usage.basic".into(), " ntc [选项]".into());

    m.insert(
        "cli.usage.clipboard".into(),
        " ntc -i <路径> --cp 将报告复制到剪贴板\n".into(),
    );

    m.insert("cli.usage.header".into(), "用法：".into());

    m.insert(
        "cli.usage.input".into(),
        " ntc -i <路径> [-o <输出>]".into(),
    );
}
