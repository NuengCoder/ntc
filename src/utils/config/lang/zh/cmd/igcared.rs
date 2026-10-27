use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.igcared.extra_exts".into(), "额外扩展名：{:?}".into());

    m.insert("cmd.igcared.extra_files".into(), "额外文件：{:?}".into());

    m.insert("cmd.igcared.ignored_dirs".into(), "忽略的目录：{:?}".into());

    m.insert(
        "cmd.igcared.ignored_exts".into(),
        "忽略的扩展名：{:?}".into(),
    );

    m.insert(
        "cmd.igcared.ignored_files".into(),
        "忽略的文件：{:?}".into(),
    );

    m.insert("cmd.igcared.local_config".into(), "    本地配置：{}".into());

    m.insert("cmd.igcared.section_global".into(), "[全局]".into());

    m.insert("cmd.igcared.section_local".into(), "[本地]".into());

    m.insert("cmd.igcared.cared_dirs".into(), "关注的目录：".into());

    m.insert("cmd.igcared.cared_exts".into(), "关注的扩展名：".into());

    m.insert("cmd.igcared.cared_files".into(), "关注的文件：".into());

    m.insert("cmd.igcared.none".into(), "（无）".into());

    m.insert(
        "cmd.igcared.no_local".into(),
        "（当前目录没有本地配置）".into(),
    );

    m.insert(
        "cmd.igcared.scope_help".into(),
        "本地条目优先；全局条目继续生效，全局配置不会被修改。".into(),
    );
}
