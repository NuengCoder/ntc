use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ral.title".into(), "运行别名命令：".into());

    m.insert(
        "help.ral.add".into(),
        "ral add <名称> \"<命令>\" ## 创建别名".into(),
    );

    m.insert(
        "help.ral.add_param".into(),
        "ral add <名称>(x) \"<命令>\" ## 创建带参数的别名（使用 ${x}）".into(),
    );

    m.insert(
        "help.ral.env_ref".into(),
        "ral add <名称> \"<命令>\" ## 裸 $NAME 运行时为环境变量；参数请用 ${x}".into(),
    );

    m.insert(
        "help.ral.add_defaults".into(),
        "ral add <名称>({x=a,e=txt}) \"<命令>\" ## 带默认值的别名".into(),
    );

    m.insert(
        "help.ral.add_array".into(),
        "ral add <名称>(x[],y) \"<命令>\" ## 带数组参数的别名".into(),
    );

    m.insert(
        "help.ral.add_rest".into(),
        "ral add <名称>(x) \"echo $*{x}\" ## 捕获一个参数（保留空格，去除引号）".into(),
    );

    m.insert(
        "help.ral.edit".into(),
        "ral edit <名称> \"<命令>\" ## 更新别名".into(),
    );

    m.insert(
        "help.ral.rnm".into(),
        "ral rnm <旧名称> to <新名称> ## 重命名别名".into(),
    );

    m.insert(
        "help.ral.rm".into(),
        "ral rm <名称>[, ...] ## 删除别名".into(),
    );

    m.insert("help.ral.list".into(), "ral list ## 显示所有别名".into());

    m.insert(
        "help.ral.info".into(),
        "ral info <名称> [--edit] ## 显示别名详情（--edit：将编辑命令复制到剪贴板）".into(),
    );

    m.insert(
        "help.ral.cls".into(),
        "ral cls ## 清除所有别名（需确认）".into(),
    );

    m.insert(
        "help.ral.export_all".into(),
        "ral export --all <名称> ## 将所有别名导出到 <名称>.ntc.ral".into(),
    );

    m.insert(
        "help.ral.export_select".into(),
        "ral export --select <名称> ## 选择要导出的别名".into(),
    );

    m.insert(
        "help.ral.import".into(),
        "ral import <文件> ## 从 .ntc.ral 文件导入别名".into(),
    );

    m.insert(
        "help.ral.make".into(),
        "ral make [名称] ## 在当前目录创建别名模板".into(),
    );

    m.insert(
        "help.ral.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## 管理命名值列表（--hint-n/--select-n 用）".into(),
    );

    m.insert(
        "help.ral.example1".into(),
        "ral add build \"cargo build\" ## 创建简单别名".into(),
    );

    m.insert(
        "help.ral.example2".into(),
        "ral add greet(x) \"echo Hello ${x}\" ## 创建参数化别名".into(),
    );

    m.insert(
        "help.ral.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## 带默认值的别名".into(),
    );

    m.insert(
        "help.ral.example4".into(),
        "ral list ## 显示所有别名".into(),
    );

    m.insert(
        "help.ral.example5".into(),
        "ral export --all myaliases ## 导出别名".into(),
    );

    m.insert(
        "help.ral.example6".into(),
        "ral hintn add port [8080,5050] ## 命名列表供 #x --hint-n port 使用".into(),
    );

    m.insert(
        "help.ral.install".into(),
        "ral install ## 从 ntc GitHub 目录安装运行别名".into(),
    );
}
