use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fake.title".into(),
        "fake - 生成假数据/随机数据".into(),
    );

    m.insert(
        "help.fake.gen".into(),
        "  fake gen <类型> [-n N] [-f 格式] [-s 种子] [-o 文件]   生成行".into(),
    );

    m.insert(
        "help.fake.gen_options".into(),
        "  格式：json | txt | sql | csv | xlsx（默认：txt 表格）".into(),
    );

    m.insert(
        "help.fake.gen_group".into(),
        "  fake gen -g <分组> ...                          从已保存的分组生成".into(),
    );

    m.insert(
        "help.fake.gen_unique".into(),
        "  -u, --unique [列]                              整行或指定列唯一".into(),
    );

    m.insert("help.fake.gen_locale".into(), "  -l, --locale <代码>                            区域数据（en_US en_GB zh_CN th_TH lo_LA）".into());

    m.insert(
        "help.fake.types".into(),
        "  fake types                                     列出所有类型".into(),
    );

    m.insert(
        "help.fake.types_search".into(),
        "  fake types -s <模式>                           模糊搜索类型".into(),
    );

    m.insert(
        "help.fake.locales".into(),
        "  fake locales                                   列出 --locale 代码".into(),
    );

    m.insert(
        "help.fake.group_add".into(),
        "  fake group add <名称> [别名 as 类型, ...]      创建分组".into(),
    );

    m.insert(
        "help.fake.group_edit".into(),
        "  fake group edit <名称> [别名 as 类型, ...]     替换分组的列".into(),
    );

    m.insert(
        "help.fake.group_rm".into(),
        "  fake group rm <名称>                           删除分组".into(),
    );

    m.insert(
        "help.fake.group_rnm".into(),
        "  fake group rnm <旧名> <新名>                   重命名分组".into(),
    );

    m.insert(
        "help.fake.group_ls".into(),
        "  fake group ls [模式]                           列出分组（通配符，如 u*）".into(),
    );

    m.insert(
        "help.fake.group_info".into(),
        "  fake group info <名称>                         显示分组的列".into(),
    );

    m.insert(
        "help.fake.group_clear".into(),
        "  fake group clear [-f]                          删除所有分组".into(),
    );

    m.insert(
        "help.fake.group_export".into(),
        "  fake group export <名称>|--all [-o 文件]       将分组导出为 .ntc_fake".into(),
    );

    m.insert(
        "help.fake.group_import".into(),
        "  fake group import <文件> [--force] [--prefix p]  从文件加载分组".into(),
    );

    m.insert(
        "help.fake.group_ref".into(),
        "  列语法：'别名 as 类型'，'类型 !' = 唯一，'别名 as ref:分组.列'".into(),
    );

    m.insert(
        "help.fake.help".into(),
        "  fake help                                      显示此帮助".into(),
    );

    m.insert(
        "help.fake.example1".into(),
        "fake gen name,email,city -n 5 ## txt 表格".into(),
    );

    m.insert(
        "help.fake.example2".into(),
        "fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "help.fake.example3".into(),
        "fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "help.fake.example4".into(),
        "fake gen -g users -f sql -n 10 ## INSERT INTO users ...".into(),
    );
}
