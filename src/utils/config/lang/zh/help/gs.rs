use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gs.title".into(), "GS 命令：".into());

    m.insert(
        "help.gs.usage".into(),
        "gs <模式> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-A <n>] [-B <n>] [-I] [-x] [-h|-g|-w] [-z] ## 搜索文件内容（简单grep）".into(),
    );

    m.insert(
        "help.gs.interactive".into(),
        "gs -I [模式] [-d <n>] ## 交互式grep（每页10条，f=模式，e/Enter=打开，0-9=行，?=按键，x/h/g/w/z=开关）".into(),
    );

    m.insert(
        "help.gs.pipe".into(),
        "cmd | gs <模式> ## 搜索管道输入的内容（也支持 qur -s <模式>）".into(),
    );

    m.insert(
        "help.gs.context".into(),
        "gs <模式> -A <n> -B <n> ## 显示每个匹配后/前 n 行（默认10个匹配，可用 -l <n> 调整）"
            .into(),
    );

    m.insert(
        "help.gs.filters".into(),
        "gs -x <字面> -h <开头> -g <结尾> -w <整行> -z <大小写> ## -x 按字面搜索（a\\|b 就是 'a|b'，不再表示“或”）；-h/-g/-w 匹配去掉首尾空白后的行，三者不能同时使用；-z 区分大小写"
            .into(),
    );

    m.insert(
        "help.gs.example1".into(),
        "gs fn main ## 在文件中搜索 'fn main'".into(),
    );

    m.insert(
        "help.gs.example2".into(),
        "gs -I config ## 交互式搜索结果".into(),
    );

    m.insert(
        "help.gs.example3".into(),
        "gs TODO < app.log ## 搜索管道/重定向输入的内容".into(),
    );

    m.insert(
        "help.gs.example4".into(),
        "gs error\\|warning ## 或搜索：匹配任意一个词 (a\\|b)".into(),
    );

    m.insert(
        "help.gs.example5".into(),
        "gs main -A 3 -B 2 ## 匹配行加后3行、前2行".into(),
    );

    m.insert(
        "help.gs.example6".into(),
        "gs -w main ## 只匹配整行就是 'main' 的行".into(),
    );

    m.insert(
        "help.gs.example7".into(),
        "gs -z Main -h ## 区分大小写：以 'Main' 开头的行".into(),
    );
}
