use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "idle.tp".into(),
        "试试：tp add <目录名>  然后：@<目录名>   （保存并跳回此路径）".into(),
    );

    m.insert(
        "idle.explorer".into(),
        "试试：nfe   （用文件浏览器查看此目录；f 查找文件）".into(),
    );

    m.insert(
        "idle.ne".into(),
        "试试：ne --init <文件>   （从模板新建文件）".into(),
    );

    m.insert(
        "idle.view".into(),
        "试试：view -d 2   （显示两层目录树）".into(),
    );

    m.insert(
        "idle.search".into(),
        "试试：gs <文本>   （在此目录的文件中查找该文本）".into(),
    );

    m.insert(
        "idle.todo".into(),
        "试试：todo add \"任务名\"   （此目录已有待办清单）".into(),
    );

    m.insert(
        "idle.git".into(),
        "试试：git status   （这是一个 git 仓库）".into(),
    );

    m.insert(
        "idle.serve".into(),
        "试试：serve files --lock   （以只读方式通过 HTTP 共享此目录）".into(),
    );

    m.insert(
        "idle.tutorial".into(),
        "试试：tutorial   （在可随时删除的沙盒里上手练习）".into(),
    );

    m.insert(
        "idle.ral".into(),
        "试试：ral add <名称> \"<命令>\"   （之后输入 <名称> 即可重跑）".into(),
    );

    m.insert(
        "idle.ran".into(),
        "试试：ran list   （此目录的 NTCRANFILE.toml 定义了任务）".into(),
    );

    m.insert(
        "idle.pipe".into(),
        "试试：gs <文本> | sort -u | head 10   （把一个命令的输出交给下一个）".into(),
    );

    m.insert(
        "idle.math".into(),
        "试试：math 3+4*5   （内置计算器，支持函数）".into(),
    );

    m.insert(
        "idle.qr".into(),
        "试试：qr <文本|url>   （打印可扫描的二维码）".into(),
    );

    m.insert(
        "idle.theme".into(),
        "试试：theme list   （然后：theme <名称> 换配色）".into(),
    );

    m.insert(
        "idle.backup".into(),
        "试试：backup create   （大改之前先给此目录拍个快照）".into(),
    );

    m.insert(
        "idle.empty".into(),
        "试试：mkf notes.md   （此目录为空，先建个文件）".into(),
    );

    m.insert(
        "idle.help".into(),
        "试试：help   （列出所有命令，或：help <主题>）".into(),
    );

    m.insert(
        "idle.tl".into(),
        "试试：tl dir .   （统计此目录下受支持文件的总行数）".into(),
    );

    m.insert("idle.size".into(), "试试：size   （此目录的总大小）".into());

    m.insert(
        "idle.report".into(),
        "试试：report project md   （在此生成 Markdown 项目报告）".into(),
    );

    m.insert(
        "idle.scr".into(),
        "试试：scr init demo   （创建一个可复用的 .ntc 脚本）".into(),
    );

    m.insert(
        "idle.time".into(),
        "试试：time bench --runs 3 <命令>   （测量命令的平均耗时）".into(),
    );

    m.insert(
        "idle.encode".into(),
        "试试：encode b64 \"hello\"   （base64、hex、url 等编码）".into(),
    );

    m.insert(
        "idle.fmt".into(),
        "试试：fmt Cargo.toml to json   （转换配置格式）".into(),
    );

    m.insert(
        "idle.arc".into(),
        "试试：arc list <文件.zip>   （不解除压缩直接查看归档）".into(),
    );

    m.insert(
        "idle.scan".into(),
        "试试：scan ext rs   （列出此目录所有 .rs 文件）".into(),
    );

    m.insert(
        "idle.sys".into(),
        "试试：sys   （实时 CPU、内存和进程监控）".into(),
    );

    m.insert(
        "idle.showcg".into(),
        "试试：showcg   （一眼查看当前所有设置）".into(),
    );

    m.insert(
        "idle.ntpkg".into(),
        "试试：ntpkg list   （通过 ntc 安装的运行时）".into(),
    );

    m.insert(
        "idle.ndb".into(),
        "试试：ndb   （浏览此目录中的 SQLite 数据库）".into(),
    );

    m.insert(
        "idle.nsheet".into(),
        "试试：nsheet open <文件.csv>   （浏览电子表格）".into(),
    );

    m.insert(
        "idle.fgo".into(),
        "试试：fgo task.txt   （按名称找文件并跳过去）".into(),
    );

    m.insert(
        "idle.back".into(),
        "试试：back   （上一级；gos、gosc、godrive 也是单词选择器）".into(),
    );

    m.insert(
        "idle.viewf".into(),
        "试试：viewf .   （只看文件）· viewd . 只看文件夹".into(),
    );

    m.insert(
        "idle.views".into(),
        "试试：views .   （带 --size 和 --sort 的目录列表）".into(),
    );

    m.insert(
        "idle.txtf".into(),
        "试试：txtf task.txt --head 20   （在终端显示文件）· txtc 复制到剪贴板".into(),
    );

    m.insert(
        "idle.text".into(),
        "试试：head 5 task.txt   （还有 tail、betw 2:4、sort、rev）".into(),
    );

    m.insert(
        "idle.count".into(),
        "试试：view task.txt | count ntc   （统计管道文本中的匹配数）· trim 删除".into(),
    );

    m.insert(
        "idle.files".into(),
        "试试：mkf notes.md   （之后 mkd、cp、mv、rnmf、rmf、rmd）".into(),
    );

    m.insert(
        "idle.config".into(),
        "试试：config output   （单项设置；showcg 查看全部）".into(),
    );

    m.insert(
        "idle.opencg".into(),
        "试试：opencg   （编辑 config.toml）· resetcg、restorecg 可恢复".into(),
    );

    m.insert(
        "idle.local".into(),
        "试试：config local init   （只对本目录生效的设置）".into(),
    );

    m.insert(
        "idle.project".into(),
        "试试：init   （在此创建 NTCRANFILE.toml）· deinit 移除".into(),
    );

    m.insert(
        "idle.tpb".into(),
        "试试：tpb history   （之前跳转过的路径）".into(),
    );

    m.insert(
        "idle.qur".into(),
        "试试：qur *.txt -s \"old\" -dp   （先选，再用 -rp 替换）".into(),
    );

    m.insert(
        "idle.fsc".into(),
        "试试：fsc task.txt   （找到文件后打开）· fs 只找文件，ds 只找目录".into(),
    );

    m.insert(
        "idle.ignore".into(),
        "试试：ignore dir target   （扫描时跳过该目录）· care 取消".into(),
    );

    m.insert(
        "idle.igcare".into(),
        "试试：igcared   （查看所有忽略与 care 规则）".into(),
    );

    m.insert(
        "idle.http".into(),
        "试试：http --help   （HTTP 客户端：方法、请求头、正文）".into(),
    );

    m.insert(
        "idle.dl".into(),
        "试试：dl --help   （支持断点续传的下载）".into(),
    );

    m.insert(
        "idle.dns".into(),
        "试试：dns example.com -t MX   （查询 DNS 记录）".into(),
    );

    m.insert(
        "idle.up".into(),
        "试试：up --help   （通过 HTTP 上传文件）".into(),
    );

    m.insert(
        "idle.ping".into(),
        "试试：ping 127.0.0.1   （检查主机是否响应）".into(),
    );

    m.insert(
        "idle.trace".into(),
        "试试：trace 127.0.0.1   （逐跳路由）".into(),
    );

    m.insert(
        "idle.smtp".into(),
        "试试：smtp --help   （发邮件；密码保存在 deadlock）".into(),
    );

    m.insert(
        "idle.ipcfg".into(),
        "试试：ipcfg   （本机网卡与地址）".into(),
    );

    m.insert(
        "idle.wi".into(),
        "试试：wi .   （实时监视此目录变化；q 退出）".into(),
    );

    m.insert(
        "idle.spec".into(),
        "试试：spec   （CPU、内存、磁盘和系统摘要）".into(),
    );

    m.insert(
        "idle.tmprt".into(),
        "试试：tmprt city   （设置默认城市）· tmprt com 查看传感器".into(),
    );

    m.insert(
        "idle.clip".into(),
        "试试：clip   （剪贴板历史）· clip 2 取回第 2 项".into(),
    );

    m.insert(
        "idle.qsc".into(),
        "试试：qsc status   （全局热键守护进程是否在运行）".into(),
    );

    m.insert(
        "idle.pathman".into(),
        "试试：usrpath show   （用户 PATH）· syspath show 查看系统 PATH".into(),
    );

    m.insert(
        "idle.autoup".into(),
        "试试：autoup list   （开机启动项）".into(),
    );

    m.insert(
        "idle.mode".into(),
        "试试：mode   （当前管理员/普通模式）".into(),
    );

    m.insert(
        "idle.python".into(),
        "试试：showenv   （会话环境变量）· py_activate 启用虚拟环境".into(),
    );

    m.insert(
        "idle.decode".into(),
        "试试：decode b64 \"aGVsbG8=\"   （encode 的反向操作）· cvcode 格式转换".into(),
    );

    m.insert(
        "idle.image".into(),
        "试试：image --help   （在终端内显示图片）".into(),
    );

    m.insert(
        "idle.nssh".into(),
        "试试：nssh ls   （已保存的 SSH 配置）".into(),
    );

    m.insert(
        "idle.nms".into(),
        "试试：nms scan .   （查找此处的音视频文件）".into(),
    );

    m.insert(
        "idle.fake".into(),
        "试试：fake gen name,email -n 3   （测试假数据；fake types 列出全部）".into(),
    );

    m.insert(
        "idle.deadlock".into(),
        "试试：deadlock   （加密密码库）".into(),
    );

    m.insert(
        "idle.ntpx".into(),
        "试试：ntpx   （运行 ntpkg 安装的运行时）".into(),
    );

    m.insert(
        "idle.pathcp".into(),
        "试试：pathcp   （把当前路径复制到剪贴板）".into(),
    );

    m.insert(
        "idle.say".into(),
        "试试：say hello --loop 3   （打印文本；print 同义）".into(),
    );

    m.insert(
        "idle.ui".into(),
        "试试：ui   （显示 UI 模式）· version 显示版本".into(),
    );
}
