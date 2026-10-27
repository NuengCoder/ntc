use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.views.usage".into(),
        "用法：views [路径] [-n] [--size [最小大小]] [--sort <name|ext|size>]".into(),
    );

    m.insert(
        "cmd.views.invalid_size".into(),
        "大小无效：{}（使用 1024、10KB、5MB 等）".into(),
    );

    m.insert("cmd.views.invalid_sort".into(), "--sort 缺少值".into());

    m.insert(
        "cmd.views.invalid_sort_value".into(),
        "未知排序键：{}（使用 name、ext 或 size）".into(),
    );

    m.insert("cmd.views.empty".into(), "（空目录）".into());
}
