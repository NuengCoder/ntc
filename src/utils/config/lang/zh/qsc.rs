use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "qsc.macos_listen_hint".into(),
        "QSC：在添加支持后运行 'qsc listen'。".into(),
    );

    m.insert(
        "qsc.macos_not_implemented".into(),
        "QSC：macOS 全局热键支持尚未实现。".into(),
    );

    m.insert(
        "qsc.unsupported_platform".into(),
        "QSC：不支持的平台".into(),
    );
}
