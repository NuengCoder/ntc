use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.showcg.title".into(), "SHOWCG ຄຳສັ່ງ:".into());

    m.insert(
        "help.showcg.usage".into(),
        "showcg ## ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.showcg.example1".into(),
        "showcg ## ສະແດງພາບລວມການຕັ້ງຄ່າປັດຈຸບັນ".into(),
    );
}
