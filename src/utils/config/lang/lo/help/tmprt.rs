use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.tmprt.title".into(),
        "ຄຳສັ່ງ TEMPERATURE & WEATHER:".into(),
    );

    m.insert(
        "help.tmprt.com".into(),
        "tmprt com [secs] ## ຕິດຕາມອຸນຫະພູມ hardware ແບບສົດ (q=quit s=freeze c='oC/'oF)".into(),
    );

    m.insert(
        "help.tmprt.weather".into(),
        "tmprt weather [city] ## ສະພາບອາກາດປັດຈຸບັນຜ່ານ OpenWeather API".into(),
    );

    m.insert(
        "help.tmprt.key".into(),
        "tmprt key [API_KEY] ## ບັນທຶກ/ສະແດງ OpenWeather API key".into(),
    );

    m.insert(
        "help.tmprt.city".into(),
        "tmprt city [name] ## ບັນທຶກ/ສະແດງເມືອງເລີ່ມຕົ້ນສຳລັບ weather".into(),
    );

    m.insert(
        "help.tmprt.sensors".into(),
        "tmprt sensors <status|install|uninstall|debug> ## ຈັດການ sensor backends ຮາດແວ (LHM)"
            .into(),
    );

    m.insert(
        "help.tmprt.example1".into(),
        "tmprt com 5 ## ມຸມມອງສົດ refresh ທຸກໆ 5 ວິນາທີ".into(),
    );

    m.insert(
        "help.tmprt.example2".into(),
        "tmprt weather Bangkok ## ສະພາບອາກາດສຳລັບ Bangkok".into(),
    );

    m.insert(
        "help.tmprt.example3".into(),
        "tmprt key 1234567890abcdef ## ບັນທຶກ API key".into(),
    );

    m.insert(
        "help.tmprt.example4".into(),
        "tmprt city Bangkok ## ຕັ້ງເມືອງເລີ່ມຕົ້ນ".into(),
    );
}
