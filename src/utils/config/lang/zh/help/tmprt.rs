use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tmprt.title".into(), "温度与天气命令：".into());

    m.insert(
        "help.tmprt.com".into(),
        "tmprt com [秒] ## 实时硬件温度监控（q=退出 s=冻结 c=切换'oC/'oF）".into(),
    );

    m.insert(
        "help.tmprt.weather".into(),
        "tmprt weather [城市] ## 通过 OpenWeather API 获取当前天气".into(),
    );

    m.insert(
        "help.tmprt.key".into(),
        "tmprt key [API_KEY] ## 保存/显示 OpenWeather API 密钥".into(),
    );

    m.insert(
        "help.tmprt.city".into(),
        "tmprt city [名称] ## 保存/显示天气默认城市".into(),
    );

    m.insert(
        "help.tmprt.sensors".into(),
        "tmprt sensors <status|install|uninstall|debug> ## 管理硬件传感器后端（LHM）".into(),
    );

    m.insert(
        "help.tmprt.example1".into(),
        "tmprt com 5 ## 每 5 秒刷新一次的实时视图".into(),
    );

    m.insert(
        "help.tmprt.example2".into(),
        "tmprt weather Bangkok ## 查询曼谷天气".into(),
    );

    m.insert(
        "help.tmprt.example3".into(),
        "tmprt key 1234567890abcdef ## 保存 API 密钥".into(),
    );

    m.insert(
        "help.tmprt.example4".into(),
        "tmprt city Bangkok ## 设置默认城市".into(),
    );
}
