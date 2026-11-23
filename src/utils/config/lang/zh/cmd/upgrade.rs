use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.upgrade.after_restart".into(),
        "新版本将在重新启动后准备就绪。".into(),
    );

    m.insert(
        "cmd.upgrade.ahead".into(),
        "本地版本（{}）高于最新发布（{}）。".into(),
    );

    m.insert(
        "cmd.upgrade.already_latest".into(),
        "ntc {} 已是最新版本。".into(),
    );

    m.insert(
        "cmd.upgrade.api_parse_fail".into(),
        "解析 GitHub API 响应失败".into(),
    );

    m.insert(
        "cmd.upgrade.api_read_fail".into(),
        "读取 GitHub API 响应失败".into(),
    );

    m.insert(
        "cmd.upgrade.api_request_fail".into(),
        "GitHub API 请求失败：{}".into(),
    );

    m.insert("cmd.upgrade.check_fail".into(), "检查更新失败：{}".into());

    m.insert(
        "cmd.upgrade.checking".into(),
        "正在检查更新（当前版本：{}）...".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail".into(),
        "复制二进制文件失败：{}".into(),
    );

    m.insert(
        "cmd.upgrade.copy_fail_android".into(),
        "复制二进制文件失败。请检查磁盘空间或权限。".into(),
    );

    m.insert(
        "cmd.upgrade.dev_build".into(),
        "您可能正在运行开发版本。".into(),
    );

    m.insert(
        "cmd.upgrade.download_complete".into(),
        "下载完成。正在安装更新...".into(),
    );

    m.insert("cmd.upgrade.download_fail".into(), "下载失败：{}".into());

    m.insert("cmd.upgrade.downloading".into(), "正在下载更新...".into());

    m.insert("cmd.upgrade.downloading_url".into(), "正在下载：{}".into());

    m.insert(
        "cmd.upgrade.exe_path_fail".into(),
        "获取可执行文件路径失败：{}".into(),
    );

    m.insert("cmd.upgrade.install_fail".into(), "安装失败：{}".into());

    m.insert(
        "cmd.upgrade.missing_tag".into(),
        "GitHub 发布响应中缺少 'tag_name'".into(),
    );

    m.insert(
        "cmd.upgrade.new_version".into(),
        "发现新版本：ntc {}".into(),
    );

    m.insert(
        "cmd.upgrade.no_assets_release".into(),
        "在发布 v{} 中未找到资源".into(),
    );

    m.insert(
        "cmd.upgrade.perm_fail".into(),
        "设置可执行权限失败：{}".into(),
    );

    m.insert(
        "cmd.upgrade.progress_bytes".into(),
        "已下载：{} 字节".into(),
    );

    m.insert("cmd.upgrade.progress_pct".into(), "进度：{}%".into());

    m.insert(
        "cmd.upgrade.restart".into(),
        "重新启动 ntc 以使用新版本。".into(),
    );

    m.insert("cmd.upgrade.stream_fail".into(), "读取下载流失败".into());

    m.insert(
        "cmd.upgrade.sudo_hint".into(),
        "请使用 sudo/管理员权限运行".into(),
    );

    m.insert(
        "cmd.upgrade.time_remaining".into(),
        "剩余时间：{:>3} 秒  ".into(),
    );

    m.insert(
        "cmd.upgrade.tmpdir_fail".into(),
        "创建临时目录失败：{}".into(),
    );

    m.insert(
        "cmd.upgrade.unsupported_platform".into(),
        "不支持的平台：{}".into(),
    );

    m.insert("cmd.upgrade.updated".into(), "ntc 已更新到 v{}！".into());

    m.insert("cmd.upgrade.write_fail".into(), "写入下载文件失败".into());

    m.insert(
        "cmd.upgrade.extract_fail".into(),
        "读取/解析下载的压缩包失败：{}".into(),
    );
    m.insert(
        "cmd.upgrade.extracting".into(),
        "正在解压可执行文件...".into(),
    );
    m.insert("cmd.upgrade.home_fail".into(), "无法确定你的主目录".into());
    m.insert(
        "cmd.upgrade.installed_user_dir".into(),
        "ntc 已安装到 {}（用户安装，无需管理员/sudo）。".into(),
    );
    m.insert(
        "cmd.upgrade.installer_launch".into(),
        "正在启动 ntc 安装程序（静默）...".into(),
    );
    m.insert(
        "cmd.upgrade.installer_launch_fail".into(),
        "启动安装程序失败：{}".into(),
    );
    m.insert(
        "cmd.upgrade.no_arch_asset".into(),
        "此架构没有预编译的 ntc 可执行文件（{}）。请从源码构建：cargo build --release".into(),
    );
    m.insert(
        "cmd.upgrade.no_asset_match".into(),
        "没有与平台键“{}”匹配的发布资源".into(),
    );
    m.insert(
        "cmd.upgrade.no_file_in_archive".into(),
        "下载的压缩包中找不到“{}”".into(),
    );
    m.insert(
        "cmd.upgrade.no_installer_asset".into(),
        "找不到可用于更新 ntc 的 Windows 安装程序资源".into(),
    );
    m.insert(
        "cmd.upgrade.path_exported".into(),
        "PATH 已更新到 {}".into(),
    );
}
