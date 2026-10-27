use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "error.failed_write_completions".into(),
        "写入补全失败：{}".into(),
    );

    m.insert(
        "error.unsupported_shell".into(),
        "不支持的Shell：{}。使用：bash, zsh, fish, powershell".into(),
    );
}
