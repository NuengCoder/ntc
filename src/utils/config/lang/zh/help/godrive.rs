use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.godrive.title".into(), "GODRIVE 命令：".into());

    m.insert(
        "help.godrive.usage".into(),
        "godrive ## 列出并选择驱动器（Windows）或 /mnt 挂载点（Linux）".into(),
    );

    m.insert(
        "help.godrive.example1".into(),
        "godrive ## 选择驱动器或挂载点".into(),
    );

    m.insert(
        "help.godrive.example2".into(),
        "godrive c ## 跳转到 Windows 的 C: 盘或 Linux 的 /mnt/c".into(),
    );
}
