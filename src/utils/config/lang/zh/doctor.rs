use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("doctor.title".into(), "ntc doctor".into());

    m.insert("doctor.platform".into(), "ntc {} ({}) 运行于 {}/{}".into());

    m.insert(
        "doctor.no_config_dir".into(),
        "此系统上没有可用的配置目录".into(),
    );

    m.insert(
        "doctor.config_new".into(),
        "配置目录尚未创建：{}（首次保存时创建）".into(),
    );

    m.insert("doctor.config_ok".into(), "配置：{}".into());

    m.insert(
        "doctor.config_unwritable".into(),
        "配置目录不可写：{}（{}）".into(),
    );

    m.insert("doctor.settings".into(), "语言：{}，主题：{}".into());

    m.insert("doctor.install".into(), "{} 已在 PATH 中：{}".into());

    m.insert(
        "doctor.install_missing".into(),
        "{} 不在 PATH 中 \u{2014} 只有当前构建目录能运行它".into(),
    );

    m.insert(
        "doctor.ran_none".into(),
        "ran：{} 中没有 ranfile（此处无可运行任务）".into(),
    );

    m.insert("doctor.ran_ok".into(), "ran：{} 共 {} 个目标".into());

    m.insert(
        "doctor.ran_env".into(),
        "ran：{} 个环境变量，{} 个变量".into(),
    );

    m.insert(
        "doctor.ran_problems".into(),
        "ran：{} 有 {} 个问题 \u{2014} 运行 `{}`".into(),
    );

    m.insert("doctor.ran_error".into(), "ran 出错：{}：{}".into());

    m.insert("doctor.ntpkg_none".into(), "ntpkg：未安装任何运行时".into());

    m.insert(
        "doctor.ntpkg_ok".into(),
        "ntpkg：已安装 {} 个版本，{} 个已激活".into(),
    );

    m.insert("doctor.ntpkg_error".into(), "ntpkg 出错：{}：{}".into());

    m.insert(
        "doctor.ntpkg_path".into(),
        "ntpkg：{} 个已激活的 bin 目录不在 PATH 中".into(),
    );

    m.insert("doctor.ok".into(), "未发现问题".into());

    m.insert("doctor.issues".into(), "有 {} 个问题需要处理".into());
}
