use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "命令总览".into());

    m.insert(
        "help.compact.footer".into(),
        "输入 'help <命令>' 查看详情，'help --all' 查看完整参考。".into(),
    );

    m.insert("help.compact.arc".into(), "创建压缩包".into());

    m.insert(
        "help.compact.autoup".into(),
        "列出所有启动条目（CU/LM/SF来源） [Windows Only]".into(),
    );

    m.insert("help.compact.back".into(), "返回父目录（或n层）".into());

    m.insert("help.compact.backup".into(), "创建当前项目的备份".into());

    m.insert("help.compact.betw".into(), "打印第 A 到 B 行".into());

    m.insert("help.compact.care".into(), "停止忽略目录".into());

    m.insert("help.compact.cd".into(), "导航到目录".into());

    m.insert("help.compact.clear".into(), "清除屏幕".into());

    m.insert("help.compact.clip".into(), "显示剪贴板历史".into());

    m.insert("help.compact.config".into(), "显示/设置输出路径".into());

    m.insert(
        "help.compact.count".into(),
        "统计 <needle> 出现的次数（文本或管道输入）".into(),
    );

    m.insert(
        "help.compact.cp".into(),
        "复制文件或目录（通配符源需目录目标）".into(),
    );

    m.insert("help.compact.cvcode".into(), "在格式之间转换".into());

    m.insert(
        "help.compact.deadlock".into(),
        "  deadlock                          打开保险库（会要求输入主密码）".into(),
    );

    m.insert("help.compact.decode".into(), "二进制解码".into());

    m.insert(
        "help.compact.deinit".into(),
        "删除项目文件（每次删除前都会确认）".into(),
    );

    m.insert("help.compact.dl".into(), "下载文件".into());

    m.insert("help.compact.dns".into(), "DNS查询".into());

    m.insert(
        "help.compact.doctor".into(),
        "检查配置、PATH、ranfile 与 ntpkg".into(),
    );

    m.insert("help.compact.ds".into(), "按名称搜索目录".into());

    m.insert("help.compact.encode".into(), "二进制编码（0101...）".into());

    m.insert("help.compact.exit".into(), "退出ntc".into());

    m.insert(
        "help.compact.explorer".into(),
        "启动全屏终端文件浏览器".into(),
    );

    m.insert(
        "help.compact.fake".into(),
        "  fake gen <类型> [-n N] [-f 格式] [-s 种子] [-o 文件]   生成行".into(),
    );

    m.insert(
        "help.compact.fgo".into(),
        "查找文件，导航到所选文件的父目录".into(),
    );

    m.insert("help.compact.fmt".into(), "将JSON转换为YAML或TOML".into());

    m.insert("help.compact.fs".into(), "按名称搜索文件".into());

    m.insert(
        "help.compact.fsc".into(),
        "搜索文件，选择后显示并可选择复制".into(),
    );

    m.insert("help.compact.git".into(), "漂亮的仓库状态输出".into());

    m.insert(
        "help.compact.godrive".into(),
        "切换驱动器或 /mnt 挂载点".into(),
    );

    m.insert("help.compact.gos".into(), "列出子目录并选择一个".into());

    m.insert("help.compact.gosc".into(), "连续导航".into());

    m.insert("help.compact.gs".into(), "搜索文件内容（简单grep）".into());

    m.insert("help.compact.head".into(), "打印前 N 行（默认 10）".into());

    m.insert("help.compact.help".into(), "显示此帮助".into());

    m.insert("help.compact.http".into(), "HTTP API客户端".into());

    m.insert(
        "help.compact.igcare".into(),
        "将所有设置导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "help.compact.igcared".into(),
        "显示所有忽略/关注设置".into(),
    );

    m.insert("help.compact.ignore".into(), "按名称忽略目录".into());

    m.insert(
        "help.compact.image".into(),
        "从当前目录选取图片并在终端内显示".into(),
    );

    m.insert("help.compact.init".into(), "初始化项目文件".into());

    m.insert(
        "help.compact.ipcfg".into(),
        "显示网卡信息（类似 ipconfig/ifconfig）".into(),
    );

    m.insert(
        "help.compact.local".into(),
        "创建 ntconfig.toml 模板（已注释）".into(),
    );

    m.insert("help.compact.locate".into(), "同时搜索文件和目录".into());

    m.insert("help.compact.math".into(), "计算数学表达式".into());

    m.insert("help.compact.mkd".into(), "创建新目录".into());

    m.insert("help.compact.mkf".into(), "创建新文件".into());

    m.insert(
        "help.compact.mode".into(),
        "退出并以管理员身份重新启动ntc".into(),
    );

    m.insert(
        "help.compact.mv".into(),
        "移动/重命名文件或目录（通配符源需目录目标）".into(),
    );

    m.insert(
        "help.compact.ndb".into(),
        "在当前目录打开 ndb 数据库外壳（.db 浏览器）".into(),
    );

    m.insert(
        "help.compact.ne".into(),
        "在内置文本编辑器中打开已有文件".into(),
    );

    m.insert("help.compact.nms".into(), "在终端播放声音".into());

    m.insert(
        "help.compact.nsheet".into(),
        "全屏表格浏览器（CSV/TSV/JSON）".into(),
    );

    m.insert("help.compact.nssh".into(), "保存 SSH 配置".into());

    m.insert("help.compact.ntpkg".into(), "下载并安装运行时".into());

    m.insert(
        "help.compact.ntpx".into(),
        "运行已安装包的二进制文件".into(),
    );

    m.insert(
        "help.compact.opencg".into(),
        "在编辑器中打开 config.toml".into(),
    );

    m.insert(
        "help.compact.pathcp".into(),
        "将当前目录路径复制到剪贴板".into(),
    );

    m.insert("help.compact.ping".into(), "Ping 主机".into());

    m.insert(
        "help.compact.pipe".into(),
        "将 a 的输出作为 b 的输入 [NON-COMMAND]".into(),
    );

    m.insert(
        "help.compact.py_activate".into(),
        "激活Python虚拟环境（自动检测）".into(),
    );

    m.insert(
        "help.compact.py_deactivate".into(),
        "停用当前虚拟环境".into(),
    );

    m.insert(
        "help.compact.qr".into(),
        "编码文本并在终端中显示QR码".into(),
    );

    m.insert(
        "help.compact.qsc".into(),
        "启动QSC守护进程（后台进程）".into(),
    );

    m.insert("help.compact.qur".into(), "查询文件、选择内容、替换".into());

    m.insert("help.compact.ral".into(), "创建别名".into());

    m.insert(
        "help.compact.ran".into(),
        "运行 'standalone' 目标（默认）".into(),
    );

    m.insert("help.compact.report".into(), "项目报告保存到 <file>".into());

    m.insert(
        "help.compact.resetcg".into(),
        "重置配置为默认值（可选备份）".into(),
    );

    m.insert("help.compact.restorecg".into(), "从备份恢复配置".into());

    m.insert("help.compact.rev".into(), "反转每行的字符".into());

    m.insert(
        "help.compact.rmd".into(),
        "删除目录；通配符确认后删除所有匹配项".into(),
    );

    m.insert("help.compact.rmf".into(), "删除文件（需确认）".into());

    m.insert("help.compact.rnmd".into(), "重命名目录".into());

    m.insert("help.compact.rnmf".into(), "重命名文件".into());

    m.insert("help.compact.run".into(), "使用 'run' 执行别名".into());

    m.insert(
        "help.compact.say".into(),
        "打印文本（无需文件重定向）".into(),
    );

    m.insert("help.compact.nprintf".into(), "Bash 风格格式化输出".into());

    m.insert(
        "help.compact.scan".into(),
        "按扩展名列出文件（不统计行数）".into(),
    );

    m.insert("help.compact.scr".into(), "运行自动化脚本".into());

    m.insert(
        "help.compact.serve".into(),
        "选择服务（单独 `serve` 列出全部）".into(),
    );

    m.insert("help.compact.showcg".into(), "显示当前配置概览".into());

    m.insert(
        "help.compact.showenv".into(),
        "显示环境变量（前5个PATH条目）".into(),
    );

    m.insert("help.compact.size".into(), "显示当前目录大小".into());

    m.insert("help.compact.skill".into(), "从 GitHub 下载技能".into());

    m.insert("help.compact.smtp".into(), "SMTP邮件客户端".into());

    m.insert(
        "help.compact.sort".into(),
        "排序行（倒序/数值/去重/忽略大小写）".into(),
    );

    m.insert("help.compact.spec".into(), "显示电脑/系统规格".into());

    m.insert("help.compact.sys".into(), "实时系统监视器".into());

    m.insert(
        "help.compact.syspath".into(),
        "显示系统级PATH条目".into(),
    );

    m.insert("help.compact.tail".into(), "打印后 N 行（默认 10）".into());

    m.insert(
        "help.compact.theme".into(),
        "交互式主题管理器（菜单）".into(),
    );

    m.insert(
        "help.compact.time".into(),
        "  time run <命令>                       运行 NTC 命令、别名或外部命令".into(),
    );

    m.insert("help.compact.tl".into(), "统计单个文件的行数".into());

    m.insert(
        "help.compact.tmprt".into(),
        "实时硬件温度监控（q=退出 s=冻结 c=切换'oC/'oF）".into(),
    );

    m.insert(
        "help.compact.todo".into(),
        "显示所有任务（来自当前目录的 future_todo.txt）".into(),
    );

    m.insert("help.compact.tp".into(), "交互式菜单".into());

    m.insert(
        "help.compact.tpb".into(),
        "传送到上一个位置（撤销传送）".into(),
    );

    m.insert("help.compact.trace".into(), "逐跳路由追踪".into());

    m.insert(
        "help.compact.trim".into(),
        "删除 <pattern> 的每一次出现（文本或管道输入）".into(),
    );

    m.insert(
        "help.compact.tutorial".into(),
        "在沙盒中动手实践的课程".into(),
    );

    m.insert(
        "help.compact.txtc".into(),
        "将文件内容复制到剪贴板（行号开关）".into(),
    );

    m.insert(
        "help.compact.txtf".into(),
        "显示文件内容或从目录中选择文件（保留原始行号）".into(),
    );

    m.insert("help.compact.ui".into(), "显示当前UI模式".into());

    m.insert("help.compact.up".into(), "上传文件".into());

    m.insert(
        "help.compact.usrpath".into(),
        "显示用户级PATH条目".into(),
    );

    m.insert("help.compact.version".into(), "显示版本".into());

    m.insert("help.compact.view".into(), "显示目录树或文件内容".into());

    m.insert("help.compact.viewd".into(), "显示仅目录树".into());

    m.insert("help.compact.viewf".into(), "显示深度 1 的仅文件树".into());

    m.insert(
        "help.compact.views".into(),
        "像 ls 一样平铺列出单层目录".into(),
    );

    m.insert(
        "help.compact.where".into(),
        "显示 ntc 路径或在 PATH 中查找程序".into(),
    );

    m.insert("help.compact.wi".into(), "实时文件更改流（q=退出）".into());

    m.insert(
        "help.compact.line".into(),
        "为每行编号".into(),
    );
}
