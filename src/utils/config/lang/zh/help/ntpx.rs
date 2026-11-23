use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ntpx.title".into(), "NTPX 命令：".into());

    m.insert(
        "help.ntpx.usage".into(),
        "ntpx <包> [二进制] [参数] ## 运行已安装包的二进制文件".into(),
    );

    m.insert(
        "help.ntpx.example1".into(),
        "ntpx node npm init ## 通过 ntpx 运行 npm".into(),
    );
}
