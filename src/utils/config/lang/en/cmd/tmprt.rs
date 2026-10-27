use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.tmprt.unknown_sub".into(),
        "Unknown tmprt subcommand: {}".into(),
    );

    m.insert("cmd.tmprt.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.tmprt.usage_com".into(),
        "  tmprt com [secs]        Live hardware temperature monitor".into(),
    );

    m.insert(
        "cmd.tmprt.usage_weather".into(),
        "  tmprt weather [city]    Current weather via OpenWeather API".into(),
    );

    m.insert(
        "cmd.tmprt.usage_key".into(),
        "  tmprt key [API_KEY]     Save/display the OpenWeather API key".into(),
    );

    m.insert(
        "cmd.tmprt.usage_city".into(),
        "  tmprt city [name]       Save/display the default weather city".into(),
    );

    m.insert(
        "cmd.tmprt.usage_sensors".into(),
        "  tmprt sensors <sub>     Manage hardware sensor backends".into(),
    );

    m.insert("cmd.tmprt.examples_title".into(), "Examples:".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert(
        "cmd.tmprt.example4".into(),
        "tmprt key 1234567890abcdef".into(),
    );

    m.insert(
        "cmd.tmprt.com.no_sensors".into(),
        "No temperature sensors detected.".into(),
    );

    m.insert("cmd.tmprt.com.no_sensors_hint".into(), "On Windows this may require administrator rights; on Linux a hwmon/thermal sensor must be present.".into());

    m.insert("cmd.tmprt.com.footer".into(), " Press q to quit".into());

    m.insert(
        "cmd.tmprt.com.lhm_missing".into(),
        "No CPU temperature backend found (ACPI exposes no sensor on this machine).".into(),
    );

    m.insert("cmd.tmprt.com.lhm_prompt".into(), "Install LibreHardwareMonitorLib (auto-downloaded, ~7 MB) for CPU/GPU/board/storage temperatures?".into());

    m.insert(
        "cmd.tmprt.com.lhm_confirm".into(),
        "Install now? [Y/n]".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installing".into(),
        "Downloading and installing LibreHardwareMonitorLib...".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installed".into(),
        "LibreHardwareMonitorLib installed.".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_install_failed".into(),
        "LibreHardwareMonitorLib install failed: {}".into(),
    );

    m.insert("cmd.tmprt.com.admin_hint".into(), "CPU temps need the PawnIO driver and an elevated process. Run 'mode admin' to relaunch ntc elevated.".into());

    m.insert(
        "cmd.tmprt.com.pawnio_hint".into(),
        "CPU temps need the PawnIO driver. Run 'tmprt sensors install' (needs Administrator)."
            .into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key".into(),
        "No OpenWeather API key found.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key_hint".into(),
        "Set one with: tmprt key <API_KEY>  (or export OPENWEATHER_API_KEY)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city".into(),
        "No city given and no default city configured.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city_hint".into(),
        "Usage: tmprt weather <city>   or set a default: tmprt city <name>".into(),
    );

    m.insert(
        "cmd.tmprt.weather.request_failed".into(),
        "Weather request failed: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.read_failed".into(),
        "Failed to read weather response: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.bad_key".into(),
        "OpenWeather rejected the API key (HTTP 401).".into(),
    );

    m.insert(
        "cmd.tmprt.weather.city_not_found".into(),
        "City not found: {}  (HTTP 404)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.http_error".into(),
        "OpenWeather error (HTTP {}): {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.parse_failed".into(),
        "Failed to parse weather data: {}".into(),
    );

    m.insert("cmd.tmprt.weather.title".into(), "Weather:".into());

    m.insert("cmd.tmprt.weather.condition".into(), "Condition".into());

    m.insert("cmd.tmprt.weather.temp".into(), "Temperature".into());

    m.insert("cmd.tmprt.weather.feels".into(), "feels like".into());

    m.insert("cmd.tmprt.weather.range".into(), "Min / Max".into());

    m.insert("cmd.tmprt.weather.humidity".into(), "Humidity".into());

    m.insert("cmd.tmprt.weather.pressure".into(), "Pressure".into());

    m.insert("cmd.tmprt.weather.wind".into(), "Wind".into());

    m.insert("cmd.tmprt.weather.visibility".into(), "Visibility".into());

    m.insert("cmd.tmprt.weather.clouds".into(), "Clouds".into());

    m.insert("cmd.tmprt.weather.sunrise".into(), "Sunrise".into());

    m.insert("cmd.tmprt.weather.sunset".into(), "Sunset".into());

    m.insert("cmd.tmprt.weather.coords".into(), "Coords".into());

    m.insert(
        "cmd.tmprt.key.env_source".into(),
        "OpenWeather API key (from OPENWEATHER_API_KEY env): {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.status".into(),
        "OpenWeather API key: {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.not_set".into(),
        "No OpenWeather API key is configured.".into(),
    );

    m.insert(
        "cmd.tmprt.key.saved".into(),
        "OpenWeather API key saved: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.status".into(),
        "Default weather city: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.not_set".into(),
        "No default weather city is configured.".into(),
    );

    m.insert(
        "cmd.tmprt.city.saved".into(),
        "Default weather city saved: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_title".into(),
        "tmprt sensors subcommands:".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_status".into(),
        "  tmprt sensors status        Show which sensor backends are available".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_install".into(),
        "  tmprt sensors install       Download LibreHardwareMonitorLib (Windows)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_uninstall".into(),
        "  tmprt sensors uninstall     Remove the downloaded LibreHardwareMonitorLib files".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_title".into(),
        "Hardware sensor backends:".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_acpi".into(),
        "ACPI/hwmon (sysinfo)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_nvidia".into(),
        "NVIDIA (nvidia-smi)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_lhm".into(),
        "LibreHardwareMonitor".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.src_unavailable".into(),
        "unavailable".into(),
    );

    m.insert("cmd.tmprt.sensors.src_installed".into(), "installed".into());

    m.insert(
        "cmd.tmprt.sensors.src_not_installed".into(),
        "not installed".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_hint".into(),
        "Run 'tmprt sensors install' to enable LibreHardwareMonitor-based sensors (Windows)."
            .into(),
    );

    m.insert(
        "cmd.tmprt.sensors.installed".into(),
        "LibreHardwareMonitorLib installed.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.install_failed".into(),
        "LibreHardwareMonitorLib install failed: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstalled".into(),
        "LibreHardwareMonitorLib files removed.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstall_failed".into(),
        "Failed to remove LibreHardwareMonitorLib files: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.not_installed".into(),
        "LibreHardwareMonitorLib is not installed.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.windows_only".into(),
        "LibreHardwareMonitor is only supported on Windows.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.downloading".into(),
        "Downloading LibreHardwareMonitorLib".into(),
    );

    m.insert("cmd.tmprt.sensors.elevation".into(), "Elevation".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "Administrator".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "standard user".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "CPU temps via LHM need an elevated process (PawnIO driver). Run 'mode admin' to relaunch ntc elevated.".into());

    m.insert(
        "cmd.tmprt.sensors.src_pawnio".into(),
        "PawnIO driver".into(),
    );

    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "The PawnIO driver is not installed - CPU temps will stay '--'. Run 'tmprt sensors install' to install it (needs Administrator).".into());

    m.insert(
        "cmd.tmprt.sensors.pawnio_present".into(),
        "PawnIO driver is already installed.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_elevating".into(),
        "Elevating to install the PawnIO driver (accept the UAC prompt)...".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_downloading".into(),
        "Downloading PawnIO driver".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_installed".into(),
        "PawnIO driver installed.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_reboot".into(),
        "PawnIO driver installed - restart your PC to activate it.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_failed".into(),
        "PawnIO driver install failed: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_debug".into(),
        "  tmprt sensors debug         Dump detected hardware + sensor values (diagnostics)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_title".into(),
        "Sensor diagnostics (LibreHardwareMonitor):".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_none".into(),
        "LibreHardwareMonitor is not installed - run 'tmprt sensors install' first.".into(),
    );
}
