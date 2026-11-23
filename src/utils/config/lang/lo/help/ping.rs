use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ping.title".into(), "PING ຄຳສັ່ງ:".into());

    m.insert(
        "help.ping.usage".into(),
        "ping <host> ## ICMP ping (-c ຈຳນວນ)".into(),
    );

    m.insert(
        "help.ping.flags".into(),
        "ping <ໂຮສ> [-c N] [--forever] [-w MS] [-s ໄບຕ໌] [-t TTL] [-4|-6] ## ping ພ້ອມຕົວເລືອກ".into(),
    );

    m.insert(
        "help.ping.example1".into(),
        "ping google.com ## ping ໄປຫາ google.com".into(),
    );
}
