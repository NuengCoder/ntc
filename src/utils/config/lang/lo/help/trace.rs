use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trace.title".into(), "TRACE ຄຳສັ່ງ:".into());

    m.insert(
        "help.trace.usage".into(),
        "trace <ໂຮສ> [-m N] [-w MS] ## ຕິດຕາມເສັ້ນທາງແຕ່ລະ hop (tracert/traceroute ຫຼື TTL)".into(),
    );

    m.insert(
        "help.trace.example1".into(),
        "trace example.com ## ຕິດຕາມເສັ້ນທາງໄປຫາ example.com".into(),
    );
}
