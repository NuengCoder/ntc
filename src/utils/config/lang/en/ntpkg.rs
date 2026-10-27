use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("ntpkg.add_to_path".into(), "Add to your PATH:".into());

    m.insert(
        "ntpkg.add_to_sys_path".into(),
        "Add to your system PATH (requires admin):".into(),
    );

    m.insert(
        "ntpkg.add_to_user_path".into(),
        "Add to your user PATH:".into(),
    );

    m.insert(
        "ntpkg.already_installed".into(),
        "{} {} is already installed at {}".into(),
    );

    m.insert(
        "ntpkg.available_hint".into(),
        " Use 'ntpkg available <language>' to list versions.".into(),
    );

    m.insert(
        "ntpkg.configuring".into(),
        "Configuring (./configure)...".into(),
    );

    m.insert("ntpkg.downloading".into(), "Downloading {}".into());

    m.insert(
        "ntpkg.enter_number".into(),
        "Enter number (1-{}) or 0: ".into(),
    );

    m.insert("ntpkg.extracting".into(), "Extracting...".into());

    m.insert("ntpkg.extracting_indent".into(), " Extracting...".into());

    m.insert(
        "ntpkg.extracting_source".into(),
        "Extracting source tarball...".into(),
    );

    m.insert("ntpkg.filter_label".into(), "(filter: \"{}\")".into());

    m.insert(
        "ntpkg.git_not_found".into(),
        "Git not found on PATH. Downloading MinGit...".into(),
    );

    m.insert(
        "ntpkg.help_usage".into(),
        "ntpkg \u{2014} ntc package manager for language runtimes".into(),
    );

    m.insert("ntpkg.help_usage_title".into(), "  Usage:".into());

    m.insert(
        "ntpkg.help_install".into(),
        "    ntpkg install <language> [version]   Download & install a runtime".into(),
    );

    m.insert(
        "ntpkg.help_uninstall".into(),
        "    ntpkg uninstall <language> [version]  Remove an installed runtime".into(),
    );

    m.insert(
        "ntpkg.help_list".into(),
        "    ntpkg list                           Show installed runtimes".into(),
    );

    m.insert(
        "ntpkg.help_use".into(),
        "    ntpkg use <language> <version>        Activate a specific version".into(),
    );

    m.insert(
        "ntpkg.help_which".into(),
        "    ntpkg which <language>                Show path to active runtime".into(),
    );

    m.insert(
        "ntpkg.help_available".into(),
        "    ntpkg available [language]             List languages or downloadable versions".into(),
    );

    m.insert(
        "ntpkg.help_help".into(),
        "    ntpkg help                            Show this help".into(),
    );

    m.insert(
        "ntpkg.ntpx_header".into(),
        "  ntpx \u{2014} run any installed package's binary".into(),
    );

    m.insert(
        "ntpkg.ntpx_usage_line".into(),
        "    ntpx <package> [binary] [args...]".into(),
    );

    m.insert("ntpkg.help_examples_title".into(), "  Examples:".into());

    m.insert(
        "ntpkg.help_ex_ntpx_go".into(),
        "    ntpx go version              # run 'go version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_py".into(),
        "    ntpx python --version        # run 'python --version'".into(),
    );

    m.insert(
        "ntpkg.help_ex_ntpx_msys2".into(),
        "    ntpx msys2 gcc --version     # run 'gcc --version' (explicit binary)".into(),
    );

    m.insert("ntpkg.available_title".into(), "Available packages".into());

    m.insert("ntpkg.package_info_title".into(), "Package info: {}".into());

    m.insert("ntpkg.active_label".into(), "Active:".into());

    m.insert("ntpkg.installed_label".into(), "Installed:".into());

    m.insert("ntpkg.location_label".into(), "Location:".into());

    m.insert("ntpkg.binaries_label".into(), "Binaries:".into());

    m.insert("ntpkg.not_set".into(), "(not set)".into());

    m.insert("ntpkg.none".into(), "(none)".into());

    m.insert("ntpkg.active_marker".into(), " (active)".into());

    m.insert(
        "ntpkg.info_usage".into(),
        "Usage: ntpkg info <language>".into(),
    );

    m.insert("ntpkg.install_failed".into(), "Install failed: {}".into());

    m.insert("ntpkg.install_loc".into(), "Install location: {}".into());

    m.insert(
        "ntpkg.install_usage".into(),
        "Usage: ntpkg install <language> [version]".into(),
    );

    m.insert("ntpkg.installed_msg".into(), "{} {} installed!".into());

    m.insert(
        "ntpkg.installing".into(),
        "Installing (make install)...".into(),
    );

    m.insert("ntpkg.latest_suffix".into(), "(latest)".into());

    m.insert(
        "ntpkg.launch_installer".into(),
        "Launching installer. Complete the setup, then close the installer window...".into(),
    );

    m.insert("ntpkg.magick_macos".into(), "ImageMagick on macOS:".into());

    m.insert(
        "ntpkg.magick_manual_reg".into(),
        "If installed, register manually: ntpkg use imagemagick <version>".into(),
    );

    m.insert(
        "ntpkg.magick_not_found".into(),
        "Could not auto-detect magick.exe after install.".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_incomplete".into(),
        "GCC installation may not have completed. You can install it later via pacman.".into(),
    );

    m.insert(
        "ntpkg.msys2_gcc_not_found".into(),
        "GCC not found yet. Run pacman to install it:".into(),
    );

    m.insert(
        "ntpkg.msys2_installed".into(),
        "MSYS2 installed! Install optional packages via pacman:".into(),
    );

    m.insert(
        "ntpkg.msys2_linux_gcc".into(),
        "On this platform, install GCC with your package manager:".into(),
    );

    m.insert(
        "ntpkg.msys2_running_pacman".into(),
        "Running: pacman -S mingw-w64-ucrt-x86_64-gcc".into(),
    );

    m.insert(
        "ntpkg.no_active_version".into(),
        "No active version for '{}' and no version specified".into(),
    );

    m.insert("ntpkg.no_match".into(), "No packages match '{}'".into());

    m.insert(
        "ntpkg.no_packages".into(),
        "No packages installed. Use 'ntpkg install <language>'".into(),
    );

    m.insert(
        "ntpkg.no_versions".into(),
        "No versions tracked for '{}'".into(),
    );

    m.insert(
        "ntpkg.no_versions_for".into(),
        "No versions available for '{}'".into(),
    );

    m.insert(
        "ntpkg.not_installed".into(),
        "{} {} is not installed".into(),
    );

    m.insert("ntpkg.now_using".into(), "Now using {} {}".into());

    m.insert(
        "ntpkg.npm_bin_in_path".into(),
        "npm global bin is in PATH".into(),
    );

    m.insert(
        "ntpkg.npm_detected".into(),
        "Node.js and npm detected".into(),
    );

    m.insert(
        "ntpkg.npm_not_detected".into(),
        "Node.js/npm not detected.".into(),
    );

    m.insert(
        "ntpkg.npm_section_node_first".into(),
        "  ── Install Node.js first ──".into(),
    );

    m.insert(
        "ntpkg.npm_node_found_at".into(),
        "  ✓ Node.js found at: {}".into(),
    );

    m.insert(
        "ntpkg.npm_node_not_found".into(),
        "  ✗ Node.js not found".into(),
    );

    m.insert(
        "ntpkg.npm_install_via_pkg".into(),
        "  Install via pkg:".into(),
    );

    m.insert(
        "ntpkg.npm_install_via_ntpkg".into(),
        "  Install via ntpkg:".into(),
    );

    m.insert("ntpkg.npm_ntpkg_cmd".into(), "    ntpkg install {}".into());

    m.insert("ntpkg.npm_cmd".into(), "    npm i -g {}".into());

    m.insert(
        "ntpkg.npm_or_download_nodejs".into(),
        "  Or download from: https://nodejs.org".into(),
    );

    m.insert(
        "ntpkg.npm_or_fnm_nvm_volta".into(),
        "  Or install via fnm/nvm/volta".into(),
    );

    m.insert("ntpkg.npm_found_at".into(), "  ✓ npm found at: {}".into());

    m.insert("ntpkg.npm_not_found".into(), "  ✗ npm not found".into());

    m.insert("ntpkg.npm_then_run".into(), "  Then run:".into());

    m.insert("ntpkg.npm_then_rerun".into(), "  Then re-run:".into());

    m.insert(
        "ntpkg.npm_bin_not_in_path".into(),
        "npm global bin not found in PATH ({})".into(),
    );

    m.insert("ntpkg.path_add_label".into(), "  Add to PATH:".into());

    m.insert(
        "ntpkg.npm_install_section".into(),
        "  ── Install {} ──".into(),
    );

    m.insert(
        "ntpkg.npm_run_cmd_label".into(),
        "  Run this command:".into(),
    );

    m.insert(
        "ntpkg.ntpx_usage".into(),
        "Usage: ntpx <package> [binary] [args...]".into(),
    );

    m.insert("ntpkg.package_count".into(), "{} packages".into());

    m.insert(
        "ntpkg.php_skip_register".into(),
        "Skipped registering PHP with ntpkg.".into(),
    );

    m.insert("ntpkg.removed_msg".into(), "{} {} removed".into());

    m.insert(
        "ntpkg.select_jdk_version".into(),
        "Select JDK {} version".into(),
    );

    m.insert("ntpkg.select_version".into(), "Select {} version".into());

    m.insert(
        "ntpkg.skip_register".into(),
        "Skipped registering {} with ntpkg.".into(),
    );

    m.insert(
        "ntpkg.termux_use_pkg".into(),
        "On Termux, install via pkg instead of ntpkg:".into(),
    );

    m.insert("ntpkg.termux_cmd".into(), "    pkg install {}".into());

    m.insert(
        "ntpkg.termux_no_pkg".into(),
        "No Termux pkg package available for '{}'.".into(),
    );

    m.insert(
        "ntpkg.src_act".into(),
        "act - Run GitHub Actions locally".into(),
    );

    m.insert(
        "ntpkg.src_cloudflare".into(),
        "cloudflared - Cloudflare Tunnel client".into(),
    );

    m.insert(
        "ntpkg.src_devops_tool".into(),
        "Azure DevOps CLI tool".into(),
    );

    m.insert(
        "ntpkg.src_docker".into(),
        "Docker - Container runtime".into(),
    );

    m.insert(
        "ntpkg.src_docker_compose".into(),
        "Docker Compose - Multi-container orchestration".into(),
    );

    m.insert(
        "ntpkg.src_dotnet".into(),
        ".NET SDK - Cross-platform development".into(),
    );

    m.insert("ntpkg.src_ffmpeg".into(), "FFmpeg - Media converter".into());

    m.insert(
        "ntpkg.src_freebuff".into(),
        "Freebuff - Free AI coding agent".into(),
    );

    m.insert("ntpkg.src_flutter".into(), "Flutter - UI toolkit".into());

    m.insert(
        "ntpkg.src_gh".into(),
        "GitHub CLI - GitHub from terminal".into(),
    );

    m.insert("ntpkg.src_git".into(), "Git - Version control".into());

    m.insert("ntpkg.src_go".into(), "Go - Programming language".into());

    m.insert(
        "ntpkg.src_helm".into(),
        "Helm - Kubernetes package manager".into(),
    );

    m.insert(
        "ntpkg.src_imagemagick".into(),
        "ImageMagick - Image manipulation".into(),
    );

    m.insert("ntpkg.src_java".into(), "Java - Runtime environment".into());

    m.insert(
        "ntpkg.src_kilocode".into(),
        "Kiloc - AI code assistant".into(),
    );

    m.insert(
        "ntpkg.src_kind".into(),
        "kind - Kubernetes in Docker".into(),
    );

    m.insert(
        "ntpkg.src_kotlin".into(),
        "Kotlin - Programming language".into(),
    );

    m.insert(
        "ntpkg.src_kubectl".into(),
        "kubectl - Kubernetes CLI".into(),
    );

    m.insert(
        "ntpkg.src_kustomize".into(),
        "Kustomize - Kubernetes config customization".into(),
    );

    m.insert("ntpkg.src_lua".into(), "Lua - Scripting language".into());

    m.insert(
        "ntpkg.src_mimocode".into(),
        "Mimic - AI coding agent".into(),
    );

    m.insert(
        "ntpkg.src_msys2".into(),
        "MSYS2 - Software distribution and building platform".into(),
    );

    m.insert("ntpkg.src_nim".into(), "Nim - Programming language".into());

    m.insert(
        "ntpkg.src_node".into(),
        "Node.js - JavaScript runtime".into(),
    );

    m.insert(
        "ntpkg.src_omniroute".into(),
        "OmniRoute - Visualization tool".into(),
    );

    m.insert(
        "ntpkg.src_opencode".into(),
        "opencode - CLI coding agent".into(),
    );

    m.insert("ntpkg.src_php".into(), "PHP - Scripting language".into());

    m.insert(
        "ntpkg.src_python".into(),
        "Python - Programming language".into(),
    );

    m.insert(
        "ntpkg.src_ruby".into(),
        "Ruby - Programming language".into(),
    );

    m.insert(
        "ntpkg.src_rust".into(),
        "Rust - Systems programming language".into(),
    );

    m.insert(
        "ntpkg.src_scala".into(),
        "Scala - Programming language".into(),
    );

    m.insert(
        "ntpkg.src_sqlite".into(),
        "SQLite - Embedded database".into(),
    );

    m.insert(
        "ntpkg.src_sqlitestudio".into(),
        "SQLiteStudio - SQLite manager".into(),
    );

    m.insert(
        "ntpkg.src_swift".into(),
        "Swift - Programming language".into(),
    );

    m.insert("ntpkg.src_vscode".into(), "VS Code - Code editor".into());

    m.insert("ntpkg.src_zig".into(), "Zig - Programming language".into());

    m.insert(
        "ntpkg.src_tesseract".into(),
        "Tesseract - OCR engine".into(),
    );

    m.insert(
        "ntpkg.tesseract_running_installer".into(),
        "Running Tesseract installer silently (elevated)...".into(),
    );

    m.insert("ntpkg.tesseract_not_found".into(), "Could not auto-detect tesseract.exe after install. Add its folder to PATH with: usrpath add <dir> (then 'mode admin' + 'syspath add <dir>').".into());

    m.insert(
        "ntpkg.tesseract_linux".into(),
        "Install Tesseract on Linux via your package manager:".into(),
    );

    m.insert(
        "ntpkg.tesseract_macos".into(),
        "Install Tesseract on macOS via Homebrew:".into(),
    );

    m.insert(
        "ntpkg.tesseract_unsupported".into(),
        "Tesseract installation is not supported on this platform.".into(),
    );

    m.insert(
        "ntpkg.src_nginx".into(),
        "nginx - Web server / reverse proxy".into(),
    );

    m.insert(
        "ntpkg.nginx_unsupported".into(),
        "nginx installation is not supported on this platform.".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_linux".into(),
        "Install nginx on Linux via your package manager:".into(),
    );

    m.insert(
        "ntpkg.nginx_system_hint_macos".into(),
        "Install nginx on macOS via Homebrew:".into(),
    );

    m.insert(
        "ntpkg.nginx_termux".into(),
        "Install nginx on Termux via its package manager:".into(),
    );

    m.insert(
        "ntpkg.nginx_choose_method".into(),
        "No official prebuilt Linux/macOS binary. Choose an install method:".into(),
    );

    m.insert(
        "ntpkg.nginx_option_source".into(),
        "Build from source (downloads nginx source tarball)".into(),
    );

    m.insert(
        "ntpkg.nginx_option_system".into(),
        "Install via system package manager".into(),
    );

    m.insert(
        "ntpkg.nginx_run_commands_system".into(),
        "Install nginx with the package-manager commands shown above (not tracked by ntpkg)."
            .into(),
    );

    m.insert("ntpkg.nginx_build_confirm".into(), "Build nginx {} from source into the ntpkg packages dir? Requires make/gcc + PCRE/Zlib/OpenSSL dev packages.".into());

    m.insert(
        "ntpkg.nginx_build_deps".into(),
        "Build deps hint: build-essential libpcre2-dev zlib1g-dev libssl-dev (Debian/Ubuntu)."
            .into(),
    );

    m.insert(
        "ntpkg.nginx_configuring".into(),
        "Running ./configure...".into(),
    );

    m.insert("ntpkg.nginx_compiling".into(), "Compiling...".into());

    m.insert(
        "ntpkg.supported_langs".into(),
        " Supported languages:".into(),
    );

    m.insert(
        "ntpkg.uninstall_failed".into(),
        "Uninstall failed: {}".into(),
    );

    m.insert(
        "ntpkg.uninstall_usage".into(),
        "Usage: ntpkg uninstall <language> [version]".into(),
    );

    m.insert(
        "ntpkg.unknown_subcmd".into(),
        "Unknown ntpkg subcommand: '{}'. Use 'ntpkg help'".into(),
    );

    m.insert(
        "ntpkg.use_usage".into(),
        "Usage: ntpkg use <language> <version>".into(),
    );

    m.insert(
        "ntpkg.which_usage".into(),
        "Usage: ntpkg which <language> [version]".into(),
    );
}
