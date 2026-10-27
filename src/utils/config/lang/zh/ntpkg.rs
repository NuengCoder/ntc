use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ntpkg.add_to_path".into(), "添加到您的PATH：".into());

    m.insert(
        "ntpkg.add_to_sys_path".into(),
        "添加到您的系统PATH（需要管理员权限）：".into(),
    );

    m.insert(
        "ntpkg.add_to_user_path".into(),
        "添加到您的用户PATH：".into(),
    );

    m.insert("ntpkg.already_installed".into(), "{} {} 已安装在 {}".into());

    m.insert(
        "ntpkg.available_hint".into(),
        " 使用 'ntpkg available <语言>' 列出版本。".into(),
    );

    m.insert(
        "ntpkg.configuring".into(),
        "正在配置（./configure）...".into(),
    );

    m.insert("ntpkg.downloading".into(), "正在下载 {}".into());

    m.insert("ntpkg.enter_number".into(), "输入数字（1-{}）或0：".into());

    m.insert("ntpkg.extracting".into(), "正在解压...".into());

    m.insert("ntpkg.extracting_indent".into(), " 正在解压...".into());

    m.insert(
        "ntpkg.extracting_source".into(),
        "正在解压源代码压缩包...".into(),
    );

    m.insert("ntpkg.filter_label".into(), "（过滤：\"{}\"）".into());

    m.insert(
        "ntpkg.git_not_found".into(),
        "在PATH中未找到Git。正在下载MinGit...".into(),
    );

    m.insert(
        "ntpkg.help_usage".into(),
        "ntpkg \u{2014} ntc 语言运行时包管理器".into(),
    );

    m.insert("ntpkg.help_usage_title".into(), "  用法：".into());

    m.insert(
        "ntpkg.help_install".into(),
        "    ntpkg install <语言> [版本]   下载并安装运行时".into(),
    );

    m.insert(
        "ntpkg.help_uninstall".into(),
        "    ntpkg uninstall <语言> [版本]  卸载已安装的运行时".into(),
    );

    m.insert(
        "ntpkg.help_list".into(),
        "    ntpkg list                          显示已安装的运行时".into(),
    );

    m.insert(
        "ntpkg.help_use".into(),
        "    ntpkg use <语言> <版本>       激活特定版本".into(),
    );

    m.insert(
        "ntpkg.help_which".into(),
        "    ntpkg which <语言>               显示活动运行时的路径".into(),
    );

    m.insert(
        "ntpkg.help_available".into(),
        "    ntpkg available [语言]            列出语言或可下载的版本".into(),
    );

    m.insert(
        "ntpkg.help_help".into(),
        "    ntpkg help                           显示此帮助".into(),
    );

    m.insert(
        "ntpkg.ntpx_header".into(),
        "  ntpx \u{2014} 运行已安装软件包的二进制文件".into(),
    );

    m.insert(
        "ntpkg.ntpx_usage_line".into(),
        "    ntpx <包> [二进制] [参数...]".into(),
    );

    m.insert("ntpkg.help_examples_title".into(), "  示例：".into());

    m.insert(
        "ntpkg.help_ex_ntpx_go".into(),
        "    ntpx go version              # 运行 'go version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_py".into(),
        "    ntpx python --version        # 运行 'python --version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_msys2".into(),
        "    ntpx msys2 gcc --version     # 运行 'gcc --version'（显式指定二进制）".into(),
    );

    m.insert("ntpkg.available_title".into(), "可用软件包".into());

    m.insert("ntpkg.package_info_title".into(), "软件包信息：{}".into());

    m.insert("ntpkg.active_label".into(), "活动版本：".into());

    m.insert("ntpkg.installed_label".into(), "已安装：".into());

    m.insert("ntpkg.location_label".into(), "位置：".into());

    m.insert("ntpkg.binaries_label".into(), "二进制：".into());

    m.insert("ntpkg.not_set".into(), "（未设置）".into());

    m.insert("ntpkg.none".into(), "（无）".into());

    m.insert("ntpkg.active_marker".into(), " （活动）".into());

    m.insert("ntpkg.info_usage".into(), "用法：ntpkg info <语言>".into());

    m.insert("ntpkg.install_failed".into(), "安装失败：{}".into());

    m.insert("ntpkg.install_loc".into(), "安装位置：{}".into());

    m.insert(
        "ntpkg.install_usage".into(),
        "用法：ntpkg install <语言> [版本]".into(),
    );

    m.insert("ntpkg.installed_msg".into(), "{} {} 已安装！".into());

    m.insert(
        "ntpkg.installing".into(),
        "正在安装（make install）...".into(),
    );

    m.insert("ntpkg.latest_suffix".into(), "（最新）".into());

    m.insert(
        "ntpkg.launch_installer".into(),
        "正在启动安装程序。完成设置后关闭安装程序窗口...".into(),
    );

    m.insert("ntpkg.magick_macos".into(), "macOS上的ImageMagick：".into());

    m.insert(
        "ntpkg.magick_manual_reg".into(),
        "如果已安装，请手动注册：ntpkg use imagemagick <版本>".into(),
    );

    m.insert(
        "ntpkg.magick_not_found".into(),
        "安装后无法自动检测 magick.exe。".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_incomplete".into(),
        "GCC安装可能未完成。您可以稍后通过pacman安装。".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_not_found".into(),
        "尚未找到GCC。运行pacman安装：".into(),
    );

    m.insert(
        "ntpkg.msys2_installed".into(),
        "MSYS2已安装！通过pacman安装可选包：".into(),
    );

    m.insert(
        "ntpkg.msys2_linux_gcc".into(),
        "在此平台上，请使用包管理器安装GCC：".into(),
    );

    m.insert(
        "ntpkg.msys2_running_pacman".into(),
        "正在运行：pacman -S mingw-w64-ucrt-x86_64-gcc".into(),
    );

    m.insert(
        "ntpkg.no_active_version".into(),
        "'{}' 没有活动版本且未指定版本".into(),
    );

    m.insert("ntpkg.no_match".into(), "没有匹配 '{}' 的包".into());

    m.insert(
        "ntpkg.no_packages".into(),
        "未安装任何包。使用 'ntpkg install <语言>'".into(),
    );

    m.insert("ntpkg.no_versions".into(), "'{}' 没有追踪的版本".into());

    m.insert("ntpkg.no_versions_for".into(), "'{}' 没有可用版本".into());

    m.insert("ntpkg.not_installed".into(), "{} {} 未安装".into());

    m.insert("ntpkg.now_using".into(), "正在使用 {} {}".into());

    m.insert(
        "ntpkg.npm_bin_in_path".into(),
        "npm全局二进制文件在PATH中".into(),
    );

    m.insert("ntpkg.npm_detected".into(), "已检测到Node.js和npm".into());

    m.insert(
        "ntpkg.npm_not_detected".into(),
        "未检测到Node.js/npm。".into(),
    );

    m.insert(
        "ntpkg.npm_section_node_first".into(),
        "  ── 先安装 Node.js ──".into(),
    );

    m.insert(
        "ntpkg.npm_node_found_at".into(),
        "  ✓ 在 {} 找到 Node.js".into(),
    );

    m.insert(
        "ntpkg.npm_node_not_found".into(),
        "  ✗ 未找到 Node.js".into(),
    );

    m.insert(
        "ntpkg.npm_install_via_pkg".into(),
        "  通过 pkg 安装：".into(),
    );

    m.insert(
        "ntpkg.npm_install_via_ntpkg".into(),
        "  通过 ntpkg 安装：".into(),
    );

    m.insert("ntpkg.npm_ntpkg_cmd".into(), "    ntpkg install {}".into());

    m.insert("ntpkg.npm_cmd".into(), "    npm i -g {}".into());

    m.insert(
        "ntpkg.npm_or_download_nodejs".into(),
        "  或从以下地址下载：https://nodejs.org".into(),
    );

    m.insert(
        "ntpkg.npm_or_fnm_nvm_volta".into(),
        "  或通过 fnm/nvm/volta 安装".into(),
    );

    m.insert("ntpkg.npm_found_at".into(), "  ✓ 在 {} 找到 npm".into());

    m.insert("ntpkg.npm_not_found".into(), "  ✗ 未找到 npm".into());

    m.insert("ntpkg.npm_then_run".into(), "  然后运行：".into());

    m.insert("ntpkg.npm_then_rerun".into(), "  然后重新运行：".into());

    m.insert(
        "ntpkg.npm_bin_not_in_path".into(),
        "npm 全局二进制目录不在 PATH 中（{}）".into(),
    );

    m.insert("ntpkg.path_add_label".into(), "  添加到 PATH：".into());

    m.insert("ntpkg.npm_install_section".into(), "  ── 安装 {} ──".into());

    m.insert("ntpkg.npm_run_cmd_label".into(), "  运行此命令：".into());

    m.insert(
        "ntpkg.ntpx_usage".into(),
        "用法：ntpx <包> [二进制] [参数...]".into(),
    );

    m.insert("ntpkg.package_count".into(), "{} 个包".into());

    m.insert(
        "ntpkg.php_skip_register".into(),
        "跳过用 ntpkg 注册PHP。".into(),
    );

    m.insert("ntpkg.removed_msg".into(), "{} {} 已删除".into());

    m.insert("ntpkg.select_jdk_version".into(), "选择 JDK {} 版本".into());

    m.insert("ntpkg.select_version".into(), "选择 {} 版本".into());

    m.insert(
        "ntpkg.skip_register".into(),
        "跳过用 ntpkg 注册 {}。".into(),
    );

    m.insert(
        "ntpkg.termux_use_pkg".into(),
        "在 Termux 上请用 pkg 而非 ntpkg 安装：".into(),
    );

    m.insert("ntpkg.termux_cmd".into(), "    pkg install {}".into());

    m.insert(
        "ntpkg.termux_no_pkg".into(),
        "Termux 中没有与 '{}' 对应的 pkg 包。".into(),
    );

    m.insert(
        "ntpkg.src_act".into(),
        "act - 本地运行 GitHub Actions".into(),
    );

    m.insert(
        "ntpkg.src_cloudflare".into(),
        "cloudflared - Cloudflare 隧道客户端".into(),
    );

    m.insert(
        "ntpkg.src_devops_tool".into(),
        "Azure DevOps CLI 工具".into(),
    );

    m.insert("ntpkg.src_docker".into(), "Docker - 容器运行时".into());

    m.insert(
        "ntpkg.src_docker_compose".into(),
        "Docker Compose - 多容器编排".into(),
    );

    m.insert("ntpkg.src_dotnet".into(), ".NET SDK - 跨平台开发".into());

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - 媒体转换器".into());

    m.insert(
        "ntpkg.src_freebuff".into(),
        "Freebuff - 免费 AI 编码代理".into(),
    );

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI 工具包".into());

    m.insert("ntpkg.src_gh".into(), "GitHub CLI - 终端使用 GitHub".into());

    m.insert("ntpkg.src_git".into(), "Git - 版本控制".into());

    m.insert("ntpkg.src_go".into(), "Go - 编程语言".into());

    m.insert("ntpkg.src_helm".into(), "Helm - Kubernetes 包管理器".into());

    m.insert(
        "ntpkg.src_imagemagick".into(),
        "ImageMagick - 图像处理".into(),
    );

    m.insert("ntpkg.src_java".into(), "Java - 运行时环境".into());

    m.insert("ntpkg.src_kilocode".into(), "Kiloc - AI 代码助手".into());

    m.insert(
        "ntpkg.src_kind".into(),
        "kind - Kubernetes（Docker 内）".into(),
    );

    m.insert("ntpkg.src_kotlin".into(), "Kotlin - 编程语言".into());

    m.insert(
        "ntpkg.src_kubectl".into(),
        "kubectl - Kubernetes 命令行工具".into(),
    );

    m.insert(
        "ntpkg.src_kustomize".into(),
        "Kustomize - Kubernetes 配置自定义".into(),
    );

    m.insert("ntpkg.src_lua".into(), "Lua - 脚本语言".into());

    m.insert("ntpkg.src_mimocode".into(), "Mimic - AI 编码代理".into());

    m.insert(
        "ntpkg.src_msys2".into(),
        "MSYS2 - 软件发行与构建平台".into(),
    );

    m.insert("ntpkg.src_nim".into(), "Nim - 编程语言".into());

    m.insert(
        "ntpkg.src_node".into(),
        "Node.js - JavaScript 运行时".into(),
    );

    m.insert(
        "ntpkg.src_omniroute".into(),
        "OmniRoute - 可视化工具".into(),
    );

    m.insert(
        "ntpkg.src_opencode".into(),
        "opencode - CLI 编码代理".into(),
    );

    m.insert("ntpkg.src_php".into(), "PHP - 脚本语言".into());

    m.insert("ntpkg.src_python".into(), "Python - 编程语言".into());

    m.insert("ntpkg.src_ruby".into(), "Ruby - 编程语言".into());

    m.insert("ntpkg.src_rust".into(), "Rust - 系统编程语言".into());

    m.insert("ntpkg.src_scala".into(), "Scala - 编程语言".into());

    m.insert("ntpkg.src_sqlite".into(), "SQLite - 嵌入式数据库".into());

    m.insert(
        "ntpkg.src_sqlitestudio".into(),
        "SQLiteStudio - SQLite 管理器".into(),
    );

    m.insert("ntpkg.src_swift".into(), "Swift - 编程语言".into());

    m.insert("ntpkg.src_vscode".into(), "VS Code - 代码编辑器".into());

    m.insert("ntpkg.src_zig".into(), "Zig - 编程语言".into());

    m.insert("ntpkg.src_tesseract".into(), "Tesseract - OCR 引擎".into());

    m.insert(
        "ntpkg.tesseract_running_installer".into(),
        "正在以静默方式运行 Tesseract 安装程序（提升权限）...".into(),
    );

    m.insert("ntpkg.tesseract_not_found".into(), "安装后无法自动检测 tesseract.exe。请用以下命令将其所在文件夹添加到 PATH：usrpath add <目录>（然后执行 'mode admin' + 'syspath add <目录>'）。".into());

    m.insert(
        "ntpkg.tesseract_linux".into(),
        "在 Linux 上通过包管理器安装 Tesseract：".into(),
    );

    m.insert(
        "ntpkg.tesseract_macos".into(),
        "在 macOS 上通过 Homebrew 安装 Tesseract：".into(),
    );

    m.insert(
        "ntpkg.tesseract_unsupported".into(),
        "此平台不支持安装 Tesseract。".into(),
    );

    m.insert(
        "ntpkg.src_nginx".into(),
        "nginx - Web 服务器 / 反向代理".into(),
    );

    m.insert(
        "ntpkg.nginx_unsupported".into(),
        "此平台不支持安装 nginx。".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_linux".into(),
        "在 Linux 上通过包管理器安装 nginx：".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_macos".into(),
        "在 macOS 上通过 Homebrew 安装 nginx：".into(),
    );

    m.insert(
        "ntpkg.nginx_termux".into(),
        "在 Termux 上通过其包管理器安装 nginx：".into(),
    );

    m.insert(
        "ntpkg.nginx_choose_method".into(),
        "没有官方的 Linux/macOS 预编译二进制。请选择安装方式：".into(),
    );

    m.insert(
        "ntpkg.nginx_option_source".into(),
        "从源码构建（下载 nginx 源码包）".into(),
    );

    m.insert(
        "ntpkg.nginx_option_system".into(),
        "通过系统包管理器安装".into(),
    );

    m.insert(
        "ntpkg.nginx_run_commands_system".into(),
        "使用上面显示的包管理器命令安装 nginx（不受 ntpkg 跟踪）。".into(),
    );

    m.insert(
        "ntpkg.nginx_build_confirm".into(),
        "将 nginx {} 源码构建到 ntpkg 包目录？需要 make/gcc 以及 PCRE/Zlib/OpenSSL 开发包。".into(),
    );

    m.insert(
        "ntpkg.nginx_build_deps".into(),
        "构建依赖提示：build-essential libpcre2-dev zlib1g-dev libssl-dev (Debian/Ubuntu)。".into(),
    );

    m.insert(
        "ntpkg.nginx_configuring".into(),
        "正在执行 ./configure...".into(),
    );

    m.insert("ntpkg.nginx_compiling".into(), "正在编译...".into());

    m.insert("ntpkg.supported_langs".into(), " 支持的语言：".into());

    m.insert("ntpkg.uninstall_failed".into(), "卸载失败：{}".into());

    m.insert(
        "ntpkg.uninstall_usage".into(),
        "用法：ntpkg uninstall <语言> [版本]".into(),
    );

    m.insert(
        "ntpkg.unknown_subcmd".into(),
        "未知的 ntpkg 子命令：'{}'。使用 'ntpkg help'".into(),
    );

    m.insert(
        "ntpkg.use_usage".into(),
        "用法：ntpkg use <语言> <版本>".into(),
    );

    m.insert(
        "ntpkg.which_usage".into(),
        "用法：ntpkg which <语言> [版本]".into(),
    );
}
