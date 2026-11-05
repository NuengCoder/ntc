use std::collections::HashMap;

/// 紧凑 `help` 总览的一行说明（`help --all` 保留完整参考）。
pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "命令总览".into());
    m.insert(
        "help.compact.footer".into(),
        "输入 'help <命令>' 查看详情，'help --all' 查看完整参考。".into(),
    );

    m.insert("help.compact.nav".into(), "切换目录、返回、跳转".into());
    m.insert("help.compact.config".into(), "查看或修改 ntc 设置".into());
    m.insert("help.compact.editor".into(), "打开内置编辑器".into());
    m.insert("help.compact.theme".into(), "切换、添加、导出主题".into());
    m.insert(
        "help.compact.teleport".into(),
        "已保存的路径（tp jump <name>）".into(),
    );
    m.insert(
        "help.compact.ran".into(),
        "运行 NTCRANFILE.toml 中的任务".into(),
    );
    m.insert("help.compact.scr".into(), "运行已保存的脚本文件".into());
    m.insert(
        "help.compact.alias".into(),
        "运行别名（ral add <name> ...）".into(),
    );
    m.insert("help.compact.view".into(), "目录与文件树".into());
    m.insert("help.compact.size".into(), "目录大小".into());
    m.insert(
        "help.compact.report".into(),
        "项目报告（report project）".into(),
    );
    m.insert("help.compact.backup".into(), "备份与恢复项目".into());
    m.insert("help.compact.search".into(), "查找文本、文件和目录".into());
    m.insert("help.compact.igcare".into(), "扫描的忽略/关注规则".into());
    m.insert(
        "help.compact.network".into(),
        "dl、up、http、serve、ping、dns、smtp".into(),
    );
    m.insert("help.compact.tmprt".into(), "温度与天气".into());
    m.insert("help.compact.time".into(), "计时与基准测试".into());
    m.insert(
        "help.compact.git".into(),
        "Git 快捷命令（status、branch、log）".into(),
    );
    m.insert(
        "help.compact.clipboard".into(),
        "剪贴板历史（clip list）".into(),
    );
    m.insert("help.compact.qsc".into(), "快速命令监听器".into());
    m.insert("help.compact.todo".into(), "项目待办列表".into());
    m.insert("help.compact.arc".into(), "创建与解压归档".into());
    m.insert("help.compact.scan".into(), "扫描扩展名、端口、IP".into());
    m.insert("help.compact.tl".into(), "统计文件行数".into());
    m.insert("help.compact.ntpkg".into(), "安装与管理工具（ntpx）".into());
    m.insert("help.compact.qr".into(), "生成与读取二维码".into());
    m.insert(
        "help.compact.pathman".into(),
        "管理 PATH 条目（usrpath、syspath）".into(),
    );
    m.insert("help.compact.autoup".into(), "管理开机启动项".into());
    m.insert("help.compact.mode".into(), "管理员 / 普通模式".into());
    m.insert("help.compact.explorer".into(), "文件浏览器（nfe）".into());
    m.insert("help.compact.math".into(), "计算数学表达式".into());
    m.insert("help.compact.python".into(), "激活 Python 虚拟环境".into());
    m.insert(
        "help.compact.encdec".into(),
        "encode、decode、cvcode".into(),
    );
    m.insert("help.compact.fmt".into(), "格式化文件".into());
    m.insert("help.compact.image".into(), "在终端内显示图像".into());
    m.insert("help.compact.ndb".into(), "浏览 SQLite 数据库".into());
    m.insert("help.compact.nsheet".into(), "查看电子表格".into());
    m.insert("help.compact.nssh".into(), "SSH 配置与文件传输".into());
    m.insert("help.compact.nms".into(), "终端音乐播放器".into());
    m.insert("help.compact.fake".into(), "生成假数据".into());
    m.insert("help.compact.deadlock".into(), "加密密码库".into());
    m.insert("help.compact.syscmd".into(), "运行任意系统命令".into());
    m.insert("help.compact.pipe".into(), "管道、重定向与过滤器".into());
    m.insert(
        "help.compact.fileops".into(),
        "mkf、mkd、rmf、rmd、mv、cp".into(),
    );
    m.insert(
        "help.compact.other".into(),
        "clear、version、say、ui、exit、help".into(),
    );
}
