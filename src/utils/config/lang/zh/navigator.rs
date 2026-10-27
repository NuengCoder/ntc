use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "navigator.already_root".into(),
        "已在根目录，无法再后退".into(),
    );

    m.insert(
        "navigator.cannot_navigate_to".into(),
        "无法导航到：{}".into(),
    );

    m.insert(
        "navigator.cannot_read_dir".into(),
        "无法读取目录：{}".into(),
    );

    m.insert(
        "navigator.drive_windows_only".into(),
        "盘符导航仅在 Windows 上受支持".into(),
    );

    m.insert("navigator.not_a_directory".into(), "{} 不是目录".into());

    m.insert(
        "navigator.venv_activate_unix".into(),
        "    运行：source ./venv/bin/activate".into(),
    );

    m.insert(
        "navigator.venv_activate_win".into(),
        "   Run: .\\\\\\\\venv\\\\\\\\Scripts\\\\\\\\activate".into(),
    );

    m.insert(
        "navigator.venv_tip".into(),
        "    运行：source ./venv/bin/activate".into(),
    );
}
