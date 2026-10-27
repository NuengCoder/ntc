use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.tmprt.unknown_sub".into(),
        "tmprt subcommand ບໍ່ຮູ້ຈັກ: {}".into(),
    );

    m.insert("cmd.tmprt.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.tmprt.usage_com".into(),
        "  tmprt com [secs]        ຕິດຕາມອຸນຫະພູມ hardware ແບບສົດ".into(),
    );

    m.insert(
        "cmd.tmprt.usage_weather".into(),
        "  tmprt weather [city]    ສະພາບອາກາດປັດຈຸບັນຜ່ານ OpenWeather API".into(),
    );

    m.insert(
        "cmd.tmprt.usage_key".into(),
        "  tmprt key [API_KEY]     ບັນທຶກ/ສະແດງ OpenWeather API key".into(),
    );

    m.insert(
        "cmd.tmprt.usage_city".into(),
        "  tmprt city [name]       ບັນທຶກ/ສະແດງເມືອງເລີ່ມຕົ້ນສຳລັບ weather".into(),
    );

    m.insert(
        "cmd.tmprt.usage_sensors".into(),
        "  tmprt sensors <sub>     ຈັດການ sensor backends ຮາດແວ".into(),
    );

    m.insert("cmd.tmprt.examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert("cmd.tmprt.example1".into(), "tmprt com".into());

    m.insert("cmd.tmprt.example2".into(), "tmprt com 5".into());

    m.insert("cmd.tmprt.example3".into(), "tmprt weather Bangkok".into());

    m.insert(
        "cmd.tmprt.example4".into(),
        "tmprt key 1234567890abcdef".into(),
    );

    m.insert("cmd.tmprt.com.no_sensors".into(), "ບໍ່ພົບເຊັນເຊີອຸນຫະພູມ.".into());

    m.insert(
        "cmd.tmprt.com.no_sensors_hint".into(),
        "ເທິງ Windows ອາດຕ້ອງການສິດ admin; ເທິງ Linux ຕ້ອງມີເຊັນເຊີ hwmon/thermal.".into(),
    );

    m.insert("cmd.tmprt.com.footer".into(), " ກົດ q ເພື່ອອອກ".into());

    m.insert(
        "cmd.tmprt.com.lhm_missing".into(),
        "ບໍ່ພົບ backend ອຸນຫະພູມ CPU (ACPI ບໍ່ expose ເຊັນເຊີເທິງເຄື່ອງນີ້).".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_prompt".into(),
        "ຕິດຕັ້ງ LibreHardwareMonitorLib (auto-downloaded, ~7 MB) ສຳລັບອຸນຫະພູມ CPU/GPU/board/storage?"
            .into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_confirm".into(),
        "ຕິດຕັ້ງຕອນນີ້ບໍ? [Y/n]".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installing".into(),
        "ກຳລັງດາວໂຫຼດ ແລະ ຕິດຕັ້ງ LibreHardwareMonitorLib...".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_installed".into(),
        "ຕິດຕັ້ງ LibreHardwareMonitorLib ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.tmprt.com.lhm_install_failed".into(),
        "ຕິດຕັ້ງ LibreHardwareMonitorLib ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert("cmd.tmprt.com.admin_hint".into(), "ອຸນຫະພູມ CPU ຕ້ອງການ PawnIO driver ແລະ ຂະບວນການ elevated. ເຮັດວຽກ 'mode admin' ເພື່ອເລີ່ມ ntc ໃໝ່ elevated.".into());

    m.insert(
        "cmd.tmprt.com.pawnio_hint".into(),
        "ອຸນຫະພູມ CPU ຕ້ອງການ PawnIO driver. ເຮັດວຽກ 'tmprt sensors install' (ຕ້ອງການ Administrator)."
            .into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key".into(),
        "ບໍ່ພົບ OpenWeather API key.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_key_hint".into(),
        "ຕັ້ງດ້ວຍ: tmprt key <API_KEY>  (ຫຼື export OPENWEATHER_API_KEY)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city".into(),
        "ບໍ່ມີເມືອງທີ່ກຳນົດ ແລະ ບໍ່ມີເມືອງເລີ່ມຕົ້ນ.".into(),
    );

    m.insert(
        "cmd.tmprt.weather.no_city_hint".into(),
        "Usage: tmprt weather <city>   ຫຼື ຕັ້ງເລີ່ມຕົ້ນ: tmprt city <name>".into(),
    );

    m.insert(
        "cmd.tmprt.weather.request_failed".into(),
        "ຄຳຂໍ weather ລົ້ມເຫຼວ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.read_failed".into(),
        "ອ່ານຜົນລັບ weather ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.bad_key".into(),
        "OpenWeather ປະຕິເສດ API key (HTTP 401).".into(),
    );

    m.insert(
        "cmd.tmprt.weather.city_not_found".into(),
        "ບໍ່ພົບເມືອງ: {}  (HTTP 404)".into(),
    );

    m.insert(
        "cmd.tmprt.weather.http_error".into(),
        "OpenWeather error (HTTP {}): {}".into(),
    );

    m.insert(
        "cmd.tmprt.weather.parse_failed".into(),
        "parse ຂໍ້ມູນ weather ບໍ່ສຳເລັດ: {}".into(),
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
        "OpenWeather API key (ຈາກ OPENWEATHER_API_KEY env): {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.status".into(),
        "OpenWeather API key: {}".into(),
    );

    m.insert(
        "cmd.tmprt.key.not_set".into(),
        "ບໍ່ມີ OpenWeather API key ທີ່ຕັ້ງຄ່າ.".into(),
    );

    m.insert(
        "cmd.tmprt.key.saved".into(),
        "ບັນທຶກ OpenWeather API key: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.status".into(),
        "Default weather city: {}".into(),
    );

    m.insert(
        "cmd.tmprt.city.not_set".into(),
        "ບໍ່ມີເມືອງເລີ່ມຕົ້ນສຳລັບ weather ທີ່ຕັ້ງຄ່າ.".into(),
    );

    m.insert(
        "cmd.tmprt.city.saved".into(),
        "ບັນທຶກເມືອງເລີ່ມຕົ້ນ: {} ແລ້ວ".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_title".into(),
        "tmprt sensors subcommands:".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_status".into(),
        "  tmprt sensors status        ສະແດງ sensor backends ທີ່ມີ".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_install".into(),
        "  tmprt sensors install       ດາວໂຫຼດ LibreHardwareMonitorLib (Windows)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_uninstall".into(),
        "  tmprt sensors uninstall     ລຶບໄຟລ໌ LibreHardwareMonitorLib ທີ່ດາວໂຫຼດ".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_title".into(),
        "sensor backends ຮາດແວ:".into(),
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

    m.insert("cmd.tmprt.sensors.src_unavailable".into(), "ບໍ່ພ້ອມໃຊ້".into());

    m.insert("cmd.tmprt.sensors.src_installed".into(), "ຕິດຕັ້ງ".into());

    m.insert(
        "cmd.tmprt.sensors.src_not_installed".into(),
        "ຍັງບໍ່ຕິດຕັ້ງ".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.status_hint".into(),
        "ເຮັດວຽກ 'tmprt sensors install' ເພື່ອເປີດໃຊ້ sensors ທີ່ອີງໃສ່ LibreHardwareMonitor (Windows)."
            .into(),
    );

    m.insert(
        "cmd.tmprt.sensors.installed".into(),
        "ຕິດຕັ້ງ LibreHardwareMonitorLib ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.install_failed".into(),
        "ຕິດຕັ້ງ LibreHardwareMonitorLib ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstalled".into(),
        "ລຶບໄຟລ໌ LibreHardwareMonitorLib ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.uninstall_failed".into(),
        "ລຶບໄຟລ໌ LibreHardwareMonitorLib ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.not_installed".into(),
        "LibreHardwareMonitorLib ຍັງບໍ່ໄດ້ຕິດຕັ້ງ.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.windows_only".into(),
        "LibreHardwareMonitor ຮອງຮັບພຽງ Windows.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.downloading".into(),
        "ກຳລັງດາວໂຫຼດ LibreHardwareMonitorLib".into(),
    );

    m.insert("cmd.tmprt.sensors.elevation".into(), "Elevation".into());

    m.insert("cmd.tmprt.sensors.admin_yes".into(), "Administrator".into());

    m.insert("cmd.tmprt.sensors.admin_no".into(), "standard user".into());

    m.insert("cmd.tmprt.sensors.need_admin".into(), "ອຸນຫະພູມ CPU ຜ່ານ LHM ຕ້ອງການຂະບວນການ elevated (PawnIO driver). ເຮັດວຽກ 'mode admin' ເພື່ອເລີ່ມ ntc ໃໝ່ elevated.".into());

    m.insert(
        "cmd.tmprt.sensors.src_pawnio".into(),
        "PawnIO driver".into(),
    );

    m.insert("cmd.tmprt.sensors.pawnio_missing".into(), "PawnIO driver ຍັງບໍ່ໄດ້ຕິດຕັ້ງ - ອຸນຫະພູມ CPU ຈະຢູ່ '--'. ເຮັດວຽກ 'tmprt sensors install' ເພື່ອຕິດຕັ້ງ (ຕ້ອງການ Administrator).".into());

    m.insert(
        "cmd.tmprt.sensors.pawnio_present".into(),
        "PawnIO driver ຕິດຕັ້ງແລ້ວ.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_elevating".into(),
        "ກຳລັງ elevate ເພື່ອຕິດຕັ້ງ PawnIO driver (ຍອມຮັບ UAC prompt)...".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_downloading".into(),
        "ກຳລັງດາວໂຫຼດ PawnIO driver".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_installed".into(),
        "ຕິດຕັ້ງ PawnIO driver ແລ້ວ.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_reboot".into(),
        "ຕິດຕັ້ງ PawnIO driver ແລ້ວ - restart PC ຂອງທ່ານເພື່ອເປີດໃຊ້.".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.pawnio_failed".into(),
        "ຕິດຕັ້ງ PawnIO driver ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.help_debug".into(),
        "  tmprt sensors debug         dump ຮາດແວທີ່ກວດພົບ + sensor values (diagnostics)".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_title".into(),
        "Sensor diagnostics (LibreHardwareMonitor):".into(),
    );

    m.insert(
        "cmd.tmprt.sensors.debug_none".into(),
        "LibreHardwareMonitor ຍັງບໍ່ໄດ້ຕິດຕັ້ງ - ເຮັດວຽກ 'tmprt sensors install' ກ່ອນ.".into(),
    );
}
