use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.tmprt.unknown_sub".into(),
        "未知的 tmprt 子命令：{}".into(),
    );

    m.insert("cmd.tmprt.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.tmprt.usage_com".into(),
        "  tmprt com [秒]        实时硬件温度监控".into(),
    );

    m.insert(
        "cmd.tmprt.usage_weather".into(),
        "  tmprt weather [城市]  通过 OpenWeather API 获取当前天气".into(),
    );

    m.insert(
        "cmd.tmprt.usage_key".into(),
        "  tmprt key [API_KEY]   保存/显示 OpenWeather API 密钥".into(),
    );

    m.insert(
        "cmd.tmprt.usage_city".into(),
        "  tmprt city [名称]     保存/显示天气默认城市".into(),
    );

    m.insert(
        "cmd.tmprt.usage_sensors".into(),
        "  tmprt sensors <子命令>  管理硬件温度传感器后端".into(),
    );

    m.insert("cmd.tmprt.examples_title".into(), "示例：".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert(
        "cmd.tmprt.example4".into(),
        "tmprt key 1234567890abcdef".into(),
    );

    m.insert(
        "cmd.tmprt.com.no_sensors".into(),
        "未检测到温度传感器。".into(),
    );

    m.insert(
        "cmd.tmprt.com.no_sensors_hint".into(),
        "在 Windows 上可能需要管理员权限；在 Linux 上需要存在 hwmon/thermal 传感器。".into(),
    );

    m.insert("cmd.tmprt.com.footer".into(), " 按 q 退出".into());

    m.insert(
        "cmd.tmprt.com.lhm_missing".into(),
        "未找到 CPU 温度后端（此机器的 ACPI 未暴露任何传感器）。".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_prompt".into(),
        "是否安装 LibreHardwareMonitorLib（自动下载，约 7 MB）以获取 CPU/GPU/主板/硬盘温度？"
            .into(),
    );

    m.insert("cmd.tmprt.com.lhm_confirm".into(), "立即安装？[Y/n]".into());

    m.insert(
        "cmd.tmprt.com.lhm_installing".into(),
        "正在下载并安装 LibreHardwareMonitorLib...".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installed".into(),
        "LibreHardwareMonitorLib 已安装。".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_install_failed".into(),
        "LibreHardwareMonitorLib 安装失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.com.admin_hint".into(),
        "CPU 温度需 PawnIO 驱动与管理员权限。运行 'mode admin' 以管理员身份重新启动 ntc。".into(),
    );

    m.insert(
        "cmd.tmprt.com.pawnio_hint".into(),
        "CPU 温度需要 PawnIO 驱动。运行 'tmprt sensors install'（需要管理员权限）。".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key".into(),
        "未找到 OpenWeather API 密钥。".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key_hint".into(),
        "请设置：tmprt key <API_KEY>（或导出 OPENWEATHER_API_KEY）".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city".into(),
        "未提供城市且未配置默认城市。".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city_hint".into(),
        "用法：tmprt weather <城市> 或设置默认：tmprt city <名称>".into(),
    );

    m.insert(
        "cmd.tmprt.weather.request_failed".into(),
        "天气请求失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.read_failed".into(),
        "读取天气响应失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.bad_key".into(),
        "OpenWeather 拒绝了该 API 密钥（HTTP 401）。".into(),
    );

    m.insert(
        "cmd.tmprt.weather.city_not_found".into(),
        "未找到城市：{}（HTTP 404）".into(),
    );

    m.insert(
        "cmd.tmprt.weather.http_error".into(),
        "OpenWeather 错误（HTTP {}）：{}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.parse_failed".into(),
        "解析天气数据失败：{}".into(),
    );

    m.insert("cmd.tmprt.weather.title".into(), "天气：".into());

    m.insert("cmd.tmprt.weather.condition".into(), "天气状况".into());

    m.insert("cmd.tmprt.weather.temp".into(), "温度".into());

    m.insert("cmd.tmprt.weather.feels".into(), "体感".into());

    m.insert("cmd.tmprt.weather.range".into(), "最低/最高".into());

    m.insert("cmd.tmprt.weather.humidity".into(), "湿度".into());

    m.insert("cmd.tmprt.weather.pressure".into(), "气压".into());

    m.insert("cmd.tmprt.weather.wind".into(), "风力".into());

    m.insert("cmd.tmprt.weather.visibility".into(), "能见度".into());

    m.insert("cmd.tmprt.weather.clouds".into(), "云量".into());

    m.insert("cmd.tmprt.weather.sunrise".into(), "日出".into());

    m.insert("cmd.tmprt.weather.sunset".into(), "日落".into());

    m.insert("cmd.tmprt.weather.coords".into(), "坐标".into());

    m.insert(
        "cmd.tmprt.key.env_source".into(),
        "OpenWeather API 密钥（来自 OPENWEATHER_API_KEY 环境变量）：{}".into(),
    );

    m.insert(
        "cmd.tmprt.key.status".into(),
        "OpenWeather API 密钥：{}".into(),
    );

    m.insert(
        "cmd.tmprt.key.not_set".into(),
        "未配置 OpenWeather API 密钥。".into(),
    );

    m.insert(
        "cmd.tmprt.key.saved".into(),
        "OpenWeather API 密钥已保存：{}".into(),
    );

    m.insert("cmd.tmprt.city.status".into(), "默认天气城市：{}".into());

    m.insert(
        "cmd.tmprt.city.not_set".into(),
        "未配置默认天气城市。".into(),
    );

    m.insert(
        "cmd.tmprt.city.saved".into(),
        "默认天气城市已保存：{}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_title".into(),
        "tmprt sensors 子命令：".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_status".into(),
        "  tmprt sensors status     显示可用的传感器后端".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_install".into(),
        "  tmprt sensors install    下载 LibreHardwareMonitorLib（Windows）".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_uninstall".into(),
        "  tmprt sensors uninstall  删除已下载的 LibreHardwareMonitorLib 文件".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_title".into(),
        "硬件传感器后端：".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_acpi".into(),
        "ACPI/hwmon（sysinfo）".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_nvidia".into(),
        "NVIDIA（nvidia-smi）".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_lhm".into(),
        "LibreHardwareMonitor".into(),
    );

    m.insert("cmd.tmprt.sensors.src_unavailable".into(), "不可用".into());

    m.insert("cmd.tmprt.sensors.src_installed".into(), "已安装".into());

    m.insert(
        "cmd.tmprt.sensors.src_not_installed".into(),
        "未安装".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_hint".into(),
        "运行 'tmprt sensors install' 可启用基于 LibreHardwareMonitor 的传感器（Windows）。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.installed".into(),
        "LibreHardwareMonitorLib 已安装。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.install_failed".into(),
        "LibreHardwareMonitorLib 安装失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstalled".into(),
        "LibreHardwareMonitorLib 文件已删除。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstall_failed".into(),
        "删除 LibreHardwareMonitorLib 文件失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.not_installed".into(),
        "LibreHardwareMonitorLib 尚未安装。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.windows_only".into(),
        "LibreHardwareMonitor 仅支持 Windows。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.downloading".into(),
        "正在下载 LibreHardwareMonitorLib".into(),
    );

    m.insert("cmd.tmprt.sensors.elevation".into(), "权限级别".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "管理员".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "标准用户".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "通过 LHM 读取 CPU 温度需要管理员权限（PawnIO 驱动）。运行 'mode admin' 以管理员身份重新启动 ntc。".into());

    m.insert("cmd.tmprt.sensors.src_pawnio".into(), "PawnIO 驱动".into());

    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "未安装 PawnIO 驱动 - CPU 温度将保持 '--'。运行 'tmprt sensors install' 安装（需要管理员权限）。".into());

    m.insert(
        "cmd.tmprt.sensors.pawnio_present".into(),
        "PawnIO 驱动已安装。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_elevating".into(),
        "正在提升权限以安装 PawnIO 驱动（请接受 UAC 提示）...".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_downloading".into(),
        "正在下载 PawnIO 驱动".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_installed".into(),
        "PawnIO 驱动已安装。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_reboot".into(),
        "PawnIO 驱动已安装 - 请重启电脑以启用。".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_failed".into(),
        "PawnIO 驱动安装失败：{}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_debug".into(),
        "  tmprt sensors debug         转储检测到的硬件与传感器值（诊断）".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_title".into(),
        "传感器诊断（LibreHardwareMonitor）：".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_none".into(),
        "LibreHardwareMonitor 尚未安装 - 请先运行 'tmprt sensors install'。".into(),
    );
}
